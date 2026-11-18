//! mu-keygen — creates what a μ Gate daemon needs in one directory ($MU_HOME):
//!
//!   mu-keygen init  <dir> [--limit USDC] [--threshold USDC] [--ceiling USDC]
//!                         [--chain ID] [--allow 0xADDR[=label]]...
//!   mu-keygen show  <dir>     public values only (wallet address, public keys)
//!   mu-keygen check <dir>     key files readable only by you, μ object verifies, chain intact
//!
//! Keys come from the OS random generator and are written with mode 600 from the first byte.
//! Nothing existing is ever overwritten. Private keys are never printed.
use mu_common::{amount::display_minor, parse_decimal, Amount, CanonAddress};
use mu_connect::x402::address_of;
use mu_core::object::Omega;
use mu_core::Mu;
use mu_log::{Kind, Log};
use mu_policy::{canon_address_checked, delta_hash, to_checksum, Delta, WlEntry};
use mu_vault::backend::SoftVault;
use mu_vault::keyfile::{read_seed, to_hex, write_new_public, write_new_seed};
use mu_vault::{DomainTag, Vault};
use rand_core::{OsRng, RngCore};
use std::path::{Path, PathBuf};

const USDC_DECIMALS: u8 = 6;

pub const MU_KEY: &str = "mu.key";
pub const OWNER_KEY: &str = "owner.key";
pub const WALLET_KEY: &str = "wallet.key";
pub const AGENT_KEY: &str = "agent.key";
pub const AGENT_PUB: &str = "agent.pub";
pub const MU_BIN: &str = "mu.bin";
pub const LOG: &str = "log.mulog";

fn die(msg: &str) -> ! {
    eprintln!("mu-keygen: {msg}");
    std::process::exit(1)
}

struct Opts {
    limit: Amount,
    threshold: Amount,
    ceiling: u128,
    chain: u64,
    allow: Vec<(String, String)>,
}

fn usdc(s: &str, what: &str) -> Amount {
    parse_decimal(s, USDC_DECIMALS).unwrap_or_else(|_| die(&format!("{what}: expected a decimal USDC amount, e.g. 25 or 0.5")))
}

fn parse_opts(args: &[String]) -> Opts {
    let mut o = Opts {
        limit: Amount::ZERO,
        threshold: Amount::ZERO,
        ceiling: 1_000_000_000, // 1000 USDC per payment
        chain: 8453,
        allow: vec![],
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut val = || it.next().unwrap_or_else(|| die(&format!("{a} needs a value"))).clone();
        match a.as_str() {
            "--limit" => o.limit = usdc(&val(), "--limit"),
            "--threshold" => o.threshold = usdc(&val(), "--threshold"),
            "--ceiling" => o.ceiling = usdc(&val(), "--ceiling").minor(),
            "--chain" => o.chain = val().parse().unwrap_or_else(|_| die("--chain: expected a number (8453 Base, 84532 Base Sepolia)")),
            "--allow" => {
                let v = val();
                let (addr, label) = v.split_once('=').unwrap_or((&v, "allowed"));
                o.allow.push((addr.to_string(), label.to_string()));
            }
            other => die(&format!("unknown option {other}")),
        }
    }
    o
}

fn random_seed(valid: impl Fn(&[u8; 32]) -> bool) -> [u8; 32] {
    loop {
        let mut s = [0u8; 32];
        OsRng.fill_bytes(&mut s);
        if valid(&s) {
            return s;
        }
    }
}

fn p256_ok(s: &[u8; 32]) -> bool { p256::ecdsa::SigningKey::from_bytes(s.into()).is_ok() }
fn k256_ok(s: &[u8; 32]) -> bool { k256::ecdsa::SigningKey::from_bytes(s.into()).is_ok() }

fn p256_pub(seed: &[u8; 32]) -> [u8; 33] {
    let v = p256::ecdsa::SigningKey::from_bytes(seed.into()).expect("valid p256 key").verifying_key().to_encoded_point(true);
    let mut a = [0u8; 33];
    a.copy_from_slice(v.as_bytes());
    a
}

