//! M7b/M7c — заглушки. RISK-M7S-1: возвращают ТОЛЬКО TxRef::Simulated.
//! Тип TxRef::Real сконструировать здесь невозможно — конструктор варианта
//! используется лишь в crypto.rs; стаб физически не создаёт Real.
use crate::{ConnErr, Connector, FailReason, Fee, Intent, TxRef, TxStatus};
use mu_common::Amount;
use mu_vault::TxSigner;

pub struct StubConnector {
    pub kind: &'static str, // "bank_stub" | "card_stub"
    pub counter: std::sync::atomic::AtomicU64,
}

impl StubConnector {
    pub fn new(kind: &'static str) -> Self {
        StubConnector { kind, counter: std::sync::atomic::AtomicU64::new(1) }
    }
}

impl Connector for StubConnector {
    fn quote(&self, _i: &Intent) -> Result<Fee, ConnErr> {
        Ok(Fee { gas_estimate: Amount::ZERO })
    }
    fn execute(&self, _i: &Intent, _signer: TxSigner) -> Result<TxRef, ConnErr> {
        let n = self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let mut id = [0u8; 16];
        id[..8].copy_from_slice(&n.to_be_bytes());
        // RISK-M7S-1: только Simulated
        Ok(TxRef::Simulated { id })
    }
    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr> {
        match r {
            TxRef::Simulated { .. } => Ok(TxStatus::Failed { reason: FailReason::Simulated }),
            TxRef::Real { .. } => Err(ConnErr::Config("stub got Real txref".into())),
            TxRef::Authorization { .. } => Err(ConnErr::Config("stub got Authorization txref".into())),
        }
    }
}
