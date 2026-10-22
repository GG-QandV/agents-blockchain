//! P0-7: policy endpoint for Composer (`MU_POLICY_SOCK`).
//!
//! Framing: the same `len:u32 LE ‖ body` as `composer-core::client`.
//! A proposal is decoded, validated with the same `mu-policy` as Composer,
//! checked against the current Δ (TOCTOU), rendered as a diff and applied only
//! after the owner confirms. The answer is a text frame the Composer prints.

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

use composer_core::client::{read_frame, write_frame, FRAME_MAX};
use composer_core::endpoint::{handle_propose, DaemonPolicy, ProposeOutcome, RejectCode};
use mu_common::Hash32;
use mu_policy::Delta;

/// What the main loop applies after the owner agreed.
pub struct Applied {
    pub old_hash: Hash32,
    pub new_hash: Hash32,
    pub delta: Delta,
}

/// Reads one proposal from the socket and answers with a text frame.
/// `approve` renders the diff and asks the owner (stdin in the daemon).
fn handle_stream(
    mut stream: UnixStream,
    state: &mut DaemonPolicy,
    approve: &mut dyn FnMut(&Delta, &Delta) -> bool,
) -> Option<ProposeOutcome> {
    let raw = match read_frame(&mut stream) {
        Ok(r) => r,
        Err(_) => {
            let _ = write_frame(&mut stream, b"rejected: malformed");
            return None;
        }
    };
    if raw.len() > FRAME_MAX {
        let _ = write_frame(&mut stream, b"rejected: too_large");
        return None;
    }
    let outcome = handle_propose(state, &raw, |a, b| approve(a, b));
    let text = match &outcome {
        ProposeOutcome::Applied { new_hash } => format!("applied {}", hex(new_hash)),
        ProposeOutcome::Rejected(RejectCode::Stale) => "rejected: stale".to_string(),
        ProposeOutcome::Rejected(RejectCode::HumanDenied) => "rejected: human_denied".to_string(),
        ProposeOutcome::Rejected(RejectCode::TooLarge) => "rejected: too_large".to_string(),
        ProposeOutcome::Rejected(RejectCode::Malformed) => "rejected: malformed".to_string(),
        ProposeOutcome::Rejected(RejectCode::Invalid(errs)) => format!("rejected: invalid {errs:?}"),
        ProposeOutcome::Rejected(RejectCode::Internal) => "rejected: internal".to_string(),
    };
    let _ = write_frame(&mut stream, text.as_bytes());

    match outcome {
        ProposeOutcome::Applied { new_hash } => Some(ProposeOutcome::Applied { new_hash }),
        _ => None,
    }
}

pub fn hex(h: &Hash32) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect()
}

/// Runs the policy socket in a background thread. Every applied Δ is sent to
/// the main loop, which writes `DeltaChanged` and swaps the runtime policy.
pub fn spawn(
    sock_path: PathBuf,
    mut state: DaemonPolicy,
    tx: Sender<Applied>,
) -> std::io::Result<()> {
    let _ = std::fs::remove_file(&sock_path);
    let listener = UnixListener::bind(&sock_path)?;
    std::fs::set_permissions(&sock_path, std::os::unix::fs::PermissionsExt::from_mode(0o600))?;
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let old = state.current.clone();
            let mut ask = |a: &Delta, b: &Delta| ask_owner(a, b);
            if let Some(ProposeOutcome::Applied { new_hash }) =
                handle_stream(stream, &mut state, &mut ask)
            {
                let old_hash = mu_policy::delta_hash(&old);
                let _ = tx.send(Applied { old_hash, new_hash, delta: state.current.clone() });
            }
        }
    });
    Ok(())
}

