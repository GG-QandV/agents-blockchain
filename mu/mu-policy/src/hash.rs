//! delta_hash — детерминированный SHA-256 канонической байтовой формы Δ.
//! Используется для base_delta_hash (TOCTOU, спека §5.1) и DeltaChanged в логе.
use crate::types::Delta;
use mu_common::Hash32;
use sha2::{Digest, Sha256};

pub fn delta_hash(d: &Delta) -> Hash32 {
    let mut h = Sha256::new();
    h.update(d.daily_limit.minor().to_be_bytes());
    h.update((d.whitelist.len() as u32).to_be_bytes());
    // канонический порядок: сортировка по (chain_id, addr) — порядок ввода не влияет на hash
    let mut wl: Vec<_> = d.whitelist.iter().collect();
    wl.sort_by_key(|e| (e.address.chain_id(), *e.address.bytes()));
    for e in wl {
        h.update(e.address.chain_id().to_be_bytes());
        h.update(e.address.bytes());
        h.update((e.label.len() as u32).to_be_bytes());
        h.update(e.label.as_bytes());
    }
    h.update(d.confirm_threshold.minor().to_be_bytes());
    let out = h.finalize();
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    Hash32(a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::WlEntry;
    use mu_common::{Amount, CanonAddress};

    fn wl(a: &str, l: &str) -> WlEntry {
        WlEntry { address: CanonAddress::canon(a, 8453).unwrap(), label: l.into() }
    }
    #[test]
    fn order_independent() {
        let a = wl("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "A");
        let b = wl("0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", "B");
        let d1 = Delta { daily_limit: Amount::from_minor(1), whitelist: vec![a.clone(), b.clone()], confirm_threshold: Amount::ZERO };
        let d2 = Delta { daily_limit: Amount::from_minor(1), whitelist: vec![b, a], confirm_threshold: Amount::ZERO };
        assert_eq!(delta_hash(&d1), delta_hash(&d2));
    }
    #[test]
    fn any_field_changes_hash() {
        let base = Delta { daily_limit: Amount::from_minor(1), whitelist: vec![], confirm_threshold: Amount::ZERO };
        let mut c1 = base.clone(); c1.daily_limit = Amount::from_minor(2);
        let mut c2 = base.clone(); c2.confirm_threshold = Amount::from_minor(1);
        assert_ne!(delta_hash(&base), delta_hash(&c1));
        assert_ne!(delta_hash(&base), delta_hash(&c2));
    }
}
