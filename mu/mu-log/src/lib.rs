//! M5 mu-log — WAL + hash-chain, источник истины для лимитов и аудита.
//!
//! RISK-M5-1: append = write → flush/fsync → Ok; WalWritten у M6 порождается только этим Ok.
//! RISK-M5-2: hash-chain + подпись каждой записи (домен mu.log.v1) + HEAD; фальсификация → verify Err.
//! RISK-M5-3: reconcile — табличная résolution, default = Keep (никакого «else → Failed»).
//! RISK-M5-4: window_sum — единственная реализация окна 24ч; Simulated/Failed исключены.
//! RISK-M5-5: битый хвост → truncate до последней валидной + запись TailTruncated.
#![forbid(unsafe_code)]

pub mod entry;
pub mod store;
pub mod reconcile;

pub use entry::{Entry, Kind};
pub use reconcile::{resolve, Resolution};
pub use store::{Log, LogErr};
