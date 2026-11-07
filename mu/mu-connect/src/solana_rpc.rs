//! P11d (Workflow μ Gate §3): the HTTP client of a Solana node for the anchor connector's
//! [`RpcNode`] — `getLatestBlockhash`, `getFeeForMessage`, `simulateTransaction`,
//! `sendTransaction`, `getSignatureStatuses` (+ `isBlockhashValid`), and `getBalance`.
//!
//! Two things are kept apart, as everywhere in `mu-connect`:
//!
//! - the **transport** is a function (`Post`): the client builds a JSON-RPC request, hands the
//!   body to it and reads the answer; the real one is [`http_post`] (feature `http`, 10 s
//!   timeout), a test's is anything. Nothing here opens a socket by itself;
//! - what the node **says** and what a node that says nothing means: a transport error, a
//!   timeout, garbage, a JSON-RPC error that is not a refusal — [`NodeErr::Unreachable`] (the
//!   outcome is not known); a refusal the node gives *before* the transaction reaches the chain
//!   (a failed preflight, a bad request) — [`NodeErr::Rejected`]. One node down is therefore
//!   `Unknown` for the anchor, never `Failed` (RISK-M7-1).
//!
//! A signature is judged `Expired` only with the blockhash its transaction was built on, which
//! the client remembers when it sends: asked about a signature it did not send, it can say
//! `NotFound`, never `Expired` — it cannot know, and "cannot know" must not become "failed".

use std::cell::RefCell;
use std::collections::BTreeMap;

use base64::Engine as _;
use serde_json::{json, Value};

use crate::solana::{base58, NodeErr, RpcNode, SigState, Sim};

/// How long one request may take, milliseconds (Workflow μ Gate §3: 10 s).
pub const TIMEOUT_MS: u64 = 10_000;

/// The transport: `(url, json body, timeout ms)` → the answer's body, or why there is none.
pub type Post = fn(&str, &str, u64) -> Result<String, String>;

/// A Solana node reached over JSON-RPC.
pub struct SolanaRpc {
    name: String,
    url: String,
    timeout_ms: u64,
    post: Post,
    /// The blockhash each transaction this client sent was built on, by signature.
    sent: RefCell<BTreeMap<String, [u8; 32]>>,
}

impl SolanaRpc {
    /// A client for the node at `url`, called `name` in messages.
    #[must_use]
    pub fn new(name: &str, url: &str, post: Post) -> Self {
        Self {
            name: name.to_owned(),
            url: url.to_owned(),
            timeout_ms: TIMEOUT_MS,
            post,
            sent: RefCell::new(BTreeMap::new()),
        }
    }

    /// The same, with another timeout (tests).
    #[must_use]
    pub fn with_timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// One JSON-RPC call: the `result`, or what the failure means.
    fn call(&self, method: &str, params: &Value) -> Result<Value, NodeErr> {
        let body = json!({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).to_string();
        let answer = (self.post)(&self.url, &body, self.timeout_ms)
            .map_err(|e| NodeErr::Unreachable(format!("{method}: {e}")))?;
        let v: Value = serde_json::from_str(&answer)
            .map_err(|_| NodeErr::Unreachable(format!("{method}: the answer is not JSON")))?;
        if let Some(err) = v.get("error") {
            let code = err.get("code").and_then(Value::as_i64).unwrap_or(0);
            let message = err.get("message").and_then(Value::as_str).unwrap_or("").to_owned();
            // The node's refusals before anything reached the chain: a failed preflight
            // (-32002), a signature that does not verify (-32003), a request that is not one
            // (-32600, -32602). Anything else — a node that is behind, busy, unhealthy
            // (-32005, -32004, -32007 …) — is not a "no": the outcome is not known.
            return Err(if matches!(code, -32002 | -32003 | -32600 | -32602) {
                NodeErr::Rejected(format!("{method}: {message} ({code})"))
            } else {
                NodeErr::Unreachable(format!("{method}: {message} ({code})"))
            });
        }
        v.get("result")
            .cloned()
            .ok_or_else(|| NodeErr::Unreachable(format!("{method}: the answer has no result")))
    }

    /// The balance of `address` (base58), lamports.
    ///
    /// # Errors
    ///
    /// [`NodeErr`] when the node does not answer.
    pub fn balance(&self, address: &str) -> Result<u64, NodeErr> {
        let r = self.call("getBalance", &json!([address, {"commitment": "confirmed"}]))?;
        r.get("value")
            .and_then(Value::as_u64)
            .ok_or_else(|| NodeErr::Unreachable("getBalance: no balance in the answer".into()))
    }
}

/// The address of a public key, as a node names it (base58).
#[must_use]
pub fn address_of(public: &[u8; 32]) -> String {
    base58(public)
}

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

/// base58 → 32 bytes.
fn base58_32(text: &str) -> Option<[u8; 32]> {
    const A: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut out: Vec<u8> = Vec::new();
    for c in text.bytes() {
        let mut carry = A.iter().position(|a| *a == c)? as u32;
        for b in &mut out {
            carry += u32::from(*b) * 58;
            *b = (carry & 0xff) as u8;
            carry >>= 8;
        }
        while carry > 0 {
            out.push((carry & 0xff) as u8);
            carry >>= 8;
        }
    }
    out.extend(std::iter::repeat_n(0u8, text.bytes().take_while(|c| *c == b'1').count()));
    out.reverse();
    out.try_into().ok()
}

/// The blockhash a signed transaction was built on: `[count][signatures][header 3]
/// [account count][accounts][blockhash 32]…` (legacy message).
fn blockhash_of(tx: &[u8]) -> Option<[u8; 32]> {
    let sigs = usize::from(*tx.first()?);
    let msg = tx.get(1 + 64 * sigs..)?;
    let accounts = usize::from(*msg.get(3)?);
    let at = 4 + 32 * accounts;
    msg.get(at..at + 32)?.try_into().ok()
}

impl RpcNode for SolanaRpc {
    fn name(&self) -> &str {
        &self.name
    }

