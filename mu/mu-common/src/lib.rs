//! X mu-common — сквозной слой: Amount, Clock, идентификаторы.
//! RISK-X-1: Amount не имеет операторов +,-,* — только checked_*.
//! RISK-X-2: два несмешиваемых времени (unix u64 vs монотонный Instant).
#![forbid(unsafe_code)]

pub mod amount;
pub mod clock;
pub mod ids;

pub use amount::{parse_decimal, Amount, AmtErr};
pub use clock::{Clock, SysClock};
pub use ids::{CanonAddress, ConnectorId, Hash32, Ticket};
