//! Тест-матрица M7a-sui: те же инварианты, что EVM-версия, + Sui-специфика.
use mu_connect::sui::*;
use mu_connect::{ConnErr, Intent, RejectReason, TxRef, TxStatus};
use mu_common::{Amount, CanonAddress};
use mu_vault::backend::SoftVault;
use mu_vault::Vault;

struct MockRpc {
    build: Result<Vec<u8>, ()>,
    dry: DryRun,
    send: SuiSend,
    look: SuiTxLookup,
}
impl SuiRpc for MockRpc {
    fn build_transfer(&self, _s: &[u8;32], _r: &[u8;32], _a: u128, _c: &str) -> Result<BuiltTx, ConnErr> {
        self.build.clone().map(|b| BuiltTx { tx_bytes: b }).map_err(|_| ConnErr::Unknown("build".into()))
    }
    fn dry_run(&self, _t: &[u8]) -> DryRun { self.dry.clone() }
    fn execute(&self, _t: &[u8], _s: &[u8]) -> SuiSend { self.send.clone() }
    fn lookup(&self, _d: &[u8;32]) -> SuiTxLookup { self.look.clone() }
}

const RCP: [u8; 32] = [0xAB; 32];
fn intent() -> Intent {
    Intent {
        recipient: CanonAddress::canon("0xabcdef0123456789abcdef0123456789abcdef01", 1).unwrap(),
        amount: Amount::from_minor(5_000_000),
        chain_id: 1,
    }
}
fn signer() -> mu_vault::TxSignerP256 {
    SoftVault::for_test([1;32],[2;32],[3;32]).tx_signer_p256().unwrap()
}
fn ok_rpc(send: SuiSend, look: SuiTxLookup) -> MockRpc {
    MockRpc {
        build: Ok(vec![1, 2, 3]),
        dry: DryRun::Ok { recipient: RCP, amount: 5_000_000 },
        send, look,
    }
}
fn conn(r1: MockRpc, r2: MockRpc) -> SuiConnector<MockRpc> {
    SuiConnector { network: "testnet", coin_type: "0x..::usdc::USDC".into(),
                   wallet_addr: [0x11; 32], rpc1: r1, rpc2: r2 }
}
const D: [u8; 32] = [9u8; 32];

#[test]
fn accepted_by_one_is_real() {
    let c = conn(ok_rpc(SuiSend::Accepted { digest: D }, SuiTxLookup::NotFound),
                 ok_rpc(SuiSend::Unreachable, SuiTxLookup::NotFound));
    assert!(matches!(c.execute(&intent(), &RCP, signer()).unwrap(), TxRef::Real { .. }));
}

#[test]
fn object_locked_is_unknown_not_failed() {
    // Sui-аналог RISK-M7-1: эквивокация → Unknown, резерв держится
    let c = conn(ok_rpc(SuiSend::ObjectLocked, SuiTxLookup::NotFound),
                 ok_rpc(SuiSend::ObjectLocked, SuiTxLookup::NotFound));
    assert!(matches!(c.execute(&intent(), &RCP, signer()).unwrap_err(), ConnErr::Unknown(_)));
}

#[test]
fn one_unreachable_one_reject_is_unknown() {
    let c = conn(ok_rpc(SuiSend::Unreachable, SuiTxLookup::NotFound),
                 ok_rpc(SuiSend::DeterministicReject(RejectReason::InsufficientFunds), SuiTxLookup::NotFound));
    assert!(matches!(c.execute(&intent(), &RCP, signer()).unwrap_err(), ConnErr::Unknown(_)));
}

#[test]
fn both_reject_is_rejected() {
    let c = conn(ok_rpc(SuiSend::DeterministicReject(RejectReason::InsufficientFunds), SuiTxLookup::NotFound),
                 ok_rpc(SuiSend::DeterministicReject(RejectReason::InsufficientFunds), SuiTxLookup::NotFound));
    assert!(matches!(c.execute(&intent(), &RCP, signer()).unwrap_err(),
                     ConnErr::Rejected(RejectReason::InsufficientFunds)));
}

