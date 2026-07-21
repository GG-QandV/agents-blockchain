//! X mu-common — cross-cutting layer: Amount, Clock, identifiers.
//! RISK-X-1: Amount has no +,-,* operators — only checked_*.
//! RISK-X-2: two non-interchangeable times (unix u64 vs monotonic Instant).
#![forbid(unsafe_code)]

pub mod amount;
pub mod clock;
pub mod ids;

pub use amount::{parse_decimal, Amount, AmtErr};
pub use clock::{Clock, SysClock};
pub use ids::{CanonAddress, ConnectorId, Hash32, Ticket};
