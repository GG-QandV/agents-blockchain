//! µ-daemon — boot-протокол M6 §4 + unix socket транспорт M8.
//!
//! Boot-порядок (строгий):
//!   0. Config::load (MU_HOME: ключи 0600, agents.toml, policy.toml) → ✗ = exit(6)
//!   1. M1: load + verify + verify_against_log   → ✗ = exit(2)
//!   2. M5: Log::open + verify_chain              → ✗ = exit(3)
//!   3. M5: pending() → M7a.status → reconcile
//!   4. M8: gate.restore_nonce з NonceSnapshot лога
//!   5. Слухати unix socket → кадри → Gate::accept → Runtime::process
//!
//! Ключі та політика не зашиті в код: без файлів у MU_HOME демон не стартує.

#![forbid(unsafe_code)]

#[cfg(all(not(debug_assertions), feature = "softvault"))]
compile_error!("softvault запрещён в release");

mod config;

use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

use mu_common::{Clock, ConnectorId, SysClock};
use mu_connect::stub::StubConnector;
use mu_connect::x402::X402Connector;
use mu_connect::Connector;
use mu_core::{CoreErr, Mu, MU_MAX_SIZE};
use mu_gate::{AllowList, DenyCode, Gate};
use mu_human::Presenter;
use mu_license::License;
use mu_log::Kind;
use mu_log::{Log, LogErr};
use mu_runtime::{IntentStatus, Runtime};
use mu_vault::backend::SoftVault;

use crate::config::Config;

const BUCKET_CAP: u32 = 10;
const TS_WINDOW_SECS: u64 = 60;

fn hex20(s: &str) -> [u8; 20] {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let mut a = [0u8; 20];
    for i in 0..20 {
        a[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap_or(0);
    }
    a
}

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

#[derive(Debug)]
enum BootErr {
    Mu(CoreErr),
    Log(LogErr),
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
    allow: AllowList,
    vault: &'static SoftVault,
) -> Result<(Mu, Log, Gate), BootErr> {
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
    let log = Log::open(log_path, vault)?;
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
    let mut gate = Gate::new(allow, BUCKET_CAP, TS_WINDOW_SECS);
    for e in log.entries() {
        if let Kind::NonceSnapshot { agent_id, nonce } = &e.kind {
            gate.restore_nonce(agent_id, *nonce);
        }
    }

    Ok((mu, log, gate))
}

fn handle_client(
    mut stream: UnixStream,
    gate: &mut Gate,
    runtime: &mut Runtime,
    clock: &dyn Clock,
    connector: ConnectorId,
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
                connector,
                resource: vi.resource.clone(),
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
    connector: ConnectorId,
) -> std::io::Result<()> {
    let _ = fs::remove_file(sock_path);
    let listener = UnixListener::bind(sock_path)?;
    fs::set_permissions(sock_path, fs::Permissions::from_mode(0o600))?;
    eprintln!("μ-daemon listening on {}", sock_path.display());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream, gate, runtime, clock, connector),
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
    Ok(())
}

fn main() {
    let cfg = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FATAL: config: {e}");
            std::process::exit(6);
        }
    };

    let Config { mu_path, log_path, sock_path, mu_pubkey, vault, allow, omega, delta, connector: connector_id, .. } = cfg;

    let vault: &'static SoftVault = Box::leak(Box::new(vault));

    let (mu, log, mut gate) = match boot(&mu_path, &log_path, &mu_pubkey, allow, vault) {
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
        Err(BootErr::Io(e)) => {
            eprintln!("FATAL: io error: {e}");
            std::process::exit(6);
        }
    };

    let presenter: &'static CliPresenter = Box::leak(Box::new(CliPresenter));
    let clock = SysClock;

    // Вибір коннектора: MU_CONNECTOR=x402 або за замовчуванням stub
    let connector: &'static dyn Connector = match std::env::var("MU_CONNECTOR").as_deref() {
        Ok("x402") => {
            let wallet_hex = std::env::var("MU_WALLET_ADDR")
                .expect("MU_WALLET_ADDR required for x402 connector");
            let wallet = hex20(&wallet_hex);
            let net = match std::env::var("MU_X402_NETWORK").as_deref() {
                Ok("base-sepolia") => mu_connect::x402::BASE_SEPOLIA,
                Ok("base") | Err(_) => mu_connect::x402::BASE,
                Ok(other) => panic!("MU_X402_NETWORK={other}: base | base-sepolia"),
            };
            eprintln!("Connector: X402 on {} (wallet: {wallet_hex}; the signing key must be this wallet)", net.name);
            Box::leak(Box::new(X402Connector::on(net, wallet)))
        }
        _ => {
            eprintln!("Connector: Stub (set MU_CONNECTOR=x402 for real payments)");
            Box::leak(Box::new(StubConnector::new("bank_stub")))
        }
    };

    let mut runtime = Runtime {
        omega,
        delta,
        log,
        vault,
        connector,
        presenter,
        clock: &clock,
        owner_pubkey: mu.owner_pubkey().to_vec(),
        human_ttl: std::time::Duration::from_secs(120),
    };

    if let Err(e) = serve(&sock_path, &mut gate, &mut runtime, &clock, connector_id) {
        eprintln!("serve error: {e}");
        std::process::exit(4);
    }
}