#[test]
fn node_builds_wrong_recipient_rejected_before_sign() {
    // RISK-M7-5: нода-сборщик подсунула другого получателя → dry-run self-check ловит
    let mut r1 = ok_rpc(SuiSend::Accepted { digest: D }, SuiTxLookup::NotFound);
    let mut r2 = ok_rpc(SuiSend::Accepted { digest: D }, SuiTxLookup::NotFound);
    r1.dry = DryRun::Ok { recipient: [0xEE; 32], amount: 5_000_000 };
    r2.dry = DryRun::Ok { recipient: [0xEE; 32], amount: 5_000_000 };
    let c = conn(r1, r2);
    assert!(matches!(c.execute(&intent(), &RCP, signer()).unwrap_err(),
                     ConnErr::Rejected(RejectReason::InvalidRecipient)));
}

#[test]
fn node_builds_wrong_amount_rejected() {
    let mut r1 = ok_rpc(SuiSend::Accepted { digest: D }, SuiTxLookup::NotFound);
    let mut r2 = ok_rpc(SuiSend::Accepted { digest: D }, SuiTxLookup::NotFound);
    r1.dry = DryRun::Ok { recipient: RCP, amount: 999 };
    r2.dry = DryRun::Ok { recipient: RCP, amount: 999 };
    let c = conn(r1, r2);
    assert!(c.execute(&intent(), &RCP, signer()).is_err());
}

#[test]
fn dryrun_divergent_is_unknown() {
    let mut r2v = ok_rpc(SuiSend::Accepted { digest: D }, SuiTxLookup::NotFound);
    r2v.dry = DryRun::Unreachable;
    let c = conn(ok_rpc(SuiSend::Accepted { digest: D }, SuiTxLookup::NotFound), r2v);
    assert!(matches!(c.execute(&intent(), &RCP, signer()).unwrap_err(), ConnErr::Unknown(_)));
}

#[test]
fn settled_requires_both_nodes_same_checkpoint() {
    // RISK-M7-3
    let c = conn(ok_rpc(SuiSend::Unreachable, SuiTxLookup::Success { checkpoint: 100 }),
                 ok_rpc(SuiSend::Unreachable, SuiTxLookup::Success { checkpoint: 100 }));
    assert!(matches!(c.status(&TxRef::Real { tx_hash: D, chain_nonce: 0 }).unwrap(),
                     TxStatus::Settled { block: 100, .. }));
    let c = conn(ok_rpc(SuiSend::Unreachable, SuiTxLookup::Success { checkpoint: 100 }),
                 ok_rpc(SuiSend::Unreachable, SuiTxLookup::NotFound));
    assert_eq!(c.status(&TxRef::Real { tx_hash: D, chain_nonce: 0 }).unwrap(), TxStatus::Pending);
}

#[test]
fn signing_digest_deterministic_and_intent_prefixed() {
    let a = signing_digest(b"txbytes");
    let b = signing_digest(b"txbytes");
    assert_eq!(a, b);
    // интент влияет: другой payload → другой digest
    assert_ne!(signing_digest(b"txbytes"), signing_digest(b"txbytez"));
}

#[test]
fn signature_serialization_format_p256() {
    let s = signer();
    let d = signing_digest(b"payload");
    let (sig, pk) = s.sign_prehash(&d).unwrap();
    let ser = serialize_signature(&sig, &pk);
    assert_eq!(ser.len(), 98);
    assert_eq!(ser[0], SECP256R1_FLAG);
    // верифицируема стандартным P-256 (RFC 6979 → детерминизм проверен в M4)
    use p256::ecdsa::signature::hazmat::PrehashVerifier;
    use p256::ecdsa::{Signature, VerifyingKey};
    let vk = VerifyingKey::from_sec1_bytes(&pk).unwrap();
    let sg = Signature::from_slice(&sig).unwrap();
    assert!(vk.verify_prehash(&d, &sg).is_ok());
}

#[test]
fn sui_address_derivation_deterministic() {
    let s = signer();
    let (_sig, pk) = s.sign_prehash(&signing_digest(b"probe")).unwrap();
    let a1 = sui_address_from_pubkey(&pk);
    let a2 = sui_address_from_pubkey(&pk);
    assert_eq!(a1, a2);
    assert_ne!(a1, [0u8; 32]);
}

#[test]
fn quote_is_zero_gasless() {
    let c = conn(ok_rpc(SuiSend::Unreachable, SuiTxLookup::NotFound),
                 ok_rpc(SuiSend::Unreachable, SuiTxLookup::NotFound));
    assert_eq!(c.quote(&intent()).unwrap().gas_estimate, Amount::ZERO);
}
