//! M7 mu-connect — sole contact point with external money.
//!
//! RISK-M7-1 (main): Unknown ≠ Failed. execute() CANNOT return Failed by type.
//!   Failed is only emitted by status()/reconcile based on chain data.
//! RISK-M7-5: recipient/contract are non-spoofable — calldata built by single function + self-check.
//! RISK-M7S-1: stubs return TxRef::Simulated, unreachable for real accounting.
#![forbid(unsafe_code)]

pub mod crypto;
pub mod stub;
pub mod sui;
pub mod sui_jsonrpc;

use mu_common::{Amount, CanonAddress};
use mu_vault::TxSigner;

/// Payment intent, already passed Ω/Δ (address canonicalized).
#[derive(Clone, Debug)]
pub struct Intent {
    pub recipient: CanonAddress,
    pub amount: Amount,
    pub chain_id: u64,
}

/// Transaction reference. Real and Simulated are DISTINCT variants (RISK-M7S-1):
/// Simulated can never be confused with a real tx_hash.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TxRef {
    Real { tx_hash: [u8; 32], chain_nonce: u64 },
    Simulated { id: [u8; 16] },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TxStatus {
    Pending,
    Settled { block: u64, effective_gas: u128 },
    Failed { reason: FailReason },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FailReason {
    OnChainRevert,
    NonceConsumedByOther,
    Simulated,
}

/// KEY ERROR CONTRACT (RISK-M7-1):
/// - Rejected: reliable refusal BEFORE tx enters the network → M6 can Failed+rollback.
/// - Unknown:  network/timeout/divergence → M6 MUST keep Pending (reserve is held).
/// There is NO Failed variant here: execute cannot "kill" a payment whose fate is unclear.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConnErr {
    Rejected(RejectReason),
    Unknown(String),
    Config(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RejectReason {
    InsufficientFunds,
    NonceTooLow,
    InvalidRecipient,
    FeeCapTooLow,
    ChainMismatch,
    SimulateRevert,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fee {
    pub gas_estimate: Amount,
}

/// Connector contract. execute takes TxSigner BY MOVE (RISK-M4-1):
/// after execute, the key handle is destroyed, cannot sign again.
pub trait Connector {
    fn quote(&self, i: &Intent) -> Result<Fee, ConnErr>;
    fn execute(&self, i: &Intent, signer: TxSigner) -> Result<TxRef, ConnErr>;
    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr>;
}

// Re-export for connector tests: access to SoftVault constructor for TxSigner.
#[cfg(feature = "softvault")]
pub use mu_vault::backend as vault_backend;
