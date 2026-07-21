//! Transport implementation of SuiRpc over JSON-RPC (HTTP POST).
//! Minimal client without heavy SDKs: manual request serialization, responses —
//! narrow parsing of only needed fields (same discipline as hardened parsers).
//!
//! WARNING (honesty boundary): method names are SET BY CONFIG (SuiRpcConfig),
//! defaults are stable documented endpoints; the gasless transfer build method
//! Address Balances — post-cutoff feature, the name MUST be verified by the agent against official
//! documentation (sources in ETAP2-instruction) before live execution.
use crate::sui::{BuiltTx, DryRun, SuiRpc, SuiSend, SuiTxLookup};
use crate::{ConnErr, RejectReason};
use base64::Engine;

pub struct SuiRpcConfig {
    pub url: String,
    /// stablecoin transfer build; default candidate verified by agent against docs
    pub m_build: String,     // e.g. "unsafe_pay" | gasless method from docs
    pub m_dry: String,       // "sui_dryRunTransactionBlock"
    pub m_exec: String,      // "sui_executeTransactionBlock"
    pub m_get: String,       // "sui_getTransactionBlock"
    pub timeout_ms: u64,
}

pub struct JsonRpcClient {
    pub cfg: SuiRpcConfig,
    /// HTTP transport is injected (tests/offline environments); prod: reqwest/ureq by agent.
    pub post: fn(url: &str, body: &str, timeout_ms: u64) -> Result<String, String>,
}

fn b64(v: &[u8]) -> String { base64::engine::general_purpose::STANDARD.encode(v) }
fn unb64(s: &str) -> Option<Vec<u8>> { base64::engine::general_purpose::STANDARD.decode(s).ok() }
fn hex32(s: &str) -> Option<[u8; 32]> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.len() != 64 { return None; }
    let mut a = [0u8; 32];
    for i in 0..32 { a[i] = u8::from_str_radix(&s[i*2..i*2+2], 16).ok()?; }
    Some(a)
}
fn addr_hex(a: &[u8; 32]) -> String {
    let mut s = String::from("0x");
    for b in a { s.push_str(&format!("{b:02x}")); }
    s
}

/// Narrow JSON extractor: value of string field "key":"..." (without full parser —
/// sufficient for known node responses; a malicious node is already covered by dry-run/consensus).
fn jstr<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":\"");
    let i = json.find(&pat)? + pat.len();
    let rest = &json[i..];
    let j = rest.find('"')?;
    Some(&rest[..j])
}
fn jnum(json: &str, key: &str) -> Option<u64> {
    // supports both number and string-number ("checkpoint":"123")
    if let Some(s) = jstr(json, key) { return s.parse().ok(); }
    let pat = format!("\"{key}\":");
    let i = json.find(&pat)? + pat.len();
    let rest: String = json[i..].chars().take_while(|c| c.is_ascii_digit()).collect();
    rest.parse().ok()
}

impl JsonRpcClient {
    fn call(&self, method: &str, params_json: &str) -> Result<String, ConnErr> {
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"{method}","params":{params_json}}}"#
        );
        (self.post)(&self.cfg.url, &body, self.cfg.timeout_ms)
            .map_err(|e| ConnErr::Unknown(format!("rpc {method}: {e}")))
    }
}

impl SuiRpc for JsonRpcClient {
    fn build_transfer(&self, sender: &[u8; 32], recipient: &[u8; 32], amount: u128, coin_type: &str)
        -> Result<BuiltTx, ConnErr>
    {
        let params = format!(
            r#"["{}","{}","{}","{}"]"#,
            addr_hex(sender), addr_hex(recipient), amount, coin_type
        );
        let resp = self.call(&self.cfg.m_build, &params)?;
        let txb = jstr(&resp, "txBytes")
            .and_then(unb64)
            .ok_or_else(|| ConnErr::Unknown("build: no txBytes".into()))?;
        Ok(BuiltTx { tx_bytes: txb })
    }

