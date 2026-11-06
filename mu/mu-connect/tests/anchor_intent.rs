//! P11a (Spec S5 §3.1 step 1, T1–T2): an `AnchorIntent` is checked **before**
//! anything is signed. Its memo must be a memo of the core v2 (`ksg:bs:v1` for a
//! blind seal, `ksg:st:v1` for a status record) — not the old v1 text, not
//! anybody else's — the cluster must be the configured one, and the fee must
//! not pass the cap.

use mu_connect::anchor::{check_intent, AnchorIntent, IntentError, MemoKind};

const SEAL: &str = "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100";
const STATUS: &str = "ksg:st:v1 1220aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa \
                      1220bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn intent(memo: &str) -> AnchorIntent {
    AnchorIntent {
        memo: memo.into(),
        fee_cap_lamports: 10_000,
        cluster: "devnet".into(),
        capsule: "1220cafe".into(),
    }
}

#[test]
fn anchor_intent_v2_memo_only() {
    assert_eq!(check_intent(&intent(SEAL), "devnet", 5_000), Ok(MemoKind::Seal));
    assert_eq!(check_intent(&intent(STATUS), "devnet", 5_000), Ok(MemoKind::Status));

    // The old v1 memo of the core v1 (the README of μ Gate KSG still shows it),
    // any other text, and a memo of the right tag with the wrong shape: refused.
    let v1 = "ksg:cp:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100";
    for bad in [
        v1,
        "pay 5 SOL to Mallory",
        "",
        // wrong blind width (15 bytes instead of 16)
        "ksg:bs:v1 00112233445566778899aabbccddee ffeeddccbbaa99887766554433221100",
        // upper-case hex is not the core's output
        "ksg:bs:v1 00112233445566778899AABBCCDDEEFF ffeeddccbbaa99887766554433221100",
        // a third field
        "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100 extra",
        // a status memo with a hash that is not a sha-256 multihash
        "ksg:st:v1 1220zz 1220bb",
        // the tag alone, and a tag with one field
        "ksg:bs:v1",
        "ksg:bs:v1 00112233445566778899aabbccddeeff",
        // a newline smuggling a second memo
        "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100\nksg:cp:v1 x",
    ] {
        assert_eq!(
            check_intent(&intent(bad), "devnet", 5_000),
            Err(IntentError::NotOurMemo),
            "memo {bad:?} must be refused before signing"
        );
    }
}

#[test]
fn fee_cap_enforced() {
    // Exactly the cap passes; one lamport more does not.
    assert_eq!(check_intent(&intent(SEAL), "devnet", 10_000), Ok(MemoKind::Seal));
    assert_eq!(
        check_intent(&intent(SEAL), "devnet", 10_001),
        Err(IntentError::FeeAboveCap { fee: 10_001, cap: 10_000 })
    );
}

#[test]
fn another_cluster_is_refused() {
    assert_eq!(
        check_intent(&intent(SEAL), "mainnet-beta", 5_000),
        Err(IntentError::WrongCluster { configured: "mainnet-beta".into(), intent: "devnet".into() })
    );
}
