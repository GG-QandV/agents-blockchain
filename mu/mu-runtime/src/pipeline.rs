//! Конвейер: Ω → Δ → [human] → WAL → execute → Settled/Failed/ReconcilePending.
use crate::policy::{delta_check, needs_human, omega_check, resource_check, Delta, Omega};
use mu_common::{Amount, CanonAddress, Clock, ConnectorId, Hash32};
use mu_connect::{AuthParams, ConnErr, Connector, ExecCtx, Intent as ConnIntent, TxRef, TxStatus};
use mu_connect::x402::X402Challenge;
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
        // P0-10: the owner proof is bound to the connector and the resource too,
        // otherwise two intents differing only there share one proof.
        h.update([self.connector as u8]);
        match &self.resource {
            Some(r) => {
                h.update([1u8]);
                h.update((r.len() as u32).to_be_bytes());
                h.update(r.as_bytes());
            }
            None => h.update([0u8]),
        }
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
    /// x402 had no 402 offer to answer: nothing was signed (wire code 0x15).
    DeniedNoOffer,
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
    auth: Option<AuthParams>,
) -> Result<WalWritten, mu_log::LogErr> {
    let kind = match auth {
        Some(a) => Kind::PendingAuth {
            intent_hash,
            amount_total: amount_total.minor(),
            nonce: a.nonce,
            valid_after: a.valid_after,
            valid_before: a.valid_before,
        },
        None => Kind::Pending {
            intent_hash,
            amount_total: amount_total.minor(),
            chain_nonce,
        },
    };
    log.append(kind, ts, vault)?; // fsync внутри (RISK-M5-1)
    Ok(WalWritten { intent_hash, _priv: () })
}

fn exec_after_wal(
    proof: WalWritten,
    connector: &dyn Connector,
    intent: &ConnIntent,
    signer: mu_vault::TxSigner,
    ctx: ExecCtx,
) -> (Hash32, Result<TxRef, ConnErr>) {
    let r = connector.execute_auth(intent, signer, ctx);
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
    /// An intent with no 402 offer: a connector that needs one refuses, so
    /// nothing is signed and nothing is paid.
    pub fn process(&mut self, i: &RtIntent) -> IntentStatus {
        self.process_offer(i, None)
    }

    /// The offer is the 402 answer for *this* intent. It is passed down to the
    /// connector with the execution, never stored, so it cannot leak into the
    /// next payment.
    pub fn process_offer(&mut self, i: &RtIntent, offer: Option<X402Challenge>) -> IntentStatus {
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

        // 3b. Resource-check. A missing resource is not a free pass: for x402
        // and for any policy with price caps it is a denial, otherwise the
        // per-call ceiling could be bypassed by leaving the field out.
        let resource_required = self.connector.pre_auth() || !self.delta.resource_allowlist.is_empty();
        if i.resource.is_none() && resource_required {
            let _ = self.log.append(Kind::DeniedDelta { intent_hash: ih }, now, self.vault);
            return IntentStatus::DeniedDelta;
        }
        if let Some(ref resource) = i.resource {
            let (host, path) = split_resource(resource);
            if resource_check(host, path, total, &self.delta).is_err() {
                let _ = self.log.append(Kind::DeniedDelta { intent_hash: ih }, now, self.vault);
                return IntentStatus::DeniedDelta;
            }
            // The 402 answer must be for the same resource we are paying for.
            if let Some(offer) = offer.as_ref() {
                if split_resource(&offer.resource) != (host, path) {
                    let _ = self.log.append(Kind::DeniedDelta { intent_hash: ih }, now, self.vault);
                    return IntentStatus::DeniedDelta;
                }
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
        // x402: the nonce must be on disk before the money moves, so a crash
        // after execute reuses the same single-use nonce instead of minting a
        // second authorization.
        let auth = if self.connector.pre_auth() {
            // No offer → no window → nothing is signed, nothing is paid.
            // The refusal is recorded, so the journal shows why nothing moved.
            let Some(window) = self.connector.auth_window_secs(offer.as_ref()) else {
                let _ = self.log.append(Kind::DeniedNoOffer { intent_hash: ih }, now, self.vault);
                return IntentStatus::DeniedNoOffer;
            };
            let mut nonce = [0u8; 32];
            if rand_core::RngCore::try_fill_bytes(&mut rand_core::OsRng, &mut nonce).is_err() {
                return IntentStatus::Failed;
            }
            // The wall clock, because the x402 connector signs with it: the
            // runtime clock may be a test double.
            let wall = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(now);
            Some(AuthParams {
                nonce,
                valid_after: wall.saturating_sub(60),
                // Exactly the window the 402 answer allows.
                valid_before: wall.saturating_add(window),
            })
        } else {
            None
        };
        let wal = match write_wal(&mut self.log, self.vault, ih, total, 0, now, auth) {
            Ok(w) => w,
            Err(_) => return IntentStatus::Failed, // WAL не записан → денег не трогаем
        };

        // 6. исполнение
        let (ih2, res) = exec_after_wal(
        wal,
        self.connector,
        &conn_intent,
        signer,
        ExecCtx { auth, offer },
    );
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
/// The x402 authorization window written before the money moves; the challenge
/// may shorten it, never lengthen it (changelog 2026-10-20).


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
    use super::{split_resource, RtIntent};
    use mu_common::{Amount, CanonAddress, ConnectorId};
    #[test]
    fn resources_split_into_host_and_slash_path() {
        assert_eq!(split_resource("https://issuer.example/bare/issue?x=1"), ("issuer.example", "/bare/issue"));
        assert_eq!(split_resource("api.coingecko.com/api/v3/simple/price"), ("api.coingecko.com", "/api/v3/simple/price"));
        assert_eq!(split_resource("issuer.example"), ("issuer.example", "/"));
    }

    fn intent(connector: ConnectorId, resource: Option<&str>) -> RtIntent {
        RtIntent {
            recipient: CanonAddress::canon(
                "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
                8453,
            )
            .unwrap(),
            amount: Amount::from_minor(1),
            chain_id: 8453,
            agent_id: "agent-1".into(),
            connector,
            resource: resource.map(str::to_string),
        }
    }

    #[test]
    fn the_hash_binds_the_connector() {
        assert_ne!(
            intent(ConnectorId::Crypto, None).hash(),
            intent(ConnectorId::BankStub, None).hash()
        );
    }

    #[test]
    fn the_hash_binds_the_resource() {
        let a = intent(ConnectorId::Crypto, Some("https://a.example/x"));
        let b = intent(ConnectorId::Crypto, Some("https://b.example/x"));
        assert_ne!(a.hash(), b.hash());
        assert_ne!(a.hash(), intent(ConnectorId::Crypto, None).hash());
    }
}
