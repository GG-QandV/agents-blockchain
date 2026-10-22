//! P0-5: reconcile open reserves at start.
//!
//! - `PendingAuth` (x402): ask the connector about `(nonce, valid_before)`.
//!   Settled → `Settled`; Failed → `Failed`; still pending past the deadline
//!   (with a clock skew) → `Failed` (the authorization can no longer settle).
//! - legacy `Pending` with `chain_nonce == 0`: no on-chain key exists, so the
//!   reserve is closed by a fixed TTL, with an `Alert` (see changelog).
//! - divergence or an unavailable node keeps the reserve and writes `Alert`.

use mu_connect::{Connector, TxRef, TxStatus};
use mu_log::{Kind, Log};
use mu_vault::Vault;

/// Clock skew allowance when comparing `valid_before` with the local clock.
pub const SKEW_SECS: u64 = 60;
/// TTL for legacy `Pending` records without a chain key.
pub const LEGACY_TTL_SECS: u64 = 3600 + SKEW_SECS;

/// One alert per (code, intent). The journal is evidence, not a log: the same
/// unresolved reserve seen on the next start is reported on stderr instead.
fn alert_once(log: &mut Log, vault: &dyn Vault, code: u16, intent_hash: mu_common::Hash32, now: u64) {
    let seen = log
        .entries()
        .iter()
        .any(|e| matches!(&e.kind, Kind::AlertAbout { code: c, intent_hash: h } if *c == code && *h == intent_hash));
    if seen {
        let h = intent_hash.0.iter().map(|b| format!("{b:02x}")).collect::<String>();
        eprintln!("reconcile: alert 0x{code:04X} for 0x{h} still stands");
        return;
    }
    let _ = log.append(Kind::AlertAbout { code, intent_hash }, now, vault);
}