/// Shows the diff and asks the owner on stdin. Anything but an explicit yes
/// leaves Δ unchanged.
fn ask_owner(old: &Delta, new: &Delta) -> bool {
    eprintln!("policy change requested:");
    if old.daily_limit != new.daily_limit {
        eprintln!("  daily_limit: {} -> {}", old.daily_limit.minor(), new.daily_limit.minor());
    }
    if old.confirm_threshold != new.confirm_threshold {
        eprintln!(
            "  confirm_threshold: {} -> {}",
            old.confirm_threshold.minor(),
            new.confirm_threshold.minor()
        );
    }
    if old.whitelist.len() != new.whitelist.len() {
        eprintln!("  whitelist: {} -> {} entries", old.whitelist.len(), new.whitelist.len());
    }
    eprintln!("apply? [y/N]");
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

/// The socket path the Composer should be pointed at (`MU_POLICY_SOCK`).
pub fn sock_path(home: &Path) -> PathBuf {
    home.join("mu-policy.sock")
}

#[cfg(test)]
mod tests {
    use super::*;
    use composer_core::proposal::{encode_proposal, DeltaProposal};
    use mu_common::Amount;
    use mu_policy::types::WlEntry;
    use mu_policy::{Delta, OmegaView};

    fn policy(daily: u64) -> Delta {
        Delta {
            daily_limit: Amount::from_minor(daily.into()),
            whitelist: vec![],
            confirm_threshold: Amount::from_minor(0),
            resource_allowlist: vec![],
        }
    }

    fn state(daily: u64) -> DaemonPolicy {
        DaemonPolicy {
            current: policy(daily),
            omega: OmegaView { max_ceiling: Amount::from_minor(10_000_000_000), supported_chain_ids: vec![8453] },
        }
    }

    fn proposal(base: &Delta, daily: u64) -> Vec<u8> {
        encode_proposal(&DeltaProposal {
            base_delta_hash: mu_policy::delta_hash(base),
            ts: 0,
            new_delta: {
                let mut d = base.clone();
                d.daily_limit = Amount::from_minor(daily.into());
                d
            },
        })
        .unwrap()
    }

    #[test]
    fn an_approved_limit_change_is_applied() {
        let mut s = state(5_000_000);
        let raw = proposal(&s.current, 7_000_000);
        let out = handle_propose(&mut s, &raw, |_, _| true);
        assert!(matches!(out, ProposeOutcome::Applied { .. }));
        assert_eq!(s.current.daily_limit.minor(), 7_000_000);
    }

    #[test]
    fn a_denied_change_leaves_the_policy_untouched() {
        let mut s = state(5_000_000);
        let before = mu_policy::delta_hash(&s.current);
        let raw = proposal(&s.current, 7_000_000);
        let out = handle_propose(&mut s, &raw, |_, _| false);
        assert_eq!(out, ProposeOutcome::Rejected(RejectCode::HumanDenied));
        assert_eq!(s.current.daily_limit.minor(), 5_000_000);
        assert_eq!(mu_policy::delta_hash(&s.current), before);
    }

    #[test]
    fn a_proposal_built_on_a_stale_policy_is_refused() {
        let mut s = state(5_000_000);
        let stale = policy(4_000_000);
        let raw = proposal(&stale, 7_000_000);
        let out = handle_propose(&mut s, &raw, |_, _| true);
        assert_eq!(out, ProposeOutcome::Rejected(RejectCode::Stale));
        assert_eq!(s.current.daily_limit.minor(), 5_000_000);
    }

    #[test]
    fn a_policy_over_the_ceiling_is_refused_by_validation() {
        let mut s = state(5_000_000);
        let raw = proposal(&s.current, 20_000_000_000);
        let out = handle_propose(&mut s, &raw, |_, _| true);
        assert!(matches!(out, ProposeOutcome::Rejected(RejectCode::Invalid(_))));
        assert_eq!(s.current.daily_limit.minor(), 5_000_000);
    }

    #[test]
    fn the_socket_path_sits_in_the_home() {
        assert_eq!(
            sock_path(std::path::Path::new("/tmp/x")),
            std::path::PathBuf::from("/tmp/x/mu-policy.sock")
        );
    }

    // WlEntry is used to keep the import meaningful for future whitelist tests.
    #[allow(dead_code)]
    fn _wl() -> WlEntry {
        WlEntry { address: mu_common::CanonAddress::canon("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed", 8453).unwrap(), label: "x".into() }
    }
}
