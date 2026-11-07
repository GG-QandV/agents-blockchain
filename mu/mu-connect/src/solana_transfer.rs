//! P11f (Workflow μ Gate §3): the move of lamports from the main wallet to a capsule's sub-wallet —
//! the refill (Spec S5 §3.2).
//!
//! The same discipline as the anchor's (`solana.rs`), spelled for one System-program transfer:
//! the message is built by one function and **decoded again** before it is signed — one signer, one
//! instruction, the System program, the two accounts and the amount it was asked for; both nodes
//! dry-run the very bytes; a send whose outcome is not known is `Unknown` with the signature, never
//! `Failed` (RISK-M7-1). Nothing here opens a socket or holds a key.

use crate::solana::{base58, AnchorErr, Ed25519, RpcNode, Sent, Sim, SolanaCfg};

/// The System program's address: 32 zero bytes (`1111…1111`).
pub const SYSTEM_PROGRAM_ID: [u8; 32] = [0u8; 32];

/// Builds the one message: `payer` pays the fee and sends `lamports` to `to`.
#[must_use]
pub fn build_transfer_message(payer: &[u8; 32], to: &[u8; 32], lamports: u64, blockhash: &[u8; 32]) -> Vec<u8> {
    let mut m = vec![1, 0, 1]; // one signature, none read-only among the signers, one read-only account
    m.push(3); // three accounts (compact-u16 < 128 is one byte)
    m.extend_from_slice(payer);
    m.extend_from_slice(to);
    m.extend_from_slice(&SYSTEM_PROGRAM_ID);
    m.extend_from_slice(blockhash);
    m.push(1); // one instruction
    m.push(2); // program id index: the System program
    m.push(2); // two accounts
    m.extend_from_slice(&[0, 1]); // from, to
    m.push(12); // twelve bytes of data
    m.extend_from_slice(&2u32.to_le_bytes()); // instruction 2: Transfer
    m.extend_from_slice(&lamports.to_le_bytes());
    m
}

/// Decodes `message` again: exactly what [`build_transfer_message`] makes for these arguments.
///
/// # Errors
///
/// A reason in words when the message is anything else.
pub fn check_transfer_message(message: &[u8], payer: &[u8; 32], to: &[u8; 32], lamports: u64) -> Result<(), String> {
    let again = {
        // the blockhash sits after the header, the count and three keys
        let at = 3 + 1 + 96;
        let Some(bh) = message.get(at..at + 32) else { return Err("the message is cut off".into()) };
        let Ok(bh): Result<[u8; 32], _> = bh.try_into() else { return Err("the blockhash is not 32 bytes".into()) };
        build_transfer_message(payer, to, lamports, &bh)
    };
    if message == again.as_slice() {
        Ok(())
    } else {
        Err("the message is not the transfer that was asked for (accounts, program or amount differ)".into())
    }
}

/// Moves `lamports` from the signer's wallet to `to`, through two nodes.
///
/// # Errors
///
/// [`AnchorErr::Rejected`] when nothing was sent and nothing will be (the message is not the
/// request's, a dry run does not pass, the fee passes `fee_cap`); [`AnchorErr::Unknown`] when a node
/// did not answer or the send's outcome is not known — with the signature when one was signed.
pub fn execute_transfer(
    a: &dyn RpcNode,
    b: &dyn RpcNode,
    signer: &dyn Ed25519,
    to: &[u8; 32],
    lamports: u64,
    fee_cap_lamports: u64,
    _cfg: &SolanaCfg,
) -> Result<Sent, AnchorErr> {
    let unknown = |node: &dyn RpcNode, e: &crate::solana::NodeErr| AnchorErr::Unknown {
        reason: format!("node {}: {e:?}", node.name()),
        signature: None,
    };
    let payer = signer.public();
    let blockhash = a.latest_blockhash().map_err(|e| unknown(a, &e))?;
    let message = build_transfer_message(&payer, to, lamports, &blockhash);
    let fee = a.quote_fee(&message).map_err(|e| unknown(a, &e))?.max(b.quote_fee(&message).map_err(|e| unknown(b, &e))?);
    if fee > fee_cap_lamports {
        return Err(AnchorErr::Rejected(format!("the fee {fee} passes the cap {fee_cap_lamports}")));
    }
    check_transfer_message(&message, &payer, to, lamports).map_err(AnchorErr::Rejected)?;

    let signature = signer.sign(&message);
    let mut tx = vec![1u8];
    tx.extend_from_slice(&signature);
    tx.extend_from_slice(&message);
    let signature_text = base58(&signature);

    let sim_a: Sim = a.simulate(&tx).map_err(|e| unknown(a, &e))?;
    let sim_b: Sim = b.simulate(&tx).map_err(|e| unknown(b, &e))?;
    if !(sim_a.ok && sim_b.ok) {
        return Err(AnchorErr::Rejected(format!(
            "the dry run of the transfer does not pass on both nodes ({}: {}, {}: {})",
            a.name(),
            if sim_a.ok { "ok".into() } else { sim_a.err.unwrap_or_default() },
            b.name(),
            if sim_b.ok { "ok".into() } else { sim_b.err.unwrap_or_default() },
        )));
    }
    match a.send(&tx) {
        Ok(reported) => Ok(Sent {
            signature: if reported.is_empty() { signature_text } else { reported },
            fee_lamports: fee,
        }),
        Err(crate::solana::NodeErr::Rejected(why)) => Err(AnchorErr::Rejected(format!("node {}: {why}", a.name()))),
        Err(crate::solana::NodeErr::Unreachable(why)) => Err(AnchorErr::Unknown {
            reason: format!("the send's outcome is not known (node {}: {why})", a.name()),
            signature: Some(signature_text),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_message_is_what_a_transfer_is() {
        let m = build_transfer_message(&[1; 32], &[2; 32], 5_000_000, &[9; 32]);
        // header, 3 accounts, blockhash, one instruction (program 2, accounts 0 and 1, 12 bytes: 2u32 + u64)
        assert_eq!(m.len(), 3 + 1 + 96 + 32 + 1 + 1 + 1 + 2 + 1 + 12);
        assert_eq!(&m[m.len() - 12..m.len() - 8], &2u32.to_le_bytes());
        assert_eq!(&m[m.len() - 8..], &5_000_000u64.to_le_bytes());
        check_transfer_message(&m, &[1; 32], &[2; 32], 5_000_000).expect("it is what was asked for");
    }

    #[test]
    fn another_amount_another_recipient_or_another_payer_is_not_signed() {
        let m = build_transfer_message(&[1; 32], &[2; 32], 5_000_000, &[9; 32]);
        assert!(check_transfer_message(&m, &[1; 32], &[2; 32], 5_000_001).is_err(), "the amount");
        assert!(check_transfer_message(&m, &[1; 32], &[3; 32], 5_000_000).is_err(), "the recipient");
        assert!(check_transfer_message(&m, &[4; 32], &[2; 32], 5_000_000).is_err(), "the payer");
        let mut cut = m.clone();
        cut.pop();
        assert!(check_transfer_message(&cut, &[1; 32], &[2; 32], 5_000_000).is_err(), "cut off");
        let mut more = m;
        more.push(0);
        assert!(check_transfer_message(&more, &[1; 32], &[2; 32], 5_000_000).is_err(), "bytes after the instruction");
    }
}
