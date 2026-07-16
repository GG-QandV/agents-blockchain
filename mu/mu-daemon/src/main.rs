//! µ-daemon — boot-протокол M6 §4 + unix socket транспорт M8.
//!
//! Boot-порядок (строгий):
//!   1. M1: load + verify + verify_against_log   → ✗ = exit(2)
//!   2. M5: Log::open + verify_chain              → ✗ = exit(3)
//!   3. M5: pending() → M7a.status → reconcile
//!   4. M8: gate.restore_nonce з NonceSnapshot лога
//!   5. Слухати unix socket → кадри → Gate::accept → Runtime::process

#![forbid(unsafe_code)]

#[cfg(all(not(debug_assertions), feature = "softvault"))]
compile_error!("softvault запрещён в release");

use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

use mu_common::{Clock, ConnectorId, SysClock};
use mu_connect::stub::StubConnector;
use mu_connect::Connector;
use mu_core::{CoreErr, Mu, MU_MAX_SIZE};
use mu_gate::{AllowList, DenyCode, Gate};
use mu_human::Presenter;
use mu_license::License;
use mu_log::Kind;
use mu_log::{Log, LogErr};
use mu_runtime::{IntentStatus, Runtime};
use mu_vault::backend::SoftVault;

// ── Конфіґ (MVP: хардкод; прод: env/конфіґ) ─────────────────────────────

const MU_PATH: &str = "mu.bin";
const LOG_PATH: &str = "mu.log";
const SOCK_PATH: &str = "/tmp/mu-daemon.sock";
const MU_PUBKEY_HEX: &str = "026ff03b949241ce1dadd43519e6960e0a85b41a69a05c328103aa2bce1594ca16";
const BUCKET_CAP: u32 = 10;
const TS_WINDOW_SECS: u64 = 60;

// ── Допоміжні ───────────────────────────────────────────────────────────

fn hex_decode(s: &str) -> Vec<u8> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

// ── Presenter-заглушка (M9) ─────────────────────────────────────────────

struct CliPresenter;

impl Presenter for CliPresenter {
    fn present(
        &self,
        _req: &mu_human::PayConfirm,
        _budget: std::time::Duration,
    ) -> mu_human::PresenterChoice {
        mu_human::PresenterChoice::Deny
    }
}

// ── Boot-протокол (до виходу на сокет) ──────────────────────────────────

#[derive(Debug)]
enum BootErr {
    Mu(CoreErr),
    Log(LogErr),
    Vault(String),
    Io(std::io::Error),
}

impl From<CoreErr> for BootErr {
    fn from(e: CoreErr) -> Self { BootErr::Mu(e) }
}
impl From<LogErr> for BootErr {
    fn from(e: LogErr) -> Self { BootErr::Log(e) }
}
impl From<std::io::Error> for BootErr {
    fn from(e: std::io::Error) -> Self { BootErr::Io(e) }
}

fn boot(
    mu_path: &Path,
    log_path: &Path,
    mu_pubkey: &[u8],
) -> Result<(Mu, Log, Gate, SoftVault, StubConnector, CliPresenter, SysClock), BootErr> {
    // ── Крок 0: License check ──────────────────────────────────────────
    let license_path = Path::new("license.key");
    match License::load(license_path) {
        Ok(l) => match l.mode {
            mu_license::LicenseMode::Personal => {
                println!("License: PERSONAL (full functionality, embed license available at $299)");
            }
            mu_license::LicenseMode::Commercial => {
                println!("License: COMMERCIAL EMBED");
            }
        },
        Err(e) => {
            eprintln!("License error: {e:?} — aborting");
            std::process::exit(2);
        }
    };

    // ── Крок 1: M1 — load + verify + verify_against_log (RISK-M1-2) ──
    let mu = Mu::load(mu_path)?;
    mu.verify(mu_pubkey).map_err(BootErr::Mu)?;

    // ── Крок 2: M5 — Log::open + verify_chain (RISK-M5-2) ────────────
    let vault = SoftVault::for_test([1u8; 32], [2u8; 32], [3u8; 32]);
    let log = Log::open(log_path, &vault)?;
    log.verify_chain()?;

    // verify_against_log: log_head == хвіст лога (RISK-M1-2)
    mu.verify_against_log(log.last_hash())?;

    // ── Крок 3: pending() → status → reconcile ──────────────────────
    let stub = StubConnector::new("bank_stub");
    for entry in log.pending() {
        if let Kind::Pending { .. } = &entry.kind {
            let _ = stub.status(&mu_connect::TxRef::Simulated { id: [0u8; 16] });
        }
    }

    // ── Крок 4: M8 — restore_nonce з NonceSnapshot логу (RISK-M8-1) ──
    let mut allow = AllowList::new();
    // SMOKE: додаємо тестового агента (seed [42;32])
    let agent_pubkey: [u8; 32] = [
        0x19, 0x7f, 0x6b, 0x23, 0xe1, 0x6c, 0x85, 0x32,
        0xc6, 0xab, 0xc8, 0x38, 0xfa, 0xcd, 0x5e, 0xa7,
        0x89, 0xbe, 0x0c, 0x76, 0xb2, 0x92, 0x03, 0x34,
        0x03, 0x9b, 0xfa, 0x8b, 0x3d, 0x36, 0x8d, 0x61,
    ];
    allow.add("agent-1", agent_pubkey);
    let mut gate = Gate::new(allow, BUCKET_CAP, TS_WINDOW_SECS);
    for e in log.entries() {
        if let Kind::NonceSnapshot { agent_id, nonce } = &e.kind {
            gate.restore_nonce(agent_id, *nonce);
        }
    }

    Ok((mu, log, gate, vault, stub, CliPresenter, SysClock))
}

