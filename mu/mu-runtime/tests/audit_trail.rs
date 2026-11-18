//! M6 pipeline risk tests on SoftVault + mock connector.
use mu_common::{Amount, CanonAddress, Clock, ConnectorId, Hash32};
use mu_connect::{ConnErr, Connector, Fee, Intent as ConnIntent, TxRef, TxStatus};
use mu_human::{PayConfirm, Presenter, PresenterChoice};
use mu_log::{deny, Kind, Log};
use mu_runtime::pipeline::{IntentStatus, RtIntent, Runtime};
use mu_runtime::policy::{policy_hash, Delta, Omega};
use mu_vault::backend::SoftVault;
use std::time::Duration;

// ── mocks ─────────────────────────────────────────────────────────────────
struct MockConn {
    exec_result: fn() -> Result<TxRef, ConnErr>,
    status_result: fn() -> TxStatus,
    exec_calls: std::cell::Cell<u32>,
}
impl Connector for MockConn {
    fn quote(&self, _i: &ConnIntent) -> Result<Fee, ConnErr> {
        Ok(Fee { gas_estimate: Amount::from_minor(10) })
    }
    fn execute(&self, _i: &ConnIntent, _s: mu_vault::TxSigner) -> Result<TxRef, ConnErr> {
        self.exec_calls.set(self.exec_calls.get() + 1);
        (self.exec_result)()
    }
    fn status(&self, _r: &TxRef) -> Result<TxStatus, ConnErr> {
        Ok((self.status_result)())
    }
}

struct FixedClock(u64);
impl Clock for FixedClock {
    fn now_unix(&self) -> u64 { self.0 }
    fn monotonic(&self) -> std::time::Instant { std::time::Instant::now() }
}

struct FixedPresenter(PresenterChoice);
impl Presenter for FixedPresenter {
    fn present(&self, _r: &PayConfirm, _b: Duration) -> PresenterChoice { self.0 }
}

fn recipient() -> CanonAddress {
    CanonAddress::canon("0xabcdef0123456789abcdef0123456789abcdef01", 8453).unwrap()
}
fn owner_pubkey() -> Vec<u8> {
    use p256_helper::*;
    owner_pk()
}
mod p256_helper {
    pub fn owner_pk() -> Vec<u8> {
        use p256::ecdsa::SigningKey;
        let sk = SigningKey::from_bytes((&[2u8; 32]).into()).unwrap();
        sk.verifying_key().to_encoded_point(true).as_bytes().to_vec()
    }
}

struct Setup {
    dir: tempfile::TempDir,
}
fn make_runtime<'a>(
    vault: &'a SoftVault,
    conn: &'a MockConn,
    presenter: &'a FixedPresenter,
    clock: &'a FixedClock,
    setup: &Setup,
    daily_limit: u128,
    threshold: u128,
) -> Runtime<'a> {
    let log = Log::open(&setup.dir.path().join("log.mulog"), vault).unwrap();
    Runtime {
        omega: Omega { connectors: vec![ConnectorId::Crypto], max_ceiling: Amount::from_minor(1_000_000) },
        delta: Delta {
            daily_limit: Amount::from_minor(daily_limit),
            whitelist: vec![recipient()],
            confirm_threshold: Amount::from_minor(threshold),
            resource_allowlist: vec![],
        },
        log,
        vault,
        connector: conn,
        presenter,
        clock,
        owner_pubkey: owner_pubkey(),
        human_ttl: Duration::from_secs(5),
    }
}
fn intent(amount: u128) -> RtIntent {
    RtIntent {
        recipient: recipient(),
        amount: Amount::from_minor(amount),
        chain_id: 8453,
        agent_id: "agent-1".into(),
        connector: ConnectorId::Crypto,
        resource: None,
    }
}

// ── audit-trail ladder: rungs 2–4 (rung 1 is mu-log/tests/quoted_log.rs) ───

fn ok_conn() -> MockConn {
    MockConn {
        exec_result: || Ok(TxRef::Real { tx_hash: [7; 32], chain_nonce: 0 }),
        status_result: || TxStatus::Settled { block: 1, effective_gas: 1 },
        exec_calls: Default::default(),
    }
}
fn kinds(rt: &Runtime) -> Vec<Kind> { rt.log.entries().iter().map(|e| e.kind.clone()).collect() }