    fn dry_run(&self, tx_bytes: &[u8]) -> DryRun {
        let params = format!(r#"["{}"]"#, b64(tx_bytes));
        let Ok(resp) = self.call(&self.cfg.m_dry, &params) else { return DryRun::Unreachable };
        match jstr(&resp, "status") {
            Some("success") => {
                // recipient/amount from balanceChanges — narrow parse
                let rec = jstr(&resp, "recipient_owner").or_else(|| jstr(&resp, "AddressOwner"));
                let amt = jstr(&resp, "amount").and_then(|s| s.parse::<i128>().ok());
                match (rec.and_then(hex32), amt) {
                    (Some(r), Some(a)) if a > 0 => DryRun::Ok { recipient: r, amount: a as u128 },
                    _ => DryRun::Unreachable, // could not verify = do not trust
                }
            }
            Some("failure") => DryRun::WouldFail(
                jstr(&resp, "error").unwrap_or("unknown").to_string()
            ),
            _ => DryRun::Unreachable,
        }
    }

    fn execute(&self, tx_bytes: &[u8], serialized_sig: &[u8]) -> SuiSend {
        let params = format!(
            r#"["{}",["{}"],null,"WaitForLocalExecution"]"#,
            b64(tx_bytes), b64(serialized_sig)
        );
        let Ok(resp) = self.call(&self.cfg.m_exec, &params) else { return SuiSend::Unreachable };
        if let Some(d) = jstr(&resp, "digest").and_then(|s| bs58_32(s)) {
            return SuiSend::Accepted { digest: d };
        }
        let err = jstr(&resp, "message").unwrap_or("");
        if err.contains("already") { // "transaction already executed"
            if let Some(d) = jstr(&resp, "digest").and_then(|s| bs58_32(s)) {
                return SuiSend::AlreadyExecuted { digest: d };
            }
        }
        if err.contains("locked") || err.contains("reserved") {
            return SuiSend::ObjectLocked; // equivocation → Unknown above
        }
        if err.contains("Insufficient") || err.contains("balance") {
            return SuiSend::DeterministicReject(RejectReason::InsufficientFunds);
        }
        if err.contains("signature") || err.contains("serializ") {
            return SuiSend::DeterministicReject(RejectReason::InvalidRecipient);
        }
        SuiSend::Unreachable // unclassified error = unknown, not refusal
    }

    fn lookup(&self, digest: &[u8; 32]) -> SuiTxLookup {
        let params = format!(r#"["{}",{{"showEffects":true}}]"#, to_bs58(digest));
        let Ok(resp) = self.call(&self.cfg.m_get, &params) else { return SuiTxLookup::Unreachable };
        if resp.contains("Could not find") || resp.contains("not found") {
            return SuiTxLookup::NotFound;
        }
        match (jstr(&resp, "status"), jnum(&resp, "checkpoint")) {
            (Some("success"), Some(cp)) => SuiTxLookup::Success { checkpoint: cp },
            (Some("failure"), Some(cp)) => SuiTxLookup::FailedOnChain { checkpoint: cp },
            _ => SuiTxLookup::Unreachable,
        }
    }
}

// ── base58 (Sui digest in base58) — minimal implementation ────────────────
const B58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
fn to_bs58(bytes: &[u8; 32]) -> String {
    let mut num = bytes.to_vec();
    let mut out = Vec::new();
    let zeros = num.iter().take_while(|&&b| b == 0).count();
    while num.iter().any(|&b| b != 0) {
        let mut rem = 0u32;
        for b in num.iter_mut() {
            let acc = (rem << 8) | u32::from(*b);
            *b = (acc / 58) as u8;
            rem = acc % 58;
        }
        out.push(B58[rem as usize]);
    }
    for _ in 0..zeros { out.push(b'1'); }
    out.reverse();
    String::from_utf8(out).unwrap_or_default()
}
fn bs58_32(s: &str) -> Option<[u8; 32]> {
    let mut num: Vec<u8> = Vec::new();
    for c in s.bytes() {
        let v = B58.iter().position(|&b| b == c)? as u32;
        let mut carry = v;
        for b in num.iter_mut() {
            let acc = u32::from(*b) * 58 + carry;
            *b = (acc & 0xff) as u8;
            carry = acc >> 8;
        }
        while carry > 0 { num.push((carry & 0xff) as u8); carry >>= 8; }
    }
    for c in s.bytes() { if c == b'1' { num.push(0); } else { break; } }
    num.reverse();
    if num.len() > 32 { return None; }
    let mut a = [0u8; 32];
    a[32 - num.len()..].copy_from_slice(&num);
    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bs58_roundtrip() {
        let d = [7u8; 32];
        let s = to_bs58(&d);
        assert_eq!(bs58_32(&s), Some(d));
        let mut z = [0u8; 32]; z[31] = 1;
        assert_eq!(bs58_32(&to_bs58(&z)), Some(z));
    }
    #[test]
    fn jstr_extracts() {
        let j = r#"{"result":{"digest":"abc","status":"success","checkpoint":"42"}}"#;
        assert_eq!(jstr(j, "digest"), Some("abc"));
        assert_eq!(jnum(j, "checkpoint"), Some(42));
    }
}
