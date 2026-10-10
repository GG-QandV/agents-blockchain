//! μ Gate KSG smoke (test networks only — the wallet is the fixed test key
//! 0x11…11, behind SoftVault, which a release build refuses).
//!
//!   ksg-smoke < 402.json > x-payment.txt
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
    let vault = SoftVault::for_test([1; 32], [2; 32], [0x11; 32]);
    let payer = address_of(&vault.tx_signer().expect("signer")).expect("address");
    let conn = X402Connector::on(BASE_SEPOLIA, payer);
    conn.set_challenge(&ch);
    let pay_to = CanonAddress::canon(&ch.pay_to, BASE_SEPOLIA.chain_id).expect("payTo");
    let (host, path) = split_resource(&ch.resource);
    let owner_pk = {
        use p256::ecdsa::SigningKey;
        let sk = SigningKey::from_bytes((&[2u8; 32]).into()).unwrap();
        sk.verifying_key().to_encoded_point(true).as_bytes().to_vec()
    };
    let dir = tempfile::tempdir().expect("tmp");
    let mut rt = Runtime {
        omega: Omega { connectors: vec![ConnectorId::Crypto], max_ceiling: Amount::from_minor(10_000_000) },
        delta: Delta {
            daily_limit: Amount::from_minor(5_000_000),
            whitelist: vec![pay_to],
            confirm_threshold: Amount::ZERO,
            resource_allowlist: vec![ResourceRule {
                host: host.into(),
                path_prefix: path.into(),
                max_price_per_call: Amount::from_minor(1_000_000),
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
