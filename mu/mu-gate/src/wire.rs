//! RISK-M8-3: hardened wire format parser. Manual, no common CBOR libraries,
//! fixed schema, strict limits, zero panics on any input.
//!
//! Frame format: len:u32 LE (≤ MAX_FRAME) ‖ body
//! body = payload ‖ sig[64]
//! payload (our minimal TLV schema, deterministic):
//!   v:u16 ‖ recipient_len:u8 ‖ recipient[..] ‖ amount:u128(BE)
//!   ‖ chain_id:u64(BE) ‖ agent_len:u8 ‖ agent_id[..] ‖ nonce:u64(BE) ‖ ts:u64(BE)

pub const MAX_FRAME: usize = 4096;
const MAX_RECIPIENT: usize = 64;
const MAX_AGENT: usize = 64;
const MAX_RESOURCE: usize = 256;
const SIG_LEN: usize = 64;

#[derive(Debug, PartialEq, Eq)]
pub enum WireErr {
    TooShort,
    TooLong,
    BadVersion,
    BadLength,
    Truncated,
    /// The offer is larger than the protocol allows.
    OfferTooLong,
    /// The offer is not the strict 402 JSON we accept.
    BadOffer,
}

/// The largest 402 body a v3 payload may carry.
pub const MAX_OFFER: usize = 2048;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WireIntent {
    pub recipient: String,
    pub amount: u128,
    pub chain_id: u64,
    pub agent_id: String,
    pub nonce: u64,
    pub ts: u64,
    /// Resource URL (v2+): host + path for the x402 resource check.
    pub resource: Option<String>,
    /// The 402 body (v3 only), inside the signed payload.
    pub offer: Option<String>,
    /// payload bytes (for signature verification) and the signature itself.
    pub signed_bytes: Vec<u8>,
    pub sig: [u8; SIG_LEN],
}

/// Bounds-checked cursor: no read exceeds the buffer (RISK-M8-3).
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

/// Parse full frame (without length prefix — transport strips it).
pub fn parse_wire(frame: &[u8]) -> Result<WireIntent, WireErr> {
    if frame.len() > MAX_FRAME { return Err(WireErr::TooLong); }
    if frame.len() < SIG_LEN + 4 { return Err(WireErr::TooShort); }
    // payload = everything except the last 64 bytes (signature)
    let split = frame.len().checked_sub(SIG_LEN).ok_or(WireErr::TooShort)?;
    let (payload, sig_bytes) = frame.split_at(split);

    let mut c = Cur::new(payload);
    let v = c.u16()?;
    if !(1..=3).contains(&v) { return Err(WireErr::BadVersion); }

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

    // v2 adds the resource (host + path) to the signed payload; v1 has none.
    let resource = if v >= 2 {
        let rlen = usize::from(c.u8()?);
        if rlen > MAX_RESOURCE { return Err(WireErr::BadLength); }
        if rlen == 0 {
            None
        } else {
            let rbytes = c.take(rlen)?;
            Some(ascii_string(rbytes)?)
        }
    } else {
        None
    };

    // v3 adds the 402 offer: u16 BE length + the body, inside the signature.
    let offer = if v == 3 {
        let olen = usize::from(c.u16()?);
        if olen == 0 || olen > MAX_OFFER { return Err(WireErr::OfferTooLong); }
        let obytes = c.take(olen)?;
        let s = std::str::from_utf8(obytes).map_err(|_| WireErr::BadOffer)?;
        validate_offer(s)?;
        Some(s.to_string())
    } else {
        None
    };

    // extra bytes in payload are forbidden (strict schema, RISK-M8-3)
    if c.remaining() != 0 { return Err(WireErr::BadLength); }

    let mut sig = [0u8; SIG_LEN];
    sig.copy_from_slice(sig_bytes);

    Ok(WireIntent {
        recipient, amount, chain_id, agent_id, nonce, ts, resource, offer,
        signed_bytes: payload.to_vec(),
        sig,
    })
}

/// Strict 402 body: only the keys we read, nothing else. A permissive parser
/// would let an agent smuggle fields past the connector's own checks.
fn validate_offer(s: &str) -> Result<(), WireErr> {
    let v: serde_json::Value = serde_json::from_str(s).map_err(|_| WireErr::BadOffer)?;
    let obj = v.as_object().ok_or(WireErr::BadOffer)?;
    for k in obj.keys() {
        if !matches!(k.as_str(), "x402Version" | "error" | "accepts") {
            return Err(WireErr::BadOffer);
        }
    }
    if !matches!(obj.get("x402Version").and_then(|x| x.as_u64()), Some(1)) {
        return Err(WireErr::BadOffer);
    }
    let accepts = obj.get("accepts").and_then(|a| a.as_array()).ok_or(WireErr::BadOffer)?;
    if accepts.is_empty() {
        return Err(WireErr::BadOffer);
    }
    for a in accepts {
        let e = a.as_object().ok_or(WireErr::BadOffer)?;
        for k in e.keys() {
            if !matches!(
                k.as_str(),
                "scheme" | "network" | "maxAmountRequired" | "payTo" | "asset" | "resource"
                    | "description" | "mimeType" | "maxTimeoutSeconds" | "extra"
            ) {
                return Err(WireErr::BadOffer);
            }
        }
        let extra = e.get("extra").and_then(|x| x.as_object());
        if let Some(x) = extra {
            for k in x.keys() {
                if !matches!(k.as_str(), "name" | "version") {
                    return Err(WireErr::BadOffer);
                }
            }
        }
    }
    Ok(())
}

