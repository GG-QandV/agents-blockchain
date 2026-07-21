//! M8 verification pipeline: strict order, early exit, single deny{code}.
use crate::wire::{parse_wire, WireIntent};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::collections::HashMap;

/// Denial codes. RISK-M8-5: only the code is exposed, no Δ/Ω details.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenyCode {
    Parse = 0x00,
    UnknownAgent = 0x01,
    BadSignature = 0x02,
    StaleTimestamp = 0x03,
    ReplayNonce = 0x04,
    RateLimited = 0x05,
    Busy = 0x06,
}

/// Registry of allowed agents: agent_id → ed25519 pubkey.
pub struct AllowList {
    keys: HashMap<String, VerifyingKey>,
}
impl AllowList {
    pub fn new() -> Self { AllowList { keys: HashMap::new() } }
    pub fn add(&mut self, agent_id: &str, pubkey: [u8; 32]) -> bool {
        match VerifyingKey::from_bytes(&pubkey) {
            Ok(vk) => { self.keys.insert(agent_id.to_string(), vk); true }
            Err(_) => false,
        }
    }
    fn get(&self, agent_id: &str) -> Option<&VerifyingKey> { self.keys.get(agent_id) }
}
impl Default for AllowList { fn default() -> Self { Self::new() } }

/// Authenticated, fresh, non-replayed intent — the only way out of Gate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedIntent {
    pub recipient: String,
    pub amount: u128,
    pub chain_id: u64,
    pub agent_id: String,
    pub nonce: u64,
}

/// Token bucket per agent.
struct Bucket { tokens: u32, cap: u32 }

pub struct Gate {
    allow: AllowList,
    /// RISK-M8-1: last accepted nonce per agent (survives restart — loaded from log).
    last_nonce: HashMap<String, u64>,
    buckets: HashMap<String, Bucket>,
    bucket_cap: u32,
    ts_window_secs: u64,
}

impl Gate {
    pub fn new(allow: AllowList, bucket_cap: u32, ts_window_secs: u64) -> Self {
        Gate {
            allow,
            last_nonce: HashMap::new(),
            buckets: HashMap::new(),
            bucket_cap,
            ts_window_secs,
        }
    }

    /// RISK-M8-1: restore nonce windows from log on daemon startup.
    pub fn restore_nonce(&mut self, agent_id: &str, nonce: u64) {
        let e = self.last_nonce.entry(agent_id.to_string()).or_insert(0);
        *e = (*e).max(nonce);
    }

    /// Full verification pipeline for a single frame. now — from daemon Clock.
    pub fn accept(&mut self, frame: &[u8], now: u64) -> Result<VerifiedIntent, DenyCode> {
        // 1-3: hardened parser
        let w: WireIntent = parse_wire(frame).map_err(|_| DenyCode::Parse)?;

        // 4: agent ∈ allowlist → pubkey (RISK-M8-2: key from registry, not from frame)
        let vk = self.allow.get(&w.agent_id).ok_or(DenyCode::UnknownAgent)?;

        // 5: signature covers entire payload including agent_id (RISK-M8-2)
        let sig = Signature::from_bytes(&w.sig);
        vk.verify(&w.signed_bytes, &sig).map_err(|_| DenyCode::BadSignature)?;

        // 6: timestamp freshness (after verify — so code 0x03 cannot be obtained unauthenticated)
        let diff = now.abs_diff(w.ts);
        if diff > self.ts_window_secs {
            return Err(DenyCode::StaleTimestamp);
        }

        // 7: monotonic nonce (RISK-M8-1)
        let last = self.last_nonce.get(&w.agent_id).copied().unwrap_or(0);
        if w.nonce <= last {
            return Err(DenyCode::ReplayNonce);
        }

        // 8: rate-limit
        let cap = self.bucket_cap;
        let bucket = self.buckets.entry(w.agent_id.clone()).or_insert(Bucket { tokens: cap, cap });
        if bucket.tokens == 0 {
            return Err(DenyCode::RateLimited);
        }
        bucket.tokens = bucket.tokens.saturating_sub(1);

        // accepted → persist nonce
        self.last_nonce.insert(w.agent_id.clone(), w.nonce);

        Ok(VerifiedIntent {
            recipient: w.recipient,
            amount: w.amount,
            chain_id: w.chain_id,
            agent_id: w.agent_id,
            nonce: w.nonce,
        })
    }

