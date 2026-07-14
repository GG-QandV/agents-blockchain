//! RISK-M4-5: доменные теги. sign_* сами добавляют тег в digest,
//! поэтому одну и ту же подпись нельзя переиспользовать в другом домене.
use mu_common::Hash32;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DomainTag {
    MuCore,
    MuLog,
    MuDelta,
    MuHuman,
}

impl DomainTag {
    pub fn as_bytes(self) -> &'static [u8] {
        match self {
            DomainTag::MuCore => b"mu.core.v1",
            DomainTag::MuLog => b"mu.log.v1",
            DomainTag::MuDelta => b"mu.delta.v1",
            DomainTag::MuHuman => b"mu.human.v1",
        }
    }
}

/// digest_с_тегом = SHA256(tag ‖ payload_digest).
pub fn tagged_digest(tag: DomainTag, payload: &Hash32) -> Hash32 {
    let mut h = Sha256::new();
    h.update(tag.as_bytes());
    h.update(payload.0);
    let out = h.finalize();
    let mut d = [0u8; 32];
    d.copy_from_slice(&out);
    Hash32(d)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tags_separate_domains() {
        let p = Hash32([7u8; 32]);
        // один payload, разные теги → разные итоговые digests (RISK-M4-5)
        assert_ne!(
            tagged_digest(DomainTag::MuCore, &p).0,
            tagged_digest(DomainTag::MuDelta, &p).0
        );
        assert_ne!(
            tagged_digest(DomainTag::MuLog, &p).0,
            tagged_digest(DomainTag::MuHuman, &p).0
        );
    }
}
