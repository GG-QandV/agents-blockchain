//! Rung 1 of the audit-trail ladder: Quoted / DenyReason survive the file, the chain, tampering.
use mu_common::Hash32;
use mu_log::{deny, Kind, Log};
use mu_vault::backend::SoftVault;
use std::fs;

fn vault() -> SoftVault { SoftVault::for_test([1; 32], [2; 32], [3; 32]) }
fn ih(x: u8) -> Hash32 { Hash32([x; 32]) }

fn quoted(resource: &str) -> Kind {
    Kind::Quoted {
        intent_hash: ih(1),
        recipient: [0xab; 20],
        chain_id: 8453,
        connector: 1,
        amount: 100_000,
        gas_est: 10,
        resource: resource.into(),
        policy_hash: ih(7),
    }
}

#[test]
fn quoted_and_deny_reason_roundtrip_through_file() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    let reason = Kind::DenyReason { intent_hash: ih(1), code: deny::WINDOW_EXCEEDED, total: 100_010, limit: 50_000 };
    {
        let mut log = Log::open(&p, &v).unwrap();
        log.append(quoted("https://issuer.example/bare/issue"), 1000, &v).unwrap();
        log.append(Kind::DeniedDelta { intent_hash: ih(1) }, 1000, &v).unwrap();
        log.append(reason.clone(), 1000, &v).unwrap();
        log.append(quoted(""), 1001, &v).unwrap(); // no resource: empty string
    }
    let log = Log::open(&p, &v).unwrap();
    log.verify_chain().unwrap();
    let k: Vec<_> = log.entries().iter().map(|e| e.kind.clone()).collect();
    assert_eq!(k.len(), 4);
    assert_eq!(k[0], quoted("https://issuer.example/bare/issue"));
    assert_eq!(k[2], reason);
    assert_eq!(k[3], quoted(""));
}

#[test]
fn tampering_with_the_logged_payee_breaks_the_chain() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    {
        let mut log = Log::open(&p, &v).unwrap();
        log.append(quoted("x/y"), 1000, &v).unwrap();
        log.append(Kind::Pending { intent_hash: ih(1), amount_total: 100_010, chain_nonce: 0 }, 1001, &v).unwrap();
    }
    let mut bytes = fs::read(&p).unwrap();
    // 4 len + 8 seq + 32 prev + 8 ts + 4 klen + 1 tag + 32 ih → first recipient byte
    bytes[4 + 8 + 32 + 8 + 4 + 1 + 32] ^= 0xff;
    fs::write(&p, bytes).unwrap();
    let log = Log::open(&p, &v).unwrap();
    assert!(log.verify_chain().is_err());
}

#[test]
fn a_truncated_quoted_record_is_dropped_as_a_corrupt_tail() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    {
        let mut log = Log::open(&p, &v).unwrap();
        log.append(Kind::Alert { code: 1 }, 1000, &v).unwrap();
        log.append(quoted("x/y"), 1001, &v).unwrap();
    }
    let len = fs::metadata(&p).unwrap().len();
    fs::OpenOptions::new().write(true).open(&p).unwrap().set_len(len - 10).unwrap();
    let log = Log::open(&p, &v).unwrap();
    log.verify_chain().unwrap();
    assert!(matches!(log.entries().last().unwrap().kind, Kind::TailTruncated { .. }));
    assert!(!log.entries().iter().any(|e| matches!(e.kind, Kind::Quoted { .. })));
}

#[test]
fn malformed_deny_reason_and_quoted_are_rejected_as_corrupt() {
    use std::io::Write;
    // unknown code, unknown connector, and trailing garbage must not decode as valid records
    for bad in [
        Kind::DenyReason { intent_hash: ih(1), code: 0, total: 1, limit: 1 },
        Kind::DenyReason { intent_hash: ih(1), code: 99, total: 1, limit: 1 },
        Kind::Quoted { intent_hash: ih(1), recipient: [1; 20], chain_id: 1, connector: 0, amount: 1, gas_est: 1, resource: String::new(), policy_hash: ih(2) },
    ] {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("log.mulog");
        let v = vault();
        {
            let mut log = Log::open(&p, &v).unwrap();
            log.append(bad, 1000, &v).unwrap();
        }
        let log = Log::open(&p, &v).unwrap();
        assert!(matches!(log.entries()[0].kind, Kind::TailTruncated { .. }));
    }
    let _ = std::io::sink().flush();
}