    fn quote_fee(&self, message: &[u8]) -> Result<u64, NodeErr> {
        let r = self.call("getFeeForMessage", &json!([b64(message), {"commitment": "confirmed"}]))?;
        r.get("value")
            .and_then(Value::as_u64)
            .ok_or_else(|| NodeErr::Unreachable("getFeeForMessage: no fee in the answer (the blockhash is not known to the node)".into()))
    }

    fn latest_blockhash(&self) -> Result<[u8; 32], NodeErr> {
        let r = self.call("getLatestBlockhash", &json!([{"commitment": "finalized"}]))?;
        r.pointer("/value/blockhash")
            .and_then(Value::as_str)
            .and_then(base58_32)
            .ok_or_else(|| NodeErr::Unreachable("getLatestBlockhash: no blockhash in the answer".into()))
    }

    fn simulate(&self, tx: &[u8]) -> Result<Sim, NodeErr> {
        let r = self.call(
            "simulateTransaction",
            &json!([b64(tx), {"encoding": "base64", "sigVerify": true, "commitment": "confirmed"}]),
        )?;
        match r.pointer("/value/err") {
            Some(Value::Null) => Ok(Sim { ok: true, err: None }),
            Some(e) => Ok(Sim { ok: false, err: Some(e.to_string()) }),
            None => Err(NodeErr::Unreachable("simulateTransaction: the answer says neither ok nor err".into())),
        }
    }

    fn send(&self, tx: &[u8]) -> Result<String, NodeErr> {
        let r = self.call(
            "sendTransaction",
            &json!([b64(tx), {"encoding": "base64", "preflightCommitment": "confirmed"}]),
        )?;
        let signature = r
            .as_str()
            .ok_or_else(|| NodeErr::Unreachable("sendTransaction: the answer is not a signature".into()))?
            .to_owned();
        if let Some(bh) = blockhash_of(tx) {
            self.sent.borrow_mut().insert(signature.clone(), bh);
        }
        // The signature the node reports must be the one of the bytes sent: the first 64
        // bytes after the count. A node that reports another is not trusted with the outcome.
        if let Some(first) = tx.get(1..65) {
            if base58(first) != signature {
                return Err(NodeErr::Unreachable(format!(
                    "sendTransaction: the node reports {signature}, which is not the signature of what was sent"
                )));
            }
        }
        Ok(signature)
    }

