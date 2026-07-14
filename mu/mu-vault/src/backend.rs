//! Бэкенды Vault.
//! SoftVault — полнофункциональный, ТОЛЬКО для тестов/CI (feature="softvault").
//! Platform-бэкенды (Enclave/StrongBox/TPM) — каркас с явными TODO под FFI устройства.
//!
//! RISK-M4-4: деградация всегда fail-stop. attest().hw_backed=false + требование hw → halt (решает демон).
use crate::domain::tagged_digest;
use crate::signer::TxSigner;
use crate::{DomainTag, P256Sig, Vault, VaultErr, VaultInfo};
use mu_common::Hash32;
use p256::ecdsa::{signature::hazmat::PrehashSigner, SigningKey as P256Signing};
use zeroize::Zeroizing;

// ─────────────────────────────────────────────────────────────────────────
// SoftVault — ключи в памяти процесса. НЕ для продакшена.
// RISK-M4-4: release-профиль обязан НЕ включать feature "softvault".
// В lib.rs демона это проверяется через compile_error! (см. комментарий ниже).
// ─────────────────────────────────────────────────────────────────────────
#[cfg(feature = "softvault")]
pub struct SoftVault {
    mu_key: Zeroizing<[u8; 32]>,
    owner_key: Zeroizing<[u8; 32]>,
    wallet_key: Zeroizing<[u8; 32]>,
    /// Симуляция «свежая биометрия получена». В реальном бэкенде — платформенный обряд.
    owner_auth_ok: bool,
}

#[cfg(feature = "softvault")]
impl SoftVault {
    pub fn for_test(mu: [u8; 32], owner: [u8; 32], wallet: [u8; 32]) -> Self {
        SoftVault {
            mu_key: Zeroizing::new(mu),
            owner_key: Zeroizing::new(owner),
            wallet_key: Zeroizing::new(wallet),
            owner_auth_ok: true,
        }
    }
    /// Тест RISK-M4-3: смоделировать отказ биометрии.
    pub fn set_owner_auth(&mut self, ok: bool) {
        self.owner_auth_ok = ok;
    }

    fn p256_sign(key: &[u8; 32], tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr> {
        let tagged = tagged_digest(tag, digest);
        let sk = P256Signing::from_bytes(key.into())
            .map_err(|e| VaultErr::Backend(format!("p256 key: {e}")))?;
        let sig: p256::ecdsa::Signature = sk
            .sign_prehash(&tagged.0)
            .map_err(|e| VaultErr::Backend(format!("p256 sign: {e}")))?;
        let b = sig.to_bytes();
        let mut out = [0u8; 64];
        out.copy_from_slice(&b);
        Ok(P256Sig(out))
    }
}

#[cfg(feature = "softvault")]
impl Vault for SoftVault {
    fn sign_mu(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr> {
        // RISK-M4-5: только структурные домены μ, не delta/human
        match tag {
            DomainTag::MuCore | DomainTag::MuLog => Self::p256_sign(&self.mu_key, tag, digest),
            _ => Err(VaultErr::Backend("sign_mu wrong domain".into())),
        }
    }
    fn owner_sign(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr> {
        // RISK-M4-3: без свежей биометрии — отказ
        if !self.owner_auth_ok {
            return Err(VaultErr::UserAuthRequired);
        }
        match tag {
            DomainTag::MuDelta | DomainTag::MuHuman => Self::p256_sign(&self.owner_key, tag, digest),
            _ => Err(VaultErr::Backend("owner_sign wrong domain".into())),
        }
    }
    fn tx_signer(&self) -> Result<TxSigner, VaultErr> {
        Ok(TxSigner::from_bytes(*self.wallet_key))
    }
    fn tx_signer_p256(&self) -> Result<crate::signer::TxSignerP256, VaultErr> {
        Ok(crate::signer::TxSignerP256::from_bytes(*self.wallet_key))
    }
    fn attest(&self) -> VaultInfo {
        VaultInfo { backend: "softvault", hw_backed: false }
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Platform-бэкенды: каркас. Реальная реализация — FFI под устройство,
// пишется и проверяется на самих устройствах (device-smoke из спеки M4 §6).
// ─────────────────────────────────────────────────────────────────────────

/// Apple Secure Enclave. Ключи μ/owner — SecKey P-256 в железе;
/// owner-ключ создаётся с kSecAccessControlBiometryCurrentSet (RISK-M4-3).
/// Ключ кошелька — AES-256-GCM(privkey, KEK-из-Enclave), AAD=mu_id‖chain_id (RISK-M4-2).
pub struct AppleEnclave {
    // TODO(device): дескрипторы SecKey, ссылка на wrap-файл.
    _priv: (),
}

impl AppleEnclave {
    pub fn open() -> Result<Self, VaultErr> {
        Err(VaultErr::Backend("AppleEnclave: реализуется на устройстве (FFI Security.framework)".into()))
    }
}

/// Android StrongBox Keystore. setIsStrongBoxBacked(true);
/// owner: setUserAuthenticationRequired + BiometricPrompt CryptoObject.
/// Fallback на TEE-Keystore → attest().hw_backed=false (решение о допуске у демона).
pub struct AndroidStrongBox {
    _priv: (),
}

impl AndroidStrongBox {
    pub fn open() -> Result<Self, VaultErr> {
        Err(VaultErr::Backend("AndroidStrongBox: реализуется на устройстве (JNI Keystore)".into()))
    }
}

/// TPM 2.0 (ESAPI). persistent handle + PolicyAuthValue; owner-auth = Windows Hello.
pub struct Tpm2 {
    _priv: (),
}

impl Tpm2 {
    pub fn open() -> Result<Self, VaultErr> {
        Err(VaultErr::Backend("TPM2: реализуется на устройстве (ESAPI/tss-esapi)".into()))
    }
}

#[cfg(all(test, feature = "softvault"))]
mod tests {
    use super::*;
    #[test]
    fn softvault_domain_separation() {
        let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        let d = Hash32([0xAB; 32]);
        // sign_mu не принимает delta-домен (RISK-M4-5)
        assert!(v.sign_mu(DomainTag::MuDelta, &d).is_err());
        assert!(v.sign_mu(DomainTag::MuCore, &d).is_ok());
        // owner_sign не принимает core-домен
        assert!(v.owner_sign(DomainTag::MuCore, &d).is_err());
        assert!(v.owner_sign(DomainTag::MuDelta, &d).is_ok());
    }
    #[test]
    fn owner_sign_requires_auth() {
        let mut v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        v.set_owner_auth(false);
        let d = Hash32([1; 32]);
        // RISK-M4-3: без биометрии — UserAuthRequired
        assert!(matches!(
            v.owner_sign(DomainTag::MuDelta, &d),
            Err(VaultErr::UserAuthRequired)
        ));
    }
    #[test]
    fn tx_signer_signs() {
        let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        let s = v.tx_signer().unwrap();
        assert!(s.sign(&Hash32([7; 32])).is_ok());
    }
}
