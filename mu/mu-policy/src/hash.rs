//! delta_hash — deterministic SHA-256 of canonical byte representation Δ.
//! Used for base_delta_hash (TOCTOU, spec §5.1) and DeltaChanged in the log.
use crate::types::{Delta, ResourceRule, WlEntry};
use mu_common::{Amount, CanonAddress};
use mu_common::Hash32;
use sha2::{Digest, Sha256};

/// Canonical bytes of Δ: every field, including the resource rules, in a
/// sorted order, length-prefixed so no two different Δ can encode alike.
pub fn delta_bytes(d: &Delta) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&d.daily_limit.minor().to_be_bytes());
    b.extend_from_slice(&d.confirm_threshold.minor().to_be_bytes());
    let mut wl: Vec<_> = d.whitelist.iter().collect();
    wl.sort_by_key(|e| (e.address.chain_id(), *e.address.bytes()));
    b.extend_from_slice(&(wl.len() as u32).to_be_bytes());
    for e in wl {
        b.extend_from_slice(&e.address.chain_id().to_be_bytes());
        b.extend_from_slice(e.address.bytes());
        let label = e.label.as_bytes();
        b.extend_from_slice(&(label.len() as u32).to_be_bytes());
        b.extend_from_slice(label);
    }
    let mut rl: Vec<_> = d.resource_allowlist.iter().collect();
    rl.sort_by_key(|r| (r.host.clone(), r.path_prefix.clone()));
    b.extend_from_slice(&(rl.len() as u32).to_be_bytes());
    for r in rl {
        for s in [&r.host, &r.path_prefix] {
            b.extend_from_slice(&(s.len() as u32).to_be_bytes());
            b.extend_from_slice(s.as_bytes());
        }
        b.extend_from_slice(&r.max_price_per_call.minor().to_be_bytes());
    }
    b
}

/// The inverse of `delta_bytes`; `None` for anything malformed.
pub fn delta_from_bytes(b: &[u8]) -> Option<Delta> {
    let mut p = Parser { b, i: 0 };
    let daily_limit = Amount::from_minor(p.u128()?);
    let confirm_threshold = Amount::from_minor(p.u128()?);
    let n = p.u32()? as usize;
    if n > 4096 {
        return None;
    }
    let mut whitelist = Vec::with_capacity(n);
    for _ in 0..n {
        let chain_id = p.u64()?;
        let addr = p.arr20()?;
        let label = p.string()?;
        whitelist.push(WlEntry { address: CanonAddress::from_raw(addr, chain_id), label });
    }
    let m = p.u32()? as usize;
    if m > 4096 {
        return None;
    }
    let mut resource_allowlist = Vec::with_capacity(m);
    for _ in 0..m {
        let host = p.string()?;
        let path_prefix = p.string()?;
        let max_price_per_call = Amount::from_minor(p.u128()?);
        resource_allowlist.push(ResourceRule { host, path_prefix, max_price_per_call });
    }
    if p.i != b.len() {
        return None;
    }
    Some(Delta { daily_limit, whitelist, confirm_threshold, resource_allowlist })
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}
impl<'a> Parser<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let end = self.i.checked_add(n)?;
        let s = self.b.get(self.i..end)?;
        self.i = end;
        Some(s)
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_be_bytes(self.take(4)?.try_into().ok()?))
    }
    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_be_bytes(self.take(8)?.try_into().ok()?))
    }
    fn u128(&mut self) -> Option<u128> {
        Some(u128::from_be_bytes(self.take(16)?.try_into().ok()?))
    }
    fn arr20(&mut self) -> Option<[u8; 20]> {
        self.take(20)?.try_into().ok()
    }
    fn string(&mut self) -> Option<String> {
        let n = self.u32()? as usize;
        String::from_utf8(self.take(n)?.to_vec()).ok()
    }
}

pub fn delta_hash(d: &Delta) -> Hash32 {
    let mut h = Sha256::new();
    h.update(delta_bytes(d));
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
        let d1 = Delta {
            daily_limit: Amount::from_minor(1),
            whitelist: vec![a.clone(), b.clone()],
            confirm_threshold: Amount::ZERO,
            resource_allowlist: vec![],
        };
        let d2 = Delta {
            daily_limit: Amount::from_minor(1),
            whitelist: vec![b, a],
            confirm_threshold: Amount::ZERO,
            resource_allowlist: vec![],
        };
        assert_eq!(delta_hash(&d1), delta_hash(&d2));
    }
    #[test]
    fn any_field_changes_hash() {
        let base = Delta {
            daily_limit: Amount::from_minor(1),
            whitelist: vec![],
            confirm_threshold: Amount::ZERO,
            resource_allowlist: vec![],
        };
        let mut c1 = base.clone();
        c1.daily_limit = Amount::from_minor(2);
        let mut c2 = base.clone();
        c2.confirm_threshold = Amount::from_minor(1);
        assert_ne!(delta_hash(&base), delta_hash(&c1));
        assert_ne!(delta_hash(&base), delta_hash(&c2));
    }
}
