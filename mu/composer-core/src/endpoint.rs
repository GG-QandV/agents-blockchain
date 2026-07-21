//! Receiving side (per spec — in the daemon; here reference + end-to-end tests).
//! Pipeline §6: size → decode → mu-policy.validate → base_hash==current (TOCTOU) → apply.
//! Biometrics/owner signature (M9/M4) in this part is behind the apply closure: the real
//! M9→M4→M1 daemon path is injected here.
use crate::proposal::{decode_proposal, DeltaProposal, PROPOSAL_MAX};
use mu_common::Hash32;
use mu_policy::{delta_hash, validate, Delta, OmegaView, VErr};

#[derive(Debug, PartialEq, Eq)]
pub enum RejectCode {
    TooLarge,
    Malformed,
    Invalid(Vec<VErr>),
    Stale,        // TOCTOU: base_delta_hash != hash(current Δ)
    HumanDenied,
    Internal,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProposeOutcome {
    Applied { new_hash: Hash32 },
    Rejected(RejectCode),
}

/// Daemon policy state (simplified slice of M1/M9 for the endpoint).
pub struct DaemonPolicy {
    pub current: Delta,
    pub omega: OmegaView,
}

/// approve — closure "show diff and get biometrics" (real daemon: M9.confirm_delta).
pub fn handle_propose(
    state: &mut DaemonPolicy,
    raw: &[u8],
    approve: impl FnOnce(&Delta, &Delta) -> bool,
) -> ProposeOutcome {
    if raw.len() > PROPOSAL_MAX {
        return ProposeOutcome::Rejected(RejectCode::TooLarge);
    }
    let prop: DeltaProposal = match decode_proposal(raw) {
        Ok(p) => p,
        Err(_) => return ProposeOutcome::Rejected(RejectCode::Malformed),
    };
    // validation with THE SAME mu-policy as Composer (§4.2: zero divergence)
    let rep = validate(&prop.new_delta, &state.omega);
    if !rep.ok() {
        return ProposeOutcome::Rejected(RejectCode::Invalid(rep.errors));
    }
    // TOCTOU (§9.1): proposal built from STALE Δ → Stale
    let current_hash = delta_hash(&state.current);
    if prop.base_delta_hash != current_hash {
        return ProposeOutcome::Rejected(RejectCode::Stale);
    }
    // diff → biometrics (RISK-M9-1: diff is rendered by daemon from proposal, not from UI Composer)
    if !approve(&state.current, &prop.new_delta) {
        return ProposeOutcome::Rejected(RejectCode::HumanDenied);
    }
    state.current = prop.new_delta;
    ProposeOutcome::Applied { new_hash: delta_hash(&state.current) }
}
