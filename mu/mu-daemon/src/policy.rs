//! P0-7: policy endpoint for Composer (`MU_POLICY_SOCK`).
//!
//! Framing: the same `len:u32 LE ‖ body` as `composer-core::client`.
//! A proposal is decoded, validated against Ω, checked against the current Δ
//! (TOCTOU), rendered as a full diff and only then handed to the main loop,
//! which writes the journal, the `policy.toml` cache and μ. A refusal leaves Δ
//! untouched and answers `rejected: human_denied`.

use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};

use composer_core::client::{read_frame, write_frame, FRAME_MAX};
use composer_core::endpoint::{handle_propose, DaemonPolicy, ProposeOutcome, RejectCode};
use mu_common::Hash32;
use mu_policy::Delta;

/// The main loop's verdict, sent back to the waiting socket.
pub enum Answer {
    Applied { version: u16, new_hash: Hash32 },
    Rejected(String),
}

pub struct Request {
    pub delta: Delta,
    pub reply: Sender<Answer>,
}

pub fn hex(h: &Hash32) -> String {
    h.0.iter().map(|b| format!("{b:02x}")).collect()
}

/// One line per field that changes, addresses and rules included: the owner
/// must see exactly what the journal will record.
pub fn diff(old: &Delta, new: &Delta) -> String {
    let mut out = String::new();
    if old.daily_limit != new.daily_limit {
        out.push_str(&format!(
            "  daily_limit: {} -> {}\n",
            old.daily_limit.minor(),
            new.daily_limit.minor()
        ));
    }
    if old.confirm_threshold != new.confirm_threshold {
        out.push_str(&format!(
            "  confirm_threshold: {} -> {}\n",
            old.confirm_threshold.minor(),
            new.confirm_threshold.minor()
        ));
    }
    for w in &new.whitelist {
        if !old.whitelist.iter().any(|o| o.address == w.address) {
            out.push_str(&format!("  + whitelist 0x{} ({})\n", w.address.to_hex(), w.label));
        }
    }
    for w in &old.whitelist {
        if !new.whitelist.iter().any(|n| n.address == w.address) {
            out.push_str(&format!("  - whitelist 0x{} ({})\n", w.address.to_hex(), w.label));
        }
    }
    for r in &new.resource_allowlist {
        let same = old
            .resource_allowlist
            .iter()
            .find(|o| o.host == r.host && o.path_prefix == r.path_prefix);
        match same {
            None => out.push_str(&format!(
                "  + resource {}{} up to {}\n",
                r.host,
                r.path_prefix,
                r.max_price_per_call.minor()
            )),
            Some(o) if o.max_price_per_call != r.max_price_per_call => out.push_str(&format!(
                "  ~ resource {}{} price {} -> {}\n",
                r.host,
                r.path_prefix,
                o.max_price_per_call.minor(),
                r.max_price_per_call.minor()
            )),
            Some(_) => {}
        }
    }
    for o in &old.resource_allowlist {
        if !new
            .resource_allowlist
            .iter()
            .any(|r| r.host == o.host && r.path_prefix == o.path_prefix)
        {
            out.push_str(&format!("  - resource {}{}\n", o.host, o.path_prefix));
        }
    }
    if out.is_empty() {
        out.push_str("  (no change)\n");
    }
    out
}