/// Rung 2: every denial is Quoted → Denied* → DenyReason, with the rule and the cap it met.
#[test]
fn rung2_each_denial_records_quote_rule_and_cap() {
    let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
    let conn = ok_conn();
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);

    // ceiling
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 10_000_000, 10_000_000);
    rt.omega.max_ceiling = Amount::from_minor(100);
    assert_eq!(rt.process(&intent(5000)), IntentStatus::DeniedOmega);
    let k = kinds(&rt);
    assert!(matches!(&k[0], Kind::Quoted { amount: 5000, gas_est: 10, .. }));
    assert!(matches!(k[1], Kind::DeniedOmega { .. }));
    assert_eq!(k[2], Kind::DenyReason { intent_hash: intent(5000).hash(), code: deny::CEILING_EXCEEDED, total: 5010, limit: 100 });

    // whitelist
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 10_000_000, 10_000_000);
    rt.delta.whitelist.clear();
    assert_eq!(rt.process(&intent(100)), IntentStatus::DeniedDelta);
    assert!(matches!(kinds(&rt)[2], Kind::DenyReason { code: deny::NOT_WHITELISTED, total: 110, .. }));

    // window
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 150, 1_000_000);
    assert!(matches!(rt.process(&intent(100)), IntentStatus::Settled { .. }));
    assert_eq!(rt.process(&intent(100)), IntentStatus::DeniedDelta);
    assert!(matches!(rt.log.entries().last().unwrap().kind, Kind::DenyReason { code: deny::WINDOW_EXCEEDED, total: 110, limit: 150, .. }));

    // resource price cap
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 10_000_000, 10_000_000);
    rt.delta.resource_allowlist = vec![mu_policy::ResourceRule {
        host: "issuer.example".into(),
        path_prefix: "/bare".into(),
        max_price_per_call: Amount::from_minor(50),
    }];
    let mut i = intent(100);
    i.resource = Some("https://issuer.example/bare/issue".into());
    assert_eq!(rt.process(&i), IntentStatus::DeniedDelta);
    let k = kinds(&rt);
    assert!(matches!(&k[0], Kind::Quoted { resource, .. } if resource == "https://issuer.example/bare/issue"));
    assert!(matches!(k[2], Kind::DenyReason { code: deny::RESOURCE_DENIED, total: 110, limit: 50, .. }));
    assert_eq!(conn.exec_calls.get(), 1); // only the window rung's first payment reached money
}

/// Rung 3: a payment's log reads Quoted → Pending → Settled; the quote is written before money.
#[test]
fn rung3_paid_intent_logs_quote_before_reserve_and_settlement() {
    let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
    let conn = ok_conn();
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 10_000_000, 10_000_000);
    let i = intent(1234);
    assert!(matches!(rt.process(&i), IntentStatus::Settled { .. }));
    let k = kinds(&rt);
    assert_eq!(k.len(), 3);
    assert_eq!(
        k[0],
        Kind::Quoted {
            intent_hash: i.hash(),
            recipient: *recipient().bytes(),
            chain_id: 8453,
            connector: 1,
            amount: 1234,
            gas_est: 10,
            resource: String::new(),
            policy_hash: policy_hash(&rt.omega, &rt.delta),
        }
    );
    assert!(matches!(k[1], Kind::Pending { amount_total: 1244, .. }));
    assert!(matches!(k[2], Kind::Settled { .. }));
}

/// Rung 4: a reader with only the log and the policy re-derives every verdict.
/// For each Quoted the next outcome entry must equal what Ω/Δ say about the logged quote.
fn recheck(entries: &[Kind], omega: &Omega, delta: &Delta) -> Result<(), String> {
    use mu_runtime::pipeline::split_resource;
    use mu_runtime::policy::{omega_check, resource_check};
    let mut spent: u128 = 0; // window reconstructed from Settled/open Pending (all inside 24h here)
    for (n, k) in entries.iter().enumerate() {
        let Kind::Quoted { amount, gas_est, recipient, chain_id, connector: _, resource, policy_hash: ph, .. } = k else { continue };
        if *ph != policy_hash(omega, delta) { return Err(format!("#{n}: logged under another policy")); }
        let total = amount + gas_est;
        let mut verdict = omega_check(Amount::from_minor(*amount), Amount::from_minor(*gas_est), ConnectorId::Crypto, omega)
            .map(|_| ()).map_err(|_| "omega");
        if verdict.is_ok() {
            let wl = delta.whitelist.iter().any(|a| a.bytes() == recipient && a.chain_id() == *chain_id);
            let win = spent + total <= delta.daily_limit.minor();
            let res = resource.is_empty() || {
                let (h, p) = split_resource(resource);
                resource_check(h, p, Amount::from_minor(total), delta).is_ok()
            };
            if !(wl && win && res) { verdict = Err("delta"); }
        }
        let next = entries[n + 1..].iter().find(|e| {
            matches!(e, Kind::Pending { .. } | Kind::DeniedOmega { .. } | Kind::DeniedDelta { .. })
        });
        match (verdict, next) {
            (Ok(()), Some(Kind::Pending { amount_total, .. })) => {
                if *amount_total != total { return Err(format!("#{n}: reserved {amount_total}, quoted {total}")); }
                spent += total;
            }
            (Err("omega"), Some(Kind::DeniedOmega { .. })) | (Err("delta"), Some(Kind::DeniedDelta { .. })) => {}
            (v, got) => return Err(format!("#{n}: policy says {v:?}, log has {got:?}")),
        }
    }
    Ok(())
}

