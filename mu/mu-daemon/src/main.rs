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
mod boot_guard;
mod policy;
mod policy_state;
mod reconcile;

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
use mu_vault::Vault;

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

/// Owner confirmation over stdin: the daemon prints the payment and waits for
/// `y`/`yes`/`approve` or `n`/`no`/`deny` within the budget. The proof itself
/// is signed by the vault's owner key in `confirm_payment`; the presenter only
/// returns the owner's intent.
struct StdinPresenter;

impl Presenter for StdinPresenter {
    fn present(
        &self,
        req: &mu_human::PayConfirm,
        budget: std::time::Duration,
    ) -> mu_human::PresenterChoice {
        eprintln!("{}", mu_human::render_recipient_line(req));
        eprintln!(
            "amount: {} minor; budget: {}s — confirm? [y/N]",
            req.amount.minor(),
            budget.as_secs()
        );
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line);
            let _ = tx.send(line);
        });
        match rx.recv_timeout(budget) {
            Ok(line) => parse_decision(&line),
            Err(_) => mu_human::PresenterChoice::NoResponse,
        }
    }
}

/// The owner's answer: only an explicit yes approves; anything else denies.
fn parse_decision(line: &str) -> mu_human::PresenterChoice {
    match line.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" | "approve" | "a" => mu_human::PresenterChoice::Approve,
        "n" | "no" | "deny" | "d" => mu_human::PresenterChoice::Deny,
        _ => mu_human::PresenterChoice::NoResponse,
    }
}

#[derive(Debug)]
enum BootErr {
    Mu(CoreErr),
    /// The vault's owner key is not the one μ was issued with.
    OwnerKeyMismatch,
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
    connector: &dyn Connector,
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
    // The key that signs Δ must be the key μ was issued with, otherwise the
    // daemon would confirm changes under a key the object does not trust.
    if !boot_guard::owner_keys_agree(&mu, vault) {
        return Err(BootErr::OwnerKeyMismatch);
    }

    // ── Крок 2: M5 — Log::open + verify_chain (RISK-M5-2) ────────────
    let log = Log::open(log_path, vault)?;
    log.verify_chain()?;

    // verify_against_log with catch-up: a crash between the journal append and
    // the μ save leaves μ one step behind. Catching up is allowed only when
    // μ's head is a real link of the verified chain; anything else (a shorter
    // journal, a foreign μ, a rollback) stops the boot.
    let mu = boot_guard::catch_up(&mu, &log, vault, mu_path)?;

    // ── Крок 3: pending() → status → reconcile ──────────────────────
    let mut log = log;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    reconcile::reconcile(&mut log, connector, vault, now);

    // ── Крок 4: M8 — restore_nonce з NonceSnapshot логу (RISK-M8-1) ──
    let mut gate = Gate::new(allow, BUCKET_CAP, TS_WINDOW_SECS);
    for e in log.entries() {
        if let Kind::NonceSnapshot { agent_id, nonce } = &e.kind {
            gate.restore_nonce(agent_id, *nonce);
        }
    }

    Ok((mu, log, gate))
}

/// Response framing. A denial is one status byte (as before). A settled
/// payment is `0x20 ‖ len:u16 LE ‖ X-PAYMENT`, so the agent gets the header.
fn response_frame(code: u8, header: Option<String>) -> Vec<u8> {
    match (code, header) {
        (0x20, Some(h)) => {
            let body = h.into_bytes();
            let len = (body.len().min(u16::MAX as usize)) as u16;
            let mut out = vec![0x20];
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(&body[..len as usize]);
            out
        }
        (0x20, None) => vec![0x20, 0, 0],
        _ => vec![code],
    }
}

fn handle_client(
    mut stream: UnixStream,
    gate: &mut Gate,
    runtime: &mut Runtime,
    clock: &dyn Clock,
    connector: ConnectorId,
) {
    // A client that connects and says nothing must not hold the loop.
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(30)));
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
            let (code, header) = match status {
                IntentStatus::DeniedOmega => (0x10u8, None),
                IntentStatus::DeniedDelta => (0x11, None),
                // Wire v3 marker: the daemon had no 402 offer to sign.
                IntentStatus::DeniedNoOffer => (0x15, None),
                IntentStatus::DeniedHuman => (0x12, None),
                IntentStatus::TimeoutHuman => (0x13, None),
                IntentStatus::Settled { .. } => (0x20, runtime.connector.take_header()),
                IntentStatus::Failed => (0x30, None),
                IntentStatus::ReconcilePending => (0x40, None),
            };
            let _ = stream.write_all(&response_frame(code, header));
        }
        Err(code) => {
            let _ = stream.write_all(&[code as u8]);
        }
    }
}

/// What the main loop needs to keep μ and the policy cache in step.
struct PolicyCtx {
    mu_path: std::path::PathBuf,
    policy_path: std::path::PathBuf,
    version: u16,
}

