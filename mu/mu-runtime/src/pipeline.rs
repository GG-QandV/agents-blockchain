//! Конвейер: Ω → Δ → [human] → WAL → execute → Settled/Failed/ReconcilePending.
use crate::policy::{delta_check, needs_human, omega_check, Delta, Omega};
use mu_common::{Amount, CanonAddress, Clock, ConnectorId, Hash32};
use mu_connect::{ConnErr, Connector, Intent as ConnIntent, TxRef, TxStatus};
use mu_human::{confirm_payment, verify_auth_proof, HumanDecision, PayConfirm, Presenter};
use mu_log::{Kind, Log};
use mu_vault::Vault;
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};

/// Входной intent конвейера (уже прошёл M8: аутентичен, канонизирован).
#[derive(Clone, Debug)]
pub struct RtIntent {
    pub recipient: CanonAddress,
    pub amount: Amount,
    pub chain_id: u64,
    pub agent_id: String,
    pub connector: ConnectorId,
}

impl RtIntent {
    pub fn hash(&self) -> Hash32 {
        let mut h = Sha256::new();
        h.update(self.recipient.bytes());
        h.update(self.recipient.chain_id().to_be_bytes());
        h.update(self.amount.minor().to_be_bytes());
        h.update(self.agent_id.as_bytes());
        let out = h.finalize();
        let mut d = [0u8; 32];
        d.copy_from_slice(&out);
        Hash32(d)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntentStatus {
    DeniedOmega,
    DeniedDelta,
    DeniedHuman,
    TimeoutHuman,
    Settled { tx_hash: [u8; 32] },
    Failed,
    /// RISK-M6-5: исход неизвестен; резерв держится (Pending в логе), решает reconcile.
    ReconcilePending,
}

/// ═══ RISK-M6-1: типовое принуждение «WAL до денег» ═══
/// WalWritten порождается ТОЛЬКО write_wal (приватное поле, конструктора снаружи нет).
/// exec_after_wal — единственная функция, вызывающая connector.execute,
/// и она требует WalWritten по move. Вызов execute без WAL не компилируется.
pub struct WalWritten {
    intent_hash: Hash32,
    _priv: (),
}

fn write_wal(
    log: &mut Log,
    vault: &dyn Vault,
    intent_hash: Hash32,
    amount_total: Amount,
    chain_nonce: u64,
    ts: u64,
) -> Result<WalWritten, mu_log::LogErr> {
    log.append(
        Kind::Pending { intent_hash, amount_total: amount_total.minor(), chain_nonce },
        ts,
        vault,
    )?; // fsync внутри (RISK-M5-1)
    Ok(WalWritten { intent_hash, _priv: () })
}

fn exec_after_wal(
    proof: WalWritten,
    connector: &dyn Connector,
    intent: &ConnIntent,
    signer: mu_vault::TxSigner,
) -> (Hash32, Result<TxRef, ConnErr>) {
    let r = connector.execute(intent, signer);
    (proof.intent_hash, r)
}

pub struct Runtime<'a> {
    pub omega: Omega,
    pub delta: Delta,
    pub log: Log,
    pub vault: &'a dyn Vault,
    pub connector: &'a dyn Connector,
    pub presenter: &'a dyn Presenter,
    pub clock: &'a dyn Clock,
    pub owner_pubkey: Vec<u8>,
    pub human_ttl: Duration,
}

impl<'a> Runtime<'a> {
    /// Обработка одного intent'а. &mut self = сериализация (RISK-M6-3):
    /// второй process не начнётся, пока не завершён первый.
    pub fn process(&mut self, i: &RtIntent) -> IntentStatus {
        let now = self.clock.now_unix();
        let ih = i.hash();

        // 1. quote (для Ω-суммы)
        let conn_intent = ConnIntent { recipient: i.recipient, amount: i.amount, chain_id: i.chain_id };
        let gas = match self.connector.quote(&conn_intent) {
            Ok(f) => f.gas_estimate,
            Err(_) => {
                let _ = self.log.append(Kind::DeniedOmega { intent_hash: ih }, now, self.vault);
                return IntentStatus::DeniedOmega;
            }
        };

        // 2. Ω-check
        let total = match omega_check(i.amount, gas, i.connector, &self.omega) {
            Ok(t) => t,
            Err(_) => {
                let _ = self.log.append(Kind::DeniedOmega { intent_hash: ih }, now, self.vault);
                return IntentStatus::DeniedOmega;
            }
        };

        // 3. Δ-check (окно по логу)
        if delta_check(&i.recipient, total, &self.delta, &self.log, now).is_err() {
            let _ = self.log.append(Kind::DeniedDelta { intent_hash: ih }, now, self.vault);
            return IntentStatus::DeniedDelta;
        }

        // 4. human при total > threshold (RISK-M3-3: та же величина total)
        if needs_human(total, &self.delta) {
            let req = PayConfirm {
                recipient: i.recipient,
                wl_label: None,
                amount: i.amount,
                gas_est: gas,
                agent_id: i.agent_id.clone(),
                remaining_window: self
                    .delta
                    .daily_limit
                    .checked_sub(self.log.window_sum(now))
                    .unwrap_or(Amount::ZERO),
                intent_hash: ih,
                first_payment_to_recipient: false,
            };
            let deadline = Instant::now() + self.human_ttl;
            match confirm_payment(&req, self.presenter, self.vault, deadline) {
                HumanDecision::Denied => {
                    let _ = self.log.append(Kind::HumanDecision { intent_hash: ih, approved: false }, now, self.vault);
                    return IntentStatus::DeniedHuman;
                }
                HumanDecision::Timeout => {
                    let _ = self.log.append(Kind::HumanDecision { intent_hash: ih, approved: false }, now, self.vault);
                    return IntentStatus::TimeoutHuman;
                }
                HumanDecision::Approved(proof) => {
                    // RISK-M9-2: конвейер верифицирует proof, а не верит M9 на слово
                    if !verify_auth_proof(&proof, &ih, &self.owner_pubkey) {
                        let _ = self.log.append(Kind::Alert { code: 0x0901 }, now, self.vault);
                        return IntentStatus::DeniedHuman;
                    }
                    let _ = self.log.append(Kind::HumanDecision { intent_hash: ih, approved: true }, now, self.vault);
                }
            }
        }

        // 5. WAL до денег (RISK-M6-1) — единственный источник WalWritten
        let signer = match self.vault.tx_signer() {
            Ok(s) => s,
            Err(_) => return IntentStatus::Failed, // ключ недоступен ДО денег — безопасный отказ
        };
        let wal = match write_wal(&mut self.log, self.vault, ih, total, 0, now) {
            Ok(w) => w,
            Err(_) => return IntentStatus::Failed, // WAL не записан → денег не трогаем
        };

        // 6. исполнение
        let (ih2, res) = exec_after_wal(wal, self.connector, &conn_intent, signer);
        match res {
            Ok(TxRef::Real { tx_hash, .. }) => {
                // 7. финальность: Settled только при консенсусе status (RISK-M6-2/M7-3)
                match self.connector.status(&TxRef::Real { tx_hash, chain_nonce: 0 }) {
                    Ok(TxStatus::Settled { effective_gas, .. }) => {
                        let _ = self.log.append(
                            Kind::Settled { intent_hash: ih2, tx_hash, effective_gas },
                            self.clock.now_unix(),
                            self.vault,
                        );
                        IntentStatus::Settled { tx_hash }
                    }
                    Ok(TxStatus::Failed { .. }) => {
                        let _ = self.log.append(Kind::Failed { intent_hash: ih2 }, self.clock.now_unix(), self.vault);
                        IntentStatus::Failed
                    }
                    // Pending / ошибка опроса → резерв держится (RISK-M6-5)
                    _ => IntentStatus::ReconcilePending,
                }
            }
            Ok(TxRef::Simulated { .. }) => {
                // стаб: помечаем Simulated; окно НЕ трогает (RISK-M7S-1)
                let _ = self.log.append(Kind::Failed { intent_hash: ih2 }, self.clock.now_unix(), self.vault);
                let _ = self.log.append(
                    Kind::Simulated { intent_hash: ih2, connector: "stub" },
                    self.clock.now_unix(),
                    self.vault,
                );
                IntentStatus::Failed
            }
            Ok(TxRef::Authorization { nonce, valid_before }) => {
                // x402: авторизація підписана, фасилітатор проведе платіж.
                // Статус визначається пізніше через connector.status().
                // nonce — ключ для authorizationState.
                IntentStatus::ReconcilePending
            }
            Err(ConnErr::Rejected(_)) => {
                // достоверный неуход → закрываем Pending (rollback резерва)
                let _ = self.log.append(Kind::Failed { intent_hash: ih2 }, self.clock.now_unix(), self.vault);
                IntentStatus::Failed
            }
            // RISK-M6-2/M6-5: Unknown НИКОГДА не rollback — Pending остаётся, резерв держится.
            // ConnErr non_exhaustive → catch-all обязан отображаться в КОНСЕРВАТИВНУЮ ветку
            // (неизвестный вариант ошибки = неизвестность исхода = держим резерв), не в Failed.
            Err(ConnErr::Unknown(_)) | Err(ConnErr::Config(_)) => IntentStatus::ReconcilePending,
            Err(_) => IntentStatus::ReconcilePending,
        }
    }
}