    fn status(&self, signature: &str) -> Result<SigState, NodeErr> {
        let r = self.call(
            "getSignatureStatuses",
            &json!([[signature], {"searchTransactionHistory": true}]),
        )?;
        let first = r.pointer("/value/0").ok_or_else(|| NodeErr::Unreachable("getSignatureStatuses: no value".into()))?;
        if first.is_null() {
            // Not seen. Expired only if this client knows the blockhash and the node says it is gone.
            let known = self.sent.borrow().get(signature).copied();
            let Some(bh) = known else { return Ok(SigState::NotFound) };
            let valid = self.call("isBlockhashValid", &json!([base58(&bh), {"commitment": "processed"}]))?;
            return Ok(match valid.get("value").and_then(Value::as_bool) {
                Some(false) => SigState::Expired,
                _ => SigState::NotFound,
            });
        }
        // Landed with an error wrote no memo: the transaction can never land again, but the
        // blockhash may live — it is not `Landed`, and it is not known to be dead.
        if !first.get("err").is_none_or(Value::is_null) {
            return Ok(SigState::NotFound);
        }
        match first.get("confirmationStatus").and_then(Value::as_str) {
            Some("finalized" | "confirmed") => Ok(SigState::Landed {
                slot: first.get("slot").and_then(Value::as_u64).unwrap_or(0),
            }),
            _ => Ok(SigState::NotFound),
        }
    }
}

/// The real transport: HTTP POST of the body, `Content-Type: application/json`, a timeout.
///
/// # Errors
///
/// The connection, the timeout, a status other than 200, an answer that cannot be read.
#[cfg(feature = "http")]
pub fn http_post(url: &str, body: &str, timeout_ms: u64) -> Result<String, String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build();
    agent
        .post(url)
        .set("Content-Type", "application/json")
        .send_string(body)
        .map_err(|e| match e {
            ureq::Error::Status(code, _) => format!("the node answers HTTP {code}"),
            ureq::Error::Transport(t) => t.to_string(),
        })?
        .into_string()
        .map_err(|e| format!("the answer cannot be read: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static SAID: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
        static WILL: RefCell<Vec<Result<String, String>>> = const { RefCell::new(Vec::new()) };
    }

    fn fake(_url: &str, body: &str, _t: u64) -> Result<String, String> {
        SAID.with(|s| s.borrow_mut().push(body.to_owned()));
        WILL.with(|w| if w.borrow().is_empty() { Err("nothing to say".into()) } else { w.borrow_mut().remove(0) })
    }

    fn node() -> SolanaRpc {
        SAID.with(|s| s.borrow_mut().clear());
        WILL.with(|w| w.borrow_mut().clear());
        SolanaRpc::new("a", "http://x", fake)
    }

    fn will(answers: &[Result<&str, &str>]) {
        WILL.with(|w| {
            *w.borrow_mut() = answers.iter().map(|a| a.map(str::to_owned).map_err(str::to_owned)).collect();
        });
    }

    fn result(v: Value) -> String {
        json!({"jsonrpc": "2.0", "id": 1, "result": v}).to_string()
    }

    fn error(code: i64) -> String {
        json!({"jsonrpc": "2.0", "id": 1, "error": {"code": code, "message": "m"}}).to_string()
    }

    /// A transaction as the connector signs it: one signature, a memo message.
    fn tx(blockhash: [u8; 32]) -> Vec<u8> {
        let mut t = vec![1u8];
        t.extend_from_slice(&[7u8; 64]);
        t.extend_from_slice(&[1, 0, 1, 3]);
        t.extend_from_slice(&[0u8; 96]);
        t.extend_from_slice(&blockhash);
        t
    }

    #[test]
    fn the_blockhash_comes_back_as_32_bytes() {
        let n = node();
        let bh = [9u8; 32];
        will(&[Ok(&result(json!({"value": {"blockhash": base58(&bh), "lastValidBlockHeight": 1}})))]);
        assert_eq!(n.latest_blockhash(), Ok(bh));
        let said = SAID.with(|s| s.borrow()[0].clone());
        assert!(said.contains("getLatestBlockhash") && said.contains("finalized"), "{said}");
    }

    #[test]
    fn a_node_that_says_nothing_is_unreachable_never_rejected() {
        let n = node();
        will(&[Err("timed out"), Ok("<html>"), Ok(&error(-32005)), Ok(&result(json!({"value": null})))]);
        for _ in 0..3 {
            assert!(matches!(n.latest_blockhash(), Err(NodeErr::Unreachable(_))));
        }
        assert!(matches!(n.quote_fee(&[1, 2, 3]), Err(NodeErr::Unreachable(_))), "a fee the node cannot quote is not a fee");
    }

    #[test]
    fn a_failed_preflight_is_a_refusal() {
        let n = node();
        will(&[Ok(&error(-32002))]);
        assert!(matches!(n.send(&tx([1; 32])), Err(NodeErr::Rejected(_))));
    }

    #[test]
    fn the_simulation_says_ok_or_why_not() {
        let n = node();
        will(&[Ok(&result(json!({"value": {"err": null}}))), Ok(&result(json!({"value": {"err": {"InstructionError": [0, "Custom"]}}})))]);
        assert_eq!(n.simulate(&[1]), Ok(Sim { ok: true, err: None }));
        let bad = n.simulate(&[1]).expect("an answer");
        assert!(!bad.ok && bad.err.is_some_and(|e| e.contains("InstructionError")));
    }

    #[test]
    fn a_send_is_trusted_only_when_the_node_reports_the_signature_of_what_was_sent() {
        let n = node();
        let t = tx([2; 32]);
        let own = base58(&t[1..65]);
        will(&[Ok(&result(json!(own.clone()))), Ok(&result(json!(base58(&[5u8; 64]))))]);
        assert_eq!(n.send(&t), Ok(own));
        assert!(matches!(n.send(&t), Err(NodeErr::Unreachable(_))), "another signature is not ours");
    }

    #[test]
    fn a_signature_is_landed_when_confirmed_without_error() {
        let n = node();
        will(&[
            Ok(&result(json!({"value": [{"slot": 42, "err": null, "confirmationStatus": "finalized"}]}))),
            Ok(&result(json!({"value": [{"slot": 43, "err": null, "confirmationStatus": "processed"}]}))),
            Ok(&result(json!({"value": [{"slot": 44, "err": {"InstructionError": [0, "x"]}, "confirmationStatus": "finalized"}]}))),
        ]);
        assert_eq!(n.status("sig"), Ok(SigState::Landed { slot: 42 }));
        assert_eq!(n.status("sig"), Ok(SigState::NotFound), "not yet confirmed");
        assert_eq!(n.status("sig"), Ok(SigState::NotFound), "landed with an error: no memo, not Landed");
    }

    #[test]
    fn expired_only_for_a_signature_this_client_sent_and_a_blockhash_the_node_says_is_gone() {
        let n = node();
        let t = tx([3; 32]);
        let own = base58(&t[1..65]);
        will(&[
            Ok(&result(json!(own.clone()))),
            // not seen; the blockhash is gone → Expired
            Ok(&result(json!({"value": [null]}))), Ok(&result(json!({"value": false}))),
            // not seen; the blockhash is alive → NotFound (still may land)
            Ok(&result(json!({"value": [null]}))), Ok(&result(json!({"value": true}))),
            // a signature never sent here → NotFound, and isBlockhashValid is not even asked
            Ok(&result(json!({"value": [null]}))),
        ]);
        n.send(&t).expect("sent");
        assert_eq!(n.status(&own), Ok(SigState::Expired));
        assert_eq!(n.status(&own), Ok(SigState::NotFound));
        assert_eq!(n.status("another"), Ok(SigState::NotFound), "cannot know is not failed");
        assert_eq!(SAID.with(|s| s.borrow().iter().filter(|b| b.contains("isBlockhashValid")).count()), 2);
    }

    #[test]
    fn one_node_down_is_unknown_for_the_anchor_not_failed() {
        use crate::solana::{reconcile, Settled};
        let down = node();
        will(&[Err("connection refused")]);
        let up = SolanaRpc::new("b", "http://y", |_, _, _| {
            Ok(json!({"result": {"value": [{"slot": 1, "err": null, "confirmationStatus": "finalized"}]}}).to_string())
        });
        assert_eq!(reconcile(&down, &up, "s"), Settled::Landed { slot: 1 }, "the node that answers is believed");
        let nothing = SolanaRpc::new("c", "http://z", |_, _, _| Err("timed out".into()));
        assert_eq!(reconcile(&down, &nothing, "s"), Settled::Pending, "two silent nodes: Unknown, never Failed");
    }

    #[test]
    fn balance_is_lamports() {
        let n = node();
        will(&[Ok(&result(json!({"context": {"slot": 1}, "value": 9_014_156})))]);
        assert_eq!(n.balance("addr"), Ok(9_014_156));
    }

    #[test]
    fn base58_round_trips_32_bytes() {
        for seed in [0u8, 1, 57, 200] {
            let mut b = [seed; 32];
            b[0] = 0;
            assert_eq!(base58_32(&base58(&b)), Some(b));
        }
    }
}
