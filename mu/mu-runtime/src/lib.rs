//! M6 mu-runtime — оркестратор конвейера.
//!
//! RISK-M6-1: типовое принуждение «WAL до денег» — connector.execute доступен только
//!   через exec_after_wal(WalWritten, ...), а WalWritten порождается ТОЛЬКО write_wal().
//! RISK-M6-2: commit ⇐ Settled-консенсус; rollback ⇐ доказанный неуход (RollbackCause без Unknown).
//! RISK-M6-3: сериализация — обработка по одному intent (метод process — &mut self).
//! RISK-M6-5: Unknown/таймаут → ReconcilePending, резерв ДЕРЖИТСЯ (Pending остаётся в логе).
#![forbid(unsafe_code)]

pub mod policy;
pub mod pipeline;

pub use pipeline::{IntentStatus, Runtime};
