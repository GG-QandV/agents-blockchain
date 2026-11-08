//! P11g (Workflow μ Gate §3, §4): the policy template `ksg-pro-anchor` — the owner's rules for
//! anchors, read from a TOML file the owner can open in an editor.
//!
//! Everything is checked **before** the daemon starts: a value that makes no sense is a refusal to
//! load (never a silent default), and a key the template does not know is a refusal too — a typo
//! in a limit must not become "no limit". A capsule entry that still carries the placeholder of the
//! shipped template is skipped with a warning (nobody is allowed to pay until the owner names a
//! capsule): the template as shipped loads, and pays for nothing.
//!
//! The rules are [`crate::anchor_rules::AnchorRules`]; the memo kinds allowed are those of core v2
//! (`ksg:bs:v1`, `ksg:st:v1`); the old memo of core v1 (`ksg:cp:v1`) is refused by name.

use serde::Deserialize;

use crate::anchor::{SEAL_MEMO_TAG, STATUS_MEMO_TAG};
use crate::anchor_rules::AnchorRules;

/// A capsule the owner allows to pay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllowedCapsule {
    /// The capsule's container multihash (`ksg-capsule status`, field `id`).
    pub id: String,
    /// The owner's name for it.
    pub label: String,
}

/// What a template says.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorTemplate {
    /// `localnet`, `devnet`, `testnet` or `mainnet-beta`: the cluster anchors are paid on.
    pub cluster: String,
    /// The owner's rules.
    pub rules: AnchorRules,
    /// The memo kinds that may be paid for.
    pub memo_kinds: Vec<String>,
    /// The capsules allowed, with the owner's labels (the rules hold their ids).
    pub capsules: Vec<AllowedCapsule>,
    /// What the owner should know about the template as it stands (a placeholder not replaced).
    pub warnings: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Raw {
    cluster: String,
    fee_cap_lamports: u64,
    daily_budget_lamports_per_capsule: u64,
    daily_budget_lamports_total: u64,
    memo_kinds: Vec<String>,
    #[serde(default)]
    capsules: Vec<RawCapsule>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCapsule {
    id: String,
    #[serde(default)]
    label: String,
}

/// The fee of one signature, lamports: a cap below it can pay nothing.
const BASE_FEE_LAMPORTS: u64 = 5_000;
/// Clusters an anchor can go to.
const CLUSTERS: [&str; 4] = ["localnet", "devnet", "testnet", "mainnet-beta"];
/// What a capsule's identifier looks like: a sha-256 multihash in hex.
fn is_multihash(id: &str) -> bool {
    id.len() == 68 && id.starts_with("1220") && id.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

/// Reads a template.
///
/// # Errors
///
/// The first thing that does not hold, in words: a key unknown, a value out of reason, a memo kind
/// that is not core v2's, a capsule that is not a multihash.
pub fn parse(input: &str) -> Result<AnchorTemplate, String> {
    let raw: Raw = toml::from_str(input).map_err(|e| format!("TOML: {e}"))?;
    if !CLUSTERS.contains(&raw.cluster.as_str()) {
        return Err(format!("cluster {:?} is none of {CLUSTERS:?}", raw.cluster));
    }
    if raw.fee_cap_lamports < BASE_FEE_LAMPORTS {
        return Err(format!("fee_cap_lamports {} is under the fee of one signature ({BASE_FEE_LAMPORTS}): no anchor could be paid", raw.fee_cap_lamports));
    }
    if raw.daily_budget_lamports_per_capsule < raw.fee_cap_lamports {
        return Err("daily_budget_lamports_per_capsule is under one anchor's fee cap".into());
    }
    if raw.daily_budget_lamports_total < raw.daily_budget_lamports_per_capsule {
        return Err("daily_budget_lamports_total is under the budget of one capsule".into());
    }
    if raw.memo_kinds.is_empty() {
        return Err("memo_kinds is empty: nothing could be anchored".into());
    }
    for kind in &raw.memo_kinds {
        if kind == "ksg:cp:v1" {
            return Err("memo kind ksg:cp:v1 is core v1's: refused".into());
        }
        if kind != SEAL_MEMO_TAG && kind != STATUS_MEMO_TAG {
            return Err(format!("memo kind {kind:?} is none of {SEAL_MEMO_TAG:?}, {STATUS_MEMO_TAG:?}"));
        }
    }
    let mut capsules = Vec::new();
    let mut warnings = Vec::new();
    for (i, c) in raw.capsules.iter().enumerate() {
        if c.id.starts_with("PLACEHOLDER") {
            warnings.push(format!("W-CAP-01: capsules[{i}] is the template's placeholder — skipped: no capsule is allowed to pay yet"));
            continue;
        }
        if !is_multihash(&c.id) {
            return Err(format!("capsules[{i}]: {:?} is not a capsule identifier (1220 and 64 lowercase hex digits)", c.id));
        }
        if capsules.iter().any(|x: &AllowedCapsule| x.id == c.id) {
            return Err(format!("capsules[{i}]: the capsule is listed twice"));
        }
        if c.label.len() > 64 {
            return Err(format!("capsules[{i}]: label is longer than 64 characters"));
        }
        capsules.push(AllowedCapsule { id: c.id.clone(), label: c.label.clone() });
    }
    if capsules.is_empty() {
        warnings.push("W-CAP-02: no capsule is allowed to pay: every anchor will be denied (not_allowed)".into());
    }
    Ok(AnchorTemplate {
        cluster: raw.cluster,
        rules: AnchorRules {
            per_anchor_cap_lamports: raw.fee_cap_lamports,
            daily_budget_per_capsule_lamports: raw.daily_budget_lamports_per_capsule,
            daily_budget_total_lamports: raw.daily_budget_lamports_total,
            allowed_capsules: capsules.iter().map(|c| c.id.clone()).collect(),
        },
        memo_kinds: raw.memo_kinds,
        capsules,
        warnings,
    })
}

/// Reads a template from a file.
///
/// # Errors
///
/// The file does not read, or [`parse`] refuses it.
pub fn load(path: &std::path::Path) -> Result<AnchorTemplate, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    parse(&text).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = r#"
cluster = "localnet"
fee_cap_lamports = 20_000
daily_budget_lamports_per_capsule = 5_000_000
daily_budget_lamports_total = 50_000_000
memo_kinds = ["ksg:bs:v1", "ksg:st:v1"]

[[capsules]]
id = "1220aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
label = "the agent's capsule"
"#;

    fn with(from: &str, to: &str) -> String {
        assert!(GOOD.contains(from), "{from}");
        GOOD.replacen(from, to, 1)
    }

    #[test]
    fn a_good_template_gives_the_owners_rules() {
        let t = parse(GOOD).expect("loads");
        assert_eq!(t.cluster, "localnet");
        assert_eq!(t.rules.per_anchor_cap_lamports, 20_000);
        assert_eq!(t.rules.daily_budget_total_lamports, 50_000_000);
        assert_eq!(t.rules.allowed_capsules.len(), 1);
        assert_eq!(t.capsules[0].label, "the agent's capsule");
        assert!(t.warnings.is_empty());
    }

    #[test]
    fn the_template_as_shipped_loads_and_pays_for_nothing() {
        let shipped = include_str!("../../policy-templates/ksg-pro-anchor.toml");
        let t = parse(shipped).expect("the shipped template loads");
        assert!(t.rules.allowed_capsules.is_empty(), "no capsule is allowed until the owner names one");
        assert!(t.warnings.iter().any(|w| w.starts_with("W-CAP-01")), "{:?}", t.warnings);
        assert!(t.warnings.iter().any(|w| w.starts_with("W-CAP-02")), "{:?}", t.warnings);
        assert_eq!(t.memo_kinds, ["ksg:bs:v1", "ksg:st:v1"]);
    }

    #[test]
    fn a_wrong_value_is_a_refusal_to_load_never_a_default() {
        let bad = [
            (with("cluster = \"localnet\"", "cluster = \"mainet-beta\""), "cluster"),
            (with("fee_cap_lamports = 20_000", "fee_cap_lamports = 4_999"), "fee of one signature"),
            (with("fee_cap_lamports = 20_000", "fee_cap_lamports = 9_000_000"), "under one anchor's fee cap"),
            (with("daily_budget_lamports_total = 50_000_000", "daily_budget_lamports_total = 4_000_000"), "under the budget of one capsule"),
            (with("fee_cap_lamports = 20_000", "fee_cap_lamports = 20_000\nrefill_amount_lamports = 5_000_000"), "refill_amount_lamports"),
            (with("memo_kinds = [\"ksg:bs:v1\", \"ksg:st:v1\"]", "memo_kinds = []"), "memo_kinds is empty"),
            (with("\"ksg:st:v1\"", "\"ksg:cp:v1\""), "core v1"),
            (with("\"ksg:st:v1\"", "\"ksg:zz:v1\""), "none of"),
            (with("id = \"1220aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"", "id = \"1220ABC\""), "not a capsule identifier"),
        ];
        for (text, why) in bad {
            let err = parse(&text).expect_err(why);
            assert!(err.contains(why), "{why}: {err}");
        }
    }

    #[test]
    fn a_typo_in_a_key_is_not_silently_a_missing_limit() {
        let err = parse(&with("fee_cap_lamports", "fee_cap_lamport")).expect_err("an unknown key");
        assert!(err.contains("fee_cap_lamport"), "{err}");
        let err = parse(&format!("{GOOD}\nextra = 1\n")).expect_err("an unknown key at the top");
        assert!(err.contains("extra") || err.contains("unknown"), "{err}");
    }

    #[test]
    fn a_capsule_listed_twice_is_refused() {
        let twice = format!("{GOOD}\n[[capsules]]\nid = \"1220aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n");
        assert!(parse(&twice).expect_err("twice").contains("twice"));
    }
}
