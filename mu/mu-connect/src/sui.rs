//! M7a-sui — коннектор Sui (безгазовые переводы стейблкоинов, mainnet/testnet).
//!
//! ИНВАРИАНТЫ ПОЛНОСТЬЮ НАСЛЕДУЮТСЯ ОТ EVM-ВЕРСИИ:
//! RISK-M7-1: Unknown ≠ Failed — execute типом не может вернуть Failed.
//! RISK-M7-3: Settled только при консенсусе обоих RPC (checkpoint + status).
//! RISK-M7-5: получатель неподменяем — self-check: decode собранной tx (dry-run обеих нод)
//!            сверяет recipient+amount с intent до подписи.
//!
//! Sui-специфика:
//! - Идемпотентность: digest транзакции детерминирован от байт+подписи; повторная отправка
//!   тех же байт безопасна (нода отвечает "already executed" → AlreadyKnown).
//! - Эквивокация (owned-object заблокирован конкурентной tx до конца эпохи) — это
//!   НЕИЗВЕСТНОСТЬ исхода → строго Unknown, никогда Rejected (аналог RISK-M7-2).
//! - Подпись: intent [scope=0,version=0,app=0] ‖ tx_bytes → blake2b-256 → Ed25519;
//!   сериализация подписи: flag(0x00) ‖ sig(64) ‖ pubkey(32) — стабильная схема Sui.
//! - Сборка tx: server-side через RPC (tx_bytes b64) — endpoint конфигурируем строкой,
//!   т.к. имя метода для gasless Address-Balances переводов сверяется агентом с
//!   официальной документацией (пост-cutoff фича; см. ETAP2-инструкцию, источники).
use crate::{ConnErr, Fee, Intent, RejectReason, TxRef, TxStatus};
use blake2::{digest::consts::U32, Blake2b, Digest};
use mu_common::Amount;
use mu_vault::TxSignerP256;

type Blake2b256 = Blake2b<U32>;

/// Intent-префикс Sui для TransactionData: scope=0, version=0, app_id=0.
pub const SUI_INTENT: [u8; 3] = [0, 0, 0];
/// Флаг схемы подписи Secp256r1 (P-256) в Sui — родная кривая enclave (RISK-M4-1 усилен).
pub const SECP256R1_FLAG: u8 = 0x02;

/// Дайджест для подписи: blake2b256(intent ‖ tx_bytes).
pub fn signing_digest(tx_bytes: &[u8]) -> [u8; 32] {
    let mut h = Blake2b256::new();
    h.update(SUI_INTENT);
    h.update(tx_bytes);
    let out = h.finalize();
    let mut d = [0u8; 32];
    d.copy_from_slice(&out);
    d
}

/// Сериализованная подпись Sui: flag ‖ sig(64) ‖ pubkey(33, compressed) = 98 байт.
pub fn serialize_signature(sig: &[u8; 64], pubkey: &[u8; 33]) -> Vec<u8> {
    let mut out = Vec::with_capacity(98);
    out.push(SECP256R1_FLAG);
    out.extend_from_slice(sig);
    out.extend_from_slice(pubkey);
    out
}

/// Sui-адрес кошелька = blake2b256(flag ‖ pubkey). Эталон сверки: sui keytool (README-LIVE §3).
pub fn sui_address_from_pubkey(pubkey: &[u8; 33]) -> [u8; 32] {
    let mut h = Blake2b256::new();
    h.update([SECP256R1_FLAG]);
    h.update(pubkey);
    let out = h.finalize();
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    a
}

// ── RPC-абстракция (реализации: json_rpc.rs для сети, моки в тестах) ──────
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuiltTx {
    pub tx_bytes: Vec<u8>, // BCS TransactionData (raw, декодированный из b64 ответа ноды)
}

