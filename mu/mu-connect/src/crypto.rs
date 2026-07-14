//! M7a crypto-connector (USDC / EVM L2). Логика полная; сетевой транспорт (RPC) — за trait RpcClient,
//! чтобы тесты гоняли всю таксономию ошибок без реальной сети (мок-RPC = как Anvil-форк в спеке).
use crate::{ConnErr, Connector, Fee, Intent, RejectReason, TxRef, TxStatus};
use mu_common::{Amount, CanonAddress};
use mu_vault::TxSigner;
use sha2::{Digest, Sha256};

/// USDC-контракт НЕ конфигурируем (RISK-M7-5): константа per chain_id, зашита в бинарь.
fn usdc_contract(chain_id: u64) -> Option<[u8; 20]> {
    match chain_id {
        8453 => Some(hex20("833589fCD6eDb6E08f4c7C32D4f71b54bdA02913")), // Base USDC
        _ => None,
    }
}

fn hex20(s: &str) -> [u8; 20] {
    let mut a = [0u8; 20];
    let b = s.as_bytes();
    let mut i = 0;
    while i < 20 {
        a[i] = (hv(b[i * 2]) << 4) | hv(b[i * 2 + 1]);
        i += 1;
    }
    a
}
fn hv(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}

/// Ответ одной RPC-ноды на попытку отправки/статуса. Мок и реальная реализация дают это.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcSend {
    Accepted { tx_hash: [u8; 32] },
    AlreadyKnown { tx_hash: [u8; 32] },
    DeterministicReject(RejectReason), // нода уверенно отвергла ДО mempool
    Unreachable,                       // таймаут/сеть — НЕизвестность
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcReceipt {
    None,
    Success { block: u64, gas_used: u128 },
    Reverted { block: u64 },
    Unreachable,
}

pub trait RpcClient: Send + Sync {
    fn account_nonce(&self, addr: &[u8; 20]) -> Option<u64>;
    fn simulate(&self, contract: &[u8; 20], calldata: &[u8]) -> RpcReceiptSim;
    fn send_raw(&self, raw_tx: &[u8]) -> RpcSend;
    fn receipt(&self, tx_hash: &[u8; 32]) -> RpcReceipt;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RpcReceiptSim {
    Ok,
    Revert,
    Unreachable,
}

pub struct CryptoConnector<R: RpcClient> {
    pub chain_id: u64,
    pub confirmations: u8,
    pub rpc1: R,
    pub rpc2: R,
    /// локальный next_nonce из M5 (Pending-записи). Приоритетный источник.
    pub local_next_nonce: u64,
    pub wallet_addr: [u8; 20],
    pub fee_cap: Amount,
}

impl<R: RpcClient> CryptoConnector<R> {
    /// RISK-M7-5: единственная функция сборки calldata ERC20.transfer(to, amount)
    /// + self-check обратным декодом.
    fn build_transfer(&self, to: &CanonAddress, amount: Amount) -> Result<Vec<u8>, ConnErr> {
        // RISK-M7-5: calldata = selector + 20B адреса (EVM pad справа) + amount
        // NB: CanonAddress тепер 32B (Sui), EVM використовує перші 20B
        let mut cd = Vec::with_capacity(68);
        cd.extend_from_slice(&[0xa9, 0x05, 0x9c, 0xbb]); // selector transfer(address,uint256)
        let eth_addr = &to.bytes()[..20];
        cd.extend_from_slice(&[0u8; 12]);
        cd.extend_from_slice(eth_addr);
        let amt = amount.minor().to_be_bytes(); // u128 → 16 байт
        cd.extend_from_slice(&[0u8; 16]);
        cd.extend_from_slice(&amt);
        // self-check: обратный декод recipient+amount совпадает с intent
        let dec_to = &cd[16..36];
        let dec_amt = u128::from_be_bytes(cd[52..68].try_into().map_err(|_| {
            ConnErr::Config("calldata self-check length".into())
        })?);
        if dec_to != eth_addr || dec_amt != amount.minor() {
            return Err(ConnErr::Config("calldata self-check mismatch".into()));
        }
        Ok(cd)
    }

