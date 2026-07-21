//! M7a-sui — Sui connector (gasless stablecoin transfers, mainnet/testnet).
//!
//! ALL INVARIANTS ARE FULLY INHERITED FROM THE EVM VERSION:
//! RISK-M7-1: Unknown ≠ Failed — execute cannot return Failed by type.
//! RISK-M7-3: Settled only upon consensus of both RPCs (checkpoint + status).
//! RISK-M7-5: recipient is non-spoofable — self-check: decode assembled tx (dry-run of both nodes)
//!            verifies recipient+amount against intent BEFORE signing.
//!
//! Sui specifics:
//! - Idempotency: transaction digest is deterministic from bytes+signature; re-sending
//!   the same bytes is safe (node responds "already executed" → AlreadyKnown).
//! - Equivocation (owned-object locked by a concurrent tx until end of epoch) — this is
//!   UNKNOWN outcome → strictly Unknown, never Rejected (analog of RISK-M7-2).
//! - Signature: intent [scope=0,version=0,app=0] ‖ tx_bytes → blake2b-256 → Ed25519;
//!   signature serialization: flag(0x00) ‖ sig(64) ‖ pubkey(32) — stable Sui scheme.
//! - Tx build: server-side via RPC (tx_bytes b64) — endpoint is configurable by string,
//!   because the method name for gasless Address-Balances transfers must be verified by the agent
//!   against official documentation (post-cutoff feature; see ETAP2-instruction, sources).
use crate::{ConnErr, Fee, Intent, RejectReason, TxRef, TxStatus};
use blake2::{digest::consts::U32, Blake2b, Digest};
use mu_common::Amount;
use mu_vault::TxSignerP256;

type Blake2b256 = Blake2b<U32>;

/// Sui Intent prefix for TransactionData: scope=0, version=0, app_id=0.
pub const SUI_INTENT: [u8; 3] = [0, 0, 0];
/// Secp256r1 (P-256) signature scheme flag in Sui — native curve for enclave (RISK-M4-1 reinforced).
pub const SECP256R1_FLAG: u8 = 0x02;

/// Digest for signing: blake2b256(intent ‖ tx_bytes).
pub fn signing_digest(tx_bytes: &[u8]) -> [u8; 32] {
    let mut h = Blake2b256::new();
    h.update(SUI_INTENT);
    h.update(tx_bytes);
    let out = h.finalize();
    let mut d = [0u8; 32];
    d.copy_from_slice(&out);
    d
}

/// Serialized Sui signature: flag ‖ sig(64) ‖ pubkey(33, compressed) = 98 bytes.
pub fn serialize_signature(sig: &[u8; 64], pubkey: &[u8; 33]) -> Vec<u8> {
    let mut out = Vec::with_capacity(98);
    out.push(SECP256R1_FLAG);
    out.extend_from_slice(sig);
    out.extend_from_slice(pubkey);
    out
}

/// Sui wallet address = blake2b256(flag ‖ pubkey). Reference check: sui keytool (README-LIVE §3).
pub fn sui_address_from_pubkey(pubkey: &[u8; 33]) -> [u8; 32] {
    let mut h = Blake2b256::new();
    h.update([SECP256R1_FLAG]);
    h.update(pubkey);
    let out = h.finalize();
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    a
}

// ── RPC abstraction (implementations: json_rpc.rs for network, mocks in tests) ──────
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuiltTx {
    pub tx_bytes: Vec<u8>, // BCS TransactionData (raw, decoded from node's b64 response)
}

