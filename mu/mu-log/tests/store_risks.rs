//! Тесты RISK-M5-1/2/4/5 на реальном файле.
use mu_common::{Amount, Hash32};
use mu_log::{Kind, Log};
use mu_vault::backend::SoftVault;
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};

fn vault() -> SoftVault { SoftVault::for_test([1; 32], [2; 32], [3; 32]) }
fn ih(x: u8) -> Hash32 { Hash32([x; 32]) }

#[test]
fn append_and_reopen_chain_intact() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    {
        let mut log = Log::open(&p, &v).unwrap();
        log.append(Kind::Pending { intent_hash: ih(1), amount_total: 100, chain_nonce: 0 }, 1000, &v).unwrap();
        log.append(Kind::Settled { intent_hash: ih(1), tx_hash: [9; 32], effective_gas: 5 }, 1001, &v).unwrap();
        log.verify_chain().unwrap();
    }
    // переоткрытие: цепь читается и верифицируется (RISK-M5-2)
    let log = Log::open(&p, &v).unwrap();
    log.verify_chain().unwrap();
    assert_eq!(log.entries().len(), 2);
}

#[test]
fn tamper_amount_breaks_chain() {
    // RISK-M5-2: правка байта в середине файла ломает verify
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    {
        let mut log = Log::open(&p, &v).unwrap();
        log.append(Kind::Pending { intent_hash: ih(1), amount_total: 100, chain_nonce: 0 }, 1000, &v).unwrap();
        log.append(Kind::Pending { intent_hash: ih(2), amount_total: 200, chain_nonce: 1 }, 1001, &v).unwrap();
    }
    // портим байт amount в ПЕРВОЙ записи (смещение: 4 len + 8 seq + 32 prev + 8 ts + 4 klen + 1 tag + 32 ih → внутри u128)
    let mut f = fs::OpenOptions::new().read(true).write(true).open(&p).unwrap();
    f.seek(SeekFrom::Start(4 + 8 + 32 + 8 + 4 + 1 + 32 + 10)).unwrap();
    f.write_all(&[0xFF]).unwrap();
    f.sync_all().unwrap();

    let log = Log::open(&p, &v).unwrap();
    // либо цепь сломана (hash первой записи изменился → prev второй не сходится)
    assert!(log.verify_chain().is_err());
}

#[test]
fn truncated_tail_recovers_with_marker() {
    // RISK-M5-5
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    {
        let mut log = Log::open(&p, &v).unwrap();
        log.append(Kind::Pending { intent_hash: ih(1), amount_total: 100, chain_nonce: 0 }, 1000, &v).unwrap();
        log.append(Kind::Failed { intent_hash: ih(1) }, 1001, &v).unwrap();
    }
    // обрезаем последние 10 байт (эмуляция недописанной записи при крэше)
    let len = fs::metadata(&p).unwrap().len();
    let f = fs::OpenOptions::new().write(true).open(&p).unwrap();
    f.set_len(len - 10).unwrap();
    f.sync_all().unwrap();

    let log = Log::open(&p, &v).unwrap();
    log.verify_chain().unwrap();
    // осталась 1 валидная запись + маркер TailTruncated
    let kinds: Vec<_> = log.entries().iter().map(|e| &e.kind).collect();
    assert!(matches!(kinds.last().unwrap(), Kind::TailTruncated { .. }));
}

#[test]
fn window_excludes_simulated_and_failed() {
    // RISK-M5-4
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    let mut log = Log::open(&p, &v).unwrap();
    // открытый Pending 100 → в окне
    log.append(Kind::Pending { intent_hash: ih(1), amount_total: 100, chain_nonce: 0 }, 1000, &v).unwrap();
    // Pending 200 → закрыт Failed → вне окна
    log.append(Kind::Pending { intent_hash: ih(2), amount_total: 200, chain_nonce: 1 }, 1000, &v).unwrap();
    log.append(Kind::Failed { intent_hash: ih(2) }, 1001, &v).unwrap();
    // Simulated → вне окна
    log.append(Kind::Simulated { intent_hash: ih(3), connector: "bank_stub" }, 1002, &v).unwrap();
    // Pending 50 → закрыт Settled → 50 в окне (как Settled)
    log.append(Kind::Pending { intent_hash: ih(4), amount_total: 50, chain_nonce: 2 }, 1003, &v).unwrap();
    log.append(Kind::Settled { intent_hash: ih(4), tx_hash: [7; 32], effective_gas: 1 }, 1004, &v).unwrap();

    let sum = log.window_sum(2000);
    assert_eq!(sum, Amount::from_minor(150)); // 100 (open pending) + 50 (settled)
}

#[test]
fn window_respects_24h_boundary() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    let mut log = Log::open(&p, &v).unwrap();
    let now = 200_000u64;
    // старше 24ч → вне окна
    log.append(Kind::Pending { intent_hash: ih(1), amount_total: 999, chain_nonce: 0 }, now - 86_401, &v).unwrap();
    // внутри окна
    log.append(Kind::Pending { intent_hash: ih(2), amount_total: 10, chain_nonce: 1 }, now - 86_399, &v).unwrap();
    assert_eq!(log.window_sum(now), Amount::from_minor(10));
}

#[test]
fn pending_pairs_resolved() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("log.mulog");
    let v = vault();
    let mut log = Log::open(&p, &v).unwrap();
    log.append(Kind::Pending { intent_hash: ih(1), amount_total: 1, chain_nonce: 0 }, 1, &v).unwrap();
    log.append(Kind::Pending { intent_hash: ih(2), amount_total: 2, chain_nonce: 1 }, 2, &v).unwrap();
    log.append(Kind::Settled { intent_hash: ih(1), tx_hash: [0; 32], effective_gas: 0 }, 3, &v).unwrap();
    let open = log.pending();
    assert_eq!(open.len(), 1);
    assert!(matches!(open[0].kind, Kind::Pending { intent_hash, .. } if intent_hash == ih(2)));
}