pub fn reconcile(log: &mut Log, connector: &dyn Connector, vault: &dyn Vault, now: u64) {
    let open: Vec<(u64, u64, Kind)> = log
        .pending()
        .into_iter()
        .map(|e| (e.seq, e.ts, e.kind.clone()))
        .collect();

    for (_seq, ts, kind) in open {
        match kind {
            Kind::PendingAuth { intent_hash, nonce, valid_before, .. } => {
                let r = TxRef::Authorization { nonce, valid_before };
                match connector.status(&r) {
                    Ok(TxStatus::Settled { effective_gas, .. }) => {
                        // No tx hash in `TxStatus`; the nonce is the on-chain key.
                        let _ = log.append(
                            Kind::Settled { intent_hash, tx_hash: nonce, effective_gas },
                            now,
                            vault,
                        );
                    }
                    Ok(TxStatus::Failed { .. }) => {
                        let _ = log.append(Kind::Failed { intent_hash }, now, vault);
                    }
                    Ok(TxStatus::Pending) => {
                        if now > valid_before.saturating_add(SKEW_SECS) {
                            let _ = log.append(Kind::Failed { intent_hash }, now, vault);
                        }
                    }
                    Err(_) => {
                        // Nodes diverge or are unavailable: keep the reserve.
                        // The journal records the transition once; a restart
                        // only reports it on stderr.
                        alert_once(log, vault, 0x0A02, intent_hash, now);
                    }
                }
            }
            Kind::Pending { intent_hash, chain_nonce, .. } => {
                if chain_nonce == 0 && now > ts.saturating_add(LEGACY_TTL_SECS) {
                    // Legacy x402 reserve with no on-chain key: close by TTL only.
                    alert_once(log, vault, 0x0A01, intent_hash, now);
                    let _ = log.append(Kind::Failed { intent_hash }, now, vault);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mu_common::Amount;
    use mu_connect::{ConnErr, Fee, Intent, TxRef, TxStatus};
    use mu_common::Hash32;
    use mu_vault::backend::SoftVault;

    #[derive(Clone, Copy)]
    enum Out { Pending, Settled, Unknown, Diverge }

    struct Fake {
        out: Out,
        pre: bool,
    }

    impl Connector for Fake {
        fn quote(&self, _i: &Intent) -> Result<Fee, ConnErr> {
            Ok(Fee { gas_estimate: Amount::ZERO })
        }
        fn execute(&self, _i: &Intent, _s: mu_vault::TxSigner) -> Result<TxRef, ConnErr> {
            Err(ConnErr::Config("unused".into()))
        }
        fn status(&self, _r: &TxRef) -> Result<TxStatus, ConnErr> {
            match self.out {
                Out::Pending => Ok(TxStatus::Pending),
                Out::Settled => Ok(TxStatus::Settled { block: 1, effective_gas: 0 }),
                Out::Unknown => Err(ConnErr::Unknown("both RPC nodes are unavailable".into())),
                Out::Diverge => Err(ConnErr::Unknown(
                    "two RPC nodes disagree on authorizationState".into(),
                )),
            }
        }
        fn pre_auth(&self) -> bool {
            self.pre
        }
    }

    fn vault() -> SoftVault {
        SoftVault::for_test([1; 32], [2; 32], [3; 32])
    }

    fn open_log(dir: &std::path::Path, v: &SoftVault) -> Log {
        Log::open(&dir.join("mu.log"), v).expect("log")
    }

    fn append_auth(log: &mut Log, v: &SoftVault, nonce: [u8; 32], valid_before: u64, ts: u64) {
        log.append(
            Kind::PendingAuth {
                intent_hash: Hash32([7; 32]),
                amount_total: 100_000,
                nonce,
                valid_after: valid_before.saturating_sub(360),
                valid_before,
            },
            ts,
            v,
        )
        .unwrap();
    }

    fn closed(log: &Log, want: &Kind) -> bool {
        log.entries().iter().any(|e| &e.kind == want)
    }

    fn alerts(log: &Log, code: u16, ih: u8) -> usize {
        log.entries()
            .iter()
            .filter(|e| matches!(&e.kind, Kind::AlertAbout { code: c, intent_hash: h } if *c == code && h.0 == [ih; 32]))
            .count()
    }

    #[test]
    fn an_expired_authorization_is_closed_failed_after_restart() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = open_log(dir.path(), &v);
        append_auth(&mut log, &v, [9; 32], 1_000, 900);
        let c = Fake { out: Out::Pending, pre: true };
        reconcile(&mut log, &c, &v, 1_000 + SKEW_SECS + 1);
        assert!(closed(&log, &Kind::Failed { intent_hash: Hash32([7; 32]) }));
        assert!(log.pending().is_empty(), "the reserve is released");
    }

    #[test]
    fn a_used_nonce_is_settled() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = open_log(dir.path(), &v);
        append_auth(&mut log, &v, [9; 32], 10_000, 900);
        let c = Fake { out: Out::Settled, pre: true };
        reconcile(&mut log, &c, &v, 1_000);
        assert!(closed(&log, &Kind::Settled { intent_hash: Hash32([7; 32]), tx_hash: [9; 32], effective_gas: 0 }));
    }

    #[test]
    fn an_authorization_inside_its_window_keeps_the_reserve() {
        // Unused nonce, the window is still open: nothing is closed and no
        // alert is written — the payment may still settle.
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = open_log(dir.path(), &v);
        append_auth(&mut log, &v, [9; 32], 10_000, 900);
        let before = log.entries().len();
        let c = Fake { out: Out::Pending, pre: true };
        reconcile(&mut log, &c, &v, 1_000);
        assert_eq!(log.pending().len(), 1, "the reserve is kept");
        assert_eq!(log.entries().len(), before, "no record is written");
        assert_eq!(alerts(&log, 0x0A02, 7), 0);
    }

    #[test]
    fn diverging_nodes_keep_the_reserve_and_alert() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = open_log(dir.path(), &v);
        append_auth(&mut log, &v, [9; 32], 10_000, 900);
        let c = Fake { out: Out::Diverge, pre: true };
        reconcile(&mut log, &c, &v, 1_000);
        assert_eq!(log.pending().len(), 1, "no guess about the money");
        assert_eq!(alerts(&log, 0x0A02, 7), 1);
        assert!(!closed(&log, &Kind::Failed { intent_hash: Hash32([7; 32]) }));
    }

    #[test]
    fn an_unavailable_node_keeps_the_reserve_and_alerts() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = open_log(dir.path(), &v);
        append_auth(&mut log, &v, [9; 32], 10_000, 900);
        let c = Fake { out: Out::Unknown, pre: true };
        reconcile(&mut log, &c, &v, 1_000);
        assert_eq!(log.pending().len(), 1);
        assert_eq!(alerts(&log, 0x0A02, 7), 1);
    }

    #[test]
    fn a_restart_does_not_repeat_the_alert_in_the_journal() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = open_log(dir.path(), &v);
        append_auth(&mut log, &v, [9; 32], 10_000, 900);
        let c = Fake { out: Out::Diverge, pre: true };
        reconcile(&mut log, &c, &v, 1_000);
        assert_eq!(alerts(&log, 0x0A02, 7), 1);
        let after_first = log.entries().len();
        // Two more restarts: the reserve is still open, the journal stays put.
        reconcile(&mut log, &c, &v, 1_100);
        reconcile(&mut log, &c, &v, 1_200);
        assert_eq!(alerts(&log, 0x0A02, 7), 1, "one alert per transition");
        assert_eq!(log.entries().len(), after_first, "restarts write nothing");
        assert_eq!(log.pending().len(), 1);
    }

    #[test]
    fn a_legacy_pending_without_a_chain_key_is_closed_by_ttl_with_an_alert() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = open_log(dir.path(), &v);
        log.append(
            Kind::Pending { intent_hash: Hash32([8; 32]), amount_total: 5, chain_nonce: 0 },
            900,
            &v,
        )
        .unwrap();
        let c = Fake { out: Out::Pending, pre: false };
        reconcile(&mut log, &c, &v, 900 + LEGACY_TTL_SECS + 1);
        assert_eq!(alerts(&log, 0x0A01, 8), 1);
        assert!(closed(&log, &Kind::Failed { intent_hash: Hash32([8; 32]) }));
    }
}
