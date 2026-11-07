//! P12c (Workflow μ Gate §3): the owner's side of setting μ Gate up for the anchors of keysingate Pro —
//! `mu-daemon init-ksg` (once) and `mu-daemon serve-ksg` (the service).
//!
//! `init-ksg --home DIR --template FILE --socket PATH [--rpc URL] [--rpc2 URL]` checks the template
//! (every value; a wrong one, or a key unknown, is a refusal), makes the daemon's home (`0700`) with its
//! configuration and the vault's seeds (`0600`), and says which wallet is the **main** one — the address
//! the owner tops up. `serve-ksg --home DIR` then answers the readers of the capsules the template names.
//!
//! **This is the development vault.** The seeds lie in a file, as `softvault` keeps keys in memory: the
//! daemon's vault backends for a real device are not made yet (`mu-vault` platform backends are
//! skeletons), and the release build refuses `softvault` altogether. Until there is one the Pro gas
//! through μ Gate runs on `localnet` and `devnet` with test money.

use std::io::{Read, Write};
use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
use std::path::{Path, PathBuf};

use mu_connect::anchor_template;
use mu_connect::solana::{base58_public, Balances, SolanaCfg};
use mu_connect::solana_rpc::{http_post, SolanaRpc};
use mu_log::Log;
use mu_vault::backend::SoftVault;
use mu_vault::Vault;
use serde_json::{json, Value};

use crate::anchor_service::{bind_unix, serve_unix, AnchorService};

const CONFIG: &str = "ksg.json";
const SEEDS: &str = "vault.seeds";

fn flags(args: &[String]) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let name = a.strip_prefix("--").ok_or_else(|| format!("{a}: a flag is `--name value`"))?;
        out.push((name.to_owned(), it.next().ok_or_else(|| format!("--{name} needs a value"))?.clone()));
    }
    Ok(out)
}

fn get<'a>(f: &'a [(String, String)], name: &str) -> Option<&'a str> {
    f.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex32(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 {
        return None;
    }
    let mut a = [0u8; 32];
    for (i, b) in a.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(a)
}

fn urandom32() -> Result<[u8; 32], String> {
    let mut a = [0u8; 32];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut a))
        .map_err(|e| format!("/dev/urandom: {e}"))?;
    Ok(a)
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    f.write_all(bytes).and_then(|()| f.sync_all()).map_err(|e| format!("{}: {e}", path.display()))
}

fn vault_of(home: &Path) -> Result<SoftVault, String> {
    let path = home.join(SEEDS);
    let mode = std::fs::metadata(&path).map_err(|e| format!("{}: {e}", path.display()))?.permissions().mode();
    if mode & 0o077 != 0 {
        return Err(format!("{} is open to the group or to others (mode {:o})", path.display(), mode & 0o777));
    }
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let seeds: Vec<[u8; 32]> = text.lines().filter_map(unhex32).collect();
    let [mu, owner, wallet] = seeds[..] else {
        return Err(format!("{}: three seeds of 64 hex digits are expected", path.display()));
    };
    Ok(SoftVault::for_test(mu, owner, wallet))
}

