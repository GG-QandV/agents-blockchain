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
pub mod x402;

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
/// Authorization — EIP-3009 TransferWithAuthorization (x402, gasless).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TxRef {
    Real { tx_hash: [u8; 32], chain_nonce: u64 },
    Simulated { id: [u8; 16] },
    /// x402: EIP-3009 авторизація, nonce — випадковий 32B (RISK-M7-5).
    /// valid_before — unix ts, після якого authorizationState→false = остаточний Failed.
    Authorization { nonce: [u8; 32], valid_before: u64 },
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
    /// x402: the signing key is not the wallet the connector was configured for.
    WalletMismatch,
    /// x402: the 402 challenge does not match the intent (network, token, recipient, amount).
    ChallengeMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fee {
    pub gas_estimate: Amount,
}

/// Контракт коннектора. execute принимает TxSigner ПО MOVE (RISK-M4-1):
/// после execute хэндл ключа уничтожен, повторно подписать нельзя.
/// Default authorization window, when the connector names no shorter one.
pub const AUTH_WINDOW_DEFAULT: u64 = 300;

/// Authorization parameters fixed before the WAL: the nonce is single-use on
/// chain, so a retry after a crash reuses it and cannot pay twice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthParams {
    pub nonce: [u8; 32],
    /// Both time bounds are stored, so a retry after a crash signs the very
    /// same digest and yields the very same header.
    pub valid_after: u64,
    pub valid_before: u64,
}

pub trait Connector {
    fn quote(&self, i: &Intent) -> Result<Fee, ConnErr>;
    fn execute(&self, i: &Intent, signer: TxSigner) -> Result<TxRef, ConnErr>;
    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr>;

    /// The payment header produced by the last `execute`, if the connector has
    /// one (x402). Taken once: a second call returns `None`.
    fn take_header(&self) -> Option<String> {
        None
    }

    /// True when the connector needs its authorization parameters before the
    /// WAL is written (x402). The pipeline then writes `PendingAuth` first.
    fn pre_auth(&self) -> bool {
        false
    }

    /// The longest authorization window the connector may ask for. The
    /// pipeline never writes a `valid_before` beyond it, so a 402 answer that
    /// allows a shorter window is respected instead of being overrun.
    fn auth_window_secs(&self) -> u64 {
        AUTH_WINDOW_DEFAULT
    }

    /// Execute with parameters that were already written to the WAL. The
    /// default ignores them and calls `execute`.
    fn execute_auth(
        &self,
        i: &Intent,
        signer: TxSigner,
        _auth: Option<AuthParams>,
    ) -> Result<TxRef, ConnErr> {
        self.execute(i, signer)
    }
}

// Реэкспорт для тестов коннектора: доступ к SoftVault-конструктору TxSigner.
#[cfg(feature = "softvault")]
pub use mu_vault::backend as vault_backend;
