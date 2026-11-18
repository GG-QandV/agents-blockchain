//! mu-keygen end to end: the keys are random, private to the owner, never overwritten,
//! never the old public test seeds, and what it issues boots under the daemon's checks.
use mu_core::Mu;
use mu_log::Log;
use mu_vault::keyfile::read_seed;
use mu_vault::backend::SoftVault;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mu-keygen")).args(args).output().unwrap()
}
fn text(o: &Output) -> String {
    format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr))
}

#[test]
fn init_creates_private_random_keys_and_a_bootable_mu() {
    let d = tempfile::tempdir().unwrap();
    let home = d.path().join("home");
    let h = home.to_str().unwrap();
    let o = run(&["init", h, "--limit", "50", "--threshold", "10", "--allow", "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed=svc"]);
    assert!(o.status.success(), "{}", text(&o));
    let out = text(&o);
    assert!(out.contains("wallet address : 0x"));
    assert!(out.contains("daily limit 50"), "{out}");

    // modes: keys 600, dir 700
    assert_eq!(std::fs::metadata(&home).unwrap().permissions().mode() & 0o777, 0o700);
    for n in ["mu.key", "owner.key", "wallet.key", "agent.key"] {
        assert_eq!(std::fs::metadata(home.join(n)).unwrap().permissions().mode() & 0o777, 0o600, "{n}");
    }
    // never the public test seeds, all distinct
    let keys: Vec<[u8; 32]> = ["mu.key", "owner.key", "wallet.key"].iter().map(|n| *read_seed(&home.join(n)).unwrap()).collect();
    for k in &keys {
        for fixed in [1u8, 2, 3, 0x11, 42] { assert_ne!(*k, [fixed; 32]); }
    }
    assert!(keys[0] != keys[1] && keys[1] != keys[2] && keys[0] != keys[2]);
    // private keys are never printed
    for k in &keys { assert!(!out.contains(&mu_vault::keyfile::to_hex(k))); }

    // boot checks, as the daemon does them
    let v = SoftVault::from_seeds(keys[0], keys[1], keys[2]);
    let mu = Mu::load(&home.join("mu.bin")).unwrap();
    let mu_pub = {
        use p256::ecdsa::SigningKey;
        SigningKey::from_bytes((&keys[0]).into()).unwrap().verifying_key().to_encoded_point(true).as_bytes().to_vec()
    };
    mu.verify(&mu_pub).unwrap();
    let log = Log::open(&home.join("log.mulog"), &v).unwrap();
    log.verify_chain().unwrap();
    mu.verify_against_log(log.last_hash()).unwrap();
    assert_eq!(mu.delta().whitelist.len(), 1);

    assert!(run(&["check", h]).status.success());
    let shown = text(&run(&["show", h]));
    assert!(shown.contains("allow        : 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed (svc)"), "{shown}");
}

#[test]
fn two_inits_never_share_keys_and_nothing_is_overwritten() {
    let d = tempfile::tempdir().unwrap();
    let (a, b) = (d.path().join("a"), d.path().join("b"));
    assert!(run(&["init", a.to_str().unwrap()]).status.success());
    assert!(run(&["init", b.to_str().unwrap()]).status.success());
    assert_ne!(*read_seed(&a.join("wallet.key")).unwrap(), *read_seed(&b.join("wallet.key")).unwrap());

    let before = std::fs::read(a.join("wallet.key")).unwrap();
    let again = run(&["init", a.to_str().unwrap()]);
    assert!(!again.status.success());
    assert!(text(&again).contains("refusing to overwrite"));
    assert_eq!(std::fs::read(a.join("wallet.key")).unwrap(), before);
}

#[test]
fn bad_policy_leaves_no_keys_behind_and_check_catches_loose_permissions() {
    let d = tempfile::tempdir().unwrap();
    let home = d.path().join("h");
    let bad = run(&["init", home.to_str().unwrap(), "--allow", "0xnotanaddress"]);
    assert!(!bad.status.success());
    assert!(!home.join("wallet.key").exists());

    assert!(run(&["init", home.to_str().unwrap()]).status.success());
    std::fs::set_permissions(home.join("wallet.key"), std::fs::Permissions::from_mode(0o644)).unwrap();
    let c = run(&["check", home.to_str().unwrap()]);
    assert!(!c.status.success());
    assert!(text(&c).contains("FAIL  wallet.key"));
}
