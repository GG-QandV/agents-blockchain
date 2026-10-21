//! Daemon configuration from files under `MU_HOME` (default `/tmp/mu`).
//!
//! Layout:
//!   $MU_HOME/mu.bin            μ object
//!   $MU_HOME/mu.log            hash-chain WAL
//!   $MU_HOME/keys/mu.pub       μ public key, hex
//!   $MU_HOME/keys/mu.hex       μ signing key, 0600
//!   $MU_HOME/keys/owner.hex    owner key, 0600
//!   $MU_HOME/keys/wallet.hex   wallet key, 0600
//!   $MU_HOME/agents.toml       [[agent]] id, pubkey
//!   $MU_HOME/policy.toml       Δ (daily limit, whitelist, resource rules)
//!   $MU_HOME/mu-daemon.sock    unix socket
//!
//! No key or policy is compiled into the daemon: without the files it does
//! not start.

use std::path::{Path, PathBuf};

use mu_common::{Amount, ConnectorId};
use mu_gate::AllowList;
use mu_runtime::policy::{Delta, Omega};
use mu_vault::backend::SoftVault;
use serde::Deserialize;

#[derive(Debug)]
pub enum ConfigErr {
    Missing(String),
    Bad(String),
}

impl std::fmt::Display for ConfigErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigErr::Missing(s) => write!(f, "missing {s}"),
            ConfigErr::Bad(s) => write!(f, "bad {s}"),
        }
    }
}

pub struct Config {
    pub home: PathBuf,
    pub mu_path: PathBuf,
    pub log_path: PathBuf,
    pub sock_path: PathBuf,
    pub mu_pubkey: Vec<u8>,
    pub vault: SoftVault,
    pub allow: AllowList,
    pub omega: Omega,
    pub delta: Delta,
    pub connector: ConnectorId,
}

#[derive(Deserialize)]
struct AgentsFile {
    #[serde(default)]
    agent: Vec<AgentEntry>,
}

#[derive(Deserialize)]
struct AgentEntry {
    id: String,
    pubkey: String,
}

/// The connector selected by `MU_CONNECTOR`: `x402` or the default stub.
pub fn connector_id() -> Result<ConnectorId, ConfigErr> {
    match std::env::var("MU_CONNECTOR").as_deref() {
        Ok("x402") => Ok(ConnectorId::Crypto),
        Ok("") | Err(_) => Ok(ConnectorId::BankStub),
        Ok(other) => Err(ConfigErr::Bad(format!(
            "MU_CONNECTOR={other}: expected x402 or stub"
        ))),
    }
}

fn hex_decode(text: &str, what: &str) -> Result<Vec<u8>, ConfigErr> {
    let s = text.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.len() % 2 != 0 {
        return Err(ConfigErr::Bad(format!("{what}: odd hex length")));
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    for i in (0..s.len()).step_by(2) {
        let b = u8::from_str_radix(&s[i..i + 2], 16)
            .map_err(|_| ConfigErr::Bad(format!("{what}: not hex")))?;
        out.push(b);
    }
    if out.is_empty() {
        return Err(ConfigErr::Bad(format!("{what}: empty")));
    }
    Ok(out)
}

fn read_text(path: &Path) -> Result<String, ConfigErr> {
    std::fs::read_to_string(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ConfigErr::Missing(path.display().to_string())
        } else {
            ConfigErr::Bad(format!("{}: {e}", path.display()))
        }
    })
}

impl Config {
    pub fn load() -> Result<Self, ConfigErr> {
        let home = std::env::var("MU_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp/mu"));
        let keys = home.join("keys");
        let mu_pubkey = hex_decode(
            &read_text(&keys.join("mu.pub"))?,
            "keys/mu.pub",
        )?;
        if mu_pubkey.len() != 33 {
            return Err(ConfigErr::Bad("keys/mu.pub: expected 33 bytes".into()));
        }
        let vault = SoftVault::from_files(
            &keys.join("mu.hex"),
            &keys.join("owner.hex"),
            &keys.join("wallet.hex"),
        )
        .map_err(|e| ConfigErr::Bad(format!("vault: {e:?}")))?;

        let chain_id: u64 = std::env::var("MU_CHAIN_ID")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8453);

        let agents = read_text(&home.join("agents.toml"))?;
        let parsed: AgentsFile = toml::from_str(&agents)
            .map_err(|e| ConfigErr::Bad(format!("agents.toml: {e}")))?;
        if parsed.agent.is_empty() {
            return Err(ConfigErr::Bad("agents.toml: no agents".into()));
        }
        let mut allow = AllowList::new();
        for a in &parsed.agent {
            let pk = hex_decode(&a.pubkey, &format!("agents.toml[{}]", a.id))?;
            if pk.len() != 32 {
                return Err(ConfigErr::Bad(format!(
                    "agents.toml[{}]: expected 32 bytes",
                    a.id
                )));
            }
            let mut key = [0u8; 32];
            key.copy_from_slice(&pk);
            allow.add(&a.id, key);
        }

        let policy = mu_policy::config::load_policy(&home.join("policy.toml"), chain_id)
            .map_err(|e| ConfigErr::Bad(format!("policy.toml: {e}")))?;
        let delta = Delta {
            daily_limit: policy.daily_limit,
            whitelist: policy.whitelist.iter().map(|w| w.address).collect(),
            confirm_threshold: policy.confirm_threshold,
            resource_allowlist: policy.resource_allowlist,
        };

        let connector = connector_id()?;
        let max_ceiling: u128 = std::env::var("MU_MAX_CEILING_MINOR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10_000_000_000);
        let omega = Omega {
            connectors: vec![connector],
            max_ceiling: Amount::from_minor(max_ceiling),
        };

        Ok(Config {
            mu_path: home.join("mu.bin"),
            log_path: home.join("mu.log"),
            sock_path: home.join("mu-daemon.sock"),
            home,
            mu_pubkey,
            vault,
            allow,
            omega,
            delta,
            connector,
        })
    }
}
