//! E2E, μ Gate KSG edition: an agent pays a keysingate issuer for a repeated
//! `bare` through x402 on Base Sepolia — the issuer's 402 answer in, through
//! Ω → Δ (whitelist, daily window, resource allowlist) → owner's confirmation →
//! WAL → x402 connector, an `X-PAYMENT` header out. The header is the one
//! keysingate's seller side (`ksg-pay-v2`) accepts: same independent vector
//! (eth-account 0.13.4), checked across the two repositories by
//! `scripts/e2e_ksg_x402.sh`.
use base64::Engine;
use mu_common::{Amount, CanonAddress, Clock, ConnectorId};
use mu_connect::x402::{X402Challenge, X402Connector, BASE_SEPOLIA};
use mu_human::{PayConfirm, Presenter, PresenterChoice};
use mu_log::{Kind, Log};
use mu_policy::ResourceRule;
use mu_runtime::pipeline::{IntentStatus, RtIntent, Runtime};
use mu_runtime::policy::{Delta, Omega};
use mu_vault::backend::SoftVault;
use std::time::Duration;

const ISSUER: &str = "0x209693Bc6afc0C5328bA36FaF03C514EF312287C";
const PAYER: &str = "0x19e7e376e7c213b7e7e7e46cc70a5dd086daff2a";
const RESOURCE: &str = "https://issuer.example/bare/issue";

struct Now(u64);
impl Clock for Now {
    fn now_unix(&self) -> u64 {
        self.0
    }
    fn monotonic(&self) -> std::time::Instant {
        std::time::Instant::now()
    }
}

struct Owner(PresenterChoice);
impl Presenter for Owner {
    fn present(&self, r: &PayConfirm, _b: Duration) -> PresenterChoice {
        assert_eq!(r.amount, Amount::from_minor(100_000), "the owner sees the amount asked");
        self.0
    }
}

fn owner_pk() -> Vec<u8> {
    use p256::ecdsa::SigningKey;
    let sk = SigningKey::from_bytes((&[2u8; 32]).into()).unwrap();
    sk.verifying_key().to_encoded_point(true).as_bytes().to_vec()
}

fn issuer() -> CanonAddress {
    CanonAddress::canon(ISSUER, 84532).unwrap()
}

/// The issuer's 402 answer, as `ksg-pay-v2::payment_required` writes it.
fn answer_402(amount: &str) -> String {
    serde_json::json!({
        "x402Version": 1,
        "error": "X-PAYMENT header is required",
        "accepts": [{
            "scheme": "exact",
            "network": "base-sepolia",
            "maxAmountRequired": amount,
            "resource": RESOURCE,
            "description": "a repeated bare",
            "mimeType": "application/json",
            "payTo": ISSUER.to_lowercase(),
            "maxTimeoutSeconds": 300,
            "asset": "0x036cbd53842c5426634e7929541ec2318f3dcf7e",
            "extra": {"name": "USDC", "version": "2"}
        }]
    })
    .to_string()
}

fn delta(daily: u128, whitelist: Vec<CanonAddress>) -> Delta {
    Delta {
        daily_limit: Amount::from_minor(daily),
        whitelist,
        confirm_threshold: Amount::ZERO,
        resource_allowlist: vec![ResourceRule {
            host: "issuer.example".into(),
            path_prefix: "/bare/issue".into(),
            max_price_per_call: Amount::from_minor(1_000_000),
        }],
    }
}

fn pay(
    owner: PresenterChoice,
    delta: Delta,
    amount: u128,
) -> (IntentStatus, Option<String>, Vec<Kind>) {
    let vault = SoftVault::for_test([1; 32], [2; 32], [0x11; 32]);
    let payer: [u8; 20] = hex::decode(&PAYER[2..]).unwrap().try_into().unwrap();
    let conn = X402Connector::on(BASE_SEPOLIA, payer);
    conn.set_challenge(&X402Challenge::from_402_body(&answer_402(&amount.to_string()), &BASE_SEPOLIA).unwrap());
    let dir = tempfile::tempdir().unwrap();
    let presenter = Owner(owner);
    let clock = Now(1_760_000_100);
    let mut rt = Runtime {
        omega: Omega { connectors: vec![ConnectorId::Crypto], max_ceiling: Amount::from_minor(10_000_000) },
        delta,
        log: Log::open(&dir.path().join("log.mulog"), &vault).unwrap(),
        vault: &vault,
        connector: &conn,
        presenter: &presenter,
        clock: &clock,
        owner_pubkey: owner_pk(),
        human_ttl: Duration::from_secs(5),
    };
    let st = rt.process(&RtIntent {
        recipient: issuer(),
        amount: Amount::from_minor(amount),
        chain_id: 84532,
        agent_id: "ksg-holder-agent".into(),
        connector: ConnectorId::Crypto,
        resource: Some(RESOURCE.into()),
    });
    let kinds = rt.log.entries().iter().map(|e| e.kind.clone()).collect();
    (st, conn.take_header(), kinds)
}

#[test]
fn a_repeated_bare_is_paid_with_the_owners_consent() {
    let (st, header, log) = pay(PresenterChoice::Approve, delta(5_000_000, vec![issuer()]), 100_000);
    // Paid by authorization: the issuer's facilitator settles it, so the
    // reserve stays held until then (RISK-M6-5).
    assert_eq!(st, IntentStatus::ReconcilePending);
    assert!(log.iter().any(|k| matches!(k, Kind::HumanDecision { approved: true, .. })));
    assert!(log.iter().any(|k| matches!(k, Kind::Pending { amount_total: 100_000, .. })));

    let raw = base64::engine::general_purpose::STANDARD.decode(header.expect("a header")).unwrap();
    let j: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    assert_eq!(j["x402Version"], 1);
    assert_eq!(j["network"], "base-sepolia");
    let a = &j["payload"]["authorization"];
    assert_eq!(a["from"], PAYER);
    assert_eq!(a["to"], ISSUER.to_lowercase());
    assert_eq!(a["value"], "100000");
    let after: u64 = a["validAfter"].as_str().unwrap().parse().unwrap();
    let before: u64 = a["validBefore"].as_str().unwrap().parse().unwrap();
    assert!(before - after <= 300 + 60, "the window the issuer allowed, plus clock skew");
}

#[test]
fn the_owner_saying_no_signs_nothing() {
    let (st, header, _) = pay(PresenterChoice::Deny, delta(5_000_000, vec![issuer()]), 100_000);
    assert_eq!(st, IntentStatus::DeniedHuman);
    assert!(header.is_none());
}

#[test]
fn an_issuer_not_in_the_policy_is_not_paid() {
    let (st, header, _) = pay(PresenterChoice::Approve, delta(5_000_000, vec![]), 100_000);
    assert_eq!(st, IntentStatus::DeniedDelta);
    assert!(header.is_none());
}

#[test]
fn the_daily_window_and_the_price_cap_hold() {
    let (st, header, _) = pay(PresenterChoice::Approve, delta(50_000, vec![issuer()]), 100_000);
    assert_eq!(st, IntentStatus::DeniedDelta, "over the daily limit");
    assert!(header.is_none());
    let (st, header, _) = pay(PresenterChoice::Approve, delta(5_000_000, vec![issuer()]), 2_000_000);
    assert_eq!(st, IntentStatus::DeniedDelta, "over the $1 cap for /bare/issue");
    assert!(header.is_none());
}
