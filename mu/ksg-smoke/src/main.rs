//! μ Gate KSG smoke (test networks only — SoftVault, which a release build
//! refuses). The wallet is the fixed test key 0x11…11, or, for a live payment
//! on Base Sepolia, the key in the file `MU_PAYER_KEY_FILE` (64 hex chars,
//! mode 600; the key is never printed). The owner's key comes from
//! `MU_OWNER_KEY_FILE` (same format), the fixed test key otherwise.
//!
//!   ksg-smoke < 402.json > x-payment.txt
//!   MU_PAYER_KEY_FILE=~/.secrets/keysingate/base-sepolia/payer.hex ksg-smoke < 402.json
//!
//! Limits are read from the environment, in minor units: `MU_MAX_CEILING_MINOR`
//! (10 000 000), `MU_DAILY_LIMIT_MINOR` (5 000 000),
//! `MU_CONFIRM_THRESHOLD_MINOR` (0), `MU_MAX_PRICE_PER_CALL_MINOR` (1 000 000).
//!
//! Reads an issuer's 402 answer, pays it through Ω → Δ → owner (auto-approve,
//! threshold 0) → WAL → x402 on Base Sepolia, prints the `X-PAYMENT` header.
//! Exit 0 and the header, or exit 1 and the pipeline's verdict.
use mu_common::{Amount, CanonAddress, Clock, ConnectorId};
use mu_connect::x402::{address_of, X402Challenge, X402Connector, BASE_SEPOLIA};
use mu_human::{PayConfirm, Presenter, PresenterChoice};
use mu_log::Log;
use mu_policy::ResourceRule;
use mu_runtime::pipeline::{split_resource, RtIntent, Runtime};
use mu_runtime::policy::{Delta, Omega};
use mu_vault::{backend::SoftVault, Vault};
use std::io::Read;
use std::time::Duration;
use zeroize::Zeroize;

struct SystemClock;
impl Clock for SystemClock {
    fn now_unix(&self) -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    }
    fn monotonic(&self) -> std::time::Instant {
        std::time::Instant::now()
    }
}

struct AutoApprove;
impl Presenter for AutoApprove {
    fn present(&self, r: &PayConfirm, _b: Duration) -> PresenterChoice {
        eprintln!("owner confirms: {} minor units to {}", r.amount.minor(), r.recipient.redacted());
        PresenterChoice::Approve
    }
}

/// The payer's wallet key: the file named by `MU_PAYER_KEY_FILE`, else the
/// fixed test key. A file readable by others is refused.
fn wallet_key() -> [u8; 32] {
    key_from_env("MU_PAYER_KEY_FILE").unwrap_or([0x11; 32])
}

/// The owner's key: the file named by `MU_OWNER_KEY_FILE`, else the fixed
/// test key. Same format and mode rules as the payer key.
fn owner_key() -> [u8; 32] {
    key_from_env("MU_OWNER_KEY_FILE").unwrap_or([0x02; 32])
}

/// Reads a 32-byte key file named by `var`; 64 hex characters, mode 600.
/// `None` when the variable is not set.
fn key_from_env(var: &str) -> Option<[u8; 32]> {
    let Ok(path) = std::env::var(var) else {
        return None;
    };
    let fail = |why: &str| -> ! {
        eprintln!("{var}: {why}");
        std::process::exit(2)
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&path).unwrap_or_else(|_| fail("cannot read")).permissions().mode();
        if mode & 0o077 != 0 {
            fail("must be mode 600 (readable by the owner only)");
        }
    }
    let mut text = std::fs::read_to_string(&path).unwrap_or_else(|_| fail("cannot read"));
    let key = parse_key(&text);
    text.zeroize();
    Some(key.unwrap_or_else(|| fail("expected 32 bytes as 64 hex characters")))
}

/// A minor-units limit from `var`, or `default` when the variable is unset.
fn limit_from_env(var: &str, default: u128) -> u128 {
    match std::env::var(var) {
        Ok(v) => v.trim().parse().unwrap_or_else(|_| {
            eprintln!("{var}: expected an integer in minor units");
            std::process::exit(2)
        }),
        Err(_) => default,
    }
}

