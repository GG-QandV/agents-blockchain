//! FileVault: keys encrypted at rest, decrypted only into `Zeroizing` buffers.
//!
//! File layout: `MUVFILE1 ‖ salt[16] ‖ nonce[24] ‖ XChaCha20-Poly1305(keys[96])`.
//! The AEAD key is Argon2id(passphrase, salt, m=64 MiB, t=3, p=1). The AAD is
//! the magic, so a file from another format never decrypts.
//!
//! What this backend buys: no plaintext key on disk. It does not buy hardware
//! isolation — the keys still live in process memory while the daemon runs, and
//! `hw_backed` stays false.

use std::io::Write;
use std::path::Path;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use rand_core::RngCore;
use zeroize::Zeroizing;

use crate::backend::p256_sign;
use crate::domain::tagged_digest;
use crate::{DomainTag, P256Sig, TxSigner, Vault, VaultErr, VaultInfo};
use mu_common::Hash32;

const MAGIC: &[u8; 8] = b"MUVFILE1";
const SALT: usize = 16;
const NONCE: usize = 24;
/// Three 32-byte keys.
const KEYS: usize = 96;

pub struct FileVault {
    mu_key: Zeroizing<[u8; 32]>,
    owner_key: Zeroizing<[u8; 32]>,
    wallet_key: Zeroizing<[u8; 32]>,
    owner_auth_ok: bool,
}

impl FileVault {
    /// Open and decrypt. The file must be mode 0600.
    pub fn open(path: &Path, passphrase: &[u8]) -> Result<Self, VaultErr> {
        refuse_world_readable(path)?;
        let blob = std::fs::read(path).map_err(|e| VaultErr::Backend(format!("{path:?}: {e}")))?;
        Self::from_blob(&blob, passphrase)
    }

    pub fn from_blob(blob: &[u8], passphrase: &[u8]) -> Result<Self, VaultErr> {
        if blob.len() != MAGIC.len() + SALT + NONCE + KEYS + 16 || &blob[..8] != MAGIC {
            return Err(VaultErr::WrapCorrupted);
        }
        let salt = &blob[8..8 + SALT];
        let nonce = &blob[8 + SALT..8 + SALT + NONCE];
        let cipher = XChaCha20Poly1305::new(Self::derive(passphrase, salt)?.as_ref().into());
        let plain = cipher
            .decrypt(
                XNonce::from_slice(nonce),
                Payload { msg: &blob[8 + SALT + NONCE..], aad: MAGIC },
            )
            .map_err(|_| VaultErr::UserAuthFailed)?;
        if plain.len() != KEYS {
            return Err(VaultErr::WrapCorrupted);
        }
        let mut mu = Zeroizing::new([0u8; 32]);
        let mut owner = Zeroizing::new([0u8; 32]);
        let mut wallet = Zeroizing::new([0u8; 32]);
        mu.copy_from_slice(&plain[0..32]);
        owner.copy_from_slice(&plain[32..64]);
        wallet.copy_from_slice(&plain[64..96]);
        Ok(FileVault { mu_key: mu, owner_key: owner, wallet_key: wallet, owner_auth_ok: true })
    }

    fn derive(passphrase: &[u8], salt: &[u8]) -> Result<Zeroizing<[u8; 32]>, VaultErr> {
        let params = Params::new(65_536, 3, 1, Some(32))
            .map_err(|e| VaultErr::Backend(format!("argon2: {e}")))?;
        let a = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut out = Zeroizing::new([0u8; 32]);
        a.hash_password_into(passphrase, salt, out.as_mut())
            .map_err(|e| VaultErr::Backend(format!("argon2: {e}")))?;
        Ok(out)
    }

    /// Encrypt three keys into a new file (0600). The caller's buffers stay
    /// theirs: pass `&[u8; 32]` and zero them after the call.
    pub fn seal(
        path: &Path,
        passphrase: &[u8],
        mu: &[u8; 32],
        owner: &[u8; 32],
        wallet: &[u8; 32],
    ) -> Result<(), VaultErr> {
        let mut salt = [0u8; SALT];
        let mut nonce = [0u8; NONCE];
        rand_core::OsRng
            .try_fill_bytes(&mut salt)
            .map_err(|e| VaultErr::Backend(format!("rng: {e}")))?;
        rand_core::OsRng
            .try_fill_bytes(&mut nonce)
            .map_err(|e| VaultErr::Backend(format!("rng: {e}")))?;
        let cipher = XChaCha20Poly1305::new(Self::derive(passphrase, &salt)?.as_ref().into());
        let mut keys = Zeroizing::new([0u8; KEYS]);
        keys[0..32].copy_from_slice(mu);
        keys[32..64].copy_from_slice(owner);
        keys[64..96].copy_from_slice(wallet);
        let ct = cipher
            .encrypt(XNonce::from_slice(&nonce), Payload { msg: keys.as_ref(), aad: MAGIC })
            .map_err(|_| VaultErr::Backend("seal: encryption failed".into()))?;

        let mut out = Vec::with_capacity(MAGIC.len() + SALT + NONCE + ct.len());
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&salt);
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ct);

        let tmp = path.with_extension("tmp");
        {
            use std::os::unix::fs::OpenOptionsExt;
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&tmp)
                .map_err(|e| VaultErr::Backend(format!("{tmp:?}: {e}")))?;
            f.write_all(&out).map_err(|e| VaultErr::Backend(format!("{tmp:?}: {e}")))?;
            f.sync_all().map_err(|e| VaultErr::Backend(format!("{tmp:?}: {e}")))?;
        }
        std::fs::rename(&tmp, path).map_err(|e| VaultErr::Backend(format!("{path:?}: {e}")))?;
        Ok(())
    }

    /// Tests and local tools only: the flag models "fresh biometrics obtained".
    pub fn set_owner_auth(&mut self, ok: bool) {
        self.owner_auth_ok = ok;
    }
}