/// Frame v4: the agent asks for the header of an earlier request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeaderRequest {
    pub agent_id: String,
    /// The request nonce of the original intent (v3 payload `nonce`).
    pub ref_nonce: u64,
    pub ts: u64,
    pub signed_bytes: Vec<u8>,
    pub sig: [u8; SIG_LEN],
}

/// Either an intent (v1-v3) or a header request (v4).
#[derive(Debug, PartialEq, Eq)]
pub enum Frame {
    Intent(WireIntent),
    HeaderRequest(HeaderRequest),
}

pub fn parse_frame(frame: &[u8]) -> Result<Frame, WireErr> {
    if frame.len() > MAX_FRAME { return Err(WireErr::TooLong); }
    if frame.len() < SIG_LEN + 2 { return Err(WireErr::TooShort); }
    let v = u16::from_be_bytes([frame[0], frame[1]]);
    if v == 4 {
        return Ok(Frame::HeaderRequest(parse_header_request(frame)?));
    }
    Ok(Frame::Intent(parse_wire(frame)?))
}

/// v4 payload: v:u16 ‖ agent_len:u8 ‖ agent ‖ nonce:u64 ‖ ts:u64 ‖ ref_nonce:u64.
fn parse_header_request(frame: &[u8]) -> Result<HeaderRequest, WireErr> {
    let split = frame.len().checked_sub(SIG_LEN).ok_or(WireErr::TooShort)?;
    let (payload, sig_bytes) = frame.split_at(split);
    let mut c = Cur::new(payload);
    let v = c.u16()?;
    if v != 4 { return Err(WireErr::BadVersion); }
    let alen = usize::from(c.u8()?);
    if alen == 0 || alen > MAX_AGENT { return Err(WireErr::BadLength); }
    let agent_id = ascii_string(c.take(alen)?)?;
    let _nonce = c.u64()?;
    let ts = c.u64()?;
    let ref_nonce = c.u64()?;
    if c.remaining() != 0 { return Err(WireErr::BadLength); }
    let mut sig = [0u8; SIG_LEN];
    sig.copy_from_slice(sig_bytes);
    Ok(HeaderRequest { agent_id, ref_nonce, ts, signed_bytes: payload.to_vec(), sig })
}

