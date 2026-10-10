//! Загрузчик `policy.toml` — читает файл, возвращает `Delta`.
//!
//! Формат:
//! ```toml
//! daily_limit_minor = 500_000_000        # 500 USDC
//! confirm_threshold_minor = 100_000_000  # 100 USDC
//!
//! [[whitelist]]
//! address = "0x..."
//! label = "My Service"
//!
//! [[resource_allowlist]]
//! host = "api.coingecko.com"
//! path_prefix = "/api/v3/simple/price"
//! max_price_per_call_minor = 1_000_000   # $1
//! ```

use crate::types::{Delta, ResourceRule, WlEntry};
use mu_common::{Amount, CanonAddress};
use serde::Deserialize;

/// Сирі TOML-поля (serde).
#[derive(Deserialize)]
struct RawConfig {
    daily_limit_minor: u64,
    confirm_threshold_minor: u64,
    #[serde(default)]
    whitelist: Vec<RawWlEntry>,
    #[serde(default)]
    resource_allowlist: Vec<RawResourceRule>,
}

#[derive(Deserialize)]
struct RawWlEntry {
    address: String,
    #[serde(default)]
    label: String,
}

#[derive(Deserialize)]
struct RawResourceRule {
    host: String,
    #[serde(default)]
    path_prefix: String,
    max_price_per_call_minor: u64,
}

/// Завантажити `Delta` з TOML-рядка.
pub fn parse_toml(input: &str, chain_id: u64) -> Result<Delta, String> {
    let raw: RawConfig = toml::from_str(input).map_err(|e| format!("TOML: {e}"))?;

    if raw.daily_limit_minor == 0 {
        return Err("daily_limit_minor must be > 0".into());
    }
    // confirm_threshold може бути 0 (відключений)

    let mut whitelist = Vec::with_capacity(raw.whitelist.len());
    for (i, w) in raw.whitelist.iter().enumerate() {
        let address = CanonAddress::canon(&w.address, chain_id)
            .map_err(|e| format!("whitelist[{i}]: {e:?}"))?;
        if w.label.len() > 64 {
            return Err(format!("whitelist[{i}]: label > 64 chars"));
        }
        whitelist.push(WlEntry {
            address,
            label: w.label.clone(),
        });
    }

    let mut resource_allowlist = Vec::with_capacity(raw.resource_allowlist.len());
    for (i, r) in raw.resource_allowlist.iter().enumerate() {
        if r.host.is_empty() {
            return Err(format!("resource_allowlist[{i}]: empty host"));
        }
        resource_allowlist.push(ResourceRule {
            host: r.host.clone(),
            path_prefix: r.path_prefix.clone(),
            max_price_per_call: Amount::from_minor(r.max_price_per_call_minor as u128),
        });
    }

    Ok(Delta {
        daily_limit: Amount::from_minor(raw.daily_limit_minor as u128),
        whitelist,
        confirm_threshold: Amount::from_minor(raw.confirm_threshold_minor as u128),
        resource_allowlist,
    })
}

/// Завантажити з файлу.
pub fn load_policy(path: &std::path::Path, chain_id: u64) -> Result<Delta, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("read {path:?}: {e}"))?;
    parse_toml(&content, chain_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_config() {
        let toml = r#"
daily_limit_minor = 500_000_000
confirm_threshold_minor = 100_000_000
"#;
        let d = parse_toml(toml, 8453).unwrap();
        assert_eq!(d.daily_limit, Amount::from_minor(500_000_000));
        assert_eq!(d.confirm_threshold, Amount::from_minor(100_000_000));
        assert!(d.whitelist.is_empty());
        assert!(d.resource_allowlist.is_empty());
    }

    #[test]
    fn with_whitelist() {
        let toml = r#"
daily_limit_minor = 1000
confirm_threshold_minor = 0

[[whitelist]]
address = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
label = "Service A"
"#;
        let d = parse_toml(toml, 8453).unwrap();
        assert_eq!(d.whitelist.len(), 1);
        assert_eq!(d.whitelist[0].label, "Service A");
    }

    #[test]
    fn with_resource_allowlist() {
        let toml = r#"
daily_limit_minor = 1000
confirm_threshold_minor = 0

[[whitelist]]
address = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
label = "A"

[[resource_allowlist]]
host = "api.example.com"
path_prefix = "/v1/"
max_price_per_call_minor = 1_000_000
"#;
        let d = parse_toml(toml, 8453).unwrap();
        assert_eq!(d.resource_allowlist.len(), 1);
        assert_eq!(d.resource_allowlist[0].host, "api.example.com");
        assert_eq!(d.resource_allowlist[0].max_price_per_call, Amount::from_minor(1_000_000));
    }

    #[test]
    fn rejects_zero_daily_limit() {
        let toml = "daily_limit_minor = 0\nconfirm_threshold_minor = 0\n";
        assert!(parse_toml(toml, 8453).is_err());
    }

    #[test]
    fn rejects_bad_address() {
        let toml = r#"
daily_limit_minor = 100
confirm_threshold_minor = 0

[[whitelist]]
address = "not-a-hex"
label = "bad"
"#;
        assert!(parse_toml(toml, 8453).is_err());
    }

    #[test]
    fn with_underscores_in_numbers() {
        let toml = r#"
daily_limit_minor = 1_000_000
confirm_threshold_minor = 500_000
"#;
        let d = parse_toml(toml, 8453).unwrap();
        assert_eq!(d.daily_limit, Amount::from_minor(1_000_000));
    }
}