/// Dry-run result: the node decodes tx and returns the effect — used as
/// self-check for recipient/amount (RISK-M7-5) and preview of revert.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DryRun {
    Ok { recipient: [u8; 32], amount: u128 },
    WouldFail(String),
    Unreachable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuiSend {
    Accepted { digest: [u8; 32] },
    AlreadyExecuted { digest: [u8; 32] },
    /// Reliable refusal BEFORE acceptance (balance, invalid signature/structure).
    DeterministicReject(RejectReason),
    /// Owned-object lock by concurrent tx: outcome UNKNOWN until end of epoch.
    ObjectLocked,
    Unreachable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuiTxLookup {
    NotFound,
    Success { checkpoint: u64 },
    FailedOnChain { checkpoint: u64 },
    Unreachable,
}

pub trait SuiRpc: Send + Sync {
    fn build_transfer(&self, sender: &[u8; 32], recipient: &[u8; 32], amount: u128, coin_type: &str)
        -> Result<BuiltTx, ConnErr>;
    fn dry_run(&self, tx_bytes: &[u8]) -> DryRun;
    fn execute(&self, tx_bytes: &[u8], serialized_sig: &[u8]) -> SuiSend;
    fn lookup(&self, digest: &[u8; 32]) -> SuiTxLookup;
}

pub struct SuiConnector<R: SuiRpc> {
    pub network: &'static str, // "testnet" | "mainnet"
    pub coin_type: String,     // full type-tag of USDC on Sui (build constant, RISK-M7-5)
    pub wallet_addr: [u8; 32],
    pub rpc1: R,
    pub rpc2: R,
}

impl<R: SuiRpc> SuiConnector<R> {
    pub fn quote(&self, _i: &Intent) -> Result<Fee, ConnErr> {
        // Protocol gasless: no costs for parties. total = amount (Ω/Δ simplification).
        Ok(Fee { gas_estimate: Amount::ZERO })
    }

    pub fn execute(&self, i: &Intent, recipient32: &[u8; 32], signer: TxSignerP256) -> Result<TxRef, ConnErr> {
        // 1. server-side build (rpc1, fallback rpc2)
        let built = match self.rpc1.build_transfer(&self.wallet_addr, recipient32, i.amount.minor(), &self.coin_type) {
            Ok(b) => b,
            Err(_) => self.rpc2.build_transfer(&self.wallet_addr, recipient32, i.amount.minor(), &self.coin_type)?,
        };

        // 2. self-check via dry-run of BOTH nodes (RISK-M7-5): the assembled tx actually
        //    pays intent.recipient exactly intent.amount — protects against a lying build-node.
        let d1 = self.rpc1.dry_run(&built.tx_bytes);
        let d2 = self.rpc2.dry_run(&built.tx_bytes);
        match (&d1, &d2) {
            (DryRun::Ok { recipient: r1, amount: a1 }, DryRun::Ok { recipient: r2, amount: a2 }) => {
                if r1 != recipient32 || r2 != recipient32 || *a1 != i.amount.minor() || *a2 != i.amount.minor() {
                    // node built SOMETHING ELSE than what was requested → reliable refusal before signing
                    return Err(ConnErr::Rejected(RejectReason::InvalidRecipient));
                }
            }
            (DryRun::WouldFail(_), DryRun::WouldFail(_)) => {
                return Err(ConnErr::Rejected(RejectReason::SimulateRevert));
            }
            _ => return Err(ConnErr::Unknown("dry-run unavailable/divergent".into())),
        }

        // 3. signature (intent + blake2b256 + Ed25519); signer by move — no second signature possible
        let digest = signing_digest(&built.tx_bytes);
        let (sig, pubkey) = signer.sign_prehash(&digest).map_err(|e| ConnErr::Unknown(format!("sign: {e:?}")))?;
        let ser_sig = serialize_signature(&sig, &pubkey);

        // 4. send to both RPCs — classification strictly per RISK-M7-1
        let s1 = self.rpc1.execute(&built.tx_bytes, &ser_sig);
        let s2 = self.rpc2.execute(&built.tx_bytes, &ser_sig);
        classify_sui_send(s1, s2)
    }

    pub fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr> {
        let digest = match r {
            TxRef::Real { tx_hash, .. } => tx_hash,
            TxRef::Simulated { .. } => {
                return Ok(TxStatus::Failed { reason: crate::FailReason::Simulated })
            }
        };
        let l1 = self.rpc1.lookup(digest);
        let l2 = self.rpc2.lookup(digest);
        Ok(match (l1, l2) {
            // RISK-M7-3: Settled only when both nodes agree on checkpoint+success
            (SuiTxLookup::Success { checkpoint: c1 }, SuiTxLookup::Success { checkpoint: c2 }) if c1 == c2 => {
                TxStatus::Settled { block: c1, effective_gas: 0 }
            }
            (SuiTxLookup::FailedOnChain { .. }, SuiTxLookup::FailedOnChain { .. }) => {
                TxStatus::Failed { reason: crate::FailReason::OnChainRevert }
            }
            // unavailability/divergence/not found → Pending, reconcile decides
            _ => TxStatus::Pending,
        })
    }
}

/// RISK-M7-1 for Sui: Failed is not produced. ObjectLocked and any ambiguity → Unknown.
fn classify_sui_send(s1: SuiSend, s2: SuiSend) -> Result<TxRef, ConnErr> {
    use SuiSend::*;
    let accepted = match (&s1, &s2) {
        (Accepted { digest }, _) | (_, Accepted { digest }) => Some(*digest),
        (AlreadyExecuted { digest }, _) | (_, AlreadyExecuted { digest }) => Some(*digest),
        _ => None,
    };
    if let Some(digest) = accepted {
        return Ok(TxRef::Real { tx_hash: digest, chain_nonce: 0 });
    }
    if let (DeterministicReject(a), DeterministicReject(_)) = (&s1, &s2) {
        return Err(ConnErr::Rejected(a.clone()));
    }
    // ObjectLocked in any position = unknown (competitor's tx may or may not execute)
    Err(ConnErr::Unknown("send outcome uncertain (locked/unreachable/divergent)".into()))
}
