//! The policy record: a full Δ in the journal must survive a reopen, otherwise
//! recovery would fall back to the cache file (or lose the change).
use mu_common::Hash32;
use mu_log::{Kind, Log};
use mu_vault::backend::SoftVault;

fn vault() -> SoftVault {
    SoftVault::for_test([1; 32], [2; 32], [3; 32])
}

#[test]
fn a_delta_record_survives_a_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("l.mulog");
    let v = vault();
    let delta: Vec<u8> = (0..120u8).collect();
    {
        let mut log = Log::open(&path, &v).unwrap();
        log.append(
            Kind::DeltaApplied {
                version: 3,
                prev: Hash32([1; 32]),
                next: Hash32([2; 32]),
                delta: delta.clone(),
                owner_sig: [7; 64],
            },
            10,
            &v,
        )
        .unwrap();
    }

    let log = Log::open(&path, &v).unwrap();
    log.verify_chain().unwrap();
    assert_eq!(log.entries().len(), 1, "the record must not be a corrupted tail");
    match &log.entries()[0].kind {
        Kind::DeltaApplied { version, prev, next, delta: d, owner_sig } => {
            assert_eq!(*version, 3);
            assert_eq!(prev.0, [1; 32]);
            assert_eq!(next.0, [2; 32]);
            assert_eq!(d, &delta);
            assert_eq!(owner_sig, &[7; 64]);
        }
        other => panic!("expected DeltaApplied, got {other:?}"),
    }
}
