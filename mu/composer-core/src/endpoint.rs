//! Приёмная сторона (по спеке — в демоне; здесь референс + сквозные тесты).
//! Конвейер §6: размер → decode → mu-policy.validate → base_hash==current (TOCTOU) → apply.
//! Биометрия/подпись owner (M9/M4) в этой части — за замыканием apply: сюда подставляется
//! реальный путь M9→M4→M1 демона.
use crate::proposal::{decode_proposal, DeltaProposal, PROPOSAL_MAX};
use mu_common::Hash32;
use mu_policy::{delta_hash, validate, Delta, OmegaView, VErr};

#[derive(Debug, PartialEq, Eq)]
pub enum RejectCode {
    TooLarge,
    Malformed,
    Invalid(Vec<VErr>),
    Stale,        // TOCTOU: base_delta_hash != hash(текущей Δ)
    HumanDenied,
    Internal,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProposeOutcome {
    Applied { new_hash: Hash32 },
    Rejected(RejectCode),
}

/// Состояние политики демона (упрощённый срез M1/M9 для endpoint'а).
pub struct DaemonPolicy {
    pub current: Delta,
    pub omega: OmegaView,
}

/// approve — замыкание «покажи diff и получи биометрию» (реальный демон: M9.confirm_delta).
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
    // валидация ТЕМ ЖЕ mu-policy, что в Composer (§4.2: ноль расхождений)
    let rep = validate(&prop.new_delta, &state.omega);
    if !rep.ok() {
        return ProposeOutcome::Rejected(RejectCode::Invalid(rep.errors));
    }
    // TOCTOU (§9.1): предложение построено от УСТАРЕВШЕЙ Δ → Stale
    let current_hash = delta_hash(&state.current);
    if prop.base_delta_hash != current_hash {
        return ProposeOutcome::Rejected(RejectCode::Stale);
    }
    // diff → биометрия (RISK-M9-1: diff рендерится демоном из proposal, не из UI Composer)
    if !approve(&state.current, &prop.new_delta) {
        return ProposeOutcome::Rejected(RejectCode::HumanDenied);
    }
    state.current = prop.new_delta;
    ProposeOutcome::Applied { new_hash: delta_hash(&state.current) }
}
