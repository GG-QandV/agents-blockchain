//! M6 pipeline risk tests on SoftVault + mock connector.
use mu_common::{Amount, CanonAddress, Clock, ConnectorId};
use mu_connect::{ConnErr, Connector, Fee, Intent as ConnIntent, TxRef, TxStatus};
use mu_human::{PayConfirm, Presenter, PresenterChoice};
use mu_log::{Kind, Log};
use mu_runtime::pipeline::{IntentStatus, RtIntent, Runtime};
use mu_runtime::policy::{Delta, Omega};
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
    CanonAddress::canon("0xabcdef0123456789abcdef0123456789abcdef01000000000000000000000000", 8453).unwrap()
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
    }
}

// ── tests ────────────────────────────────────────────────────────────────

#[test]
fn omega_ceiling_denies_before_anything() {
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let conn = MockConn { exec_result: || panic!("must not execute"), status_result: || TxStatus::Pending, exec_calls: Default::default() };
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 10_000_000, 10_000_000);
    rt.omega.max_ceiling = Amount::from_minor(100);
    // amount 5000 + gas 10 > ceiling 100 → DeniedOmega, execute NOT called
    assert_eq!(rt.process(&intent(5000)), IntentStatus::DeniedOmega);
    assert_eq!(conn.exec_calls.get(), 0);
}

#[test]
fn delta_window_accumulates_and_denies() {
    // RISK-M6-3/M5-4: sequential payments are counted, limit is not exceeded
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let conn = MockConn {
        exec_result: || Ok(TxRef::Real { tx_hash: [7;32], chain_nonce: 0 }),
        status_result: || TxStatus::Settled { block: 1, effective_gas: 1 },
        exec_calls: Default::default(),
    };
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    // limit 250: two payments of (100+10)=110 go through (220), third doesn't (330>250)
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 250, 1_000_000);
    assert!(matches!(rt.process(&intent(100)), IntentStatus::Settled { .. }));
    assert!(matches!(rt.process(&intent(100)), IntentStatus::Settled { .. }));
    assert_eq!(rt.process(&intent(100)), IntentStatus::DeniedDelta);
    assert_eq!(conn.exec_calls.get(), 2); // third never reached money
}

#[test]
fn unknown_keeps_reserve_blocks_next() {
    // MAIN TEST RISK-M6-2/M6-5: Unknown → reserve is held → window is occupied
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let conn = MockConn {
        exec_result: || Err(ConnErr::Unknown("network".into())),
        status_result: || TxStatus::Pending,
        exec_calls: Default::default(),
    };
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 200, 1_000_000);
    // (150+10)=160 → Unknown → ReconcilePending, Pending in log HOLDS 160
    assert_eq!(rt.process(&intent(150)), IntentStatus::ReconcilePending);
    assert_eq!(rt.log.pending().len(), 1);
    // second payment (100+10)=110: 160+110=270 > 200 → DeniedDelta (reserve did NOT evaporate!)
    assert_eq!(rt.process(&intent(100)), IntentStatus::DeniedDelta);
}

#[test]
fn rejected_rolls_back_reserve() {
    // reliable refusal → Pending closed as Failed → window is free
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let conn = MockConn {
        exec_result: || Err(ConnErr::Rejected(mu_connect::RejectReason::InsufficientFunds)),
        status_result: || TxStatus::Pending,
        exec_calls: Default::default(),
    };
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 200, 1_000_000);
    assert_eq!(rt.process(&intent(150)), IntentStatus::Failed);
    assert_eq!(rt.log.pending().len(), 0); // rollback: Pending closed
    // window is free → next one goes through
    let conn2 = MockConn {
        exec_result: || Ok(TxRef::Real { tx_hash: [7;32], chain_nonce: 0 }),
        status_result: || TxStatus::Settled { block: 1, effective_gas: 1 },
        exec_calls: Default::default(),
    };
    rt.connector = &conn2;
    assert!(matches!(rt.process(&intent(150)), IntentStatus::Settled { .. }));
}

#[test]
fn human_threshold_and_denial() {
    // RISK-M3-3: threshold by total (amount+gas); owner denial = no payment
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let conn = MockConn {
        exec_result: || panic!("must not execute after human deny"),
        status_result: || TxStatus::Pending,
        exec_calls: Default::default(),
    };
    let pres = FixedPresenter(PresenterChoice::Deny);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    // threshold 100: amount 101 > 100 → human → Deny (gasless)
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 1_000_000, 100);
    assert_eq!(rt.process(&intent(101)), IntentStatus::DeniedHuman);
    assert_eq!(conn.exec_calls.get(), 0);
}

#[test]
fn human_approved_with_valid_proof_settles() {
    // RISK-M9-2 in pipeline: proof is verified and payment goes through
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    let conn = MockConn {
        exec_result: || Ok(TxRef::Real { tx_hash: [7;32], chain_nonce: 0 }),
        status_result: || TxStatus::Settled { block: 1, effective_gas: 1 },
        exec_calls: Default::default(),
    };
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 1_000_000, 100);
    assert!(matches!(rt.process(&intent(500)), IntentStatus::Settled { .. }));
    // log contains HumanDecision{approved:true}
    let has_approved = rt.log.entries().iter().any(|e| matches!(e.kind, Kind::HumanDecision { approved: true, .. }));
    assert!(has_approved);
}

#[test]
fn broken_biometry_cannot_settle() {
    // RISK-M4-3 → end-to-end: without biometrics Approved is impossible, payment does not go through
    let mut v = SoftVault::for_test([1;32],[2;32],[3;32]);
    v.set_owner_auth(false);
    let conn = MockConn {
        exec_result: || panic!("must not execute"),
        status_result: || TxStatus::Pending,
        exec_calls: Default::default(),
    };
    let pres = FixedPresenter(PresenterChoice::Approve); // owner "hits yes" but the ritual fails
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 1_000_000, 100);
    assert_eq!(rt.process(&intent(500)), IntentStatus::DeniedHuman);
    assert_eq!(conn.exec_calls.get(), 0);
}

#[test]
fn wal_written_before_execute() {
    // RISK-M6-1 (runtime boundary): by the time execute is called, Pending is already in the log.
    // The type-level boundary (compile-fail) is ensured by the privacy of WalWritten.
    let v = SoftVault::for_test([1;32],[2;32],[3;32]);
    // exec_result returns Unknown → Pending will remain and be visible
    let conn = MockConn {
        exec_result: || Err(ConnErr::Unknown("x".into())),
        status_result: || TxStatus::Pending,
        exec_calls: Default::default(),
    };
    let pres = FixedPresenter(PresenterChoice::Approve);
    let clk = FixedClock(10_000);
    let setup = Setup { dir: tempfile::tempdir().unwrap() };
    let mut rt = make_runtime(&v, &conn, &pres, &clk, &setup, 1_000_000, 1_000_000);
    let _ = rt.process(&intent(100));
    assert_eq!(conn.exec_calls.get(), 1);
    // execute was called AND Pending is durable in the log (re-opening the file sees it)
    let reopened = Log::open(&setup.dir.path().join("log.mulog"), &v).unwrap();
    assert_eq!(reopened.pending().len(), 1);
}
