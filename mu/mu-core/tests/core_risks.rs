//! Tests for RISK-M1-1/2/3/4 + end-to-end integration with M5 (rollback attack).
use mu_common::{Amount, CanonAddress, Hash32};
use mu_core::{CoreErr, Mu};
use mu_core::object::Omega;
use mu_log::{Kind, Log};
use mu_policy::{delta_hash, Delta, WlEntry};
use mu_vault::backend::SoftVault;
use mu_vault::domain::tagged_digest;
use mu_vault::{DomainTag, Vault};

fn vault() -> SoftVault { SoftVault::for_test([1; 32], [2; 32], [3; 32]) }

fn mu_pubkey() -> Vec<u8> {
    use p256::ecdsa::SigningKey;
    SigningKey::from_bytes((&[1u8; 32]).into()).unwrap()
        .verifying_key().to_encoded_point(true).as_bytes().to_vec()
}
fn owner_pubkey() -> [u8; 33] {
    use p256::ecdsa::SigningKey;
    let v = SigningKey::from_bytes((&[2u8; 32]).into()).unwrap()
        .verifying_key().to_encoded_point(true).as_bytes().to_vec();
    let mut a = [0u8; 33]; a.copy_from_slice(&v); a
}
fn delta(limit: u128) -> Delta {
    Delta {
        daily_limit: Amount::from_minor(limit),
        whitelist: vec![WlEntry {
            address: CanonAddress::canon("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 8453).unwrap(),
            label: "API".into(),
        }],
        confirm_threshold: Amount::from_minor(limit / 5),
    }
}
fn sign_delta(v: &SoftVault, d: &Delta) -> [u8; 64] {
    v.owner_sign(DomainTag::MuDelta, &delta_hash(d)).unwrap().0
}
fn issue(v: &SoftVault, limit: u128, log_head: Hash32) -> Mu {
    let d = delta(limit);
    let sig = sign_delta(v, &d);
    Mu::issue([7; 16], owner_pubkey(), 1000,
              Omega { connectors: vec![0], max_ceiling: 1_000_000 },
              d, sig, b"token-ref-1".to_vec(), log_head, v).unwrap()
}

#[test]
fn roundtrip_save_load_verify() {
    let v = vault();
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("mu.bin");
    let mu = issue(&v, 500, Hash32([0; 32]));
    mu.save(&p).unwrap();
    let loaded = Mu::load(&p).unwrap();
    loaded.verify(&mu_pubkey()).unwrap();
    assert_eq!(loaded.delta().daily_limit, Amount::from_minor(500));
    assert_eq!(loaded.version(), 1);
}

#[test]
fn bit_flip_anywhere_breaks_verify() {
    // RISK-M1-4 + signatures: corrupting ANY byte → verify Err (or decode Err)
    let v = vault();
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("mu.bin");
    issue(&v, 500, Hash32([0; 32])).save(&p).unwrap();
    let orig = std::fs::read(&p).unwrap();
    let step = (orig.len() / 24).max(1); // sample positions across the whole file
    for pos in (0..orig.len()).step_by(step) {
        let mut bad = orig.clone();
        bad[pos] ^= 0xFF;
        let ok = match mu_core::format::decode_mu(&bad) {
            Err(_) => true,                                  // decode caught
            Ok(m) => m.verify(&mu_pubkey()).is_err(),        // or signature caught
        };
        assert!(ok, "bit-flip at {pos} not detected");
    }
}

#[test]
fn omega_immutable_via_apply_delta() {
    // RISK-M1-1: apply_delta cannot touch Ω (field is private — only Δ changes);
    // and Δ signature by foreign key is rejected BEFORE rebuild.
    let v = vault();
    let mu = issue(&v, 500, Hash32([0; 32]));
    let d2 = delta(700);
    // garbage owner signature → SigOwner
    let bad = mu.apply_delta(d2.clone(), [0u8; 64], Hash32([1; 32]), &v);
    assert!(matches!(bad, Err(CoreErr::SigOwner)));
    // valid signature → applied, Ω unchanged
    let sig = sign_delta(&v, &d2);
    let next = mu.apply_delta(d2, sig, Hash32([1; 32]), &v).unwrap();
    assert_eq!(next.omega(), mu.omega());
    assert_eq!(next.delta().daily_limit, Amount::from_minor(700));
    next.verify(&mu_pubkey()).unwrap();
}

#[test]
fn reissue_changes_id_and_version() {
    let v = vault();
    let mu = issue(&v, 500, Hash32([0; 32]));
    let re = mu.reissue([9; 16], Omega { connectors: vec![0, 1], max_ceiling: 2_000_000 }, &v).unwrap();
    assert_ne!(re.id(), mu.id());
    assert_eq!(re.version(), 2);
    assert_eq!(re.omega().max_ceiling, 2_000_000);
    re.verify(&mu_pubkey()).unwrap();
}

#[test]
fn rollback_attack_caught_by_log_head() {
    // RISK-M1-2, end-to-end with M5: replacing μ with old copy (high limit) is caught.
    let v = vault();
    let dir = tempfile::tempdir().unwrap();
    let mu_path = dir.path().join("mu.bin");
    let log_path = dir.path().join("log.mulog");

    let mut log = Log::open(&log_path, &v).unwrap();
    // v1: limit 900 (attacker prefers this version)
    let mu_v1 = issue(&v, 900, log.last_hash());
    mu_v1.save(&mu_path).unwrap();

    // owner tightens: Δ 100; DeltaChanged is written to log, μ.log_head updated
    let d2 = delta(100);
    let sig2 = sign_delta(&v, &d2);
    let old_h = delta_hash(mu_v1.delta());
    let new_h = delta_hash(&d2);
    let head = log.append(Kind::DeltaChanged { old_hash: old_h, new_hash: new_h }, 2000, &v).unwrap();
    let mu_v2 = mu_v1.apply_delta(d2, sig2, head, &v).unwrap();
    mu_v2.save(&mu_path).unwrap();

    // ATTACK: write old μ back to disk (validly signed!)
    mu_v1.save(&mu_path).unwrap();

    // daemon boot: signatures match…
    let loaded = Mu::load(&mu_path).unwrap();
    loaded.verify(&mu_pubkey()).unwrap();
    // …but log binding catches the rollback
    assert!(matches!(
        loaded.verify_against_log(log.last_hash()),
        Err(CoreErr::LogHeadMismatch)
    ));
    // legitimate v2 passes
    mu_v2.verify_against_log(log.last_hash()).unwrap();
}

#[test]
fn crash_during_save_leaves_valid_file() {
    // RISK-M1-3: crash simulation — garbage tmp nearby does not prevent load from reading mu.bin
    let v = vault();
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("mu.bin");
    issue(&v, 500, Hash32([0; 32])).save(&p).unwrap();
    std::fs::write(p.with_extension("tmp"), b"partial garbage from crash").unwrap();
    let loaded = Mu::load(&p).unwrap();
    loaded.verify(&mu_pubkey()).unwrap(); // old valid version is in place
}

#[test]
fn trailing_byte_rejected() {
    // RISK-M1-4: strict schema
    let v = vault();
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("mu.bin");
    issue(&v, 500, Hash32([0; 32])).save(&p).unwrap();
    let mut bytes = std::fs::read(&p).unwrap();
    bytes.insert(bytes.len() - 64, 0x00); // byte inside payload
    assert!(mu_core::format::decode_mu(&bytes).is_err());
}
