//! mu-policy — единственный источник правды о правилах Δ.
//! Линкуется в демон (валидация proposal) и в Composer (C2) — ноль расхождений.
//! Спека: SPEC_Delta-Composer_v1 §4.2, §7.
#![forbid(unsafe_code)]

pub mod types;
pub mod validate;
pub mod sui_addr;
pub mod hash;

pub use types::{Delta, OmegaView, WlEntry};
pub use validate::{validate, VErr, VWarn, ValidationReport};
pub use hash::delta_hash;
pub use sui_addr::canon_address_checked;
