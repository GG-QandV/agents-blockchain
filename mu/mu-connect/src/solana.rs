//! P11b (Spec S5 §3.1 step 2): the Solana connector for anchors.
//!
//! It builds **one** transaction — a single SPL Memo instruction carrying the
//! intent's memo, fee paid by the signer — and puts it through the invariants of
//! `sui.rs`, spelled for an anchor:
//!
//! - the intent is checked before anything is built ([`crate::anchor::check_intent`]);
//! - the message is **decoded again** and compared with the intent before it is
//!   signed (the memo, the single instruction, the program, the payer) — the same
//!   self-check as RISK-M7-5;
//! - the signed transaction is dry-run on **two** nodes, and both must pass: two
//!   nodes that disagree are not a pass, and nothing is sent;
//! - `execute` cannot return a failure: a send that times out is
//!   [`AnchorErr::Unknown`] with the signature to ask the network about
//!   (RISK-M7-1, `Unknown ≠ Failed`); [`reconcile`] says `Failed` only when both
//!   nodes say the blockhash has expired and the transaction never landed.
//!
//! The nodes and the signer are traits: nothing here opens a socket or holds a key.

use crate::anchor::{check_intent, AnchorIntent, IntentError};

/// The SPL Memo program, version 2 (32 bytes of its address).
pub const MEMO_PROGRAM_ID: [u8; 32] = [
    5, 74, 83, 90, 153, 41, 33, 6, 77, 36, 232, 113, 96, 218, 56, 124, 124, 53, 181, 221, 188, 146, 187, 129, 228, 31, 168, 64, 65, 5, 68, 141,
];

/// What this connector is configured for.
#[derive(Clone, Debug)]
pub struct SolanaCfg {
    /// The cluster anchors go to; an intent for another one is refused.
    pub cluster: String,
}

/// What a node answers when it does not give an answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeErr {
    /// Timeout, connection, garbage: the outcome is not known.
    Unreachable(String),
    /// The node says no, reliably (before the transaction reached the chain).
    Rejected(String),
}

/// A node's dry run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sim {
    /// The simulation passed.
    pub ok: bool,
    /// Why not, when not.
    pub err: Option<String>,
}

/// Where a signature stands on one node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigState {
    /// The node has not seen it (yet).
    NotFound,
    /// It landed in this slot.
    Landed {
        /// The slot.
        slot: u64,
    },
    /// Not found, and the blockhash it was built on is no longer valid: it can
    /// never land on this node's view of the chain.
    Expired,
}

/// A Solana node, as far as an anchor needs one.
pub trait RpcNode {
    /// A name for messages.
    fn name(&self) -> &str;
    /// The fee for this message, lamports.
    ///
    /// # Errors
    ///
    /// [`NodeErr`] when the node does not answer.
    fn quote_fee(&self, message: &[u8]) -> Result<u64, NodeErr>;
    /// A recent blockhash.
    ///
    /// # Errors
    ///
    /// [`NodeErr`] when the node does not answer.
    fn latest_blockhash(&self) -> Result<[u8; 32], NodeErr>;
    /// A dry run of the signed transaction.
    ///
    /// # Errors
    ///
    /// [`NodeErr`] when the node does not answer.
    fn simulate(&self, tx: &[u8]) -> Result<Sim, NodeErr>;
    /// Submits the signed transaction; returns the signature the node reports.
    ///
    /// # Errors
    ///
    /// [`NodeErr::Rejected`] when the node refuses it reliably; [`NodeErr::Unreachable`]
    /// when the outcome is not known.
    fn send(&self, tx: &[u8]) -> Result<String, NodeErr>;
    /// Where a signature stands.
    ///
    /// # Errors
    ///
    /// [`NodeErr`] when the node does not answer.
    fn status(&self, signature: &str) -> Result<SigState, NodeErr>;
}

/// A node's view of a balance: lamports at an address (base58). Kept apart from [`RpcNode`]: the
/// anchor needs none, the refill needs it of the main wallet and of the sub-wallets.
pub trait Balances {
    /// The balance, lamports.
    ///
    /// # Errors
    ///
    /// [`NodeErr`] when the node does not answer.
    fn balance(&self, address: &str) -> Result<u64, NodeErr>;
}

/// An Ed25519 signer: the connector sees a public key and a function, never a key.
pub trait Ed25519 {
    /// The public key, which is also the fee payer's address.
    fn public(&self) -> [u8; 32];
    /// The signature over `msg`.
    fn sign(&self, msg: &[u8]) -> [u8; 64];
}

/// The vault's Ed25519 handle signs anchors: the connector sees the public key
/// and the signature, the handle keeps the key (feature `ksg`).
#[cfg(feature = "ksg")]
impl Ed25519 for mu_vault::TxSignerEd25519 {
    fn public(&self) -> [u8; 32] {
        self.public_key()
    }
    fn sign(&self, msg: &[u8]) -> [u8; 64] {
        self.sign_message(msg)
    }
}