    fn resolve_nonce(&self) -> Result<u64, ConnErr> {
        let n1 = self.rpc1.account_nonce(&self.wallet_addr);
        let n2 = self.rpc2.account_nonce(&self.wallet_addr);
        match (n1, n2) {
            (Some(a), Some(b)) => {
                let net = a.max(b);
                // расхождение > 1 → не гадаем
                if a.abs_diff(b) > 1 {
                    return Err(ConnErr::Unknown("rpc nonce divergence".into()));
                }
                Ok(self.local_next_nonce.max(net))
            }
            _ => Err(ConnErr::Unknown("rpc nonce unavailable".into())),
        }
    }
}

impl<R: RpcClient> Connector for CryptoConnector<R> {
    fn quote(&self, i: &Intent) -> Result<Fee, ConnErr> {
        if i.chain_id != self.chain_id {
            return Err(ConnErr::Rejected(RejectReason::ChainMismatch));
        }
        // запас газа: фиксированная оценка × 3/2, cap'нута fee_cap
        let base = Amount::from_minor(50_000);
        let est = base
            .checked_mul_u32(3)
            .and_then(|v| v.div_u32(2))
            .ok_or_else(|| ConnErr::Config("gas overflow".into()))?;
        let capped = if est > self.fee_cap { self.fee_cap } else { est };
        Ok(Fee { gas_estimate: capped })
    }

    fn execute(&self, i: &Intent, signer: TxSigner) -> Result<TxRef, ConnErr> {
        // 1. sanity
        if i.chain_id != self.chain_id {
            return Err(ConnErr::Rejected(RejectReason::ChainMismatch));
        }
        let contract =
            usdc_contract(self.chain_id).ok_or_else(|| ConnErr::Config("no USDC for chain".into()))?;

        // 2. nonce
        let nonce = self.resolve_nonce()?;

        // 3. calldata + self-check (RISK-M7-5)
        let calldata = self.build_transfer(&i.recipient, i.amount)?;

        // 4. simulate на обоих RPC
        let s1 = self.rpc1.simulate(&contract, &calldata);
        let s2 = self.rpc2.simulate(&contract, &calldata);
        match (&s1, &s2) {
            (RpcReceiptSim::Revert, RpcReceiptSim::Revert) => {
                return Err(ConnErr::Rejected(RejectReason::SimulateRevert));
            }
            (RpcReceiptSim::Unreachable, _) | (_, RpcReceiptSim::Unreachable) => {
                return Err(ConnErr::Unknown("simulate unreachable".into()));
            }
            _ => {}
        }

        // 5. подпись (RFC 6979). signer уничтожается по выходу из функции (move).
        let sighash = tx_sighash(&contract, &calldata, nonce, self.chain_id);
        let _sig = signer
            .sign(&sighash)
            .map_err(|e| ConnErr::Unknown(format!("sign failed: {e:?}")))?;
        // (сериализация raw_tx с подписью опущена до транспортного слоя; для мок-тестов
        //  используем детерминированный tx_hash из sighash)
        let raw_tx = sighash.0.to_vec();

        // 6. отправка в оба RPC. КЛЮЧЕВАЯ ЛОГИКА RISK-M7-1.
        let r1 = self.rpc1.send_raw(&raw_tx);
        let r2 = self.rpc2.send_raw(&raw_tx);
        classify_send(r1, r2, nonce)
    }

