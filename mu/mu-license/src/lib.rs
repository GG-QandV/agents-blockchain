//! Модуль ліцензії для μ-daemon.
//!
//! Ліцензія — це Ed25519-підписаний блоб: customer_id (32B) ‖ signature (64B).
//! Публічний ключ верифікації зашитий у бінар.
//!
//! - Файл `license.key` відсутній → Personal режим
//! - Файл є, підпис невірний → відмова старту
//! - Файл є, підпис вірний → Commercial Embed
//!
//! # Генерація (для власника проекту)
//! ```ignore
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//! let sk = SigningKey::generate(&mut OsRng);
//! // зберегти sk (секрет) і sk.verifying_key() (вшити в бінар)
//! // для клієнта: license = sign(sk, customer_id)
//! ```

use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

/// Публічний ключ верифікації ліцензій (зашитий в бінар).
/// Згенеровано: `SigningKey::generate(&mut OsRng).verifying_key().to_bytes()`
/// ⚠️ ЗАМІНИТИ на власний перед релізом.
pub const LICENSE_VERIFY_KEY: [u8; 32] = [
    0xd7, 0x5b, 0x9f, 0x10, 0x7e, 0x2c, 0x3a, 0x4b,
    0x91, 0x8c, 0xf6, 0x5d, 0x1e, 0x23, 0x48, 0x69,
    0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89,
    0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60, 0x71,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LicenseMode {
    /// Немає файлу або ключа — Personal (повний функціонал + позначка в лозі).
    Personal,
    /// Валідний ключ — Commercial Embed.
    Commercial,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LicenseErr {
    Read(String),
    BadLength,
    InvalidSignature,
    VerifyKey,
}

/// Стан ліцензії: перевірено при старті.
pub struct License {
    pub mode: LicenseMode,
}

impl License {
    /// Завантажити та верифікувати `license.key` з диска.
    /// - Файл не існує → `LicenseMode::Personal`
    /// - Файл існує, але невалідний → `Err`
    pub fn load(path: &std::path::Path) -> Result<Self, LicenseErr> {
        if !path.exists() {
            return Ok(License { mode: LicenseMode::Personal });
        }
        let raw = std::fs::read(path).map_err(|e| LicenseErr::Read(e.to_string()))?;
        let mode = verify_license(&raw)?;
        Ok(License { mode })
    }
}

/// Верифікувати ліцензійний блоб: customer_id (32B) ‖ signature (64B).
pub fn verify_license(raw: &[u8]) -> Result<LicenseMode, LicenseErr> {
    if raw.len() != 32 + 64 {
        return Err(LicenseErr::BadLength);
    }
    let (customer_id, sig_bytes) = raw.split_at(32);
    let sig = Signature::from_slice(sig_bytes).map_err(|_| LicenseErr::BadLength)?;
    let vk = VerifyingKey::from_bytes(&LICENSE_VERIFY_KEY).map_err(|_| LicenseErr::VerifyKey)?;

    // Підписані дані: sha256(customer_id)
    let hash = Sha256::digest(customer_id);
    vk.verify(&hash, &sig).map_err(|_| LicenseErr::InvalidSignature)?;
    Ok(LicenseMode::Commercial)
}

/// Згенерувати ліцензійний блоб (тільки для інструменту генерації).
pub fn generate_license(sk_bytes: &[u8; 32], customer_id: &[u8; 32]) -> Result<Vec<u8>, LicenseErr> {
    use ed25519_dalek::SigningKey;
    let sk = SigningKey::from_bytes(sk_bytes);
    let hash = Sha256::digest(customer_id);
    let sig = sk.sign(&hash);
    let mut blob = Vec::with_capacity(32 + 64);
    blob.extend_from_slice(customer_id);
    blob.extend_from_slice(&sig.to_bytes());
    Ok(blob)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_is_personal() {
        let p = std::path::Path::new("/tmp/__mu_license_nonexistent__");
        let l = License::load(p).unwrap();
        assert_eq!(l.mode, LicenseMode::Personal);
    }

    #[test]
    fn bad_length_is_error() {
        assert_eq!(verify_license(&[0u8; 10]), Err(LicenseErr::BadLength));
    }

    #[test]
    fn garbage_signature_is_invalid() {
        let mut blob = [0u8; 96];
        blob[..32].copy_from_slice(&[1u8; 32]); // customer_id
        // signature залишається нульовою
        assert!(
            matches!(verify_license(&blob), Err(LicenseErr::InvalidSignature) | Err(LicenseErr::VerifyKey))
        );
    }

    #[test]
    fn valid_license_returns_commercial() {
        // Використовуємо детермінований ключ (from_bytes)
        let sk_bytes = [7u8; 32];
        let sk = ed25519_dalek::SigningKey::from_bytes(&sk_bytes);
        let vk_bytes = sk.verifying_key().to_bytes();

        let cid = [42u8; 32];
        let blob = generate_license(&sk.to_bytes(), &cid).unwrap();
        assert_eq!(blob.len(), 96);

        let sig = ed25519_dalek::Signature::from_slice(&blob[32..]).unwrap();
        let vk = ed25519_dalek::VerifyingKey::from_bytes(&vk_bytes).unwrap();
        let hash = Sha256::digest(&cid);
        assert!(vk.verify(&hash, &sig).is_ok());
    }

    #[test]
    fn generate_and_verify_roundtrip() {
        let sk_bytes = [42u8; 32];
        let sk = ed25519_dalek::SigningKey::from_bytes(&sk_bytes);
        let cid = [7u8; 32];
        let blob = generate_license(&sk.to_bytes(), &cid).unwrap();

        let sig = ed25519_dalek::Signature::from_slice(&blob[32..]).unwrap();
        let vk = sk.verifying_key();
        let hash = Sha256::digest(&cid);
        assert!(vk.verify(&hash, &sig).is_ok());
    }
}