/// Why an anchor was not sent, or may not have been.
///
/// No variant is a failure: a payment whose fate is unclear is never declared
/// dead here (RISK-M7-1).
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnchorErr {
    /// A reliable refusal **before** anything reached the chain: the intent does
    /// not hold, the nodes disagree, a node refused.
    Rejected(String),
    /// The outcome is not known. With the signature, when a transaction was
    /// signed: ask the network about it ([`reconcile`]) before sending again.
    Unknown {
        /// What happened.
        reason: String,
        /// The signature of the transaction that may have landed.
        signature: Option<String>,
    },
}

/// A transaction handed to the network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sent {
    /// The signature, base58.
    pub signature: String,
    /// The fee the nodes quoted for it (the dearer of the two), lamports: what the owner's
    /// budget is charged.
    pub fee_lamports: u64,
}

/// What [`reconcile`] concludes from the nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Settled {
    /// It landed.
    Landed {
        /// The slot.
        slot: u64,
    },
    /// Not known yet: keep it pending.
    Pending,
    /// It never landed and can no longer: both nodes say the blockhash expired.
    Failed,
}

fn compact_u16(mut n: u16, out: &mut Vec<u8>) {
    loop {
        let mut byte = (n & 0x7f) as u8;
        n >>= 7;
        if n == 0 {
            out.push(byte);
            return;
        }
        byte |= 0x80;
        out.push(byte);
    }
}

fn read_compact_u16(b: &[u8], at: &mut usize) -> Option<u16> {
    let mut value: u32 = 0;
    for shift in [0u32, 7, 14] {
        let byte = *b.get(*at)?;
        *at += 1;
        value |= u32::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return u16::try_from(value).ok();
        }
    }
    None
}

/// The legacy message of one memo instruction: the payer signs and pays, the
/// memo program is the one read-only account.
fn build_message(payer: &[u8; 32], blockhash: &[u8; 32], memo: &str) -> Vec<u8> {
    let mut m = vec![1, 0, 1]; // 1 signature, 0 read-only signed, 1 read-only unsigned
    compact_u16(2, &mut m);
    m.extend_from_slice(payer);
    m.extend_from_slice(&MEMO_PROGRAM_ID);
    m.extend_from_slice(blockhash);
    compact_u16(1, &mut m); // one instruction
    m.push(1); // program id index: the memo program
    compact_u16(1, &mut m); // one account
    m.push(0); // the payer
    compact_u16(u16::try_from(memo.len()).unwrap_or(u16::MAX), &mut m);
    m.extend_from_slice(memo.as_bytes());
    m
}

/// Decodes `message` again and compares it with the intent: exactly one
/// instruction, to the memo program, carrying exactly the intent's memo, one
/// signer who is `payer`. What is signed is what was asked for — or nothing is.
///
/// # Errors
///
/// A reason in words when the message is anything else.
pub fn check_message(message: &[u8], intent: &AnchorIntent, payer: &[u8; 32]) -> Result<(), String> {
    let bad = |what: &str| Err(format!("the message is not the intent's: {what}"));
    let mut at = 0usize;
    let Some(header) = message.get(..3) else { return bad("no header") };
    if header != [1, 0, 1] {
        return bad("not one signer and one read-only program");
    }
    at += 3;
    if read_compact_u16(message, &mut at) != Some(2) {
        return bad("not two accounts");
    }
    let key = |at: &mut usize| -> Option<[u8; 32]> {
        let k = message.get(*at..*at + 32)?.try_into().ok()?;
        *at += 32;
        Some(k)
    };
    let (Some(first), Some(program)) = (key(&mut at), key(&mut at)) else {
        return bad("accounts cut off");
    };
    if &first != payer {
        return bad("another fee payer");
    }
    if program != MEMO_PROGRAM_ID {
        return bad("the program is not the memo program");
    }
    at += 32; // the blockhash
    if read_compact_u16(message, &mut at) != Some(1) {
        return bad("not exactly one instruction");
    }
    if message.get(at) != Some(&1) {
        return bad("the instruction is not for the memo program");
    }
    at += 1;
    if read_compact_u16(message, &mut at) != Some(1) || message.get(at) != Some(&0) {
        return bad("the instruction's accounts are not the payer alone");
    }
    at += 1;
    let Some(len) = read_compact_u16(message, &mut at) else { return bad("no memo length") };
    let Some(data) = message.get(at..at + usize::from(len)) else { return bad("memo cut off") };
    if data != intent.memo.as_bytes() {
        return bad("another memo");
    }
    if at + usize::from(len) != message.len() {
        return bad("bytes after the memo");
    }
    Ok(())
}

const B58: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// The address of a public key, as a node names it (base58).
#[must_use]
pub fn base58_public(public: &[u8; 32]) -> String {
    base58(public)
}