#[test]
fn rung4_log_alone_proves_the_gate_held() {
    let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
    let conn = ok_conn();
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 250, 1_000_000);
    rt.process(&intent(100)); // settled
    rt.process(&intent(100)); // settled
    rt.process(&intent(100)); // window exceeded

    let k = kinds(&rt);
    recheck(&k, &rt.omega, &rt.delta).expect("log is consistent with the policy in force");

    // a forged outcome (payment where the policy says deny) is caught
    let mut forged = k.clone();
    let i = forged.iter().rposition(|e| matches!(e, Kind::DeniedDelta { .. })).unwrap();
    forged[i] = Kind::Pending { intent_hash: Hash32([0; 32]), amount_total: 110, chain_nonce: 0 };
    assert!(recheck(&forged, &rt.omega, &rt.delta).is_err());

    // the same log read under a looser policy is flagged: the hash pins the rules that applied
    let mut other = make_runtime(&v, &conn, &pres, &clk, &Setup { dir: tempfile::tempdir().unwrap() }, 10_000, 1_000_000);
    other.delta.whitelist.push(recipient());
    assert!(recheck(&k, &other.omega, &other.delta).is_err());
}

/// Review fixes: intent_hash binds connector+resource; policy_hash ignores connector order;
/// oversized resource is refused; human denial leaves a reason.
#[test]
fn intent_hash_binds_resource_and_policy_hash_ignores_connector_order() {
    let mut a = intent(100);
    let mut b = intent(100);
    b.resource = Some("https://issuer.example/bare/issue".into());
    assert_ne!(a.hash(), b.hash());
    a.connector = ConnectorId::BankStub;
    assert_ne!(a.hash(), intent(100).hash());

    let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
    let (conn, pres, clk) = (ok_conn(), FixedPresenter(PresenterChoice::Approve), FixedClock(1));
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 1, 1);
    rt.omega.connectors = vec![ConnectorId::Crypto, ConnectorId::BankStub];
    let h1 = policy_hash(&rt.omega, &rt.delta);
    rt.omega.connectors = vec![ConnectorId::BankStub, ConnectorId::Crypto];
    assert_eq!(h1, policy_hash(&rt.omega, &rt.delta));
    rt.omega.connectors = vec![ConnectorId::Crypto];
    assert_ne!(h1, policy_hash(&rt.omega, &rt.delta));
}

#[test]
fn oversized_resource_is_refused_before_any_record_or_money() {
    let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
    let (conn, pres, clk) = (ok_conn(), FixedPresenter(PresenterChoice::Approve), FixedClock(10_000));
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 10_000_000, 10_000_000);
    let mut i = intent(100);
    i.resource = Some("h/".to_string() + &"a".repeat(70_000));
    assert_eq!(rt.process(&i), IntentStatus::Failed);
    assert_eq!(conn.exec_calls.get(), 0);
    assert!(rt.log.entries().is_empty());
}

#[test]
fn human_denial_leaves_a_reason() {
    let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
    let (conn, pres, clk) = (ok_conn(), FixedPresenter(PresenterChoice::Deny), FixedClock(10_000));
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 1_000_000, 100);
    assert_eq!(rt.process(&intent(95)), IntentStatus::DeniedHuman);
    assert!(matches!(
        rt.log.entries().last().unwrap().kind,
        Kind::DenyReason { code: deny::HUMAN_DENIED, total: 105, limit: 100, .. }
    ));
}
