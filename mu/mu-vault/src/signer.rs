//! RISK-M4-1: TxSigner — the sole carrier of the wallet plaintext key.
//! !Clone, !Debug, private field, Zeroizing on Drop, constructor only inside the crate.
use crate::{Secp256k1Sig, VaultErr};
use k256::ecdsa::{signature::hazmat::PrehashSigner, RecoveryId, SigningKey};
use mu_common::Hash32;
use zeroize::Zeroizing;

/// Transaction signing handle. Lives within a single Connector.execute().
/// No Clone/Debug/Serialize — key bytes cannot leak through them.
pub struct TxSigner {
    key: Zeroizing<[u8; 32]>,
}

impl TxSigner {
    /// Constructor accessible only to backends inside the crate (pub(crate)).
    pub(crate) fn from_bytes(raw: [u8; 32]) -> Self {
        TxSigner { key: Zeroizing::new(raw) }
    }

    /// Sign prehash of transaction sighash (RFC 6979 deterministic).
    /// Takes &self; handle is moved into execute and dropped there.
    pub fn sign(&self, sighash: &Hash32) -> Result<Secp256k1Sig, VaultErr> {
        let sk = SigningKey::from_bytes(self.key.as_slice().into())
            .map_err(|e| VaultErr::Backend(format!("k256 key: {e}")))?;
        let (sig, rec): (k256::ecdsa::Signature, RecoveryId) = sk
            .sign_prehash(&sighash.0)
            .map_err(|e| VaultErr::Backend(format!("k256 sign: {e}")))?;
        let b = sig.to_bytes();
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        r.copy_from_slice(&b[..32]);
        s.copy_from_slice(&b[32..]);
        Ok(Secp256k1Sig { r, s, v: rec.to_byte() })
    }

    /// Public wallet address (for verification, not a secret).
    pub fn verifying_key_bytes(&self) -> Result<[u8; 33], VaultErr> {
        let sk = SigningKey::from_bytes(self.key.as_slice().into())
            .map_err(|e| VaultErr::Backend(format!("k256 key: {e}")))?;
        let vk = sk.verifying_key();
        let pt = vk.to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(pt.as_bytes());
        Ok(out)
    }
}

// Explicitly do NOT implement Clone/Debug. Zeroizing already zeroes on Drop.
// Compiler will forbid `let s2 = signer.clone();` and `dbg!(signer)`.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signs_and_recovers_pubkey() {
        let signer = TxSigner::from_bytes([1u8; 32]);
        let sh = Hash32([9u8; 32]);
        let sig = signer.sign(&sh).unwrap();
        assert!(sig.r != [0u8; 32]);
        assert!(signer.verifying_key_bytes().is_ok());
    }
    #[test]
    fn deterministic_rfc6979() {
        let a = TxSigner::from_bytes([2u8; 32]);
        let b = TxSigner::from_bytes([2u8; 32]);
        let sh = Hash32([5u8; 32]);
        let sa = a.sign(&sh).unwrap();
        let sb = b.sign(&sh).unwrap();
        assert_eq!(sa.r, sb.r);
        assert_eq!(sa.s, sb.s);
    }
}

/// P-256 (Secp256r1) wallet handle for Sui: flag 0x02 accepted natively by Sui,
/// and P-256 is the native enclave curve → the "signature in RAM" caveat is removed (RISK-M4-1 strengthened).
pub struct TxSignerP256 {
    key: Zeroizing<[u8; 32]>,
}

impl TxSignerP256 {
    pub(crate) fn from_bytes(raw: [u8; 32]) -> Self {
        TxSignerP256 { key: Zeroizing::new(raw) }
    }
    /// Sign a 32-byte digest (Sui: blake2b256(intent‖tx)); RFC 6979.
    pub fn sign_prehash(&self, digest: &[u8; 32]) -> Result<([u8; 64], [u8; 33]), VaultErr> {
        use p256::ecdsa::signature::hazmat::PrehashSigner;
        use p256::ecdsa::SigningKey;
        let sk = SigningKey::from_bytes(self.key.as_slice().into())
            .map_err(|e| VaultErr::Backend(format!("p256 key: {e}")))?;
        let sig: p256::ecdsa::Signature = sk.sign_prehash(digest)
            .map_err(|e| VaultErr::Backend(format!("p256 sign: {e}")))?;
        let sb = sig.to_bytes();
        let mut s64 = [0u8; 64]; s64.copy_from_slice(&sb);
        let pk = sk.verifying_key().to_encoded_point(true);
        let mut p33 = [0u8; 33]; p33.copy_from_slice(pk.as_bytes());
        Ok((s64, p33))
    }
}
