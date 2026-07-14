//! composer-core — C2 (реэкспорт mu-policy), C3 drafts, C4 proposal+transport,
//! плюс приёмный endpoint демона (по спеке живёт в демоне; здесь для сквозных тестов
//! и как референс: та же mu-policy-валидация с обеих сторон).
#![forbid(unsafe_code)]

pub mod proposal;
pub mod drafts;
pub mod client;
pub mod endpoint;

pub use proposal::{decode_proposal, encode_proposal, DeltaProposal, PROPOSAL_MAX};
pub use endpoint::{handle_propose, DaemonPolicy, ProposeOutcome, RejectCode};
