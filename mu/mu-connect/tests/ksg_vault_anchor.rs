//! P11c: the connector signs through the vault's sub-wallet of the capsule: the
//! transaction that goes out is the capsule's own payer's, and it verifies.
#![cfg(feature = "ksg")]

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use mu_connect::anchor::AnchorIntent;
use mu_connect::solana::{check_message, execute, Ed25519, NodeErr, RpcNode, SigState, Sim, SolanaCfg};
use mu_vault::backend::SoftVault;
use mu_vault::Vault;
use std::cell::RefCell;

const SEAL: &str = "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100";

struct Node(RefCell<Vec<Vec<u8>>>);
impl RpcNode for Node {
    fn name(&self) -> &str {
        "n"
    }
    fn quote_fee(&self, _m: &[u8]) -> Result<u64, NodeErr> {
        Ok(5_000)
    }
    fn latest_blockhash(&self) -> Result<[u8; 32], NodeErr> {
        Ok([5u8; 32])
    }
    fn simulate(&self, _t: &[u8]) -> Result<Sim, NodeErr> {
        Ok(Sim { ok: true, err: None })
    }
    fn send(&self, t: &[u8]) -> Result<String, NodeErr> {
        self.0.borrow_mut().push(t.to_vec());
        Ok(String::new())
    }
    fn status(&self, _s: &str) -> Result<SigState, NodeErr> {
        Ok(SigState::NotFound)
    }
}

#[test]
fn the_capsules_sub_wallet_signs_the_anchor_that_goes_out() {
    let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
    let signer = vault.capsule_signer_ed25519("1220cafe").expect("the capsule's sub-wallet");
    let (a, b) = (Node(RefCell::new(vec![])), Node(RefCell::new(vec![])));
    let intent = AnchorIntent {
        memo: SEAL.into(),
        fee_cap_lamports: 10_000,
        cluster: "devnet".into(),
        capsule: "1220cafe".into(),
    };
    execute(&a, &b, &signer, &intent, &SolanaCfg { cluster: "devnet".into() }).expect("sent");

    let tx = a.0.borrow()[0].clone();
    let (sig, message) = (&tx[1..65], &tx[65..]);
    check_message(message, &intent, &signer.public()).expect("the message names the sub-wallet as payer");
    let vk = VerifyingKey::from_bytes(&Ed25519::public(&signer)).expect("a public key");
    assert!(
        vk.verify(message, &Signature::from_bytes(sig.try_into().expect("64 bytes"))).is_ok(),
        "the signature is the capsule's sub-wallet's"
    );
    // The main wallet is not the payer.
    let main = vault.tx_signer_ed25519().expect("main");
    assert_ne!(main.public_key(), signer.public());
}
