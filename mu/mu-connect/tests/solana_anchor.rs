//! P11b (Spec S5 §3.1 step 2, T4–T5): the Solana connector. One SPL Memo
//! transaction, checked against the intent **before** it is signed, dry-run on two
//! nodes, sent through one; `Unknown` is never `Failed`, and two nodes that
//! disagree are not a pass. No network: the nodes are fakes.

use mu_connect::anchor::AnchorIntent;
use mu_connect::solana::{
    check_message, execute, reconcile, AnchorErr, Ed25519, NodeErr, RpcNode, Settled, Sim, SigState,
    SolanaCfg,
};
use std::cell::RefCell;

const SEAL: &str = "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100";

struct Signer;
impl Ed25519 for Signer {
    fn public(&self) -> [u8; 32] {
        [9u8; 32]
    }
    fn sign(&self, msg: &[u8]) -> [u8; 64] {
        // A deterministic stand-in: the connector must not care what signs.
        let mut s = [0u8; 64];
        s[0] = msg.len() as u8;
        s[1] = msg[msg.len() - 1];
        s
    }
}

/// A node that answers from a script and notes what it was asked.
struct Node {
    name: &'static str,
    fee: u64,
    sim: Result<Sim, NodeErr>,
    send: Result<String, NodeErr>,
    state: Result<SigState, NodeErr>,
    sends: RefCell<u32>,
    sent_txs: RefCell<Vec<Vec<u8>>>,
}
impl Node {
    fn good(name: &'static str) -> Self {
        Self {
            name,
            fee: 5_000,
            sim: Ok(Sim { ok: true, err: None }),
            send: Ok("sig-from-node".into()),
            state: Ok(SigState::NotFound),
            sends: RefCell::new(0),
            sent_txs: RefCell::new(vec![]),
        }
    }
}
impl RpcNode for Node {
    fn name(&self) -> &str {
        self.name
    }
    fn quote_fee(&self, _message: &[u8]) -> Result<u64, NodeErr> {
        Ok(self.fee)
    }
    fn latest_blockhash(&self) -> Result<[u8; 32], NodeErr> {
        Ok([5u8; 32])
    }
    fn simulate(&self, _tx: &[u8]) -> Result<Sim, NodeErr> {
        self.sim.clone()
    }
    fn send(&self, tx: &[u8]) -> Result<String, NodeErr> {
        *self.sends.borrow_mut() += 1;
        self.sent_txs.borrow_mut().push(tx.to_vec());
        self.send.clone()
    }
    fn status(&self, _signature: &str) -> Result<SigState, NodeErr> {
        self.state.clone()
    }
}

fn intent(memo: &str, cap: u64) -> AnchorIntent {
    AnchorIntent { memo: memo.into(), fee_cap_lamports: cap, cluster: "devnet".into(), capsule: "1220ca".into() }
}
fn cfg() -> SolanaCfg {
    SolanaCfg { cluster: "devnet".into() }
}

#[test]
fn a_good_intent_is_sent_once_and_the_transaction_carries_exactly_the_memo() {
    let (a, b) = (Node::good("a"), Node::good("b"));
    let sent = execute(&a, &b, &Signer, &intent(SEAL, 10_000), &cfg()).expect("sent");
    assert_eq!(*a.sends.borrow(), 1, "sent through one node, once");
    assert_eq!(*b.sends.borrow(), 0);
    assert!(!sent.signature.is_empty());
    // What was sent: one signature, then a message of one memo instruction.
    let tx = a.sent_txs.borrow()[0].clone();
    assert_eq!(tx[0], 1, "one signature");
    let message = &tx[1 + 64..];
    check_message(message, &intent(SEAL, 10_000), &Signer.public()).expect("the sent message is the intent's");
}

#[test]
fn anchor_intent_checked_before_anything_is_sent() {
    // A foreign memo and a fee above the cap are refused with nothing sent.
    for (memo, cap, fee) in [("pay Mallory", 10_000, 5_000), (SEAL, 1_000, 5_000)] {
        let (mut a, b) = (Node::good("a"), Node::good("b"));
        a.fee = fee;
        let r = execute(&a, &b, &Signer, &intent(memo, cap), &cfg());
        assert!(matches!(r, Err(AnchorErr::Rejected(_))), "{memo}/{cap}: {r:?}");
        assert_eq!(*a.sends.borrow() + *b.sends.borrow(), 0, "nothing sent");
    }
}

#[test]
fn two_rpc_agreement() {
    // One node simulates ok, the other refuses: a disagreement is not a pass, nothing is sent.
    let a = Node::good("a");
    let mut b = Node::good("b");
    b.sim = Ok(Sim { ok: false, err: Some("BlockhashNotFound".into()) });
    let r = execute(&a, &b, &Signer, &intent(SEAL, 10_000), &cfg());
    assert!(matches!(r, Err(AnchorErr::Rejected(_))), "{r:?}");
    assert_eq!(*a.sends.borrow() + *b.sends.borrow(), 0, "nothing sent on a disagreement");
}

#[test]
fn unknown_is_not_failed() {
    // The send times out: the transaction may have landed. The answer is Unknown
    // with the signature to ask the network about — never a refusal, never a failure.
    let mut a = Node::good("a");
    a.send = Err(NodeErr::Unreachable("timeout".into()));
    let b = Node::good("b");
    let r = execute(&a, &b, &Signer, &intent(SEAL, 10_000), &cfg());
    let Err(AnchorErr::Unknown { signature: Some(signature), .. }) = r else {
        panic!("a timed-out send is Unknown with the signature: {r:?}");
    };
    // Asked about: not found yet, nodes silent → still Pending, not Failed.
    let (mut a, mut b) = (Node::good("a"), Node::good("b"));
    a.state = Ok(SigState::NotFound);
    b.state = Err(NodeErr::Unreachable("down".into()));
    assert_eq!(reconcile(&a, &b, &signature), Settled::Pending);
    // Landed on either node: settled.
    a.state = Ok(SigState::Landed { slot: 77 });
    assert_eq!(reconcile(&a, &b, &signature), Settled::Landed { slot: 77 });
    // Failed only when BOTH nodes say the blockhash expired and the transaction never landed.
    a.state = Ok(SigState::Expired);
    b.state = Ok(SigState::Expired);
    assert_eq!(reconcile(&a, &b, &signature), Settled::Failed);
    a.state = Ok(SigState::Expired);
    b.state = Err(NodeErr::Unreachable("down".into()));
    assert_eq!(reconcile(&a, &b, &signature), Settled::Pending, "one silent node is not an answer");
}

#[test]
fn a_message_that_is_not_the_intents_is_refused_before_signing() {
    let (a, b) = (Node::good("a"), Node::good("b"));
    execute(&a, &b, &Signer, &intent(SEAL, 10_000), &cfg()).expect("sent");
    let tx = a.sent_txs.borrow()[0].clone();
    let message = tx[1 + 64..].to_vec();
    // Another memo in the same shape.
    let other = "ksg:bs:v1 ffffffffffffffffffffffffffffffff ffeeddccbbaa99887766554433221100";
    assert!(check_message(&message, &intent(other, 10_000), &Signer.public()).is_err());
    // Another fee payer.
    assert!(check_message(&message, &intent(SEAL, 10_000), &[1u8; 32]).is_err());
    // A message with the memo cut off.
    assert!(check_message(&message[..message.len() - 3], &intent(SEAL, 10_000), &Signer.public()).is_err());
}
