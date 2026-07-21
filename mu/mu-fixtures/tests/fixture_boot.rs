//! Fixtures are actually bootable: normal passes, rollback attack is caught.
use mu_core::{CoreErr, Mu};
use mu_log::Log;
use mu_vault::backend::SoftVault;
use std::process::Command;

fn run_fixtures(home: &std::path::Path, arg: Option<&str>) {
    let mut c = Command::new(env!("CARGO_BIN_EXE_mu-fixtures"));
    c.env("MU_HOME", home);
    if let Some(a) = arg { c.arg(a); }
    assert!(c.status().unwrap().success());
}
fn mu_pubkey() -> Vec<u8> {
    use p256::ecdsa::SigningKey;
    SigningKey::from_bytes((&[1u8; 32]).into()).unwrap()
        .verifying_key().to_encoded_point(true).as_bytes().to_vec()
}

#[test]
fn normal_fixture_boots() {
    let dir = tempfile::tempdir().unwrap();
    run_fixtures(dir.path(), None);
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let mu = Mu::load(&dir.path().join("mu.bin")).unwrap();
    mu.verify(&mu_pubkey()).unwrap();
    let log = Log::open(&dir.path().join("log.mulog"), &v).unwrap();
    log.verify_chain().unwrap();
    mu.verify_against_log(log.last_hash()).unwrap();
}

#[test]
fn rollback_fixture_fails_boot() {
    let dir = tempfile::tempdir().unwrap();
    run_fixtures(dir.path(), Some("rollback-attack"));
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let mu = Mu::load(&dir.path().join("mu.bin")).unwrap();
    mu.verify(&mu_pubkey()).unwrap();
    let log = Log::open(&dir.path().join("log.mulog"), &v).unwrap();
    assert!(matches!(mu.verify_against_log(log.last_hash()), Err(CoreErr::LogHeadMismatch)));
}
