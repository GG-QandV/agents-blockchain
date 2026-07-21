use crate::format::{decode_mu, encode_payload, MuPayload};
use mu_common::Hash32;
use mu_policy::{delta_hash, Delta};
use mu_vault::domain::tagged_digest;
use mu_vault::{DomainTag, Vault};
use p256::ecdsa::signature::hazmat::PrehashVerifier;
use p256::ecdsa::{Signature as P256Signature, VerifyingKey};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::Path;

pub const MU_MAX_SIZE: usize = 3072;

#[derive(Debug)]
#[non_exhaustive]
pub enum CoreErr {
    Io(std::io::Error),
    CborMalformed,
    SigMu,
    SigOwner,
    ChecksumMismatch,
    VersionUnsupported { found: u16 },
    OmegaImmutable,
    SizeExceeded,
    LogHeadMismatch, // RISK-M1-2
    Vault(String),
}
impl From<std::io::Error> for CoreErr {
    fn from(e: std::io::Error) -> Self { CoreErr::Io(e) }
}

/// Ω-slot: fixed at issuance. Byte form is part of the signed payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Omega {
    pub connectors: Vec<u8>,   // closed ids (ConnectorId as u8) — RISK-M2-3
    pub max_ceiling: u128,
}

/// μ-object. Fields are private: only reading and legal operations from outside (RISK-M1-1).
#[derive(Clone, Debug)]
pub struct Mu {
    core_id: [u8; 16],
    owner_pubkey: [u8; 33],
    created_at: u64,
    version: u16,
    omega: Omega,
    delta: Delta,
    delta_sig: [u8; 64],    // owner signature over delta_hash (domain mu.delta.v1)
    value_ref: Vec<u8>,     // pointer to key in vault; no secrets
    log_head: Hash32,       // tail of M5 chain at last save (RISK-M1-2)
    mu_sig: [u8; 64],       // μ-key signature over payload (domain mu.core.v1)
}

impl Mu {
    // ── read (no &mut to slots) ───────────────────────────────────────────
    pub fn id(&self) -> &[u8; 16] { &self.core_id }
    pub fn version(&self) -> u16 { self.version }
    pub fn omega(&self) -> &Omega { &self.omega }
    pub fn delta(&self) -> &Delta { &self.delta }
    pub fn owner_pubkey(&self) -> &[u8; 33] { &self.owner_pubkey }
    pub fn log_head(&self) -> Hash32 { self.log_head }
    pub fn value_ref(&self) -> &[u8] { &self.value_ref }

    /// Issue new μ (genesis or reissue base).
    pub fn issue(
        id: [u8; 16],
        owner_pubkey: [u8; 33],
        created_at: u64,
        omega: Omega,
        delta: Delta,
        delta_sig: [u8; 64],
        value_ref: Vec<u8>,
        log_head: Hash32,
        vault: &dyn Vault,
    ) -> Result<Mu, CoreErr> {
        let mut mu = Mu {
            core_id: id, owner_pubkey, created_at, version: 1,
            omega, delta, delta_sig, value_ref, log_head,
            mu_sig: [0u8; 64],
        };
        mu.resign(vault)?;
        Ok(mu)
    }