/// Результат dry-run: нода декодирует tx и возвращает эффект — используем как
/// self-check получателя/суммы (RISK-M7-5) и предпросмотр revert.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DryRun {
    Ok { recipient: [u8; 32], amount: u128 },
    WouldFail(String),
    Unreachable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuiSend {
    Accepted { digest: [u8; 32] },
    AlreadyExecuted { digest: [u8; 32] },
    /// Достоверный отказ ДО принятия (баланс, невалидная подпись/структура).
    DeterministicReject(RejectReason),
    /// Owned-object lock конкурентной транзакцией: исход НЕИЗВЕСТЕН до конца эпохи.
    ObjectLocked,
    Unreachable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuiTxLookup {
    NotFound,
    Success { checkpoint: u64 },
    FailedOnChain { checkpoint: u64 },
    Unreachable,
}

pub trait SuiRpc: Send + Sync {
    fn build_transfer(&self, sender: &[u8; 32], recipient: &[u8; 32], amount: u128, coin_type: &str)
        -> Result<BuiltTx, ConnErr>;
    fn dry_run(&self, tx_bytes: &[u8]) -> DryRun;
    fn execute(&self, tx_bytes: &[u8], serialized_sig: &[u8]) -> SuiSend;
    fn lookup(&self, digest: &[u8; 32]) -> SuiTxLookup;
}

pub struct SuiConnector<R: SuiRpc> {
    pub network: &'static str, // "testnet" | "mainnet"
    pub coin_type: String,     // полный type-tag USDC на Sui (константа сборки, RISK-M7-5)
    pub wallet_addr: [u8; 32],
    pub rpc1: R,
    pub rpc2: R,
}

impl<R: SuiRpc> SuiConnector<R> {
    pub fn quote(&self, _i: &Intent) -> Result<Fee, ConnErr> {
        // Протокольный gasless: издержек для сторон нет. total = amount (упрощение Ω/Δ).
        Ok(Fee { gas_estimate: Amount::ZERO })
    }

    pub fn execute(&self, i: &Intent, recipient32: &[u8; 32], signer: TxSignerP256) -> Result<TxRef, ConnErr> {
        // 1. server-side сборка (rpc1, fallback rpc2)
        let built = match self.rpc1.build_transfer(&self.wallet_addr, recipient32, i.amount.minor(), &self.coin_type) {
            Ok(b) => b,
            Err(_) => self.rpc2.build_transfer(&self.wallet_addr, recipient32, i.amount.minor(), &self.coin_type)?,
        };

        // 2. self-check через dry-run ОБЕИХ нод (RISK-M7-5): собранная tx действительно
        //    платит intent.recipient ровно intent.amount — защита и от лживой ноды-сборщика.
        let d1 = self.rpc1.dry_run(&built.tx_bytes);
        let d2 = self.rpc2.dry_run(&built.tx_bytes);
        match (&d1, &d2) {
            (DryRun::Ok { recipient: r1, amount: a1 }, DryRun::Ok { recipient: r2, amount: a2 }) => {
                if r1 != recipient32 || r2 != recipient32 || *a1 != i.amount.minor() || *a2 != i.amount.minor() {
                    // нода собрала НЕ то, что просили → достоверный отказ до подписи
                    return Err(ConnErr::Rejected(RejectReason::InvalidRecipient));
                }
            }
            (DryRun::WouldFail(_), DryRun::WouldFail(_)) => {
                return Err(ConnErr::Rejected(RejectReason::SimulateRevert));
            }
            _ => return Err(ConnErr::Unknown("dry-run unavailable/divergent".into())),
        }

        // 3. подпись (intent + blake2b256 + Ed25519); signer по move — второй подписи не будет
        let digest = signing_digest(&built.tx_bytes);
        let (sig, pubkey) = signer.sign_prehash(&digest).map_err(|e| ConnErr::Unknown(format!("sign: {e:?}")))?;
        let ser_sig = serialize_signature(&sig, &pubkey);

        // 4. отправка в оба RPC — классификация строго по RISK-M7-1
        let s1 = self.rpc1.execute(&built.tx_bytes, &ser_sig);
        let s2 = self.rpc2.execute(&built.tx_bytes, &ser_sig);
        classify_sui_send(s1, s2)
    }

    pub fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr> {
        let digest = match r {
            TxRef::Real { tx_hash, .. } => tx_hash,
            TxRef::Simulated { .. } => {
                return Ok(TxStatus::Failed {
                    reason: crate::FailReason::Simulated,
                });
            }
            TxRef::Authorization { .. } => {
                // SuiConnector не працює з EVM x402 авторизаціями
                return Err(ConnErr::Config("Sui connector got Authorization txref".into()));
            }
        };
        let l1 = self.rpc1.lookup(digest);
        let l2 = self.rpc2.lookup(digest);
        Ok(match (l1, l2) {
            // RISK-M7-3: Settled только при согласии обеих нод о checkpoint+успехе
            (SuiTxLookup::Success { checkpoint: c1 }, SuiTxLookup::Success { checkpoint: c2 }) if c1 == c2 => {
                TxStatus::Settled { block: c1, effective_gas: 0 }
            }
            (SuiTxLookup::FailedOnChain { .. }, SuiTxLookup::FailedOnChain { .. }) => {
                TxStatus::Failed { reason: crate::FailReason::OnChainRevert }
            }
            // недоступность/расхождение/не найдена → Pending, решает reconcile
            _ => TxStatus::Pending,
        })
    }
}

/// RISK-M7-1 для Sui: Failed не производится. ObjectLocked и любые неоднозначности → Unknown.
fn classify_sui_send(s1: SuiSend, s2: SuiSend) -> Result<TxRef, ConnErr> {
    use SuiSend::*;
    let accepted = match (&s1, &s2) {
        (Accepted { digest }, _) | (_, Accepted { digest }) => Some(*digest),
        (AlreadyExecuted { digest }, _) | (_, AlreadyExecuted { digest }) => Some(*digest),
        _ => None,
    };
    if let Some(digest) = accepted {
        return Ok(TxRef::Real { tx_hash: digest, chain_nonce: 0 });
    }
    if let (DeterministicReject(a), DeterministicReject(_)) = (&s1, &s2) {
        return Err(ConnErr::Rejected(a.clone()));
    }
    // ObjectLocked в любой позиции = неизвестность (tx конкурента может исполниться/нет)
    Err(ConnErr::Unknown("send outcome uncertain (locked/unreachable/divergent)".into()))
}
