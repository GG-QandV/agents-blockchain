//! RISK-M5-3: таблична résolution для Pending при рекавери.
//! default = Keep. Вітки «else → Failed» в коді НЕТУ.
//! Sui semantics for NonceState:
//! - ConsumedByOther: coin-об'єкт витрачено іншою tx (версія зросла), digest не знайдено
//! - NotReached: версія об'єкта не змінилась (tx не могла виконатись)
//! - Unknown: не вдалось визначити
use mu_connect::crypto::RpcReceipt;

/// Ответы обеих нод по tx_hash + состояние nonce аккаунта относительно chain_nonce записи.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonceState {
    /// nonce аккаунта > chain_nonce записи И receipt нашей tx нет → потрачен ДРУГОЙ транзакцией
    ConsumedByOther,
    /// nonce аккаунта <= chain_nonce записи → наша tx не могла исполниться
    NotReached,
    /// не удалось надёжно определить
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    ToSettled { block: u64, gas: u128 },
    ToFailed,
    Keep,
}

/// Чистая решающая функция (RISK-M5-3). Полный перебор входов закреплён тестами.
pub fn resolve(rc1: &RpcReceipt, rc2: &RpcReceipt, nonce: NonceState) -> Resolution {
    use RpcReceipt::*;
    match (rc1, rc2) {
        // обе ноды видят успех в одном блоке → Settled
        (Success { block: b1, gas_used }, Success { block: b2, .. }) if b1 == b2 => {
            Resolution::ToSettled { block: *b1, gas: *gas_used }
        }
        // обе видят revert → исполнена и провалилась on-chain: закрываем Failed
        (Reverted { .. }, Reverted { .. }) => Resolution::ToFailed,
        // receipt нет у ОБЕИХ нод И nonce доказуемо ушёл другой tx → Failed
        (None, None) if nonce == NonceState::ConsumedByOther => Resolution::ToFailed,
        // receipt нет у обеих И nonce не достигнут → tx не в сети → Failed безопасен?
        // НЕТ: tx может лежать в mempool. Failed только при ConsumedByOther. Иначе Keep.
        _ => Resolution::Keep,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use RpcReceipt::*;

    fn all_receipts() -> Vec<RpcReceipt> {
        vec![
            None,
            Success { block: 10, gas_used: 21000 },
            Reverted { block: 10 },
            Unreachable,
        ]
    }
    fn all_nonce() -> [NonceState; 3] {
        [NonceState::ConsumedByOther, NonceState::NotReached, NonceState::Unknown]
    }

    /// Полный перебор 4×4×3 = 48 комбинаций: проверяем строгие свойства.
    #[test]
    fn exhaustive_table_properties() {
        for r1 in all_receipts() {
            for r2 in all_receipts() {
                for n in all_nonce() {
                    let res = resolve(&r1, &r2, n);
                    // Свойство 1: ToSettled только при согласии Success+Success (RISK-M7-3)
                    if let Resolution::ToSettled { .. } = res {
                        assert!(matches!((&r1, &r2), (Success { .. }, Success { .. })));
                    }
                    // Свойство 2: ToFailed только при (Reverted,Reverted) или (None,None,ConsumedByOther)
                    if res == Resolution::ToFailed {
                        let legal = matches!((&r1, &r2), (Reverted { .. }, Reverted { .. }))
                            || (matches!((&r1, &r2), (None, None)) && n == NonceState::ConsumedByOther);
                        assert!(legal, "illegal ToFailed for {r1:?},{r2:?},{n:?}");
                    }
                    // Свойство 3: Unreachable в любой позиции НИКОГДА не даёт ToFailed
                    if matches!(r1, Unreachable) || matches!(r2, Unreachable) {
                        assert_ne!(res, Resolution::ToFailed);
                    }
                }
            }
        }
    }

    #[test]
    fn mempool_tx_is_kept() {
        // receipt нет, nonce не достигнут → tx может быть в mempool → Keep, не Failed
        assert_eq!(resolve(&None, &None, NonceState::NotReached), Resolution::Keep);
    }
    #[test]
    fn divergent_receipts_kept() {
        let s = Success { block: 1, gas_used: 1 };
        assert_eq!(resolve(&s, &None, NonceState::Unknown), Resolution::Keep);
    }
    #[test]
    fn consumed_by_other_closes_failed() {
        assert_eq!(resolve(&None, &None, NonceState::ConsumedByOther), Resolution::ToFailed);
    }
}