/// `init-ksg`.
fn init(args: &[String]) -> Result<(), String> {
    let f = flags(args)?;
    let home = PathBuf::from(get(&f, "home").ok_or("--home DIR is required")?);
    let template = PathBuf::from(get(&f, "template").ok_or("--template FILE is required")?);
    let socket = PathBuf::from(get(&f, "socket").ok_or("--socket PATH is required")?);
    let rpc = get(&f, "rpc").unwrap_or("http://127.0.0.1:8899").to_owned();
    let rpc2 = get(&f, "rpc2").unwrap_or(&rpc).to_owned();
    // The template first: nothing is made if it does not hold.
    let t = anchor_template::load(&template)?;
    let template = std::fs::canonicalize(&template).map_err(|e| format!("{}: {e}", template.display()))?;
    if home.join(CONFIG).exists() {
        return Err(format!("{} exists: a daemon's home is never overwritten by init", home.join(CONFIG).display()));
    }
    std::fs::create_dir_all(&home).map_err(|e| format!("{}: {e}", home.display()))?;
    std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700)).map_err(|e| e.to_string())?;
    let seeds = format!("{}\n{}\n{}\n", hex(&urandom32()?), hex(&urandom32()?), hex(&urandom32()?));
    write_private(&home.join(SEEDS), seeds.as_bytes())?;
    let config = json!({"template": template, "socket": socket, "rpc": rpc, "rpc2": rpc2});
    write_private(&home.join(CONFIG), serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?.as_bytes())?;
    let vault = vault_of(&home)?;
    let main = vault.tx_signer_ed25519().map_err(|e| format!("the main wallet: {e:?}"))?;
    println!("μ Gate for keysingate Pro is set up in {}", home.display());
    println!("cluster              {}", t.cluster);
    println!("socket               {}", socket.display());
    println!("capsules allowed     {}", t.capsules.len());
    for w in &t.warnings {
        println!("warning              {w}");
    }
    println!("MAIN WALLET          {}", base58_public(&main.public_key()));
    println!("  top this address up with the SOL the anchors will cost; each capsule's sub-wallet is filled from it");
    println!("  (the development vault: the seeds are in {SEEDS}; do not use it with real money)");
    println!("start it:            mu-daemon serve-ksg --home {}", home.display());
    println!("point a capsule at it: ksg-capsule --home <capsule> anchor-config --mugate {}", socket.display());
    Ok(())
}

/// `serve-ksg`.
fn serve(args: &[String]) -> Result<(), String> {
    let f = flags(args)?;
    let home = PathBuf::from(get(&f, "home").ok_or("--home DIR is required")?);
    let config: Value = serde_json::from_slice(
        &std::fs::read(home.join(CONFIG)).map_err(|e| format!("{}: {e} (run init-ksg first)", home.join(CONFIG).display()))?,
    )
    .map_err(|e| format!("{CONFIG}: {e}"))?;
    let text = |k: &str| config.get(k).and_then(Value::as_str).map(str::to_owned).ok_or(format!("{CONFIG}: no {k}"));
    // The template is read again at every start: the owner's edit is the owner's word.
    let t = anchor_template::load(Path::new(&text("template")?))?;
    let vault = vault_of(&home)?;
    let log = Log::open(&home.join("anchor.log"), &vault).map_err(|e| format!("the log: {e:?}"))?;
    log.verify_chain().map_err(|e| format!("the log's chain does not hold: {e:?}"))?;
    let (rpc, rpc2) = (text("rpc")?, text("rpc2")?);
    let notify_path = home.join("notify.log");
    let notify = move |note: &str| {
        eprintln!("NOTICE FOR THE OWNER: {note}");
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).mode(0o600).open(&notify_path) {
            let _ = writeln!(f, "{note}");
        }
    };
    struct Both(SolanaRpc);
    impl Balances for Both {
        fn balance(&self, address: &str) -> Result<u64, mu_connect::solana::NodeErr> {
            self.0.balance(address)
        }
    }
    let mut service = AnchorService::new(
        t.rules.clone(),
        log,
        &vault,
        Box::new(SolanaRpc::new("a", &rpc, http_post)),
        Box::new(SolanaRpc::new("b", &rpc2, http_post)),
        Box::new(Both(SolanaRpc::new("a", &rpc, http_post))),
        Box::new(notify),
        SolanaCfg { cluster: t.cluster.clone() },
    );
    let socket = PathBuf::from(text("socket")?);
    let listener = bind_unix(&socket)?;
    for w in &t.warnings {
        eprintln!("warning: {w}");
    }
    eprintln!("μ Gate (keysingate Pro) answers on {} — {} capsule(s) allowed", socket.display(), t.capsules.len());
    serve_unix(
        &listener,
        &mut service,
        &|| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs()),
        0,
    );
    Ok(())
}

/// Runs `init-ksg` or `serve-ksg` if that is what was asked; the exit code, else `None`.
pub fn run(args: &[String]) -> Option<i32> {
    let result = match args.get(1).map(String::as_str) {
        Some("init-ksg") => init(&args[2..]),
        Some("serve-ksg") => serve(&args[2..]),
        _ => return None,
    };
    Some(match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("mu-daemon: {e}");
            1
        }
    })
}