fn ask_owner(shown: &str) -> bool {
    eprint!("policy change requested:\n{shown}apply? [y/N] ");
    let _ = std::io::stderr().flush();
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).is_err() {
        return false;
    }
    matches!(line.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

/// Reads one proposal, asks the owner, then waits for the main loop to commit.
fn handle_stream(
    mut stream: UnixStream,
    state: &mut DaemonPolicy,
    tx: &Sender<Request>,
    ans_tx: Sender<Answer>,
    ans_rx: &Receiver<Answer>,
) {
    let raw = match read_frame(&mut stream) {
        Ok(r) if r.len() <= FRAME_MAX => r,
        _ => {
            let _ = write_frame(&mut stream, b"rejected: malformed");
            return;
        }
    };

    let before = state.current.clone();
    let mut approved: Option<Delta> = None;
    let outcome = handle_propose(state, &raw, |_old, new| {
        if ask_owner(&diff(&before, new)) {
            approved = Some(new.clone());
            true
        } else {
            false
        }
    });

    let text = match outcome {
        ProposeOutcome::Applied { .. } => match approved {
            None => "rejected: internal".to_string(),
            Some(delta) => {
                if tx.send(Request { delta, reply: ans_tx }).is_err() {
                    state.current = before;
                    "rejected: internal".to_string()
                } else {
                    match ans_rx.recv() {
                        Ok(Answer::Applied { version, new_hash }) => {
                            format!("applied v{version} {}", hex(&new_hash))
                        }
                        Ok(Answer::Rejected(why)) => {
                            // Nothing was committed: the thread's copy must not drift.
                            state.current = before;
                            format!("rejected: {why}")
                        }
                        Err(_) => {
                            state.current = before;
                            "rejected: internal".to_string()
                        }
                    }
                }
            }
        },
        ProposeOutcome::Rejected(RejectCode::HumanDenied) => "rejected: human_denied".to_string(),
        ProposeOutcome::Rejected(RejectCode::Stale) => "rejected: stale".to_string(),
        ProposeOutcome::Rejected(RejectCode::TooLarge) => "rejected: too_large".to_string(),
        ProposeOutcome::Rejected(RejectCode::Malformed) => "rejected: malformed".to_string(),
        ProposeOutcome::Rejected(RejectCode::Invalid(e)) => format!("rejected: invalid {e:?}"),
        ProposeOutcome::Rejected(RejectCode::Internal) => "rejected: internal".to_string(),
    };
    let _ = write_frame(&mut stream, text.as_bytes());
}

/// Runs the policy socket in a background thread.
pub fn spawn(sock_path: PathBuf, mut state: DaemonPolicy, tx: Sender<Request>) -> std::io::Result<()> {
    let _ = std::fs::remove_file(&sock_path);
    let listener = UnixListener::bind(&sock_path)?;
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&sock_path, std::fs::Permissions::from_mode(0o600))?;
    }
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let (ans_tx, ans_rx) = std::sync::mpsc::channel();
            handle_stream(stream, &mut state, &tx, ans_tx, &ans_rx);
        }
    });
    Ok(())
}

/// The socket path the Composer should be pointed at (`MU_POLICY_SOCK`).
pub fn sock_path(home: &Path) -> PathBuf {
    home.join("mu-policy.sock")
}

#[cfg(test)]
mod tests {
    use super::*;
    use mu_common::{Amount, CanonAddress};
    use mu_policy::ResourceRule;

    fn addr(a: &str) -> CanonAddress {
        CanonAddress::canon(a, 8453).unwrap()
    }

    fn base() -> Delta {
        Delta {
            daily_limit: Amount::from_minor(5_000_000),
            whitelist: vec![mu_policy::WlEntry { address: addr("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"), label: "issuer".into() }],
            confirm_threshold: Amount::ZERO,
            resource_allowlist: vec![ResourceRule {
                host: "issuer.example".into(),
                path_prefix: "/bare".into(),
                max_price_per_call: Amount::from_minor(1_000_000),
            }],
        }
    }

    #[test]
    fn the_diff_names_every_added_and_removed_address() {
        let mut new = base();
        new.whitelist.push(mu_policy::WlEntry {
            address: addr("0x1111111111111111111111111111111111111111"),
            label: "new shop".into(),
        });
        let shown = diff(&base(), &new);
        assert!(shown.contains("+ whitelist 0x1111111111111111111111111111111111111111 (new shop)"), "{shown}");
        assert!(!shown.contains("- whitelist"), "nothing was removed: {shown}");
    }

    #[test]
    fn the_diff_shows_removals_and_price_changes() {
        let mut new = base();
        new.whitelist.clear();
        new.resource_allowlist[0].max_price_per_call = Amount::from_minor(2_000_000);
        let shown = diff(&base(), &new);
        assert!(shown.contains("- whitelist 0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed (issuer)"), "{shown}");
        assert!(shown.contains("~ resource issuer.example/bare price 1000000 -> 2000000"), "{shown}");
    }

    #[test]
    fn the_diff_shows_limits_and_new_rules() {
        let mut new = base();
        new.daily_limit = Amount::from_minor(7_000_000);
        new.confirm_threshold = Amount::from_minor(100_000);
        new.resource_allowlist.push(ResourceRule {
            host: "api.example".into(),
            path_prefix: "/v1".into(),
            max_price_per_call: Amount::from_minor(500),
        });
        let shown = diff(&base(), &new);
        assert!(shown.contains("daily_limit: 5000000 -> 7000000"), "{shown}");
        assert!(shown.contains("confirm_threshold: 0 -> 100000"), "{shown}");
        assert!(shown.contains("+ resource api.example/v1 up to 500"), "{shown}");
    }

    #[test]
    fn the_socket_path_sits_in_the_home() {
        assert_eq!(
            sock_path(Path::new("/tmp/x")),
            PathBuf::from("/tmp/x/mu-policy.sock")
        );
    }
}
