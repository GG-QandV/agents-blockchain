//! Pipeline: Ω → Δ → [human] → WAL → execute → Settled/Failed/ReconcilePending.
use crate::policy::{delta_check, needs_human, omega_check, Delta, Omega};
use mu_common::{Amount, CanonAddress, Clock, ConnectorId, Hash32};
use mu_connect::{ConnErr, Connector, Intent as ConnIntent, TxRef, TxStatus};
use mu_human::{confirm_payment, verify_auth_proof, HumanDecision, PayConfirm, Presenter};
use mu_log::{Kind, Log};
use mu_vault::Vault;
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};

/// Pipeline input intent (already passed M8: authenticated, canonicalized).
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
    /// RISK-M6-5: outcome unknown; reserve is held (Pending in log), reconcile decides.
    ReconcilePending,
}

/// ═══ RISK-M6-1: type-level enforcement "WAL before money" ═══
/// WalWritten is produced ONLY by write_wal (private field, no external constructor).
/// exec_after_wal — the only function that calls connector.execute,
/// and it requires WalWritten by move. Calling execute without WAL does not compile.
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
    )?; // fsync inside (RISK-M5-1)
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
    /// Process one intent. &mut self = serialization (RISK-M6-3):
    /// a second process cannot start until the first finishes.
    pub fn process(&mut self, i: &RtIntent) -> IntentStatus {
        let now = self.clock.now_unix();
        let ih = i.hash();

        // 1. Sui gasless: quote not needed (total = amount)
        // Ω-check uses only amount (no gas)
        let conn_intent = ConnIntent { recipient: i.recipient, amount: i.amount, chain_id: i.chain_id };

        // 2. Ω-check
        let total = match omega_check(i.amount, i.connector, &self.omega) {
            Ok(t) => t,
            Err(_) => {
                let _ = self.log.append(Kind::DeniedOmega { intent_hash: ih }, now, self.vault);
                return IntentStatus::DeniedOmega;
            }
        };

        // 3. Δ-check (window over log)
        if delta_check(&i.recipient, total, &self.delta, &self.log, now).is_err() {
            let _ = self.log.append(Kind::DeniedDelta { intent_hash: ih }, now, self.vault);
            return IntentStatus::DeniedDelta;
        }

        // 4. human when total > threshold (RISK-M3-3: same total value)
        if needs_human(total, &self.delta) {
            let req = PayConfirm {
                recipient: i.recipient,
                wl_label: None,
                amount: i.amount,
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
                    // RISK-M9-2: pipeline verifies proof, does not trust M9 on its word
                    if !verify_auth_proof(&proof, &ih, &self.owner_pubkey) {
                        let _ = self.log.append(Kind::Alert { code: 0x0901 }, now, self.vault);
                        return IntentStatus::DeniedHuman;
                    }
                    let _ = self.log.append(Kind::HumanDecision { intent_hash: ih, approved: true }, now, self.vault);
                }
            }
        }

        // 5. WAL before money (RISK-M6-1) — sole source of WalWritten
        let signer = match self.vault.tx_signer() {
            Ok(s) => s,
            Err(_) => return IntentStatus::Failed, // key unavailable BEFORE money — safe abort
        };
        let wal = match write_wal(&mut self.log, self.vault, ih, total, 0, now) {
            Ok(w) => w,
            Err(_) => return IntentStatus::Failed, // WAL not written → don't touch money
        };

        // 6. execution
        let (ih2, res) = exec_after_wal(wal, self.connector, &conn_intent, signer);
        match res {
            Ok(TxRef::Real { tx_hash, .. }) => {
                // 7. finality: Settled only upon status consensus (RISK-M6-2/M7-3)
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
                    // Pending / polling error → reserve is held (RISK-M6-5)
                    _ => IntentStatus::ReconcilePending,
                }
            }
            Ok(TxRef::Simulated { .. }) => {
                // stub: mark Simulated; window is NOT touched (RISK-M7S-1)
                let _ = self.log.append(Kind::Failed { intent_hash: ih2 }, self.clock.now_unix(), self.vault);
                let _ = self.log.append(
                    Kind::Simulated { intent_hash: ih2, connector: "stub" },
                    self.clock.now_unix(),
                    self.vault,
                );
                IntentStatus::Failed
            }
            Err(ConnErr::Rejected(_)) => {
                // reliable non-delivery → close Pending (rollback reserve)
                let _ = self.log.append(Kind::Failed { intent_hash: ih2 }, self.clock.now_unix(), self.vault);
                IntentStatus::Failed
            }
            // RISK-M6-2/M6-5: Unknown NEVER rollbacks — Pending remains, reserve is held.
            // ConnErr non_exhaustive → catch-all MUST map to CONSERVATIVE branch
            // (unknown error variant = unknown outcome = hold reserve), not to Failed.
            Err(ConnErr::Unknown(_)) | Err(ConnErr::Config(_)) => IntentStatus::ReconcilePending,
            Err(_) => IntentStatus::ReconcilePending,
        }
    }
}
