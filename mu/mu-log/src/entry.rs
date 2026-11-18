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
    Settled { intent_hash: Hash32, tx_hash: [u8; 32], effective_gas: u128 },
    Failed { intent_hash: Hash32 },
    Simulated { intent_hash: Hash32, connector: &'static str },
    DeltaChanged { old_hash: Hash32, new_hash: Hash32 },
    HumanDecision { intent_hash: Hash32, approved: bool },
    TailTruncated { lost_from_seq: u64 },
    NonceSnapshot { agent_id: String, nonce: u64 },
    Alert { code: u16 },
    /// What the gate was asked and what it judged against, written before any check:
    /// recipient, price (amount + gas estimate), chain/connector (the asset is the
    /// connector's per-network constant) and the x402 resource, plus the hash of the Ω/Δ
    /// in force. Together with Pending/Settled/Denied* (same intent_hash) a reader can
    /// recompute the decision from the log alone.
    Quoted {
        intent_hash: Hash32,
        recipient: [u8; 20],
        chain_id: u64,
        connector: u8,
        amount: u128,
        gas_est: u128,
        resource: String,
        policy_hash: Hash32,
    },
    /// Why a Denied* entry was written: which rule, the total checked and the cap it met.
    DenyReason { intent_hash: Hash32, code: u8, total: u128, limit: u128 },
}

/// `DenyReason.code` values.
pub mod deny {
    pub const QUOTE_FAILED: u8 = 1;
    pub const CONNECTOR_UNKNOWN: u8 = 2;
    pub const CEILING_EXCEEDED: u8 = 3;
    pub const NOT_WHITELISTED: u8 = 4;
    pub const WINDOW_EXCEEDED: u8 = 5;
    pub const RESOURCE_DENIED: u8 = 6;
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
            Kind::Quoted { intent_hash, recipient, chain_id, connector, amount, gas_est, resource, policy_hash } => {
                b.push(12); b.extend(intent_hash.0); b.extend(recipient);
                b.extend(chain_id.to_be_bytes()); b.push(*connector);
                b.extend(amount.to_be_bytes()); b.extend(gas_est.to_be_bytes());
                let r = resource.as_bytes();
                let n = r.len().min(u16::MAX as usize);
                b.extend((n as u16).to_be_bytes()); b.extend(&r[..n]);
                b.extend(policy_hash.0);
            }
            Kind::DenyReason { intent_hash, code, total, limit } => {
                b.push(13); b.extend(intent_hash.0); b.push(*code);
                b.extend(total.to_be_bytes()); b.extend(limit.to_be_bytes());
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