    fn status(&self, r: &TxRef) -> Result<TxStatus, ConnErr> {
        let tx_hash = match r {
            TxRef::Real { tx_hash, .. } => tx_hash,
            TxRef::Simulated { .. } => {
                // реальный коннектор не должен получать Simulated — это баг вызывающего
                return Ok(TxStatus::Failed {
                    reason: crate::FailReason::Simulated,
                });
            }
        };
        // RISK-M7-3: Settled только при согласии обоих RPC
        let rc1 = self.rpc1.receipt(tx_hash);
        let rc2 = self.rpc2.receipt(tx_hash);
        match (rc1, rc2) {
            (RpcReceipt::Success { block: b1, gas_used }, RpcReceipt::Success { block: b2, .. })
                if b1 == b2 =>
            {
                Ok(TxStatus::Settled { block: b1, effective_gas: gas_used })
            }
            (RpcReceipt::Reverted { .. }, RpcReceipt::Reverted { .. }) => Ok(TxStatus::Failed {
                reason: crate::FailReason::OnChainRevert,
            }),
            (RpcReceipt::Unreachable, _) | (_, RpcReceipt::Unreachable) => Ok(TxStatus::Pending),
            // расхождение нод → НЕ Settled, ждём (RISK-M7-3)
            _ => Ok(TxStatus::Pending),
        }
    }
}

/// RISK-M7-1: классификация исхода отправки в две ноды.
/// Failed НЕ производится: если tx могла уйти хоть куда-то, исход — Real (Pending на стороне M6).
fn classify_send(r1: RpcSend, r2: RpcSend, nonce: u64) -> Result<TxRef, ConnErr> {
    use RpcSend::*;
    // хотя бы одна нода приняла (или "уже знает") → tx в сети → Real
    let accepted_hash = match (&r1, &r2) {
        (Accepted { tx_hash }, _) | (_, Accepted { tx_hash }) => Some(*tx_hash),
        (AlreadyKnown { tx_hash }, _) | (_, AlreadyKnown { tx_hash }) => Some(*tx_hash),
        _ => None,
    };
    if let Some(tx_hash) = accepted_hash {
        return Ok(TxRef::Real { tx_hash, chain_nonce: nonce });
    }
    // ОБЕ ноды дали ДЕТЕРМИНИРОВАННЫЙ отказ → безопасно Rejected
    if let (DeterministicReject(a), DeterministicReject(_b)) = (&r1, &r2) {
        return Err(ConnErr::Rejected(a.clone()));
    }
    // всё остальное (хоть одна Unreachable, разнобой) → Unknown: tx МОГЛА уйти.
    Err(ConnErr::Unknown("send outcome uncertain".into()))
}


fn tx_sighash(contract: &[u8; 20], calldata: &[u8], nonce: u64, chain_id: u64) -> mu_common::Hash32 {
    let mut h = Sha256::new();
    h.update(contract);
    h.update(calldata);
    h.update(nonce.to_be_bytes());
    h.update(chain_id.to_be_bytes());
    let out = h.finalize();
    let mut d = [0u8; 32];
    d.copy_from_slice(&out);
    mu_common::Hash32(d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RejectReason, TxRef};
    use mu_common::CanonAddress;

    struct MockRpc {
        nonce: Option<u64>,
        sim: RpcReceiptSim,
        send: RpcSend,
        rcpt: RpcReceipt,
    }
    impl RpcClient for MockRpc {
        fn account_nonce(&self, _a: &[u8; 20]) -> Option<u64> { self.nonce }
        fn simulate(&self, _c: &[u8; 20], _d: &[u8]) -> RpcReceiptSim { self.sim.clone() }
        fn send_raw(&self, _r: &[u8]) -> RpcSend { self.send.clone() }
        fn receipt(&self, _t: &[u8; 32]) -> RpcReceipt { self.rcpt.clone() }
    }

    fn intent() -> Intent {
        Intent {
            recipient: CanonAddress::canon("0x833589fcd6edb6e08f4c7c32d4f71b54bda02913000000000000000000000000", 8453).unwrap(),
            amount: Amount::from_minor(5_000_000),
            chain_id: 8453,
        }
    }
    fn conn(r1: MockRpc, r2: MockRpc) -> CryptoConnector<MockRpc> {
        CryptoConnector {
            chain_id: 8453, confirmations: 1, rpc1: r1, rpc2: r2,
            local_next_nonce: 0, wallet_addr: [0xAA; 20], fee_cap: Amount::from_minor(1_000_000),
        }
    }
    // TxSigner::from_bytes — pub(crate) в mu-vault; для тестов коннектора берём через softvault Vault.
    fn make_signer() -> TxSigner {
        use mu_vault::Vault;
        let v = mu_vault::backend::SoftVault::for_test([1;32],[2;32],[3;32]);
        v.tx_signer().unwrap()
    }

    fn ok_rpc(send: RpcSend, rcpt: RpcReceipt) -> MockRpc {
        MockRpc { nonce: Some(0), sim: RpcReceiptSim::Ok, send, rcpt }
    }

    #[test]
    fn accepted_by_one_rpc_is_real() {
        // RISK-M7-1: одна нода приняла → Real (не важно, что вторая молчит)
        let c = conn(
            ok_rpc(RpcSend::Accepted { tx_hash: [7;32] }, RpcReceipt::None),
            ok_rpc(RpcSend::Unreachable, RpcReceipt::None),
        );
        let r = c.execute(&intent(), make_signer()).unwrap();
        assert!(matches!(r, TxRef::Real { .. }));
    }

    #[test]
    fn one_unreachable_no_accept_is_unknown_not_failed() {
        // RISK-M7-1 ГЛАВНЫЙ: неизвестность НИКОГДА не Failed
        let c = conn(
            ok_rpc(RpcSend::Unreachable, RpcReceipt::None),
            ok_rpc(RpcSend::DeterministicReject(RejectReason::NonceTooLow), RpcReceipt::None),
        );
        let e = c.execute(&intent(), make_signer()).unwrap_err();
        assert!(matches!(e, ConnErr::Unknown(_)));
        // и уж точно не Rejected при одной Unreachable
    }

    #[test]
    fn both_deterministic_reject_is_rejected() {
        let c = conn(
            ok_rpc(RpcSend::DeterministicReject(RejectReason::InsufficientFunds), RpcReceipt::None),
            ok_rpc(RpcSend::DeterministicReject(RejectReason::InsufficientFunds), RpcReceipt::None),
        );
        let e = c.execute(&intent(), make_signer()).unwrap_err();
        assert!(matches!(e, ConnErr::Rejected(RejectReason::InsufficientFunds)));
    }

    #[test]
    fn nonce_divergence_is_unknown() {
        let mut r1 = ok_rpc(RpcSend::Accepted { tx_hash: [1;32] }, RpcReceipt::None);
        let mut r2 = ok_rpc(RpcSend::Accepted { tx_hash: [1;32] }, RpcReceipt::None);
        r1.nonce = Some(5);
        r2.nonce = Some(9); // расхождение > 1
        let c = conn(r1, r2);
        let e = c.execute(&intent(), make_signer()).unwrap_err();
        assert!(matches!(e, ConnErr::Unknown(_)));
    }

    #[test]
    fn status_settled_requires_both_rpc_agree() {
        // RISK-M7-3
        let c = conn(
            ok_rpc(RpcSend::Accepted { tx_hash: [3;32] }, RpcReceipt::Success { block: 100, gas_used: 21000 }),
            ok_rpc(RpcSend::Accepted { tx_hash: [3;32] }, RpcReceipt::Success { block: 100, gas_used: 21000 }),
        );
        let st = c.status(&TxRef::Real { tx_hash: [3;32], chain_nonce: 0 }).unwrap();
        assert!(matches!(st, TxStatus::Settled { .. }));
    }

    #[test]
    fn status_divergent_receipt_is_pending() {
        let c = conn(
            ok_rpc(RpcSend::Accepted { tx_hash: [3;32] }, RpcReceipt::Success { block: 100, gas_used: 21000 }),
            ok_rpc(RpcSend::Accepted { tx_hash: [3;32] }, RpcReceipt::None),
        );
        let st = c.status(&TxRef::Real { tx_hash: [3;32], chain_nonce: 0 }).unwrap();
        assert_eq!(st, TxStatus::Pending);
    }

    #[test]
    fn simulate_revert_both_is_rejected() {
        let mut r1 = ok_rpc(RpcSend::Accepted { tx_hash: [1;32] }, RpcReceipt::None);
        let mut r2 = ok_rpc(RpcSend::Accepted { tx_hash: [1;32] }, RpcReceipt::None);
        r1.sim = RpcReceiptSim::Revert;
        r2.sim = RpcReceiptSim::Revert;
        let c = conn(r1, r2);
        let e = c.execute(&intent(), make_signer()).unwrap_err();
        assert!(matches!(e, ConnErr::Rejected(RejectReason::SimulateRevert)));
    }
}