// ── Транспорт M8: unix socket ───────────────────────────────────────────

fn handle_client(
    mut stream: UnixStream,
    gate: &mut Gate,
    runtime: &mut Runtime,
    clock: &dyn Clock,
) {
    let mut len_buf = [0u8; 4];
    if stream.read_exact(&mut len_buf).is_err() {
        return;
    }
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > MU_MAX_SIZE || len < 4 {
        let _ = stream.write_all(&[DenyCode::Parse as u8]);
        return;
    }
    let mut frame = vec![0u8; len];
    if stream.read_exact(&mut frame).is_err() {
        return;
    }

    let now = clock.now_unix();
    match gate.accept(&frame, now) {
        Ok(vi) => {
            let Ok(recipient) = mu_common::CanonAddress::canon(&vi.recipient, vi.chain_id)
            else {
                let _ = stream.write_all(&[0xFF]);
                return;
            };

            let intent = mu_runtime::pipeline::RtIntent {
                recipient,
                amount: mu_common::Amount::from_minor(vi.amount),
                chain_id: vi.chain_id,
                agent_id: vi.agent_id,
                connector: ConnectorId::BankStub,
            };

            let status = runtime.process(&intent);
            let response = match status {
                IntentStatus::DeniedOmega => 0x10u8,
                IntentStatus::DeniedDelta => 0x11,
                IntentStatus::DeniedHuman => 0x12,
                IntentStatus::TimeoutHuman => 0x13,
                IntentStatus::Settled { .. } => 0x20,
                IntentStatus::Failed => 0x30,
                IntentStatus::ReconcilePending => 0x40,
            };
            let _ = stream.write_all(&[response]);
        }
        Err(code) => {
            let _ = stream.write_all(&[code as u8]);
        }
    }
}

fn serve(
    sock_path: &Path,
    gate: &mut Gate,
    runtime: &mut Runtime,
    clock: &dyn Clock,
) -> std::io::Result<()> {
    let _ = fs::remove_file(sock_path);
    let listener = UnixListener::bind(sock_path)?;
    fs::set_permissions(sock_path, fs::Permissions::from_mode(0o600))?;
    eprintln!("μ-daemon listening on {}", sock_path.display());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream, gate, runtime, clock),
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
    Ok(())
}

// ── Main ────────────────────────────────────────────────────────────────

fn main() {
    let mu_path = Path::new(MU_PATH);
    let log_path = Path::new(LOG_PATH);
    let sock_path = Path::new(SOCK_PATH);
    let mu_pubkey = hex_decode(MU_PUBKEY_HEX);

    let (mu, mut log, mut gate, vault, stub, presenter, clock) =
        match boot(mu_path, log_path, &mu_pubkey) {
            Ok(v) => v,
            Err(BootErr::Mu(CoreErr::LogHeadMismatch)) => {
                eprintln!("FATAL: log_head mismatch — стара копія mu.bin");
                std::process::exit(2);
            }
            Err(BootErr::Mu(e)) => {
                eprintln!("FATAL: mu error: {e:?}");
                std::process::exit(2);
            }
            Err(BootErr::Log(LogErr::ChainBroken { at_seq })) => {
                eprintln!("FATAL: log chain broken at seq {at_seq}");
                std::process::exit(3);
            }
            Err(BootErr::Log(e)) => {
                eprintln!("FATAL: log error: {e:?}");
                std::process::exit(3);
            }
            Err(BootErr::Vault(e)) => {
                eprintln!("FATAL: vault error: {e}");
                std::process::exit(5);
            }
            Err(BootErr::Io(e)) => {
                eprintln!("FATAL: io error: {e}");
                std::process::exit(6);
            }
        };

    // Leak об'єкти, яким потрібен статичний час життя для Runtime
    let vault: &'static SoftVault = Box::leak(Box::new(vault));
    let stub: &'static StubConnector = Box::leak(Box::new(stub));
    let presenter: &'static CliPresenter = Box::leak(Box::new(presenter));

    let omega = mu_runtime::policy::Omega {
        connectors: vec![ConnectorId::BankStub, ConnectorId::CardStub],
        max_ceiling: mu_common::Amount::from_minor(10_000_000_000),
    };
    let delta = mu_runtime::policy::Delta {
        daily_limit: mu_common::Amount::from_minor(5_000_000_000),
        whitelist: vec![
            mu_common::CanonAddress::canon(
                "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed000000000000000000000000", 8453
            ).unwrap(),
        ],
        confirm_threshold: mu_common::Amount::from_minor(1_000_000_000),
        resource_allowlist: vec![],
    };

    let mut runtime = Runtime {
        omega,
        delta,
        log,
        vault,
        connector: stub,
        presenter,
        clock: &clock,
        owner_pubkey: mu.owner_pubkey().to_vec(),
        human_ttl: std::time::Duration::from_secs(120),
    };

    if let Err(e) = serve(&sock_path, &mut gate, &mut runtime, &clock) {
        eprintln!("serve error: {e}");
        std::process::exit(4);
    }
}
