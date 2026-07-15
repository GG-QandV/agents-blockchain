//! Байтовый формат μ (RISK-M1-4): строгий TLV, закрытая схема, лишний байт = ошибка.
//! Семейство hardened-парсеров mu-wire: bounds-checked курсор, без паник.
use crate::object::{CoreErr, Mu, Omega};
use mu_common::{Amount, CanonAddress, Hash32};
use mu_policy::{Delta, WlEntry};

pub const FORMAT_VERSION: u16 = 1;

/// Ссылочный вид payload для encode (без владения).
pub struct MuPayload<'a> {
    pub core_id: &'a [u8; 16],
    pub owner_pubkey: &'a [u8; 33],
    pub created_at: u64,
    pub version: u16,
    pub omega: &'a Omega,
    pub delta: &'a Delta,
    pub delta_sig: &'a [u8; 64],
    pub value_ref: &'a [u8],
    pub log_head: &'a Hash32,
}

pub fn encode_omega(o: &Omega) -> Vec<u8> {
    let mut b = Vec::new();
    b.push(o.connectors.len() as u8);
    b.extend(&o.connectors);
    b.extend(o.max_ceiling.to_be_bytes());
    b
}

fn encode_delta(d: &Delta) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend(d.daily_limit.minor().to_be_bytes());
    b.extend(d.confirm_threshold.minor().to_be_bytes());
    b.extend((d.whitelist.len() as u16).to_be_bytes());
    for e in &d.whitelist {
        b.extend(e.address.chain_id().to_be_bytes());
        b.extend(e.address.bytes());
        b.push(e.label.len() as u8);
        b.extend(e.label.as_bytes());
    }
    b
}

pub fn encode_payload(p: &MuPayload<'_>) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend(FORMAT_VERSION.to_be_bytes());
    b.extend(p.core_id);
    b.extend(p.owner_pubkey);
    b.extend(p.created_at.to_be_bytes());
    b.extend(p.version.to_be_bytes());
    let om = encode_omega(p.omega);
    b.extend((om.len() as u16).to_be_bytes());
    b.extend(om);
    let de = encode_delta(p.delta);
    b.extend((de.len() as u16).to_be_bytes());
    b.extend(de);
    b.extend(p.delta_sig);
    b.extend((p.value_ref.len() as u16).to_be_bytes());
    b.extend(p.value_ref);
    b.extend(p.log_head.0);
    b
}

/// Файл = payload ‖ mu_sig[64].
pub fn encode_mu(mu: &Mu) -> Vec<u8> {
    let (omega, delta, dsig, vref, lhead, msig, created) = mu.parts();
    let p = MuPayload {
        core_id: mu.id(), owner_pubkey: mu.owner_pubkey(), created_at: created,
        version: mu.version(), omega, delta, delta_sig: dsig, value_ref: vref, log_head: lhead,
    };
    let mut out = encode_payload(&p);
    out.extend(msig);
    out
}

pub fn decode_mu(buf: &[u8]) -> Result<Mu, CoreErr> {
    if buf.len() < 64 { return Err(CoreErr::CborMalformed); }
    let (payload, sig) = buf.split_at(buf.len() - 64);
    let mut msig = [0u8; 64];
    msig.copy_from_slice(sig);

    let mut c = Cur { b: payload, i: 0 };
    let fv = c.u16()?;
    if fv != FORMAT_VERSION { return Err(CoreErr::VersionUnsupported { found: fv }); }
    let mut core_id = [0u8; 16]; core_id.copy_from_slice(c.take(16)?);
    let mut opk = [0u8; 33]; opk.copy_from_slice(c.take(33)?);
    let created = c.u64()?;
    let ver = c.u16()?;

    let olen = c.u16()? as usize;
    let ob = c.take(olen)?;
    let omega = decode_omega(ob)?;

    let dlen = c.u16()? as usize;
    let db = c.take(dlen)?;
    let delta = decode_delta(db)?;

    let mut dsig = [0u8; 64]; dsig.copy_from_slice(c.take(64)?);
    let vlen = c.u16()? as usize;
    if vlen > 512 { return Err(CoreErr::CborMalformed); }
    let vref = c.take(vlen)?.to_vec();
    let mut lh = [0u8; 32]; lh.copy_from_slice(c.take(32)?);

    if c.remaining() != 0 { return Err(CoreErr::CborMalformed); } // строгая схема (RISK-M1-4)

    Ok(Mu::from_parts(core_id, opk, created, ver, omega, delta, dsig, vref, Hash32(lh), msig))
}

fn decode_omega(b: &[u8]) -> Result<Omega, CoreErr> {
    let mut c = Cur { b, i: 0 };
    let n = c.u8()? as usize;
    if n > 16 { return Err(CoreErr::CborMalformed); }
    let connectors = c.take(n)?.to_vec();
    let ceiling = c.u128()?;
    if c.remaining() != 0 { return Err(CoreErr::CborMalformed); }
    Ok(Omega { connectors, max_ceiling: ceiling })
}

fn decode_delta(b: &[u8]) -> Result<Delta, CoreErr> {
    let mut c = Cur { b, i: 0 };
    let daily = Amount::from_minor(c.u128()?);
    let thr = Amount::from_minor(c.u128()?);
    let n = c.u16()? as usize;
    if n > 128 { return Err(CoreErr::CborMalformed); }
    let mut wl = Vec::with_capacity(n);
    for _ in 0..n {
        let chain = c.u64()?;
        let ab = c.take(20)?;
        let mut hex = String::with_capacity(40);
        for by in ab { hex.push_str(&format!("{:02x}", by)); }
        let address = CanonAddress::canon(&hex, chain).map_err(|_| CoreErr::CborMalformed)?;
        let ll = c.u8()? as usize;
        let label = String::from_utf8(c.take(ll)?.to_vec()).map_err(|_| CoreErr::CborMalformed)?;
        wl.push(WlEntry { address, label });
    }
    if c.remaining() != 0 { return Err(CoreErr::CborMalformed); }
    Ok(Delta { daily_limit: daily, whitelist: wl, confirm_threshold: thr })
}

struct Cur<'a> { b: &'a [u8], i: usize }
impl<'a> Cur<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], CoreErr> {
        let end = self.i.checked_add(n).ok_or(CoreErr::CborMalformed)?;
        if end > self.b.len() { return Err(CoreErr::CborMalformed); }
        let s = &self.b[self.i..end]; self.i = end; Ok(s)
    }
    fn u8(&mut self) -> Result<u8, CoreErr> { Ok(self.take(1)?[0]) }
    fn u16(&mut self) -> Result<u16, CoreErr> { let s = self.take(2)?; Ok(u16::from_be_bytes([s[0], s[1]])) }
    fn u64(&mut self) -> Result<u64, CoreErr> { let s = self.take(8)?; let mut a=[0u8;8]; a.copy_from_slice(s); Ok(u64::from_be_bytes(a)) }
    fn u128(&mut self) -> Result<u128, CoreErr> { let s = self.take(16)?; let mut a=[0u8;16]; a.copy_from_slice(s); Ok(u128::from_be_bytes(a)) }
    fn remaining(&self) -> usize { self.b.len().saturating_sub(self.i) }
}