    fn payload(&self) -> MuPayload<'_> {
        MuPayload {
            core_id: &self.core_id, owner_pubkey: &self.owner_pubkey,
            created_at: self.created_at, version: self.version,
            omega: &self.omega, delta: &self.delta, delta_sig: &self.delta_sig,
            value_ref: &self.value_ref, log_head: &self.log_head,
        }
    }

    fn payload_digest(&self) -> Hash32 {
        let bytes = encode_payload(&self.payload());
        let mut h = Sha256::new();
        h.update(&bytes);
        let out = h.finalize();
        let mut d = [0u8; 32];
        d.copy_from_slice(&out);
        Hash32(d)
    }

    fn resign(&mut self, vault: &dyn Vault) -> Result<(), CoreErr> {
        let d = self.payload_digest();
        let sig = vault.sign_mu(DomainTag::MuCore, &d).map_err(|e| CoreErr::Vault(format!("{e:?}")))?;
        self.mu_sig = sig.0;
        Ok(())
    }

    /// Full verification: μ signature, owner signature on Δ (RISK-M1-4 — parser separately).
    pub fn verify(&self, mu_pubkey: &[u8]) -> Result<(), CoreErr> {
        // μ signature over payload
        let d = tagged_digest(DomainTag::MuCore, &self.payload_digest());
        verify_p256(mu_pubkey, &d, &self.mu_sig).map_err(|_| CoreErr::SigMu)?;
        // owner signature over delta_hash
        let dh = tagged_digest(DomainTag::MuDelta, &delta_hash(&self.delta));
        verify_p256(&self.owner_pubkey, &dh, &self.delta_sig).map_err(|_| CoreErr::SigOwner)?;
        Ok(())
    }

    /// RISK-M1-2: binding with log. Replacing μ with old copy → log_head ≠ chain tail → Err.
    pub fn verify_against_log(&self, log_last_hash: Hash32) -> Result<(), CoreErr> {
        if self.log_head != log_last_hash {
            return Err(CoreErr::LogHeadMismatch);
        }
        Ok(())
    }

    /// The only legal way to change Δ (RISK-M1-1).
    /// new_delta_sig — owner signature (obtained via M9→M4 owner_sign).
    pub fn apply_delta(
        &self,
        new_delta: Delta,
        new_delta_sig: [u8; 64],
        new_log_head: Hash32,
        vault: &dyn Vault,
    ) -> Result<Mu, CoreErr> {
        // verify owner signature BEFORE rebuild
        let dh = tagged_digest(DomainTag::MuDelta, &delta_hash(&new_delta));
        verify_p256(&self.owner_pubkey, &dh, &new_delta_sig).map_err(|_| CoreErr::SigOwner)?;

        let omega_before = crate::format::encode_omega(&self.omega);
        let mut next = self.clone();
        next.delta = new_delta;
        next.delta_sig = new_delta_sig;
        next.log_head = new_log_head;
        // defense-in-depth (RISK-M1-1): Ω bytes unchanged during rebuild
        let omega_after = crate::format::encode_omega(&next.omega);
        if omega_before != omega_after {
            return Err(CoreErr::OmegaImmutable);
        }
        next.resign(vault)?;
        Ok(next)
    }

    /// Ω change is ONLY possible via reissue: new id, version+1.
    pub fn reissue(&self, new_id: [u8; 16], new_omega: Omega, vault: &dyn Vault) -> Result<Mu, CoreErr> {
        let mut next = self.clone();
        next.core_id = new_id;
        next.version = self.version.checked_add(1).ok_or(CoreErr::VersionUnsupported { found: u16::MAX })?;
        next.omega = new_omega;
        next.log_head = Hash32([0u8; 32]); // new μ = new chain (old log is archived)
        next.resign(vault)?;
        Ok(next)
    }

    // ── disk (RISK-M1-3) ─────────────────────────────────────────────────
    pub fn save(&self, path: &Path) -> Result<(), CoreErr> {
        let bytes = crate::format::encode_mu(self);
        if bytes.len() > MU_MAX_SIZE {
            return Err(CoreErr::SizeExceeded);
        }
        // previous version → mu.prev (diagnostics, NOT auto-rollback)
        if path.exists() {
            let _ = fs::copy(path, path.with_extension("prev"));
        }
        let tmp = path.with_extension("tmp");
        {
            let mut f = fs::File::create(&tmp)?;
            f.write_all(&bytes)?;
            f.sync_all()?;
        }
        fs::rename(&tmp, path)?;
        if let Some(dir) = path.parent() {
            if let Ok(d) = fs::File::open(dir) { let _ = d.sync_all(); }
        }
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Mu, CoreErr> {
        let bytes = fs::read(path)?;
        if bytes.len() > MU_MAX_SIZE {
            return Err(CoreErr::SizeExceeded);
        }
        decode_mu(&bytes)
    }

    // format access (crate-private fields)
    pub(crate) fn from_parts(
        core_id: [u8; 16], owner_pubkey: [u8; 33], created_at: u64, version: u16,
        omega: Omega, delta: Delta, delta_sig: [u8; 64],
        value_ref: Vec<u8>, log_head: Hash32, mu_sig: [u8; 64],
    ) -> Mu {
        Mu { core_id, owner_pubkey, created_at, version, omega, delta, delta_sig, value_ref, log_head, mu_sig }
    }
    pub(crate) fn parts(&self) -> (&Omega, &Delta, &[u8; 64], &[u8], &Hash32, &[u8; 64], u64) {
        (&self.omega, &self.delta, &self.delta_sig, &self.value_ref, &self.log_head, &self.mu_sig, self.created_at)
    }
}

fn verify_p256(pubkey: &[u8], digest: &Hash32, sig: &[u8; 64]) -> Result<(), ()> {
    let vk = VerifyingKey::from_sec1_bytes(pubkey).map_err(|_| ())?;
    let s = P256Signature::from_slice(sig).map_err(|_| ())?;
    vk.verify_prehash(&digest.0, &s).map_err(|_| ())
}