fn key(dir: &Path, name: &str) -> zeroize::Zeroizing<[u8; 32]> {
    read_seed(&dir.join(name)).unwrap_or_else(|e| die(&format!("{}: {e}", dir.join(name).display())))
}

fn vault_of(dir: &Path) -> SoftVault {
    SoftVault::from_seeds(*key(dir, MU_KEY), *key(dir, OWNER_KEY), *key(dir, WALLET_KEY))
}

fn wallet_address(dir: &Path) -> [u8; 20] {
    let v = vault_of(dir);
    address_of(&v.tx_signer().unwrap_or_else(|_| die("wallet key unusable"))).unwrap_or_else(|_| die("wallet key unusable"))
}

fn init(dir: &Path, o: &Opts) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        std::fs::DirBuilder::new().recursive(true).mode(0o700).create(dir).unwrap_or_else(|e| die(&format!("{}: {e}", dir.display())));
    }
    #[cfg(not(unix))]
    std::fs::create_dir_all(dir).unwrap_or_else(|e| die(&format!("{}: {e}", dir.display())));
    for n in [MU_KEY, OWNER_KEY, WALLET_KEY, AGENT_KEY, AGENT_PUB, MU_BIN, LOG] {
        if dir.join(n).exists() {
            die(&format!("{} already exists — refusing to overwrite. Use an empty directory.", dir.join(n).display()));
        }
    }
    // policy first: a typo must not leave a half-made directory of keys
    let mut whitelist = Vec::new();
    for (addr, label) in &o.allow {
        let (a, _) = canon_address_checked(addr, o.chain).unwrap_or_else(|e| die(&format!("--allow {addr}: {e:?}")));
        whitelist.push(WlEntry { address: a, label: label.clone() });
    }
    if o.threshold > o.limit && o.limit != Amount::ZERO {
        eprintln!("warning: --threshold above --limit: the owner is never asked, the daily limit stops first");
    }

    let mu = random_seed(p256_ok);
    let owner = random_seed(p256_ok);
    let wallet = random_seed(k256_ok);
    let agent = random_seed(|_| true);
    write_new_seed(&dir.join(MU_KEY), &mu).unwrap_or_else(|e| die(&format!("{MU_KEY}: {e}")));
    write_new_seed(&dir.join(OWNER_KEY), &owner).unwrap_or_else(|e| die(&format!("{OWNER_KEY}: {e}")));
    write_new_seed(&dir.join(WALLET_KEY), &wallet).unwrap_or_else(|e| die(&format!("{WALLET_KEY}: {e}")));
    write_new_seed(&dir.join(AGENT_KEY), &agent).unwrap_or_else(|e| die(&format!("{AGENT_KEY}: {e}")));
    let agent_pub = ed25519_dalek::SigningKey::from_bytes(&agent).verifying_key().to_bytes();
    write_new_public(&dir.join(AGENT_PUB), &format!("{}\n", to_hex(&agent_pub))).unwrap_or_else(|e| die(&format!("{AGENT_PUB}: {e}")));

    let v = vault_of(dir);
    let mut log = Log::open(&dir.join(LOG), &v).unwrap_or_else(|e| die(&format!("log: {e:?}")));
    log.append(Kind::Alert { code: 0 }, 1_000, &v).unwrap_or_else(|e| die(&format!("log: {e:?}")));
    let d = Delta { daily_limit: o.limit, whitelist, confirm_threshold: o.threshold, resource_allowlist: vec![] };
    let dsig = v.owner_sign(DomainTag::MuDelta, &delta_hash(&d)).unwrap_or_else(|_| die("owner signature failed")).0;
    let id: [u8; 16] = random_seed(|_| true)[..16].try_into().unwrap();
    let m = Mu::issue(
        id, p256_pub(&owner), 1_000,
        Omega { connectors: vec![1, 2], max_ceiling: o.ceiling },
        d, dsig, b"wallet-ref".to_vec(), log.last_hash(), &v,
    )
    .unwrap_or_else(|e| die(&format!("issue μ: {e:?}")));
    m.save(&dir.join(MU_BIN)).unwrap_or_else(|e| die(&format!("save μ: {e:?}")));

    println!("Created {}", dir.display());
    show_public(dir);
    println!();
    println!("Next: back up {MU_KEY}, {OWNER_KEY}, {WALLET_KEY} (see docs/KEYS.md), then fund the wallet address");
    println!("above with a small amount first. Run the daemon with MU_HOME={}", dir.display());
}