fn parse_key(text: &str) -> Option<[u8; 32]> {
    let t = text.trim();
    let t = t.strip_prefix("0x").unwrap_or(t).as_bytes();
    if t.len() != 64 {
        return None;
    }
    let nib = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut key = [0u8; 32];
    for (i, b) in key.iter_mut().enumerate() {
        *b = (nib(t[2 * i])? << 4) | nib(t[2 * i + 1])?;
    }
    if key == [0; 32] {
        return None;
    }
    Some(key)
}

fn main() {
    let mut body = String::new();
    std::io::stdin().read_to_string(&mut body).expect("stdin");
    let ch = match X402Challenge::from_402_body(&body, &BASE_SEPOLIA) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("not a 402 answer we pay: {e:?}");
            std::process::exit(1)
        }
    };
    let mut key = wallet_key();
    let mut owner = owner_key();
    let vault = SoftVault::for_test([1; 32], owner, key);
    key.zeroize();
    let payer = address_of(&vault.tx_signer().expect("signer")).expect("address");
    let conn = X402Connector::on(BASE_SEPOLIA, payer);
    conn.set_challenge(&ch);
    let pay_to = CanonAddress::canon(&ch.pay_to, BASE_SEPOLIA.chain_id).expect("payTo");
    let (host, path) = split_resource(&ch.resource);
    let owner_pk = {
        use p256::ecdsa::SigningKey;
        let sk = SigningKey::from_bytes((&owner).into()).unwrap();
        sk.verifying_key().to_encoded_point(true).as_bytes().to_vec()
    };
    owner.zeroize();
    let max_ceiling = limit_from_env("MU_MAX_CEILING_MINOR", 10_000_000);
    let daily_limit = limit_from_env("MU_DAILY_LIMIT_MINOR", 5_000_000);
    let confirm_threshold = limit_from_env("MU_CONFIRM_THRESHOLD_MINOR", 0);
    let max_price_per_call = limit_from_env("MU_MAX_PRICE_PER_CALL_MINOR", 1_000_000);
    let dir = tempfile::tempdir().expect("tmp");
    let mut rt = Runtime {
        omega: Omega { connectors: vec![ConnectorId::Crypto], max_ceiling: Amount::from_minor(max_ceiling) },
        delta: Delta {
            daily_limit: Amount::from_minor(daily_limit),
            whitelist: vec![pay_to],
            confirm_threshold: Amount::from_minor(confirm_threshold),
            resource_allowlist: vec![ResourceRule {
                host: host.into(),
                path_prefix: path.into(),
                max_price_per_call: Amount::from_minor(max_price_per_call),
            }],
        },
        log: Log::open(&dir.path().join("log.mulog"), &vault).expect("log"),
        vault: &vault,
        connector: &conn,
        presenter: &AutoApprove,
        clock: &SystemClock,
        owner_pubkey: owner_pk,
        human_ttl: Duration::from_secs(5),
    };
    let amount: u128 = ch.max_amount_required.parse().expect("amount");
    let st = rt.process(&RtIntent {
        recipient: pay_to,
        amount: Amount::from_minor(amount),
        chain_id: BASE_SEPOLIA.chain_id,
        agent_id: "ksg-smoke".into(),
        connector: ConnectorId::Crypto,
        resource: Some(ch.resource.clone()),
    });
    match conn.take_header() {
        Some(h) => {
            eprintln!("pipeline: {st:?}");
            println!("{h}");
        }
        None => {
            eprintln!("not paid: {st:?}");
            std::process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_key;

    #[test]
    fn a_key_file_is_64_hex_characters() {
        let k = parse_key(&format!("0x{}\n", "ab".repeat(32))).unwrap();
        assert_eq!(k, [0xab; 32]);
        assert!(parse_key(&"ab".repeat(31)).is_none(), "too short");
        assert!(parse_key(&"zz".repeat(32)).is_none(), "not hex");
        assert!(parse_key(&"00".repeat(32)).is_none(), "the zero key");
    }
}
