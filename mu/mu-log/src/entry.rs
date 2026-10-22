//! Log entry and its byte form (deterministic, for hash-chain and signing).
use mu_common::Hash32;
use sha2::{Digest, Sha256};

pub const WINDOW_SECS: u64 = 86_400;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    DeniedOmega { intent_hash: Hash32 },
    DeniedDelta { intent_hash: Hash32 },
    /// Reserve: amount_total = amount + gas_reserve (RISK-M3-3: single value for window and threshold).
    Pending { intent_hash: Hash32, amount_total: u128, chain_nonce: u64 },
    /// x402 reserve with the authorization parameters known BEFORE the money
    /// moves: reconcile can ask the chain about `nonce` and close the reserve
    /// after `valid_before` even if the daemon died before the answer.
    PendingAuth {
        intent_hash: Hash32,
        amount_total: u128,
        nonce: [u8; 32],
        valid_after: u64,
        valid_before: u64,
    },
    Settled { intent_hash: Hash32, tx_hash: [u8; 32], effective_gas: u128 },
    Failed { intent_hash: Hash32 },
    Simulated { intent_hash: Hash32, connector: &'static str },
    DeltaChanged { old_hash: Hash32, new_hash: Hash32 },
    HumanDecision { intent_hash: Hash32, approved: bool },
    TailTruncated { lost_from_seq: u64 },
    NonceSnapshot { agent_id: String, nonce: u64 },
    Alert { code: u16 },
    /// x402 refused because no 402 offer was supplied for the intent.
    DeniedNoOffer { intent_hash: Hash32 },
    /// An alert about one intent: written once per state transition, so a
    /// restart does not repeat it in the journal.
    AlertAbout { code: u16, intent_hash: Hash32 },
    /// A Δ change the owner signed and the daemon applied. The journal is the
    /// source of truth: `delta` is the full canonical Δ, so recovery never
    /// depends on `policy.toml`.
    DeltaApplied {
        version: u16,
        prev: Hash32,
        next: Hash32,
        delta: Vec<u8>,
        owner_sig: [u8; 64],
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub seq: u64,
    pub prev_hash: Hash32,
    pub ts: u64,
    pub kind: Kind,
    pub sig: [u8; 64],
}

impl Kind {
    /// Deterministic byte form for hash/signing.
    pub fn encode(&self) -> Vec<u8> {
        let mut b = Vec::new();
        match self {
            Kind::DeniedOmega { intent_hash } => { b.push(1); b.extend(intent_hash.0); }
            Kind::DeniedDelta { intent_hash } => { b.push(2); b.extend(intent_hash.0); }
            Kind::Pending { intent_hash, amount_total, chain_nonce } => {
                b.push(3); b.extend(intent_hash.0);
                b.extend(amount_total.to_be_bytes()); b.extend(chain_nonce.to_be_bytes());
            }
            Kind::Settled { intent_hash, tx_hash, effective_gas } => {
                b.push(4); b.extend(intent_hash.0); b.extend(tx_hash);
                b.extend(effective_gas.to_be_bytes());
            }
            Kind::Failed { intent_hash } => { b.push(5); b.extend(intent_hash.0); }
            Kind::Simulated { intent_hash, connector } => {
                b.push(6); b.extend(intent_hash.0);
                b.push(connector.len() as u8); b.extend(connector.as_bytes());
            }
            Kind::DeltaChanged { old_hash, new_hash } => {
                b.push(7); b.extend(old_hash.0); b.extend(new_hash.0);
            }
            Kind::HumanDecision { intent_hash, approved } => {
                b.push(8); b.extend(intent_hash.0); b.push(u8::from(*approved));
            }
            Kind::TailTruncated { lost_from_seq } => { b.push(9); b.extend(lost_from_seq.to_be_bytes()); }
            Kind::NonceSnapshot { agent_id, nonce } => {
                b.push(10); b.push(agent_id.len() as u8);
                b.extend(agent_id.as_bytes()); b.extend(nonce.to_be_bytes());
            }
            Kind::Alert { code } => { b.push(11); b.extend(code.to_be_bytes()); }
            Kind::DeniedNoOffer { intent_hash } => {
                b.push(15);
                b.extend(intent_hash.0);
            }
            Kind::AlertAbout { code, intent_hash } => {
                b.push(14);
                b.extend(code.to_be_bytes());
                b.extend(intent_hash.0);
            }
            Kind::DeltaApplied { version, prev, next, delta, owner_sig } => {
                b.push(13);
                b.extend(version.to_be_bytes());
                b.extend(prev.0);
                b.extend(next.0);
                b.extend(owner_sig);
                b.extend((delta.len() as u32).to_be_bytes());
                b.extend(delta);
            }
            Kind::PendingAuth { intent_hash, amount_total, nonce, valid_after, valid_before } => {
                b.push(12); b.extend(intent_hash.0);
                b.extend(amount_total.to_be_bytes());
                b.extend(nonce);
                b.extend(valid_after.to_be_bytes());
                b.extend(valid_before.to_be_bytes());
            }
        }
        b
    }
}

impl Entry {
    /// hash = SHA256(seq ‖ prev_hash ‖ ts ‖ kind_bytes) — WITHOUT sig,
    /// so hash is computable before signing (this hash is what gets signed).
    pub fn hash(&self) -> Hash32 {
        entry_hash(self.seq, &self.prev_hash, self.ts, &self.kind)
    }
}

pub fn entry_hash(seq: u64, prev: &Hash32, ts: u64, kind: &Kind) -> Hash32 {
    let mut h = Sha256::new();
    h.update(seq.to_be_bytes());
    h.update(prev.0);
    h.update(ts.to_be_bytes());
    h.update(kind.encode());
    let out = h.finalize();
    let mut d = [0u8; 32];
    d.copy_from_slice(&out);
    Hash32(d)
}