pub(crate) fn base58(bytes: &[u8]) -> String {
    let mut digits: Vec<u8> = Vec::new();
    for &byte in bytes {
        let mut carry = u32::from(byte);
        for d in &mut digits {
            carry += u32::from(*d) << 8;
            *d = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }
    let zeros = bytes.iter().take_while(|b| **b == 0).count();
    let mut out = String::new();
    out.extend(std::iter::repeat_n('1', zeros));
    out.extend(digits.iter().rev().map(|d| B58[usize::from(*d)] as char));
    out
}

fn node_unknown(node: &dyn RpcNode, e: &NodeErr) -> AnchorErr {
    match e {
        NodeErr::Unreachable(why) => AnchorErr::Unknown {
            reason: format!("node {}: {why}", node.name()),
            signature: None,
        },
        NodeErr::Rejected(why) => AnchorErr::Rejected(format!("node {}: {why}", node.name())),
    }
}

/// Anchors one memo: checks the intent, builds and re-reads the message, signs,
/// dry-runs on both nodes, sends through the first.
///
/// # Errors
///
/// [`AnchorErr::Rejected`] when nothing was sent and nothing will be (the intent
/// does not hold, the message is not the intent's, the nodes disagree);
/// [`AnchorErr::Unknown`] when a node did not answer or the send's outcome is
/// not known — with the signature when a transaction was signed.
pub fn execute(
    a: &dyn RpcNode,
    b: &dyn RpcNode,
    signer: &dyn Ed25519,
    intent: &AnchorIntent,
    cfg: &SolanaCfg,
) -> Result<Sent, AnchorErr> {
    let payer = signer.public();
    let blockhash = a.latest_blockhash().map_err(|e| node_unknown(a, &e))?;
    let message = build_message(&payer, &blockhash, &intent.memo);
    // The fee both nodes quote; the dearer one is the one held against the cap.
    let fee_a = a.quote_fee(&message).map_err(|e| node_unknown(a, &e))?;
    let fee_b = b.quote_fee(&message).map_err(|e| node_unknown(b, &e))?;
    check_intent(intent, &cfg.cluster, fee_a.max(fee_b))
        .map_err(|e: IntentError| AnchorErr::Rejected(e.to_string()))?;
    check_message(&message, intent, &payer).map_err(AnchorErr::Rejected)?;

    let signature = signer.sign(&message);
    let mut tx = vec![1u8];
    tx.extend_from_slice(&signature);
    tx.extend_from_slice(&message);
    let signature_text = base58(&signature);

    // Both nodes dry-run the very bytes that will be sent; a node that is silent
    // is Unknown, a node that says no, or two that disagree, is a refusal.
    let sim_a = a.simulate(&tx).map_err(|e| node_unknown(a, &e))?;
    let sim_b = b.simulate(&tx).map_err(|e| node_unknown(b, &e))?;
    if !(sim_a.ok && sim_b.ok) {
        return Err(AnchorErr::Rejected(format!(
            "the dry run does not pass on both nodes ({}: {}, {}: {})",
            a.name(),
            if sim_a.ok { "ok".into() } else { sim_a.err.unwrap_or_default() },
            b.name(),
            if sim_b.ok { "ok".into() } else { sim_b.err.unwrap_or_default() },
        )));
    }

    match a.send(&tx) {
        Ok(reported) => Ok(Sent {
            signature: if reported.is_empty() { signature_text } else { reported },
            fee_lamports: fee_a.max(fee_b),
        }),
        Err(NodeErr::Rejected(why)) => Err(AnchorErr::Rejected(format!("node {}: {why}", a.name()))),
        Err(NodeErr::Unreachable(why)) => Err(AnchorErr::Unknown {
            reason: format!("the send's outcome is not known (node {}: {why})", a.name()),
            signature: Some(signature_text),
        }),
    }
}

/// Asks both nodes where a signature stands. `Landed` if either says so;
/// `Failed` only when **both** say the blockhash expired and it never landed;
/// anything else — a node silent, one still waiting — is `Pending`.
#[must_use]
pub fn reconcile(a: &dyn RpcNode, b: &dyn RpcNode, signature: &str) -> Settled {
    let states = [a.status(signature), b.status(signature)];
    for s in &states {
        if let Ok(SigState::Landed { slot }) = s {
            return Settled::Landed { slot: *slot };
        }
    }
    if states.iter().all(|s| matches!(s, Ok(SigState::Expired))) {
        return Settled::Failed;
    }
    Settled::Pending
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_memo_program_id_is_the_known_address() {
        // The address of the SPL Memo program v2, as `ksg-verify-v2` names it (`MEMO_PROGRAM`).
        assert_eq!(base58(&MEMO_PROGRAM_ID), "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
    }

    #[test]
    fn base58_known_vectors() {
        assert_eq!(base58(&[]), "");
        assert_eq!(base58(&[0]), "1");
        assert_eq!(base58(&[0, 0, 1]), "112");
        assert_eq!(base58(b"hello world"), "StV1DL6CwTryKyV");
    }
}
