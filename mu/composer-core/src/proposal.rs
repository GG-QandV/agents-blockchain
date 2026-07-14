//! DeltaProposal — неподписанное предложение Δ (спека §5.1).
//! Формат: тот же hardened-TLV-стиль, что mu-wire (одно семейство парсеров на все входы демона).
//! Отклонение от спеки: TLV вместо CBOR — зафиксировано в Ограничениях выполнения README.
use mu_common::{Amount, CanonAddress, Hash32};
use mu_policy::{Delta, WlEntry};

pub const PROPOSAL_MAX: usize = 8192;
pub const MAX_WL: usize = 128;
pub const VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeltaProposal {
    pub new_delta: Delta,
    pub base_delta_hash: Hash32,
    pub ts: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProposalErr {
    TooLarge,
    Malformed,
    BadVersion,
    TooManyEntries,
}

pub fn encode_proposal(p: &DeltaProposal) -> Result<Vec<u8>, ProposalErr> {
    if p.new_delta.whitelist.len() > MAX_WL {
        return Err(ProposalErr::TooManyEntries);
    }
    let mut b = Vec::new();
    b.extend(VERSION.to_be_bytes());
    b.extend(p.new_delta.daily_limit.minor().to_be_bytes());
    b.extend(p.new_delta.confirm_threshold.minor().to_be_bytes());
    b.extend((p.new_delta.whitelist.len() as u16).to_be_bytes());
    for e in &p.new_delta.whitelist {
        b.extend(e.address.chain_id().to_be_bytes());
        b.extend(e.address.bytes());
        let lb = e.label.as_bytes();
        if lb.len() > 255 { return Err(ProposalErr::Malformed); }
        b.push(lb.len() as u8);
        b.extend(lb);
    }
    b.extend(p.base_delta_hash.0);
    b.extend(p.ts.to_be_bytes());
    if b.len() > PROPOSAL_MAX {
        return Err(ProposalErr::TooLarge);
    }
    Ok(b)
}

pub fn decode_proposal(buf: &[u8]) -> Result<DeltaProposal, ProposalErr> {
    if buf.len() > PROPOSAL_MAX { return Err(ProposalErr::TooLarge); }
    let mut c = Cur { b: buf, i: 0 };
    let v = c.u16()?;
    if v != VERSION { return Err(ProposalErr::BadVersion); }
    let daily = Amount::from_minor(c.u128()?);
    let thr = Amount::from_minor(c.u128()?);
    let n = c.u16()? as usize;
    if n > MAX_WL { return Err(ProposalErr::TooManyEntries); }
    let mut wl = Vec::with_capacity(n);
    for _ in 0..n {
        let chain = c.u64()?;
        let addr_bytes = c.take(32)?;
        let mut hex = String::with_capacity(64);
        for by in addr_bytes { hex.push_str(&format!("{:02x}", by)); }
        let address = CanonAddress::canon(&hex, chain).map_err(|_| ProposalErr::Malformed)?;
        let ll = c.u8()? as usize;
        let lb = c.take(ll)?;
        let label = String::from_utf8(lb.to_vec()).map_err(|_| ProposalErr::Malformed)?;
        wl.push(WlEntry { address, label });
    }
    let bh = c.take(32)?;
    let mut base = [0u8; 32];
    base.copy_from_slice(bh);
    let ts = c.u64()?;
    if c.remaining() != 0 { return Err(ProposalErr::Malformed); } // строгая схема
    Ok(DeltaProposal {
        new_delta: Delta { daily_limit: daily, whitelist: wl, confirm_threshold: thr },
        base_delta_hash: Hash32(base),
        ts,
    })
}

struct Cur<'a> { b: &'a [u8], i: usize }
impl<'a> Cur<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], ProposalErr> {
        let end = self.i.checked_add(n).ok_or(ProposalErr::Malformed)?;
        if end > self.b.len() { return Err(ProposalErr::Malformed); }
        let s = &self.b[self.i..end];
        self.i = end;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, ProposalErr> { Ok(self.take(1)?[0]) }
    fn u16(&mut self) -> Result<u16, ProposalErr> {
        let s = self.take(2)?; Ok(u16::from_be_bytes([s[0], s[1]]))
    }
    fn u64(&mut self) -> Result<u64, ProposalErr> {
        let s = self.take(8)?; let mut a = [0u8; 8]; a.copy_from_slice(s); Ok(u64::from_be_bytes(a))
    }
    fn u128(&mut self) -> Result<u128, ProposalErr> {
        let s = self.take(16)?; let mut a = [0u8; 16]; a.copy_from_slice(s); Ok(u128::from_be_bytes(a))
    }
    fn remaining(&self) -> usize { self.b.len().saturating_sub(self.i) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mu_common::Amount;

    fn sample() -> DeltaProposal {
        DeltaProposal {
            new_delta: Delta {
                daily_limit: Amount::from_minor(500),
                whitelist: vec![WlEntry {
                    address: CanonAddress::canon("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 8453).unwrap(),
                    label: "API".into(),
                }],
                confirm_threshold: Amount::from_minor(100),
            },
            base_delta_hash: Hash32([5; 32]),
            ts: 1234,
        }
    }
    #[test]
    fn roundtrip() {
        let p = sample();
        let b = encode_proposal(&p).unwrap();
        assert_eq!(decode_proposal(&b).unwrap(), p);
    }
    #[test]
    fn trailing_bytes_rejected() {
        let mut b = encode_proposal(&sample()).unwrap();
        b.push(0);
        assert_eq!(decode_proposal(&b), Err(ProposalErr::Malformed));
    }
    #[test]
    fn no_panic_on_junk() {
        for len in 0..120 {
            let junk: Vec<u8> = (0..len).map(|i| (i * 37) as u8).collect();
            let _ = decode_proposal(&junk);
        }
    }
}
