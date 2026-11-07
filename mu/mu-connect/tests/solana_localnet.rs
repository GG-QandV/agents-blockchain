//! P11d against a real node: `solana-test-validator`. Ignored by default — it needs a validator
//! (`MU_SOLANA_LOCALNET=http://127.0.0.1:8899`) and costs nothing but a minute.
//!
//! Run: `solana-test-validator --reset --ledger DIR --quiet &` then
//! `MU_SOLANA_LOCALNET=http://127.0.0.1:8899 cargo test -p mu-connect --features ksg,http --test solana_localnet -- --ignored`
#![cfg(all(feature = "ksg", feature = "http"))]

use mu_connect::anchor::AnchorIntent;
use mu_connect::solana::{execute, reconcile, AnchorErr, Ed25519, NodeErr, RpcNode, Settled, SolanaCfg};
use mu_connect::solana_rpc::{address_of, http_post, SolanaRpc};
use mu_vault::backend::SoftVault;
use mu_vault::Vault;

const SEAL: &str = "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100";

fn url() -> Option<String> {
    std::env::var("MU_SOLANA_LOCALNET").ok()
}

fn intent() -> AnchorIntent {
    AnchorIntent { memo: SEAL.into(), fee_cap_lamports: 20_000, cluster: "localnet".into(), capsule: "1220cafe".into() }
}

fn funded(node: &SolanaRpc, address: &str) {
    // The validator keeps its state between runs: wait for the airdrop to ARRIVE (the balance rises),
    // not merely for the wallet to be non-empty.
    let before = node.balance(address).unwrap_or(0);
    let body = format!(r#"{{"jsonrpc":"2.0","id":1,"method":"requestAirdrop","params":["{address}",1000000000]}}"#);
    http_post(&url().expect("a validator"), &body, 10_000).expect("the airdrop is asked for");
    for _ in 0..60 {
        if node.balance(address).unwrap_or(0) > before {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    panic!("the airdrop did not arrive");
}

#[test]
#[ignore = "needs solana-test-validator (MU_SOLANA_LOCALNET)"]
fn a_memo_goes_out_lands_and_the_balance_moves_and_one_node_down_is_unknown() {
    let Some(url) = url() else { return };
    let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
    let signer = vault.capsule_signer_ed25519("1220cafe").expect("the capsule's sub-wallet");
    let payer = address_of(&Ed25519::public(&signer));
    let (a, b) = (SolanaRpc::new("a", &url, http_post), SolanaRpc::new("b", &url, http_post));
    funded(&a, &payer);
    let before = a.balance(&payer).expect("a balance");

    // A memo goes out through both nodes' dry run, lands, and costs the fee.
    let cfg = SolanaCfg { cluster: "localnet".into() };
    let sent = execute(&a, &b, &signer, &intent(), &cfg).expect("sent");
    let mut settled = Settled::Pending;
    for _ in 0..60 {
        settled = reconcile(&a, &b, &sent.signature);
        if matches!(settled, Settled::Landed { .. }) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    assert!(matches!(settled, Settled::Landed { .. }), "the memo landed: {settled:?}");
    let after = a.balance(&payer).expect("a balance");
    assert_eq!(before - after, 5_000, "one signature, one base fee");

    // One node down: nothing is sent, and the answer is Unknown — never Failed.
    let down = SolanaRpc::new("down", "http://127.0.0.1:1", http_post).with_timeout_ms(1_000);
    let err = execute(&a, &down, &signer, &intent(), &cfg).expect_err("the second node is silent");
    assert!(matches!(err, AnchorErr::Unknown { .. }), "{err:?}");
    assert_eq!(a.balance(&payer).expect("a balance"), after, "nothing was sent");
    assert!(matches!(down.latest_blockhash(), Err(NodeErr::Unreachable(_))));
}

#[test]
#[ignore = "needs solana-test-validator (MU_SOLANA_LOCALNET)"]
fn a_refill_moves_lamports_from_the_main_wallet_to_the_sub_wallet_on_a_real_node() {
    use mu_connect::solana_transfer::execute_transfer;
    let Some(url) = url() else { return };
    let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [7u8; 32]);
    let main = vault.tx_signer_ed25519().expect("the main wallet");
    let sub = vault.capsule_signer_ed25519("1220feed").expect("the sub-wallet"); // another capsule than the memo test's
    let (main_addr, sub_addr) = (address_of(&main.public_key()), address_of(&Ed25519::public(&sub)));
    let (a, b) = (SolanaRpc::new("a", &url, http_post), SolanaRpc::new("b", &url, http_post));
    funded(&a, &main_addr);
    let (main_before, sub_before) = (a.balance(&main_addr).expect("balance"), a.balance(&sub_addr).unwrap_or(0));

    let cfg = SolanaCfg { cluster: "localnet".into() };
    let sent = execute_transfer(&a, &b, &main, &Ed25519::public(&sub), 2_000_000, 20_000, &cfg).expect("sent");
    let mut landed = false;
    for _ in 0..60 {
        if matches!(reconcile(&a, &b, &sent.signature), Settled::Landed { .. }) {
            landed = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    assert!(landed, "the transfer landed");
    assert_eq!(a.balance(&sub_addr).expect("balance") - sub_before, 2_000_000, "the sub-wallet got the amount");
    assert_eq!(main_before - a.balance(&main_addr).expect("balance"), 2_000_000 + 5_000, "the main wallet paid the amount and the fee");
}
