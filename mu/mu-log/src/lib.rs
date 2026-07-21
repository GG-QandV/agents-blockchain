//! M5 mu-log — WAL + hash-chain, source of truth for limits and audit.
//!
//! RISK-M5-1: append = write → flush/fsync → Ok; WalWritten in M6 is only produced by this Ok.
//! RISK-M5-2: hash-chain + signature of each entry (domain mu.log.v1) + HEAD; forgery → verify Err.
//! RISK-M5-3: reconcile — tabular resolution, default = Keep (no "else → Failed" branch).
//! RISK-M5-4: window_sum — single 24h window implementation; Simulated/Failed excluded.
//! RISK-M5-5: corrupted tail → truncate to last valid + write TailTruncated entry.
#![forbid(unsafe_code)]

pub mod entry;
pub mod store;
pub mod reconcile;

pub use entry::{Entry, Kind};
pub use reconcile::{resolve, Resolution};
pub use store::{Log, LogErr};
