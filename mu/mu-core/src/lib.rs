//! M1 mu-core — sole owner of the μ-object format.
//!
//! RISK-M1-1: Ω immutable outside reissue — field is private (no &mut), apply_delta
//!            byte-compares Ω before/after rebuild (defense-in-depth).
//! RISK-M1-2: rollback attack — verify_against_log checks μ.log_head against M5 chain tail.
//! RISK-M1-3: atomic write tmp+fsync+rename+fsync(dir); NO auto-rollback from mu.prev.
//! RISK-M1-4: strict hardened parser (mu-wire family): extra byte = CborMalformed.
#![forbid(unsafe_code)]

pub mod format;
pub mod object;

pub use object::{Mu, CoreErr, MU_MAX_SIZE};
