//! M4 mu-vault — граница секретов.
//! Наружу выходят только подписи и scoped-хэндлы, НИКОГДА байты ключей.
//!
//! RISK-M4-1: plaintext ключа кошелька живёт только внутри TxSigner (Zeroizing, !Clone, !Debug).
//! RISK-M4-3: owner_sign требует свежей UserAuth (принуждается платформенным ключом).
//! RISK-M4-5: доменная сепарация — sign_* принимают DomainTag и хешируют payload с тегом.
#![forbid(unsafe_code)]

pub mod domain;
pub mod signer;
pub mod backend;

pub use domain::DomainTag;
pub use signer::{TxSigner, TxSignerP256};

use mu_common::Hash32;

#[derive(Debug)]
#[non_exhaustive]
pub enum VaultErr {
    HwUnavailable,
    UserAuthRequired,
    UserAuthFailed,
    WrapCorrupted,
    KeyMissing,
    Backend(String),
}

/// P-256 подпись (DER либо fixed 64B — здесь fixed r||s).
#[derive(Clone, PartialEq, Eq)]
pub struct P256Sig(pub [u8; 64]);

impl core::fmt::Debug for P256Sig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "P256Sig(<redacted>)")
    }
}

/// secp256k1 подпись для транзакций (r||s||v).
#[derive(Clone)]
pub struct Secp256k1Sig {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: u8,
}

pub struct VaultInfo {
    pub backend: &'static str,
    pub hw_backed: bool,
}

/// Контракт хранилища секретов.
pub trait Vault: Send + Sync {
    /// Подпись структур μ ключом μ (в железе). Домен обязателен (RISK-M4-5).
    fn sign_mu(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr>;

    /// Подпись владельца (Δ, подтверждения). Требует свежей биометрии (RISK-M4-3).
    fn owner_sign(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr>;

    /// Единственный путь к ключу кошелька. Хэндл живёт в пределах одного execute().
    fn tx_signer(&self) -> Result<TxSigner, VaultErr>;

    /// P-256 хэндл кошелька (Sui, flag 0x02). Дефолт: KeyMissing.
    fn tx_signer_p256(&self) -> Result<TxSignerP256, VaultErr> {
        Err(VaultErr::KeyMissing)
    }

    fn attest(&self) -> VaultInfo;
}