fn ascii_string(b: &[u8]) -> Result<String, WireErr> {
    // printable ASCII, no control chars (RISK-M9-3 against hidden characters in input)
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
        p.extend_from_slice(&[0u8; SIG_LEN]); // dummy signature
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
    fn offer_json(timeout: u64) -> String {
        serde_json::json!({
            "x402Version": 1,
            "accepts": [{
                "scheme": "exact", "network": "base-sepolia",
                "maxAmountRequired": "100000", "payTo": "0x1111111111111111111111111111111111111111",
                "asset": "0x036cbd53842c5426634e7929541ec2318f3dcf7e",
                "resource": "https://issuer.example/bare/issue",
                "maxTimeoutSeconds": timeout,
                "extra": {"name": "USDC", "version": "2"}
            }]
        })
        .to_string()
    }

    fn build_v3(offer: &str, resource: Option<&str>) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&3u16.to_be_bytes());
        p.push(4); p.extend_from_slice(b"0xab");
        p.extend_from_slice(&100_000u128.to_be_bytes());
        p.extend_from_slice(&84532u64.to_be_bytes());
        p.push(7); p.extend_from_slice(b"agent-1");
        p.extend_from_slice(&7u64.to_be_bytes());
        p.extend_from_slice(&1000u64.to_be_bytes());
        match resource {
            Some(r) => { p.push(r.len() as u8); p.extend_from_slice(r.as_bytes()); }
            None => p.push(0),
        }
        p.extend_from_slice(&(offer.len() as u16).to_be_bytes());
        p.extend_from_slice(offer.as_bytes());
        p.extend_from_slice(&[0x11u8; SIG_LEN]);
        p
    }

    #[test]
    fn v3_carries_the_offer_inside_the_signature() {
        let offer = offer_json(300);
        let f = build_v3(&offer, Some("https://issuer.example/bare/issue"));
        let w = parse_wire(&f).unwrap();
        assert_eq!(w.offer.as_deref(), Some(offer.as_str()));
        // the signed bytes end right before the signature and contain the offer
        assert!(w.signed_bytes.len() < f.len());
        let needle = offer.as_bytes();
        assert!(
            w.signed_bytes.windows(needle.len()).any(|win| win == needle),
            "the offer is inside the signed bytes"
        );
    }

    #[test]
    fn an_offer_over_the_limit_is_refused() {
        let big = "x".repeat(MAX_OFFER + 1);
        assert_eq!(parse_wire(&build_v3(&big, None)).unwrap_err(), WireErr::OfferTooLong);
    }

    #[test]
    fn an_offer_with_an_unknown_field_is_refused() {
        let mut v: serde_json::Value = serde_json::from_str(&offer_json(300)).unwrap();
        v["accepts"][0]["sneaky"] = serde_json::json!("x");
        let f = build_v3(&v.to_string(), None);
        assert_eq!(parse_wire(&f).unwrap_err(), WireErr::BadOffer);
    }

    #[test]
    fn a_v4_frame_parses_as_a_header_request() {
        let mut p = Vec::new();
        p.extend_from_slice(&4u16.to_be_bytes());
        p.push(7); p.extend_from_slice(b"agent-1");
        p.extend_from_slice(&9u64.to_be_bytes());   // request nonce
        p.extend_from_slice(&1000u64.to_be_bytes()); // ts
        p.extend_from_slice(&7u64.to_be_bytes());    // ref_nonce
        p.extend_from_slice(&[0x22u8; SIG_LEN]);
        match parse_frame(&p).unwrap() {
            Frame::HeaderRequest(r) => {
                assert_eq!(r.agent_id, "agent-1");
                assert_eq!(r.ref_nonce, 7);
                assert_eq!(r.ts, 1000);
            }
            Frame::Intent(_) => panic!("v4 must not parse as an intent"),
        }
    }

    #[test]
    fn rejects_bad_version() {
        let f = build(9, "0xabc", "a", 1, 1);
        assert_eq!(parse_wire(&f), Err(WireErr::BadVersion));
    }

    fn build_v2(recipient: &str, agent: &str, resource: Option<&str>) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&2u16.to_be_bytes());
        p.push(recipient.len() as u8);
        p.extend_from_slice(recipient.as_bytes());
        p.extend_from_slice(&5_000_000u128.to_be_bytes());
        p.extend_from_slice(&8453u64.to_be_bytes());
        p.push(agent.len() as u8);
        p.extend_from_slice(agent.as_bytes());
        p.extend_from_slice(&7u64.to_be_bytes());
        p.extend_from_slice(&1000u64.to_be_bytes());
        match resource {
            Some(r) => {
                p.push(r.len() as u8);
                p.extend_from_slice(r.as_bytes());
            }
            None => p.push(0),
        }
        p.extend_from_slice(&[0u8; SIG_LEN]);
        p
    }

    #[test]
    fn v1_frames_still_parse_without_resource() {
        let w = parse_wire(&build(1, "0xabc", "agent-1", 7, 1000)).unwrap();
        assert_eq!(w.resource, None);
    }

    #[test]
    fn v2_carries_the_resource() {
        let w = parse_wire(&build_v2("0xabc", "agent-1", Some("https://api.example.com/v1/price"))).unwrap();
        assert_eq!(w.resource.as_deref(), Some("https://api.example.com/v1/price"));
    }

    #[test]
    fn v2_without_resource_is_allowed() {
        let w = parse_wire(&build_v2("0xabc", "agent-1", None)).unwrap();
        assert_eq!(w.resource, None);
    }

    #[test]
    fn v2_with_a_truncated_resource_is_refused() {
        let mut f = build_v2("0xabc", "agent-1", Some("https://api.example.com/v1"));
        let cut = f.len() - SIG_LEN - 5;
        f.remove(cut);
        assert_eq!(parse_wire(&f), Err(WireErr::Truncated));
    }

    #[test]
    fn v2_trailing_bytes_are_refused() {
        let mut f = build_v2("0xabc", "agent-1", Some("https://x/y"));
        f.insert(f.len() - SIG_LEN, 0xAA);
        assert_eq!(parse_wire(&f), Err(WireErr::BadLength));
    }
    #[test]
    fn rejects_too_long() {
        let f = vec![0u8; MAX_FRAME + 1];
        assert_eq!(parse_wire(&f), Err(WireErr::TooLong));
    }
    #[test]
    fn rejects_trailing_garbage() {
        let mut f = build(1, "0xabc", "a", 1, 1);
        f.insert(f.len() - SIG_LEN, 0xFF); // extra byte in payload
        assert_eq!(parse_wire(&f), Err(WireErr::BadLength));
    }
    #[test]
    fn no_panic_on_fuzz_like_inputs() {
        // RISK-M8-3: any garbage does not crash the parser
        for len in 0..200 {
            for seed in 0..8u8 {
                let junk: Vec<u8> = (0..len).map(|i| (i as u8) ^ seed).collect();
                let _ = parse_wire(&junk); // does not panic
            }
        }
    }
    #[test]
    fn rejects_control_chars_in_id() {
        let f = build(1, "0x\x01bc", "a", 1, 1);
        assert_eq!(parse_wire(&f), Err(WireErr::BadLength));
    }
}
