//! mu-fixtures — dev artifact generator for smoke daemon (SoftVault ONLY).
//! Creates in $MU_HOME: mu.bin, log.mulog (empty valid chain), agent.key (ed25519 seed),
//! prints agent pubkey for allowlist and mu_pubkey for verify.
//! Usage: MU_HOME=/tmp/mu ./mu-fixtures [rollback-attack]
use mu_common::{Amount, CanonAddress, Hash32};
use mu_core::object::Omega;
use mu_core::Mu;
use mu_log::{Kind, Log};
use mu_policy::{delta_hash, Delta, WlEntry};
use mu_vault::backend::SoftVault;
use mu_vault::{DomainTag, Vault};
use std::path::PathBuf;

const MU_KEY: [u8; 32] = [1; 32];
const OWNER_KEY: [u8; 32] = [2; 32];
const WALLET_KEY: [u8; 32] = [3; 32];
const AGENT_SEED: [u8; 32] = [42; 32];

fn home() -> PathBuf {
    std::env::var("MU_HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/tmp/mu"))
}
fn hexs(b: &[u8]) -> String { b.iter().map(|x| format!("{x:02x}")).collect() }

fn owner_pubkey() -> [u8; 33] {
    use p256::ecdsa::SigningKey;
    let v = SigningKey::from_bytes((&OWNER_KEY).into()).unwrap()
        .verifying_key().to_encoded_point(true);
    let mut a = [0u8; 33]; a.copy_from_slice(v.as_bytes()); a
}
fn mu_pubkey() -> Vec<u8> {
    use p256::ecdsa::SigningKey;
    SigningKey::from_bytes((&MU_KEY).into()).unwrap()
        .verifying_key().to_encoded_point(true).as_bytes().to_vec()
}

fn sample_delta() -> Delta {
    Delta {
        daily_limit: Amount::from_minor(500_000_000),      // 500 USDC
        whitelist: vec![WlEntry {
            address: CanonAddress::canon("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed000000000000000000000000", 8453).unwrap(),
            label: "Test API Service".into(),
        }],
        confirm_threshold: Amount::from_minor(100_000_000), // 100 USDC
    }
}

fn main() {
    let h = home();
    std::fs::create_dir_all(&h).expect("mkdir MU_HOME");
    let v = SoftVault::for_test(MU_KEY, OWNER_KEY, WALLET_KEY);
    let attack = std::env::args().any(|a| a == "rollback-attack");

    // 1) log: empty valid chain (+ one Alert entry so last_hash ≠ genesis)
    let mut log = Log::open(&h.join("log.mulog"), &v).expect("log open");
    if log.entries().is_empty() {
        log.append(Kind::Alert { code: 0x0000 }, 1_000, &v).expect("genesis mark");
    }

    // 2) μ: issue with log_head = current chain tail
    let d = sample_delta();
    let dsig = v.owner_sign(DomainTag::MuDelta, &delta_hash(&d)).expect("owner sign").0;
    let mu_v1 = Mu::issue(
        [7; 16], owner_pubkey(), 1_000,
        Omega { connectors: vec![1, 2], max_ceiling: 1_000_000_000 }, // BankStub, CardStub
        d, dsig, b"wallet-ref-dev".to_vec(), log.last_hash(), &v,
    ).expect("issue");
    mu_v1.save(&h.join("mu.bin")).expect("save mu");

    // 3) agent key for gate allowlist
    let sk = ed25519_dalek::SigningKey::from_bytes(&AGENT_SEED);
    std::fs::write(h.join("agent.key"), AGENT_SEED).expect("agent key");
    let agent_pub = sk.verifying_key().to_bytes();

    if attack {
        // rollback scenario: Δ was tightened (log knows), old μ restored to disk
        let d2 = Delta { daily_limit: Amount::from_minor(50_000_000), ..sample_delta() };
        let s2 = v.owner_sign(DomainTag::MuDelta, &delta_hash(&d2)).unwrap().0;
        let head = log.append(Kind::DeltaChanged {
            old_hash: delta_hash(mu_v1.delta()), new_hash: delta_hash(&d2),
        }, 2_000, &v).unwrap();
        let mu_v2 = mu_v1.apply_delta(d2, s2, head, &v).unwrap();
        // save v2… and immediately overwrite with old v1 = attack
        mu_v2.save(&h.join("mu.bin")).unwrap();
        mu_v1.save(&h.join("mu.bin")).unwrap();
        println!("ROLLBACK-ATTACK fixture: daemon MUST refuse to start (LogHeadMismatch)");
    }

    println!("MU_HOME:        {}", h.display());
    println!("mu.bin:         id={} v{} limit={}", hexs(mu_v1.id()), mu_v1.version(),
             mu_common::amount::display_minor(mu_v1.delta().daily_limit, 6));
    println!("log.mulog:      {} entries, last_hash={}…", log.entries().len(), &hexs(&log.last_hash().0)[..16]);
    println!("mu_pubkey:      {}", hexs(&mu_pubkey()));
    println!("agent_id:       agent-1");
    println!("agent_pubkey:   {}   ← in daemon allowlist", hexs(&agent_pub));
    println!("agent.key:      seed for signing intents (test utility/script)");
}
