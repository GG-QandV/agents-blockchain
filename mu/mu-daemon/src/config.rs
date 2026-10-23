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
use mu_vault::Vault;
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
    /// The secret backend: `SoftVault` in tests, `FileVault` in production.
    pub vault: Box<dyn Vault>,
    pub allow: AllowList,
    pub omega: Omega,
    pub delta: Delta,
    /// The same policy in the Composer's type, for the policy endpoint.
    pub policy: mu_policy::Delta,
    pub policy_path: std::path::PathBuf,
    pub chain_id: u64,
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
    connector_from(std::env::var("MU_CONNECTOR").ok().as_deref())
}

/// Pure mapping, so the rule can be tested without touching the process env.
pub fn connector_from(value: Option<&str>) -> Result<ConnectorId, ConfigErr> {
    match value {
        Some("x402") => Ok(ConnectorId::Crypto),
        Some("") | None => Ok(ConnectorId::BankStub),
        Some(other) => Err(ConfigErr::Bad(format!(
            "MU_CONNECTOR={other}: expected x402 or stub"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x402_selects_the_crypto_connector() {
        assert_eq!(connector_from(Some("x402")).unwrap(), ConnectorId::Crypto);
    }

    #[test]
    fn the_default_is_the_bank_stub() {
        assert_eq!(connector_from(None).unwrap(), ConnectorId::BankStub);
        assert_eq!(connector_from(Some("")).unwrap(), ConnectorId::BankStub);
    }

    #[test]
    fn an_unknown_connector_is_refused() {
        assert!(connector_from(Some("paypal")).is_err());
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

/// The secret backend. `MU_VAULT=file` reads one encrypted file and a passphrase
/// file; anything else falls back to the dev-only softvault, which a release
/// build refuses at compile time.
fn open_vault(home: &std::path::Path, keys: &std::path::Path) -> Result<Box<dyn Vault>, ConfigErr> {
    match std::env::var("MU_VAULT").as_deref() {
        Ok("file") => open_file_vault(home),
        Ok(other) => Err(ConfigErr::Bad(format!(
            "MU_VAULT={other}: expected file or empty (softvault, dev only)"
        ))),
        Err(_) => open_soft_vault(keys),
    }
}

#[cfg(feature = "filevault")]
fn open_file_vault(home: &std::path::Path) -> Result<Box<dyn Vault>, ConfigErr> {
    use mu_vault::filevault::FileVault;
    let pass_file = std::env::var("MU_VAULT_PASSPHRASE_FILE").map_err(|_| {
        ConfigErr::Bad("MU_VAULT=file requires MU_VAULT_PASSPHRASE_FILE (mode 600)".into())
    })?;
    let pass = read_secret(std::path::Path::new(&pass_file))?;
    let v = FileVault::open(&home.join("keys").join("vault.enc"), pass.as_bytes())
        .map_err(|e| ConfigErr::Bad(format!("vault: {e:?}")))?;
    Ok(Box::new(v))
}

#[cfg(not(feature = "filevault"))]
fn open_file_vault(_home: &std::path::Path) -> Result<Box<dyn Vault>, ConfigErr> {
    Err(ConfigErr::Bad(
        "MU_VAULT=file needs a build with --features filevault".into(),
    ))
}

#[cfg(feature = "softvault")]
fn open_soft_vault(keys: &std::path::Path) -> Result<Box<dyn Vault>, ConfigErr> {
    use mu_vault::backend::SoftVault;
    let v = SoftVault::from_files(
        &keys.join("mu.hex"),
        &keys.join("owner.hex"),
        &keys.join("wallet.hex"),
    )
    .map_err(|e| ConfigErr::Bad(format!("vault: {e:?}")))?;
    Ok(Box::new(v))
}

#[cfg(not(feature = "softvault"))]
fn open_soft_vault(_keys: &std::path::Path) -> Result<Box<dyn Vault>, ConfigErr> {
    Err(ConfigErr::Bad(
        "no vault: set MU_VAULT=file with a build that has --features filevault".into(),
    ))
}

/// A secret file: mode 0600, trimmed.
fn read_secret(path: &std::path::Path) -> Result<String, ConfigErr> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(path)
            .map_err(|e| ConfigErr::Bad(format!("{path:?}: {e}")))?
            .permissions()
            .mode();
        if mode & 0o077 != 0 {
            return Err(ConfigErr::Bad(format!("{path:?}: must be mode 600")));
        }
    }
    let s = std::fs::read_to_string(path)
        .map_err(|e| ConfigErr::Bad(format!("{path:?}: {e}")))?;
    Ok(s.trim_end_matches(['\n', '\r']).to_string())
}

/// The chain a named x402 network pays on.
pub fn network_chain_id(name: &str) -> Option<u64> {
    match name {
        "base" => Some(mu_connect::x402::BASE.chain_id),
        "base-sepolia" => Some(mu_connect::x402::BASE_SEPOLIA.chain_id),
        _ => None,
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigErr> {
        // No implicit defaults: a daemon that guesses where its keys are is a
        // daemon that can pay with the wrong ones.
        let home = std::env::var("MU_HOME").map(PathBuf::from).map_err(|_| {
            ConfigErr::Bad("MU_HOME is required (directory with keys/, agents.toml, policy.toml)".into())
        })?;
        let keys = home.join("keys");
        let mu_pubkey = hex_decode(
            &read_text(&keys.join("mu.pub"))?,
            "keys/mu.pub",
        )?;
        if mu_pubkey.len() != 33 {
            return Err(ConfigErr::Bad("keys/mu.pub: expected 33 bytes".into()));
        }
        let vault = open_vault(&home, &keys)?;

        let chain_id: u64 = std::env::var("MU_CHAIN_ID")
            .map_err(|_| ConfigErr::Bad("MU_CHAIN_ID is required".into()))?
            .parse()
            .map_err(|_| ConfigErr::Bad("MU_CHAIN_ID must be a number".into()))?;
        let connector = connector_id()?;
        // The network the connector pays on and the chain in the intents must
        // be the same chain, or the money would go to another network.
        if connector == ConnectorId::Crypto {
            let net = std::env::var("MU_X402_NETWORK").map_err(|_| {
                ConfigErr::Bad("MU_X402_NETWORK is required for MU_CONNECTOR=x402 (base | base-sepolia)".into())
            })?;
            let expected = network_chain_id(&net)
                .ok_or_else(|| ConfigErr::Bad(format!("MU_X402_NETWORK={net}: base | base-sepolia")))?;
            if expected != chain_id {
                return Err(ConfigErr::Bad(format!(
                    "MU_X402_NETWORK={net} is chain {expected}, MU_CHAIN_ID is {chain_id}"
                )));
            }
        }

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
            resource_allowlist: policy.resource_allowlist.clone(),
        };

        let max_ceiling: u128 = std::env::var("MU_MAX_CEILING_MINOR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10_000_000_000);
        let omega = Omega {
            connectors: vec![connector],
            max_ceiling: Amount::from_minor(max_ceiling),
        };

        Ok(Config {
            policy_path: home.join("policy.toml"),
            policy: policy.clone(),
            chain_id,
            mu_path: home.join("mu.bin"),
            log_path: home.join("mu.log"),
            // Sockets live in their own directory: it must be reachable by the
            // agent group, while MU_HOME (keys, journal) stays 0700.
            sock_path: home.join("run").join("mu-daemon.sock"),
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
