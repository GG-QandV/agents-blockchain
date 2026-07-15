//! RISK-M4-1: TxSigner — единственный носитель plaintext-ключа кошелька.
//! !Clone, !Debug, приватное поле, Zeroizing при Drop, конструктор только внутри крейта.
use crate::{Secp256k1Sig, VaultErr};
use k256::ecdsa::{signature::hazmat::PrehashSigner, RecoveryId, SigningKey};
use mu_common::Hash32;
use zeroize::Zeroizing;

/// Хэндл подписи транзакций. Живёт в пределах одного Connector.execute().
/// Никаких Clone/Debug/Serialize — байты ключа не могут утечь через них.
pub struct TxSigner {
    key: Zeroizing<[u8; 32]>,
}

impl TxSigner {
    /// Конструктор доступен только backend'ам внутри крейта (pub(crate)).
    pub(crate) fn from_bytes(raw: [u8; 32]) -> Self {
        TxSigner { key: Zeroizing::new(raw) }
    }

    /// Подпись prehash sighash транзакции (RFC 6979 детерминированная).
    /// Принимает по &self; хэндл передаётся в execute по move и там роняется.
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

    /// Публичный адрес кошелька (для сверки, не секрет).
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

// Явно НЕ реализуем Clone/Debug. Zeroizing уже зануляет при Drop.
// Компилятор запретит `let s2 = signer.clone();` и `dbg!(signer)`.

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
        assert_eq!(sa.r, sb.r); // RFC 6979: детерминированная подпись
        assert_eq!(sa.s, sb.s);
    }
}



/// P-256 (Secp256r1) хэндл кошелька для Sui: flag 0x02 принят Sui нативно,
/// а P-256 — родная кривая enclave → оговорка «подпись в RAM» снимается (RISK-M4-1 усилен).
pub struct TxSignerP256 {
    key: Zeroizing<[u8; 32]>,
}

impl TxSignerP256 {
    pub(crate) fn from_bytes(raw: [u8; 32]) -> Self {
        TxSignerP256 { key: Zeroizing::new(raw) }
    }
    /// Подпись 32-байтного дайджеста (Sui: blake2b256(intent‖tx)); RFC 6979.
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
