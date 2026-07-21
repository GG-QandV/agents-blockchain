//! composer-core — C2 (reexport mu-policy), C3 drafts, C4 proposal+transport,
//! plus daemon receiving endpoint (per spec lives in the daemon; here for end-to-end tests
//! and as reference: the same mu-policy validation on both sides).
#![forbid(unsafe_code)]

pub mod proposal;
pub mod drafts;
pub mod client;
pub mod endpoint;

pub use proposal::{decode_proposal, encode_proposal, DeltaProposal, PROPOSAL_MAX};
pub use endpoint::{handle_propose, DaemonPolicy, ProposeOutcome, RejectCode};