fn refuse_world_readable(path: &Path) -> Result<(), VaultErr> {
    use std::os::unix::fs::PermissionsExt;
    let mode = std::fs::metadata(path)
        .map_err(|e| VaultErr::Backend(format!("{path:?}: {e}")))?
        .permissions()
        .mode();
    if mode & 0o077 != 0 {
        return Err(VaultErr::Backend(format!(
            "{path:?}: must be mode 600 (readable by the owner only)"
        )));
    }
    Ok(())
}

impl Vault for FileVault {
    fn sign_mu(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr> {
        match tag {
            DomainTag::MuCore | DomainTag::MuLog => p256_sign(&self.mu_key, tag, digest),
            _ => Err(VaultErr::Backend("sign_mu wrong domain".into())),
        }
    }
    fn owner_sign(&self, tag: DomainTag, digest: &Hash32) -> Result<P256Sig, VaultErr> {
        if !self.owner_auth_ok {
            return Err(VaultErr::UserAuthRequired);
        }
        match tag {
            DomainTag::MuDelta | DomainTag::MuHuman => p256_sign(&self.owner_key, tag, digest),
            _ => Err(VaultErr::Backend("owner_sign wrong domain".into())),
        }
    }
    fn tx_signer(&self) -> Result<TxSigner, VaultErr> {
        Ok(TxSigner::from_bytes(*self.wallet_key))
    }
    fn tx_signer_p256(&self) -> Result<crate::signer::TxSignerP256, VaultErr> {
        Ok(crate::signer::TxSignerP256::from_bytes(*self.wallet_key))
    }
    fn owner_pubkey(&self) -> Option<Vec<u8>> {
        let sk = p256::ecdsa::SigningKey::from_bytes((&*self.owner_key).into()).ok()?;
        Some(sk.verifying_key().to_encoded_point(true).as_bytes().to_vec())
    }
    fn attest(&self) -> VaultInfo {
        VaultInfo { backend: "filevault", hw_backed: false }
    }
}

#[allow(dead_code)]
fn _tagged(tag: DomainTag, d: &Hash32) -> Hash32 {
    tagged_digest(tag, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys() -> ([u8; 32], [u8; 32], [u8; 32]) {
        ([1; 32], [2; 32], [3; 32])
    }

    #[test]
    fn seals_and_opens() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("vault.enc");
        let (mu, owner, wallet) = keys();
        FileVault::seal(&path, b"correct horse", &mu, &owner, &wallet).unwrap();
        let v = FileVault::open(&path, b"correct horse").unwrap();
        assert_eq!(v.owner_pubkey(), {
            let sk = p256::ecdsa::SigningKey::from_bytes((&owner).into()).unwrap();
            Some(sk.verifying_key().to_encoded_point(true).as_bytes().to_vec())
        });
        assert_eq!(v.attest().backend, "filevault");
        assert!(!v.attest().hw_backed);
        // the file itself does not contain the keys
        let blob = std::fs::read(&path).unwrap();
        assert!(!blob.windows(32).any(|w| w == mu || w == owner || w == wallet));
    }

    #[test]
    fn a_wrong_passphrase_opens_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("vault.enc");
        let (mu, owner, wallet) = keys();
        FileVault::seal(&path, b"right", &mu, &owner, &wallet).unwrap();
        assert!(matches!(FileVault::open(&path, b"wrong"), Err(VaultErr::UserAuthFailed)));
    }

    #[test]
    fn a_readable_by_others_file_is_refused() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("vault.enc");
        let (mu, owner, wallet) = keys();
        FileVault::seal(&path, b"p", &mu, &owner, &wallet).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(FileVault::open(&path, b"p").is_err());
    }

    #[test]
    fn a_tampered_blob_opens_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("vault.enc");
        let (mu, owner, wallet) = keys();
        FileVault::seal(&path, b"p", &mu, &owner, &wallet).unwrap();
        let mut blob = std::fs::read(&path).unwrap();
        let last = blob.len() - 1;
        blob[last] ^= 1;
        assert!(FileVault::from_blob(&blob, b"p").is_err());
    }
}