fn show_public(dir: &Path) {
    let mu = p256_pub(&key(dir, MU_KEY));
    let owner = p256_pub(&key(dir, OWNER_KEY));
    println!("wallet address : {}   (fund this, EVM; USDC on Base)", to_checksum(&wallet_address(dir)));
    println!("mu pubkey      : {}", to_hex(&mu));
    println!("owner pubkey   : {}", to_hex(&owner));
    match std::fs::read_to_string(dir.join(AGENT_PUB)) {
        Ok(t) => println!("agent pubkey   : {}   (agent-1; the agent signs with {AGENT_KEY})", t.trim()),
        Err(_) => println!("agent pubkey   : (missing {AGENT_PUB})"),
    }
    if let Ok(m) = Mu::load(&dir.join(MU_BIN)) {
        let d = m.delta();
        println!(
            "policy         : daily limit {} USDC, owner confirms above {} USDC, ceiling {} USDC, {} allowed recipient(s)",
            display_minor(d.daily_limit, USDC_DECIMALS), display_minor(d.confirm_threshold, USDC_DECIMALS),
            display_minor(Amount::from_minor(m.omega().max_ceiling), USDC_DECIMALS), d.whitelist.len()
        );
        for w in &d.whitelist {
            let a: &CanonAddress = &w.address;
            println!("  allow        : {} ({})", to_checksum(a.bytes()), w.label);
        }
    }
}

fn check(dir: &Path) -> bool {
    let ok = std::cell::Cell::new(true);
    let report = |what: &str, r: Result<(), String>| match r {
        Ok(()) => println!("ok    {what}"),
        Err(e) => { ok.set(false); println!("FAIL  {what}: {e}") }
    };
    for n in [MU_KEY, OWNER_KEY, WALLET_KEY, AGENT_KEY] {
        report(&format!("{n} (mode 600, 64 hex)"), read_seed(&dir.join(n)).map(|_| ()).map_err(|e| e.to_string()));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(dir).map(|m| m.permissions().mode() & 0o077).unwrap_or(0o777);
        report("directory not accessible to others", if mode == 0 { Ok(()) } else { Err("chmod 700 it".into()) });
    }
    if ok.get() {
        let v = vault_of(dir);
        report("μ object verifies under the mu key", Mu::load(&dir.join(MU_BIN)).map_err(|e| format!("{e:?}")).and_then(|m| {
            m.verify(&p256_pub(&key(dir, MU_KEY))).map_err(|e| format!("{e:?}"))?;
            let log = Log::open(&dir.join(LOG), &v).map_err(|e| format!("{e:?}"))?;
            log.verify_chain().map_err(|e| format!("log chain: {e:?}"))?;
            m.verify_against_log(log.last_hash()).map_err(|e| format!("log head: {e:?}"))
        }));
    }
    ok.get()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: mu-keygen init <dir> [--limit USDC] [--threshold USDC] [--ceiling USDC] [--chain ID] [--allow 0xADDR[=label]]...\n       mu-keygen show <dir>\n       mu-keygen check <dir>";
    let (Some(cmd), Some(dir)) = (args.first(), args.get(1)) else { die(usage) };
    let dir = PathBuf::from(dir);
    match cmd.as_str() {
        "init" => init(&dir, &parse_opts(&args[2..])),
        "show" => show_public(&dir),
        "check" => {
            if !check(&dir) { std::process::exit(1) }
        }
        _ => die(usage),
    }
}
