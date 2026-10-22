//! P0-1 acceptance: keys and policy come from files, not from the source.
//!
//! Exit codes: 6 = config refused (missing/bad files), 2 = μ object error.
//! With a complete key/policy set but no `mu.bin`, the daemon must get past
//! config and fail later with 2 — that proves the config was read from files.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_mu-daemon");

fn run(home: &Path) -> (i32, String) {
    let out = Command::new(BIN)
        .env("MU_HOME", home)
        .env("MU_CHAIN_ID", "8453")
        .env_remove("MU_CONNECTOR")
        .env_remove("MU_X402_NETWORK")
        .current_dir(home)
        .output()
        .expect("spawn daemon");
    let code = out.status.code().unwrap_or(-1);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (code, text)
}

fn write_key(dir: &Path, name: &str, mode: u32) {
    let keys = dir.join("keys");
    fs::create_dir_all(&keys).unwrap();
    let path = keys.join(name);
    // keys/mu.pub is a compressed P-256 point (33 bytes); the others are seeds.
    let body = if name == "mu.pub" {
        format!("02{}\n", "11".repeat(32))
    } else {
        format!("{}\n", "11".repeat(32))
    };
    fs::write(&path, body).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
}

fn full_config(dir: &Path, mode: u32) {
    write_key(dir, "mu.pub", 0o600);
    write_key(dir, "mu.hex", mode);
    write_key(dir, "owner.hex", mode);
    write_key(dir, "wallet.hex", mode);
    fs::write(
        dir.join("agents.toml"),
        "[[agent]]\nid = \"agent-1\"\npubkey = \"197f6b23e16c8532c6abc838facd5ea789be0c76b2920334039bfa8b3d368d61\"\n",
    )
    .unwrap();
    fs::write(
        dir.join("policy.toml"),
        "daily_limit_minor = 5000000000\nconfirm_threshold_minor = 1000000000\n\n[[whitelist]]\naddress = \"0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed\"\nlabel = \"test\"\n\n[[resource_allowlist]]\nhost = \"api.example.com\"\npath_prefix = \"/v1\"\nmax_price_per_call_minor = 1000000\n",
    )
    .unwrap();
}

#[test]
fn a_daemon_without_key_files_does_not_start() {
    let dir = tempfile::tempdir().unwrap();
    let (code, text) = run(dir.path());
    assert_eq!(code, 6, "expected config refusal, got {code}: {text}");
    assert!(
        text.contains("missing") && text.contains("keys/mu.pub"),
        "the reason must name the missing file: {text}"
    );
}

#[test]
fn a_key_readable_by_others_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    full_config(dir.path(), 0o644);
    let (code, text) = run(dir.path());
    assert_eq!(code, 6, "expected refusal for mode 644, got {code}: {text}");
    assert!(text.contains("600"), "the reason must name mode 600: {text}");
}

#[test]
fn a_complete_config_gets_past_the_file_stage() {
    let dir = tempfile::tempdir().unwrap();
    full_config(dir.path(), 0o600);
    let (code, text) = run(dir.path());
    // No mu.bin yet: the daemon must fail on the μ object (2), not on config (6).
    assert_eq!(code, 2, "expected μ error after config, got {code}: {text}");
}

fn run_with(home: &Path, extra: &[(&str, &str)]) -> (i32, String) {
    let mut cmd = Command::new(BIN);
    cmd.env("MU_HOME", home).current_dir(home);
    for (k, v) in extra {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("spawn daemon");
    let code = out.status.code().unwrap_or(-1);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (code, text)
}

#[test]
fn a_missing_chain_id_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    full_config(dir.path(), 0o600);
    let out = Command::new(BIN)
        .env("MU_HOME", dir.path())
        .env_remove("MU_CHAIN_ID")
        .current_dir(dir.path())
        .output()
        .expect("spawn");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(out.status.code(), Some(6), "{text}");
    assert!(text.contains("MU_CHAIN_ID is required"), "{text}");
}

#[test]
fn a_network_on_another_chain_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    full_config(dir.path(), 0o600);
    let (code, text) = run_with(
        dir.path(),
        &[
            ("MU_CHAIN_ID", "8453"),
            ("MU_CONNECTOR", "x402"),
            ("MU_X402_NETWORK", "base-sepolia"),
            ("MU_WALLET_ADDR", "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"),
        ],
    );
    assert_eq!(code, 6, "got {code}: {text}");
    assert!(text.contains("is chain 84532"), "{text}");
}

#[test]
fn x402_without_a_network_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    full_config(dir.path(), 0o600);
    let (code, text) = run_with(
        dir.path(),
        &[
            ("MU_CHAIN_ID", "84532"),
            ("MU_CONNECTOR", "x402"),
            ("MU_WALLET_ADDR", "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"),
        ],
    );
    assert_eq!(code, 6, "got {code}: {text}");
    assert!(text.contains("MU_X402_NETWORK is required"), "{text}");
}

#[test]
fn the_source_keeps_no_keys_or_test_vault() {
    let src = include_str!("../src/main.rs");
    assert!(!src.contains("for_test"), "main.rs must not call for_test");
    assert!(!src.contains("MU_PUBKEY_HEX"), "no compiled-in pubkey");
    assert!(!src.contains("SoftVault::for_test"), "no test vault in the daemon");
}
