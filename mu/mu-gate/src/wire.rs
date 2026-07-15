//! RISK-M8-3: hardened-парсер проводного формата. Ручной, без общих CBOR-библиотек,
//! фиксированная схема, жёсткие лимиты, ни одной паники на любом входе.
//!
//! Формат кадра: len:u32 LE (≤ MAX_FRAME) ‖ body
//! body = payload ‖ sig[64]
//! payload (наша минимальная TLV-схема, детерминированная):
//!   v:u16 ‖ recipient_len:u8 ‖ recipient[..] ‖ amount:u128(BE)
//!   ‖ chain_id:u64(BE) ‖ agent_len:u8 ‖ agent_id[..] ‖ nonce:u64(BE) ‖ ts:u64(BE)

pub const MAX_FRAME: usize = 4096;
const MAX_RECIPIENT: usize = 64;
const MAX_AGENT: usize = 64;
const SIG_LEN: usize = 64;

#[derive(Debug, PartialEq, Eq)]
pub enum WireErr {
    TooShort,
    TooLong,
    BadVersion,
    BadLength,
    Truncated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WireIntent {
    pub recipient: String,
    pub amount: u128,
    pub chain_id: u64,
    pub agent_id: String,
    pub nonce: u64,
    pub ts: u64,
    /// payload-байты (для проверки подписи) и сама подпись.
    pub signed_bytes: Vec<u8>,
    pub sig: [u8; SIG_LEN],
}

/// Курсор с границами: ни один read не выходит за буфер (RISK-M8-3).
struct Cur<'a> {
    b: &'a [u8],
    i: usize,
}
impl<'a> Cur<'a> {
    fn new(b: &'a [u8]) -> Self { Cur { b, i: 0 } }
    fn take(&mut self, n: usize) -> Result<&'a [u8], WireErr> {
        let end = self.i.checked_add(n).ok_or(WireErr::BadLength)?;
        if end > self.b.len() { return Err(WireErr::Truncated); }
        let s = &self.b[self.i..end];
        self.i = end;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, WireErr> { Ok(self.take(1)?[0]) }
    fn u16(&mut self) -> Result<u16, WireErr> {
        let s = self.take(2)?;
        Ok(u16::from_be_bytes([s[0], s[1]]))
    }
    fn u64(&mut self) -> Result<u64, WireErr> {
        let s = self.take(8)?;
        let mut a = [0u8; 8]; a.copy_from_slice(s); Ok(u64::from_be_bytes(a))
    }
    fn u128(&mut self) -> Result<u128, WireErr> {
        let s = self.take(16)?;
        let mut a = [0u8; 16]; a.copy_from_slice(s); Ok(u128::from_be_bytes(a))
    }
    fn remaining(&self) -> usize { self.b.len().saturating_sub(self.i) }
}

/// Разбор кадра целиком (уже без длины-префикса — её снимает транспорт).
pub fn parse_wire(frame: &[u8]) -> Result<WireIntent, WireErr> {
    if frame.len() > MAX_FRAME { return Err(WireErr::TooLong); }
    if frame.len() < SIG_LEN + 4 { return Err(WireErr::TooShort); }
    // payload = всё кроме последних 64 байт подписи
    let split = frame.len().checked_sub(SIG_LEN).ok_or(WireErr::TooShort)?;
    let (payload, sig_bytes) = frame.split_at(split);

    let mut c = Cur::new(payload);
    let v = c.u16()?;
    if v != 1 { return Err(WireErr::BadVersion); }

    let rlen = usize::from(c.u8()?);
    if rlen == 0 || rlen > MAX_RECIPIENT { return Err(WireErr::BadLength); }
    let rbytes = c.take(rlen)?;
    let recipient = ascii_string(rbytes)?;

    let amount = c.u128()?;
    let chain_id = c.u64()?;

    let alen = usize::from(c.u8()?);
    if alen == 0 || alen > MAX_AGENT { return Err(WireErr::BadLength); }
    let abytes = c.take(alen)?;
    let agent_id = ascii_string(abytes)?;

    let nonce = c.u64()?;
    let ts = c.u64()?;

    // лишние байты в payload запрещены (строгая схема, RISK-M8-3)
    if c.remaining() != 0 { return Err(WireErr::BadLength); }

    let mut sig = [0u8; SIG_LEN];
    sig.copy_from_slice(sig_bytes);

    Ok(WireIntent {
        recipient, amount, chain_id, agent_id, nonce, ts,
        signed_bytes: payload.to_vec(),
        sig,
    })
}

fn ascii_string(b: &[u8]) -> Result<String, WireErr> {
    // печатаемый ASCII без управляющих (RISK-M9-3 против скрытых символов на входе)
    if b.iter().any(|&x| !(0x20..=0x7e).contains(&x)) {
        return Err(WireErr::BadLength);
    }
    String::from_utf8(b.to_vec()).map_err(|_| WireErr::BadLength)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(v: u16, recipient: &str, agent: &str, nonce: u64, ts: u64) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&v.to_be_bytes());
        p.push(recipient.len() as u8);
        p.extend_from_slice(recipient.as_bytes());
        p.extend_from_slice(&5_000_000u128.to_be_bytes());
        p.extend_from_slice(&8453u64.to_be_bytes());
        p.push(agent.len() as u8);
        p.extend_from_slice(agent.as_bytes());
        p.extend_from_slice(&nonce.to_be_bytes());
        p.extend_from_slice(&ts.to_be_bytes());
        p.extend_from_slice(&[0u8; SIG_LEN]); // фиктивная подпись
        p
    }

    #[test]
    fn parses_valid() {
        let f = build(1, "0xabc", "agent-1", 7, 1000);
        let w = parse_wire(&f).unwrap();
        assert_eq!(w.agent_id, "agent-1");
        assert_eq!(w.nonce, 7);
        assert_eq!(w.amount, 5_000_000);
    }
    #[test]
    fn rejects_bad_version() {
        let f = build(2, "0xabc", "a", 1, 1);
        assert_eq!(parse_wire(&f), Err(WireErr::BadVersion));
    }
    #[test]
    fn rejects_too_long() {
        let f = vec![0u8; MAX_FRAME + 1];
        assert_eq!(parse_wire(&f), Err(WireErr::TooLong));
    }
    #[test]
    fn rejects_trailing_garbage() {
        let mut f = build(1, "0xabc", "a", 1, 1);
        f.insert(f.len() - SIG_LEN, 0xFF); // лишний байт в payload
        assert_eq!(parse_wire(&f), Err(WireErr::BadLength));
    }
    #[test]
    fn no_panic_on_fuzz_like_inputs() {
        // RISK-M8-3: любой мусор не роняет парсер
        for len in 0..200 {
            for seed in 0..8u8 {
                let junk: Vec<u8> = (0..len).map(|i| (i as u8) ^ seed).collect();
                let _ = parse_wire(&junk); // не паникует
            }
        }
    }
    #[test]
    fn rejects_control_chars_in_id() {
        let f = build(1, "0x\x01bc", "a", 1, 1);
        assert_eq!(parse_wire(&f), Err(WireErr::BadLength));
    }
}
