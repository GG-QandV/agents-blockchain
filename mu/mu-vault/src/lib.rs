//! M4 mu-vault — secret boundary.
//! Only signatures and scoped handles are exposed, NEVER key bytes.
//!
//! RISK-M4-1: wallet key plaintext lives only inside TxSigner (Zeroizing, !Clone, !Debug).
//! RISK-M4-3: owner_sign requires fresh UserAuth (enforced by platform key).
//! RISK-M4-5: domain separation — sign_* accept DomainTag and hash payload with tag.
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

/// P-256 signature (DER or fixed 64B — here fixed r||s).
#[derive(Clone, PartialEq, Eq)]
pub struct P256Sig(pub [u8; 64]);

impl core::fmt::Debug for P256Sig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "P256Sig(<redacted>)")
    }
}

/// secp256k1 signature for transactions (r||s||v).
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

/// Secret storage contract.
pub trait Vault: Send + Sync {
    /// Sign μ structures with μ key (in hardware). Domain required (RISK-M4-5).
    fn sign_mu(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr>;

    /// Owner signature (Δ, confirmations). Requires fresh biometrics (RISK-M4-3).
    fn owner_sign(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr>;

    /// The only path to the wallet key. Handle lives within a single execute().
    fn tx_signer(&self) -> Result<TxSigner, VaultErr>;

    /// P-256 wallet handle (Sui, flag 0x02). Default: KeyMissing.
    fn tx_signer_p256(&self) -> Result<TxSignerP256, VaultErr> {
        Err(VaultErr::KeyMissing)
    }

    fn attest(&self) -> VaultInfo;
}
