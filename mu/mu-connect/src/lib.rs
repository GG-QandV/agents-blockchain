//! M7 mu-connect — единственная точка контакта с деньгами внешнего мира.
//!
//! RISK-M7-1 (главный): Unknown ≠ Failed. execute() ТИПОМ не может вернуть Failed.
//!   Failed выносит только status()/reconcile по данным цепи.
//! RISK-M7-5: recipient/contract неподменяемы — сборка calldata единственной функцией + self-check.
//! RISK-M7S-1: стабы возвращают TxRef::Simulated, недостижимый для реального учёта.
#![forbid(unsafe_code)]

pub mod crypto;
pub mod stub;
pub mod sui;
pub mod sui_jsonrpc;

use mu_common::{Amount, CanonAddress};
use mu_vault::TxSigner;

/// Намерение платежа, уже прошедшее Ω/Δ (адрес канонизирован).
#[derive(Clone, Debug)]
pub struct Intent {
    pub recipient: CanonAddress,
    pub amount: Amount,
    pub chain_id: u64,
}

/// Ссылка на транзакцию. Real и Simulated — РАЗНЫЕ варианты (RISK-M7S-1):
/// Simulated никогда не спутается с настоящим tx_hash.
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

/// КЛЮЧЕВОЙ КОНТРАКТ ОШИБОК (RISK-M7-1):
/// - Rejected: достоверный отказ ДО попадания tx в сеть → M6 может Failed+rollback.
/// - Unknown:  сеть/таймаут/расхождение → M6 ОБЯЗАН оставить Pending (резерв держится).
/// Варианта Failed здесь НЕТ: execute не может «убить» платёж, судьба которого неясна.
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

/// Контракт коннектора. execute принимает TxSigner ПО MOVE (RISK-M4-1):
/// после execute хэндл ключа уничтожен, повторно подписать нельзя.
pub trait Connector {
    fn quote(&self, i: &Intent) -> Result<Fee, ConnErr>;
    fn execute(&self, i: &Intent, signer: TxSigner) -> Result<TxRef, ConnErr>;
    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr>;
}

// Реэкспорт для тестов коннектора: доступ к SoftVault-конструктору TxSigner.
#[cfg(feature = "softvault")]
pub use mu_vault::backend as vault_backend;
