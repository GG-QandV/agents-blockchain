//! P12c / T9 against a real node: the real `mu-daemon` (`init-ksg`, `serve-ksg`) on a real
//! `solana-test-validator`. Ignored by default — it needs a validator (`MU_SOLANA_LOCALNET`).
//!
//! Run: `solana-test-validator --reset --ledger DIR --quiet &` then
//! `MU_SOLANA_LOCALNET=http://127.0.0.1:8899 cargo test -p mu-daemon --features ksg,softvault --test ksg_e2e -- --ignored`
#![cfg(all(feature = "ksg", feature = "softvault"))]

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};

use mu_connect::solana_rpc::{http_post, SolanaRpc};
use serde_json::{json, Value};

const BIN: &str = env!("CARGO_BIN_EXE_mu-daemon");
const SEAL: &str = "ksg:bs:v1 00112233445566778899aabbccddeeff ffeeddccbbaa99887766554433221100";

fn ask(sock: &std::path::Path, line: &Value) -> Value {
    let mut s = UnixStream::connect(sock).expect("the daemon answers");
    writeln!(s, "{line}").expect("writes");
    let mut answer = String::new();
    BufReader::new(s).read_line(&mut answer).expect("an answer");
    serde_json::from_str(answer.trim()).expect("JSON")
}

#[test]
#[ignore = "needs solana-test-validator (MU_SOLANA_LOCALNET)"]
fn the_daemon_set_up_by_init_ksg_pays_from_the_one_wallet_and_refuses_on_a_real_node() {
    let Ok(url) = std::env::var("MU_SOLANA_LOCALNET") else { return };
    let dir = std::env::temp_dir().join(format!("ksg-mu-e2e-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a directory");
    let capsule = format!("1220{}", "ab".repeat(32));
    let template = dir.join("ksg-pro-anchor.toml");
    std::fs::write(
        &template,
        format!(
            r#"cluster = "localnet"
fee_cap_lamports = 20_000
daily_budget_lamports_per_capsule = 5_000_000
daily_budget_lamports_total = 50_000_000
memo_kinds = ["ksg:bs:v1", "ksg:st:v1"]

[[capsules]]
id = "{capsule}"
label = "the test capsule"
"#
        ),
    )
    .expect("the template");

    // The owner sets it up once.
    let home = dir.join("mu");
    let sock = dir.join("mu-anchor.sock");
    std::fs::set_permissions(&dir, std::os::unix::fs::PermissionsExt::from_mode(0o700)).expect("chmod");
    let init = Command::new(BIN)
        .args(["init-ksg", "--home"]).arg(&home)
        .args(["--template"]).arg(&template)
        .args(["--socket"]).arg(&sock)
        .args(["--rpc", &url])
        .output()
        .expect("init-ksg runs");
    let out = String::from_utf8_lossy(&init.stdout).into_owned();
    assert!(init.status.success(), "{out}\n{}", String::from_utf8_lossy(&init.stderr));
    let main = out.lines().find_map(|l| l.strip_prefix("MAIN WALLET")).expect("the main wallet is named").trim().to_owned();
    // a second init never overwrites
    assert!(!Command::new(BIN).args(["init-ksg", "--home"]).arg(&home).args(["--template"]).arg(&template)
        .args(["--socket"]).arg(&sock).status().expect("runs").success());

    // The owner tops the main wallet up (here: an airdrop).
    let node = SolanaRpc::new("n", &url, http_post);
    let before = node.balance(&main).unwrap_or(0);
    http_post(&url, &format!(r#"{{"jsonrpc":"2.0","id":1,"method":"requestAirdrop","params":["{main}",2000000000]}}"#), 10_000)
        .expect("the airdrop is asked for");
    for _ in 0..60 {
        if node.balance(&main).unwrap_or(0) > before {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    let mut daemon = Command::new(BIN).args(["serve-ksg", "--home"]).arg(&home).stdout(Stdio::null()).stderr(Stdio::null()).spawn().expect("serves");
    for _ in 0..100 {
        if sock.exists() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    assert!(sock.exists(), "the daemon listens");
    let mode = std::os::unix::fs::PermissionsExt::mode(&std::fs::metadata(&sock).expect("socket").permissions()) & 0o777;
    assert_eq!(mode, 0o600, "the socket is the owner's alone");

    // One wallet pays for every capsule: the address the daemon gives is the main wallet.
    let paying = ask(&sock, &json!({"address": {}}))["address"].as_str().expect("an address").to_owned();
    assert_eq!(paying, main, "the wallet that pays is the one the owner tops up");
    let funded = node.balance(&main).expect("the main wallet is funded");
    let a = ask(&sock, &json!({"anchor": {"memo": SEAL, "cluster": "localnet", "capsule": capsule, "fee_cap_lamports": 20_000}}));
    let sig = a["sent"]["signature"].as_str().unwrap_or_else(|| panic!("{a}")).to_owned();
    let mut landed = false;
    for _ in 0..60 {
        if ask(&sock, &json!({"status": {"signature": sig}})).get("landed").is_some() {
            landed = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    assert!(landed, "the memo landed");
    assert_eq!(funded - node.balance(&main).expect("balance"), 5_000, "the main wallet paid the fee of one anchor, nothing moved to a sub-wallet");

    // A capsule the owner did not name pays nothing; a memo that is not ours is refused.
    let stranger = ask(&sock, &json!({"anchor": {"memo": SEAL, "cluster": "localnet", "capsule": format!("1220{}", "cd".repeat(32))}}));
    assert_eq!(stranger["denied"]["reason"], "not_allowed", "{stranger}");
    let bad = ask(&sock, &json!({"anchor": {"memo": "ksg:cp:v1 old", "cluster": "localnet", "capsule": capsule}}));
    assert!(bad.get("denied").is_some(), "the old memo is refused: {bad}");

    // Every decision is in the log, on disk.
    assert!(std::fs::metadata(home.join("anchor.log")).expect("the log").len() > 0);
    let _ = daemon.kill();
    let _ = daemon.wait();
    std::fs::remove_dir_all(&dir).ok();
}
