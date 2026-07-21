//! mu-policy — single source of truth for Δ rules.
//! Linked into the daemon (proposal validation) and into Composer (C2) — zero divergence.
//! Spec: SPEC_Delta-Composer_v1 §4.2, §7.
#![forbid(unsafe_code)]

pub mod types;
pub mod validate;
pub mod sui_addr;
pub mod hash;

pub use types::{Delta, OmegaView, WlEntry};
pub use validate::{validate, VErr, VWarn, ValidationReport};
pub use hash::delta_hash;
pub use sui_addr::canon_address_checked;