/// Bind μ to the log tail after an append. The file is written first: a failed
/// save leaves both the file and the in-memory copy on the old head, and the
/// next start fails closed instead of silently ignoring the new record.
fn advance_mu(mu: &mut Mu, log: &Log, vault: &dyn Vault, path: &Path) {
    match mu.advanced(log.last_hash(), vault) {
        Ok(next) => match next.save(path) {
            Ok(()) => *mu = next,
            Err(e) => eprintln!("mu save: {e:?}"),
        },
        Err(e) => eprintln!("mu advance: {e:?}"),
    }
}

fn serve(
    sock_path: &Path,
    gate: &mut Gate,
    runtime: &mut Runtime,
    clock: &dyn Clock,
    connector: ConnectorId,
    policy_rx: std::sync::mpsc::Receiver<policy::Request>,
    mu: &mut Mu,
    ctx: &mut PolicyCtx,
) -> std::io::Result<()> {
    let _ = fs::remove_file(sock_path);
    let listener = UnixListener::bind(sock_path)?;
    fs::set_permissions(sock_path, fs::Permissions::from_mode(0o600))?;
    listener.set_nonblocking(true)?;
    eprintln!("μ-daemon listening on {}", sock_path.display());
    loop {
        // Commit a policy the owner confirmed on the socket: journal (fsync),
        // then the policy.toml cache, then μ. Any failure is answered to the
        // waiting composer and leaves the previous policy in force.
        while let Ok(req) = policy_rx.try_recv() {
            let now = clock.now_unix();
            let res = policy_state::commit(
                &mut runtime.log,
                mu,
                runtime.vault,
                &ctx.policy_path,
                req.delta.clone(),
                now,
            );
            match res {
                Ok((applied, next_mu)) => match next_mu.save(&ctx.mu_path) {
                    Ok(()) => {
                        *mu = next_mu;
                        ctx.version = applied.state.version;
                        runtime.delta = mu_runtime::policy::Delta {
                            daily_limit: applied.state.delta.daily_limit,
                            whitelist: applied
                                .state
                                .delta
                                .whitelist
                                .iter()
                                .map(|w| w.address)
                                .collect(),
                            confirm_threshold: applied.state.delta.confirm_threshold,
                            resource_allowlist: applied.state.delta.resource_allowlist.clone(),
                        };
                        eprintln!(
                            "policy applied: v{} {}",
                            applied.state.version,
                            policy::hex(&applied.new_hash)
                        );
                        let _ = req.reply.send(policy::Answer::Applied {
                            version: applied.state.version,
                            new_hash: applied.new_hash,
                        });
                    }
                    Err(e) => {
                        eprintln!("policy: mu save: {e:?}");
                        let _ = req.reply.send(policy::Answer::Rejected("mu_save".into()));
                    }
                },
                Err(e) => {
                    eprintln!("policy: {e:?}");
                    let _ = req.reply.send(policy::Answer::Rejected(format!("{e:?}")));
                }
            }
        }
        match listener.accept() {
            Ok((stream, _)) => {
                let _ = stream.set_nonblocking(false);
                handle_client(stream, gate, runtime, clock, connector);
                advance_mu(mu, &runtime.log, runtime.vault, &ctx.mu_path);
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
}

fn main() {
    let cfg = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FATAL: config: {e}");
            std::process::exit(6);
        }
    };

    let Config {
        mu_path, log_path, sock_path, mu_pubkey, vault, allow, omega, delta: _file_delta,
        policy: policy_delta, policy_path, chain_id, connector: connector_id, ..
    } = cfg;

    let vault: &'static SoftVault = Box::leak(Box::new(vault));

    // Вибір коннектора: MU_CONNECTOR=x402 або за замовчуванням stub
    let connector: &'static dyn Connector = match std::env::var("MU_CONNECTOR").as_deref() {
        Ok("x402") => {
            let wallet_hex = std::env::var("MU_WALLET_ADDR")
                .expect("MU_WALLET_ADDR required for x402 connector");
            let wallet = hex20(&wallet_hex);
            let net = match std::env::var("MU_X402_NETWORK").as_deref() {
                Ok("base-sepolia") => mu_connect::x402::BASE_SEPOLIA,
                Ok("base") => mu_connect::x402::BASE,
                Ok(other) => panic!("MU_X402_NETWORK={other}: base | base-sepolia"),
                // config.rs already refused a missing value; no silent default here.
                Err(_) => panic!("MU_X402_NETWORK is required for the x402 connector"),
            };
            eprintln!("Connector: X402 on {} (wallet: {wallet_hex}; the signing key must be this wallet)", net.name);
            Box::leak(Box::new(X402Connector::on(net, wallet)))
        }
        _ => {
            eprintln!("Connector: Stub (set MU_CONNECTOR=x402 for real payments)");
            Box::leak(Box::new(StubConnector::new("bank_stub")))
        }
    };

    let (mu, log, mut gate) = match boot(&mu_path, &log_path, &mu_pubkey, allow, vault, connector) {
        Ok(v) => v,
        Err(BootErr::Mu(CoreErr::LogHeadMismatch)) => {
            eprintln!("FATAL: log_head mismatch — μ is not a link of this journal (old copy, rollback or foreign file)");
            std::process::exit(2);
        }
        Err(BootErr::OwnerKeyMismatch) => {
            eprintln!("FATAL: vault owner key != mu.owner_pubkey — refusing to start");
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

    // The journal wins over the file: policy.toml is only a cache of it.
    let policy_now = match policy_state::load(&log, mu.owner_pubkey(), &policy_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("FATAL: policy: {e:?}");
            std::process::exit(5);
        }
    };
    eprintln!("policy: v{} (journal is the source of truth)", policy_now.version);

    let presenter: &'static StdinPresenter = Box::leak(Box::new(StdinPresenter));
    let clock = SysClock;

    let mut runtime = Runtime {
        omega,
        delta: mu_runtime::policy::Delta {
            daily_limit: policy_now.delta.daily_limit,
            whitelist: policy_now.delta.whitelist.iter().map(|w| w.address).collect(),
            confirm_threshold: policy_now.delta.confirm_threshold,
            resource_allowlist: policy_now.delta.resource_allowlist.clone(),
        },
        log,
        vault,
        connector,
        presenter,
        clock: &clock,
        owner_pubkey: vault
            .owner_pubkey()
            .unwrap_or_else(|| mu.owner_pubkey().to_vec()),
        human_ttl: std::time::Duration::from_secs(120),
    };

    let policy_sock = policy::sock_path(std::path::Path::new(
        std::env::var("MU_HOME").as_deref().unwrap_or("/tmp/mu"),
    ));
    let (policy_tx, policy_rx) = std::sync::mpsc::channel();
    let policy_state = composer_core::endpoint::DaemonPolicy {
        current: policy_now.delta.clone(),
        omega: mu_policy::OmegaView {
            max_ceiling: runtime.omega.max_ceiling,
            supported_chain_ids: vec![chain_id],
        },
    };
    if let Err(e) = policy::spawn(policy_sock.clone(), policy_state, policy_tx) {
        eprintln!("policy endpoint: {e}");
    } else {
        eprintln!("policy endpoint on {} (MU_POLICY_SOCK)", policy_sock.display());
    }

    let mut mu = mu;
    let mut policy_ctx = PolicyCtx {
        mu_path: mu_path.clone(),
        policy_path: policy_path.clone(),
        version: policy_now.version,
    };
    if let Err(e) = serve(
        &sock_path,
        &mut gate,
        &mut runtime,
        &clock,
        connector_id,
        policy_rx,
        &mut mu,
        &mut policy_ctx,
    ) {
        eprintln!("serve error: {e}");
        std::process::exit(4);
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_decision, response_frame};
    #[test]
    fn a_zero_budget_is_a_timeout() {
        use mu_common::{Amount, CanonAddress, Hash32};
        use mu_human::{PayConfirm, Presenter, PresenterChoice};
        let req = PayConfirm {
            recipient: CanonAddress::canon(
                "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
                8453,
            )
            .unwrap(),
            wl_label: None,
            amount: Amount::from_minor(1),
            gas_est: Amount::ZERO,
            agent_id: "agent-1".into(),
            remaining_window: Amount::ZERO,
            intent_hash: Hash32([0; 32]),
            first_payment_to_recipient: false,
        };
        let p = super::StdinPresenter;
        assert_eq!(
            p.present(&req, std::time::Duration::from_millis(0)),
            PresenterChoice::NoResponse
        );
    }

    #[test]
    fn only_an_explicit_yes_approves() {
        use mu_human::PresenterChoice::*;
        assert_eq!(parse_decision("y\n"), Approve);
        assert_eq!(parse_decision("YES"), Approve);
        assert_eq!(parse_decision("approve"), Approve);
        assert_eq!(parse_decision("n"), Deny);
        assert_eq!(parse_decision("no"), Deny);
        assert_eq!(parse_decision(""), NoResponse);
        assert_eq!(parse_decision("maybe"), NoResponse);
    }

    #[test]
    fn a_denial_is_one_byte() {
        assert_eq!(response_frame(0x11, None), vec![0x11]);
    }

    #[test]
    fn a_settled_payment_carries_the_header_once() {
        let f = response_frame(0x20, Some("X-PAYMENT: abc".into()));
        assert_eq!(f[0], 0x20);
        assert_eq!(u16::from_le_bytes([f[1], f[2]]) as usize, f.len() - 3);
        assert_eq!(&f[3..], b"X-PAYMENT: abc");
        assert_eq!(response_frame(0x20, None), vec![0x20, 0, 0]);
    }
}
