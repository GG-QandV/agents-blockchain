//! Compatibility with journals written before journal format 2.
//!
//! Format 2 only *added* tag 12 (`PendingAuth`); every older tag keeps its
//! bytes and its tag number, so an old file must read and verify as before.
use mu_common::Hash32;
use mu_log::{Kind, Log};
use mu_vault::backend::SoftVault;

fn vault() -> SoftVault {
    SoftVault::for_test([1; 32], [2; 32], [3; 32])
}
fn ih(x: u8) -> Hash32 {
    Hash32([x; 32])
}

#[test]
fn old_tags_keep_their_bytes_and_lengths() {
    // Any shift here would silently break old journals and the hash chain.
    let pending = Kind::Pending { intent_hash: ih(1), amount_total: 100, chain_nonce: 7 }.encode();
    assert_eq!(pending[0], 3, "Pending is tag 3");
    assert_eq!(pending.len(), 1 + 32 + 16 + 8);

    let settled = Kind::Settled { intent_hash: ih(1), tx_hash: [9; 32], effective_gas: 5 }.encode();
    assert_eq!(settled[0], 4, "Settled is tag 4");
    assert_eq!(settled.len(), 1 + 32 + 32 + 16);

    let denied = Kind::DeniedOmega { intent_hash: ih(1) }.encode();
    assert_eq!(denied[0], 1);
    assert_eq!(denied.len(), 33);

    let alert = Kind::Alert { code: 0x0A02 }.encode();
    assert_eq!(alert[0], 11);
    assert_eq!(alert.len(), 3);
}

#[test]
fn a_journal_of_the_old_format_opens_and_verifies() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("old.mulog");
    let v = vault();
    {
        let mut log = Log::open(&path, &v).unwrap();
        log.append(Kind::Pending { intent_hash: ih(1), amount_total: 100, chain_nonce: 7 }, 1_000, &v)
            .unwrap();
        // A different intent: the Pending reserve above must stay open.
        log.append(Kind::Settled { intent_hash: ih(9), tx_hash: [9; 32], effective_gas: 5 }, 1_001, &v)
            .unwrap();
        log.append(Kind::DeltaChanged { old_hash: ih(2), new_hash: ih(3) }, 1_002, &v).unwrap();
        log.append(Kind::HumanDecision { intent_hash: ih(1), approved: true }, 1_003, &v).unwrap();
        log.verify_chain().unwrap();
    }

    // Reopen: no tag 12 anywhere, exactly like a 0.2.0 journal.
    let log = Log::open(&path, &v).unwrap();
    log.verify_chain().unwrap();
    assert_eq!(log.entries().len(), 4);
    assert_eq!(log.pending().len(), 1);
    assert!(log
        .entries()
        .iter()
        .all(|e| !matches!(e.kind, Kind::PendingAuth { .. })));
}

#[test]
fn a_mixed_journal_reads_both_forms_and_keeps_chaining() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mixed.mulog");
    let v = vault();
    {
        let mut log = Log::open(&path, &v).unwrap();
        log.append(Kind::Pending { intent_hash: ih(1), amount_total: 100, chain_nonce: 0 }, 1_000, &v)
            .unwrap();
        log.append(
            Kind::PendingAuth {
                intent_hash: ih(2),
                amount_total: 200,
                nonce: [5; 32],
                valid_after: 900,
                valid_before: 1_200,
            },
            1_001,
            &v,
        )
        .unwrap();
    }

    let mut log = Log::open(&path, &v).unwrap();
    log.verify_chain().unwrap();
    assert_eq!(log.pending().len(), 2, "both reserves are visible");

    // The chain continues across the reopen, so the old part stays verifiable.
    log.append(Kind::Failed { intent_hash: ih(2) }, 1_300, &v).unwrap();
    log.verify_chain().unwrap();
    assert_eq!(log.pending().len(), 1);
}
