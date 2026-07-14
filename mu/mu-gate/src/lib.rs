//! M8 mu-gate — периметр. Всё недоверенное умирает здесь.
//!
//! RISK-M8-1: replay — монотонный nonce на агента, переживает рестарт (восстановление из лога).
//! RISK-M8-2: подпись покрывает ВЕСЬ кадр включая agent_id; pubkey берётся из allowlist по id.
//! RISK-M8-3: hardened-парсер — фикс. схема, лимиты размеров, без паник.
//! RISK-M8-5: единый deny{code}, без деталей Δ/Ω.
#![forbid(unsafe_code)]

pub mod wire;
pub mod gate;

pub use gate::{AllowList, DenyCode, Gate, VerifiedIntent};
pub use wire::{parse_wire, WireIntent, WireErr, MAX_FRAME};
