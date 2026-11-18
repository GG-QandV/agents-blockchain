//! Конвейер: Ω → Δ → [human] → WAL → execute → Settled/Failed/ReconcilePending.
use crate::policy::{
    connector_byte, delta_check, needs_human, omega_check, policy_hash, resource_cap, resource_check, Delta,
    DeltaDeny, Omega, OmegaDeny,
};
use mu_common::{Amount, CanonAddress, Clock, ConnectorId, Hash32};
use mu_connect::{ConnErr, Connector, Intent as ConnIntent, TxRef, TxStatus};
use mu_human::{confirm_payment, verify_auth_proof, HumanDecision, PayConfirm, Presenter};
use mu_log::{deny, Kind, Log};
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
    /// URL ресурсу для x402 (хост+шлях). None = перевірка пропускається.
    pub resource: Option<String>,
}

impl RtIntent {
    pub fn hash(&self) -> Hash32 {
        let mut h = Sha256::new();
        h.update(self.recipient.bytes());
        h.update(self.recipient.chain_id().to_be_bytes());
        h.update(self.amount.minor().to_be_bytes());
        h.update(self.agent_id.as_bytes());
        // connector and resource are part of what is asked: same payee/amount on another
        // connector or x402 resource is another intent (log entries join on this hash)
        h.update([connector_byte(self.connector)]);
        h.update((self.resource.as_deref().unwrap_or("").len() as u32).to_be_bytes());
        h.update(self.resource.as_deref().unwrap_or("").as_bytes());
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
    /// Denied* followed by its reason. False if either record could not be written:
    /// the caller then reports `Failed` (audit hole), not a clean denial.
    fn log_denial(&mut self, kind: Kind, ih: Hash32, code: u8, total: u128, limit: u128, now: u64) -> bool {
        self.log.append(kind, now, self.vault).is_ok()
            && self.log.append(Kind::DenyReason { intent_hash: ih, code, total, limit }, now, self.vault).is_ok()
    }

    /// Обработка одного intent'а. &mut self = сериализация (RISK-M6-3):
    /// второй process не начнётся, пока не завершён первый.
    pub fn process(&mut self, i: &RtIntent) -> IntentStatus {
        let now = self.clock.now_unix();
        let ih = i.hash();

        // a resource the log cannot hold whole would make the record lie: refuse before anything
        if i.resource.as_deref().map_or(false, |r| r.len() > u16::MAX as usize) {
            return IntentStatus::Failed;
        }

        // 1. quote (для Ω-суммы)
        let conn_intent = ConnIntent { recipient: i.recipient, amount: i.amount, chain_id: i.chain_id };
        let gas = match self.connector.quote(&conn_intent) {
            Ok(f) => f.gas_estimate,
            Err(_) => {
                let logged = self.log_denial(Kind::DeniedOmega { intent_hash: ih }, ih, deny::QUOTE_FAILED, 0, 0, now);
                return if logged { IntentStatus::DeniedOmega } else { IntentStatus::Failed };
            }
        };

        // 1b. что спросили и по каким правилам судим — в лог ДО проверок.
        // Нет записи — нет платежа (fail closed): без Quoted читатель не пересчитает решение.
        let quoted = Kind::Quoted {
            intent_hash: ih,
            recipient: *i.recipient.bytes(),
            chain_id: i.chain_id,
            connector: connector_byte(i.connector),
            amount: i.amount.minor(),
            gas_est: gas.minor(),
            resource: i.resource.clone().unwrap_or_default(),
            policy_hash: policy_hash(&self.omega, &self.delta),
        };
        if self.log.append(quoted, now, self.vault).is_err() {
            return IntentStatus::Failed;
        }

        // 2. Ω-check
        let total = match omega_check(i.amount, gas, i.connector, &self.omega) {
            Ok(t) => t,
            Err(e) => {
                let (code, limit) = match e {
                    OmegaDeny::ConnectorUnknown => (deny::CONNECTOR_UNKNOWN, 0),
                    OmegaDeny::CeilingExceeded => (deny::CEILING_EXCEEDED, self.omega.max_ceiling.minor()),
                };
                let total = i.amount.checked_add(gas).map(|a| a.minor()).unwrap_or(u128::MAX);
                let logged = self.log_denial(Kind::DeniedOmega { intent_hash: ih }, ih, code, total, limit, now);
                return if logged { IntentStatus::DeniedOmega } else { IntentStatus::Failed };
            }
        };

        // 3. Δ-check (окно по логу)
        if let Err(e) = delta_check(&i.recipient, total, &self.delta, &self.log, now) {
            let (code, limit) = match e {
                DeltaDeny::NotWhitelisted => (deny::NOT_WHITELISTED, 0),
                DeltaDeny::WindowExceeded => (deny::WINDOW_EXCEEDED, self.delta.daily_limit.minor()),
                DeltaDeny::ResourceDenied => (deny::RESOURCE_DENIED, 0),
            };
            let logged = self.log_denial(Kind::DeniedDelta { intent_hash: ih }, ih, code, total.minor(), limit, now);
            return if logged { IntentStatus::DeniedDelta } else { IntentStatus::Failed };
        }

        // 3b. Resource-check для x402
        if let Some(ref resource) = i.resource {
            let (host, path) = split_resource(resource);
            if resource_check(host, path, total, &self.delta).is_err() {
                let cap = resource_cap(host, path, &self.delta);
                let logged = self.log_denial(Kind::DeniedDelta { intent_hash: ih }, ih, deny::RESOURCE_DENIED, total.minor(), cap, now);
                return if logged { IntentStatus::DeniedDelta } else { IntentStatus::Failed };
            }
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
            let decision = confirm_payment(&req, self.presenter, self.vault, deadline);
            match decision {
                HumanDecision::Denied | HumanDecision::Timeout => {
                    let (code, status) = if matches!(decision, HumanDecision::Denied) {
                        (deny::HUMAN_DENIED, IntentStatus::DeniedHuman)
                    } else {
                        (deny::HUMAN_TIMEOUT, IntentStatus::TimeoutHuman)
                    };
                    let logged = self.log.append(Kind::HumanDecision { intent_hash: ih, approved: false }, now, self.vault).is_ok()
                        && self.log.append(Kind::DenyReason { intent_hash: ih, code, total: total.minor(), limit: self.delta.confirm_threshold.minor() }, now, self.vault).is_ok();
                    return if logged { status } else { IntentStatus::Failed };
                }
                HumanDecision::Approved(proof) => {
                    // RISK-M9-2: конвейер верифицирует proof, а не верит M9 на слово
                    if !verify_auth_proof(&proof, &ih, &self.owner_pubkey) {
                        let logged = self.log.append(Kind::Alert { code: 0x0901 }, now, self.vault).is_ok();
                        return if logged { IntentStatus::DeniedHuman } else { IntentStatus::Failed };
                    }
                    // an approval that is not on record does not pay
                    if self.log.append(Kind::HumanDecision { intent_hash: ih, approved: true }, now, self.vault).is_err() {
                        return IntentStatus::Failed;
                    }
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
            Ok(TxRef::Authorization { .. }) => {
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

/// `host` and `/path` of an x402 resource: a full URL (`https://host/path?q`,
/// as a 402 answer gives it) or `host/path`. The path keeps its leading `/`,
/// as the policy's `path_prefix` is written; the query is not part of it.
pub fn split_resource(resource: &str) -> (&str, &str) {
    let r = resource
        .strip_prefix("https://")
        .or_else(|| resource.strip_prefix("http://"))
        .unwrap_or(resource);
    let r = r.split(['?', '#']).next().unwrap_or(r);
    match r.find('/') {
        Some(i) => (&r[..i], &r[i..]),
        None => (r, "/"),
    }
}

#[cfg(test)]
mod resource_tests {
    use super::split_resource;
    #[test]
    fn resources_split_into_host_and_slash_path() {
        assert_eq!(split_resource("https://issuer.example/bare/issue?x=1"), ("issuer.example", "/bare/issue"));
        assert_eq!(split_resource("api.coingecko.com/api/v3/simple/price"), ("api.coingecko.com", "/api/v3/simple/price"));
        assert_eq!(split_resource("issuer.example"), ("issuer.example", "/"));
    }
}
