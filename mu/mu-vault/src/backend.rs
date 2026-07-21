//! Vault backends.
//! SoftVault — fully functional, ONLY for tests/CI (feature="softvault").
//! Platform backends (Enclave/StrongBox/TPM) — skeleton with explicit TODO for device FFI.
//!
//! RISK-M4-4: degradation is always fail-stop. attest().hw_backed=false + hw requirement → halt (decided by daemon).
use crate::domain::tagged_digest;
use crate::signer::TxSigner;
use crate::{DomainTag, P256Sig, Vault, VaultErr, VaultInfo};
use mu_common::Hash32;
use p256::ecdsa::{signature::hazmat::PrehashSigner, SigningKey as P256Signing};
use zeroize::Zeroizing;

// ─────────────────────────────────────────────────────────────────────────
// SoftVault — keys in process memory. NOT for production.
// RISK-M4-4: release profile MUST NOT enable feature "softvault".
// This is checked in the daemon's lib.rs via compile_error! (see comment below).
// ─────────────────────────────────────────────────────────────────────────
#[cfg(feature = "softvault")]
pub struct SoftVault {
    mu_key: Zeroizing<[u8; 32]>,
    owner_key: Zeroizing<[u8; 32]>,
    wallet_key: Zeroizing<[u8; 32]>,
    /// Simulates "fresh biometrics obtained". In a real backend — platform-specific ritual.
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
    /// RISK-M4-3 test: simulate biometric failure.
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
        // RISK-M4-5: only structural μ domains, not delta/human
        match tag {
            DomainTag::MuCore | DomainTag::MuLog => Self::p256_sign(&self.mu_key, tag, digest),
            _ => Err(VaultErr::Backend("sign_mu wrong domain".into())),
        }
    }
    fn owner_sign(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr> {
        // RISK-M4-3: without fresh biometrics — deny
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
// Platform backends: skeleton. Real implementation — FFI per device,
// written and tested on actual devices (device-smoke from spec M4 §6).
// ─────────────────────────────────────────────────────────────────────────

/// Apple Secure Enclave. μ/owner keys — SecKey P-256 in hardware;
/// owner key created with kSecAccessControlBiometryCurrentSet (RISK-M4-3).
/// Wallet key — AES-256-GCM(privkey, KEK-from-Enclave), AAD=mu_id‖chain_id (RISK-M4-2).
pub struct AppleEnclave {
    // TODO(device): SecKey descriptors, reference to wrap file.
    _priv: (),
}

impl AppleEnclave {
    pub fn open() -> Result<Self, VaultErr> {
        Err(VaultErr::Backend("AppleEnclave: реализуется на устройстве (FFI Security.framework)".into()))
    }
}

/// Android StrongBox Keystore. setIsStrongBoxBacked(true);
/// owner: setUserAuthenticationRequired + BiometricPrompt CryptoObject.
/// Fallback to TEE-Keystore → attest().hw_backed=false (admission decision by daemon).
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
        // sign_mu does not accept delta domain (RISK-M4-5)
        assert!(v.sign_mu(DomainTag::MuDelta, &d).is_err());
        assert!(v.sign_mu(DomainTag::MuCore, &d).is_ok());
        // owner_sign does not accept core domain
        assert!(v.owner_sign(DomainTag::MuCore, &d).is_err());
        assert!(v.owner_sign(DomainTag::MuDelta, &d).is_ok());
    }
    #[test]
    fn owner_sign_requires_auth() {
        let mut v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        v.set_owner_auth(false);
        let d = Hash32([1; 32]);
        // RISK-M4-3: without biometrics — UserAuthRequired
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
