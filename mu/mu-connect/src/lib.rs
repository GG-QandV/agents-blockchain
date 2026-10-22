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
    /// x402: no 402 offer was supplied for this intent, so nothing may be signed.
    NoChallenge,
    /// x402: this offer was already paid once; a second payment needs a new 402.
    OfferAlreadyUsed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fee {
    pub gas_estimate: Amount,
}

/// Контракт коннектора. execute принимает TxSigner ПО MOVE (RISK-M4-1):
/// после execute хэндл ключа уничтожен, повторно подписать нельзя.
/// Authorization parameters fixed before the WAL: the nonce is single-use on
/// chain, so a retry after a crash reuses it and cannot pay twice.
/// Everything one execution needs besides the intent: the authorization
/// parameters already written to the WAL and the 402 offer they answer.
/// The offer travels per intent, so an old challenge cannot leak into the
/// next payment.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExecCtx {
    pub auth: Option<AuthParams>,
    pub offer: Option<crate::x402::X402Challenge>,
}

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

    /// The authorization window for this offer, or `None` when the connector
    /// cannot pay it. There is no default: x402 takes the window from the 402
    /// answer only, so a missing or stale offer never turns into a payment.
    fn auth_window_secs(&self, _offer: Option<&crate::x402::X402Challenge>) -> Option<u64> {
        None
    }

    /// Execute with parameters that were already written to the WAL. The
    /// default ignores them and calls `execute`.
    fn execute_auth(&self, i: &Intent, signer: TxSigner, _ctx: ExecCtx) -> Result<TxRef, ConnErr> {
        self.execute(i, signer)
    }
}

// Реэкспорт для тестов коннектора: доступ к SoftVault-конструктору TxSigner.
#[cfg(feature = "softvault")]
pub use mu_vault::backend as vault_backend;
