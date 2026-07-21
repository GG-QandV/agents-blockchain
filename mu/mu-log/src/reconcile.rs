//! RISK-M5-3: tabular resolution for Pending during recovery.
//! default = Keep. There is NO "else → Failed" branch in code.
//! Sui semantics for NonceState:
//! - ConsumedByOther: coin object spent by another tx (version increased), digest not found
//! - NotReached: object version unchanged (tx could not have executed)
//! - Unknown: could not determine
use mu_connect::crypto::RpcReceipt;

/// Responses from both nodes by tx_hash + account nonce state relative to the entry's chain_nonce.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonceState {
    /// account nonce > entry's chain_nonce AND no receipt for our tx → spent by ANOTHER transaction
    ConsumedByOther,
    /// account nonce <= entry's chain_nonce → our tx could not have executed
    NotReached,
    /// could not reliably determine
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    ToSettled { block: u64, gas: u128 },
    ToFailed,
    Keep,
}

/// Pure resolving function (RISK-M5-3). Full input coverage is verified by tests.
pub fn resolve(rc1: &RpcReceipt, rc2: &RpcReceipt, nonce: NonceState) -> Resolution {
    use RpcReceipt::*;
    match (rc1, rc2) {
        // both nodes see success in the same block → Settled
        (Success { block: b1, gas_used }, Success { block: b2, .. }) if b1 == b2 => {
            Resolution::ToSettled { block: *b1, gas: *gas_used }
        }
        // both see revert → executed and failed on-chain: close as Failed
        (Reverted { .. }, Reverted { .. }) => Resolution::ToFailed,
        // no receipt at BOTH nodes AND nonce provably consumed by another tx → Failed
        (None, None) if nonce == NonceState::ConsumedByOther => Resolution::ToFailed,
        // no receipt at both AND nonce not reached → tx not on-chain → Failed safe?
        // NO: tx may be in mempool. Failed only with ConsumedByOther. Otherwise Keep.
        _ => Resolution::Keep,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use RpcReceipt::*;

    fn all_receipts() -> Vec<RpcReceipt> {
        vec![
            None,
            Success { block: 10, gas_used: 21000 },
            Reverted { block: 10 },
            Unreachable,
        ]
    }
    fn all_nonce() -> [NonceState; 3] {
        [NonceState::ConsumedByOther, NonceState::NotReached, NonceState::Unknown]
    }

    /// Full enumeration 4×4×3 = 48 combinations: verify strict properties.
    #[test]
    fn exhaustive_table_properties() {
        for r1 in all_receipts() {
            for r2 in all_receipts() {
                for n in all_nonce() {
                    let res = resolve(&r1, &r2, n);
                    // Property 1: ToSettled only when both agree Success+Success (RISK-M7-3)
                    if let Resolution::ToSettled { .. } = res {
                        assert!(matches!((&r1, &r2), (Success { .. }, Success { .. })));
                    }
                    // Property 2: ToFailed only with (Reverted,Reverted) or (None,None,ConsumedByOther)
                    if res == Resolution::ToFailed {
                        let legal = matches!((&r1, &r2), (Reverted { .. }, Reverted { .. }))
                            || (matches!((&r1, &r2), (None, None)) && n == NonceState::ConsumedByOther);
                        assert!(legal, "illegal ToFailed for {r1:?},{r2:?},{n:?}");
                    }
                    // Property 3: Unreachable in any position NEVER yields ToFailed
                    if matches!(r1, Unreachable) || matches!(r2, Unreachable) {
                        assert_ne!(res, Resolution::ToFailed);
                    }
                }
            }
        }
    }

    #[test]
    fn mempool_tx_is_kept() {
        // no receipt, nonce not reached → tx may be in mempool → Keep, not Failed
        assert_eq!(resolve(&None, &None, NonceState::NotReached), Resolution::Keep);
    }
    #[test]
    fn divergent_receipts_kept() {
        let s = Success { block: 1, gas_used: 1 };
        assert_eq!(resolve(&s, &None, NonceState::Unknown), Resolution::Keep);
    }
    #[test]
    fn consumed_by_other_closes_failed() {
        assert_eq!(resolve(&None, &None, NonceState::ConsumedByOther), Resolution::ToFailed);
    }
}