    /// Refill buckets (called by ticker periodically).
    pub fn refill(&mut self) {
        for b in self.buckets.values_mut() {
            b.tokens = b.cap;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn frame_signed(sk: &SigningKey, agent: &str, nonce: u64, ts: u64) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&1u16.to_be_bytes());
        let recipient = "0xabc";
        p.push(recipient.len() as u8);
        p.extend_from_slice(recipient.as_bytes());
        p.extend_from_slice(&5_000_000u128.to_be_bytes());
        p.extend_from_slice(&8453u64.to_be_bytes());
        p.push(agent.len() as u8);
        p.extend_from_slice(agent.as_bytes());
        p.extend_from_slice(&nonce.to_be_bytes());
        p.extend_from_slice(&ts.to_be_bytes());
        let sig = sk.sign(&p);
        p.extend_from_slice(&sig.to_bytes());
        p
    }

    fn setup() -> (Gate, SigningKey) {
        let sk = SigningKey::from_bytes(&[42u8; 32]);
        let mut allow = AllowList::new();
        allow.add("agent-1", sk.verifying_key().to_bytes());
        (Gate::new(allow, 10, 60), sk)
    }

    #[test]
    fn accepts_valid() {
        let (mut g, sk) = setup();
        let f = frame_signed(&sk, "agent-1", 1, 1000);
        assert!(g.accept(&f, 1000).is_ok());
    }

    #[test]
    fn replay_same_nonce_denied() {
        // RISK-M8-1
        let (mut g, sk) = setup();
        let f = frame_signed(&sk, "agent-1", 5, 1000);
        assert!(g.accept(&f, 1000).is_ok());
        assert_eq!(g.accept(&f, 1000), Err(DenyCode::ReplayNonce));
    }

    #[test]
    fn nonce_survives_restart() {
        // RISK-M8-1: after restore, old nonce is rejected
        let (mut g, sk) = setup();
        g.restore_nonce("agent-1", 100);
        let old = frame_signed(&sk, "agent-1", 50, 1000);
        assert_eq!(g.accept(&old, 1000), Err(DenyCode::ReplayNonce));
        let fresh = frame_signed(&sk, "agent-1", 101, 1000);
        assert!(g.accept(&fresh, 1000).is_ok());
    }

    #[test]
    fn wrong_key_denied() {
        // RISK-M8-2: signature with foreign key
        let (mut g, _sk) = setup();
        let other = SigningKey::from_bytes(&[7u8; 32]);
        let f = frame_signed(&other, "agent-1", 1, 1000);
        assert_eq!(g.accept(&f, 1000), Err(DenyCode::BadSignature));
    }

    #[test]
    fn agent_id_spoof_breaks_signature() {
        // RISK-M8-2: signature is valid for agent-1, but frame is tagged agent-1 (in allowlist)
        // spoof id inside payload → signature won't match
        let (mut g, sk) = setup();
        let mut f = frame_signed(&sk, "agent-1", 1, 1000);
        // agent_id starts after v(2)+rlen(1)+recipient(5)+amount(16)+chain(8)+alen(1)
        let off = 2 + 1 + 5 + 16 + 8 + 1;
        f[off] = b'X'; // corrupt first char of agent_id
        // now id "Xgent-1" is not in allowlist OR signature won't match
        let r = g.accept(&f, 1000);
        assert!(matches!(r, Err(DenyCode::UnknownAgent) | Err(DenyCode::BadSignature)));
    }

    #[test]
    fn stale_timestamp_denied() {
        let (mut g, sk) = setup();
        let f = frame_signed(&sk, "agent-1", 1, 1000);
        assert_eq!(g.accept(&f, 2000), Err(DenyCode::StaleTimestamp)); // |2000-1000|>60
    }

    #[test]
    fn unknown_agent_denied() {
        let (mut g, sk) = setup();
        let f = frame_signed(&sk, "ghost", 1, 1000);
        assert_eq!(g.accept(&f, 1000), Err(DenyCode::UnknownAgent));
    }

    #[test]
    fn rate_limit_kicks_in() {
        let sk = SigningKey::from_bytes(&[42u8; 32]);
        let mut allow = AllowList::new();
        allow.add("agent-1", sk.verifying_key().to_bytes());
        let mut g = Gate::new(allow, 2, 60); // cap=2
        for n in 1..=2u64 {
            let f = frame_signed(&sk, "agent-1", n, 1000);
            assert!(g.accept(&f, 1000).is_ok());
        }
        let f3 = frame_signed(&sk, "agent-1", 3, 1000);
        assert_eq!(g.accept(&f3, 1000), Err(DenyCode::RateLimited));
    }
}
