//! M6 mu-runtime — pipeline orchestrator.
//!
//! RISK-M6-1: type-level enforcement "WAL before money" — connector.execute is accessible only
//!   through exec_after_wal(WalWritten, ...), and WalWritten is produced ONLY by write_wal().
//! RISK-M6-2: commit ⇐ Settled-consensus; rollback ⇐ proven non-delivery (RollbackCause without Unknown).
//! RISK-M6-3: serialization — one intent at a time (method process — &mut self).
//! RISK-M6-5: Unknown/timeout → ReconcilePending, reserve is HELD (Pending remains in log).
#![forbid(unsafe_code)]

pub mod policy;
pub mod pipeline;

pub use pipeline::{IntentStatus, Runtime};
