//! M8 mu-gate — perimeter. Everything untrusted dies here.
//!
//! RISK-M8-1: replay — monotonic nonce per agent, survives restart (restored from log).
//! RISK-M8-2: signature covers ENTIRE frame including agent_id; pubkey taken from allowlist by id.
//! RISK-M8-3: hardened parser — fixed schema, size limits, no panics.
//! RISK-M8-5: single deny{code}, no Δ/Ω details.
#![forbid(unsafe_code)]

pub mod wire;
pub mod gate;

pub use gate::{AllowList, DenyCode, Gate, VerifiedIntent};
pub use wire::{parse_wire, WireIntent, WireErr, MAX_FRAME};
