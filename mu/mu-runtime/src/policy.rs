//! Minimal Ω/Δ for pipeline operation.
//! NB: per module map these are M2/M3 (separate crates mu-omega/mu-delta);
//! here they are embedded in runtime as temporary placement — interfaces match the specs.
use mu_common::{Amount, CanonAddress, ConnectorId};
use mu_log::Log;

// ── Ω (M2): static capability filter ─────────────────────────────────
pub struct Omega {
    pub connectors: Vec<ConnectorId>,
    pub max_ceiling: Amount,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OmegaDeny {
    ConnectorUnknown,
    CeilingExceeded,
}

/// Sui gasless: total = amount (no gas_estimate).
pub fn omega_check(
    amount: Amount,
    connector: ConnectorId,
    o: &Omega,
) -> Result<Amount, OmegaDeny> {
    if !o.connectors.contains(&connector) {
        return Err(OmegaDeny::ConnectorUnknown);
    }
    if amount > o.max_ceiling {
        return Err(OmegaDeny::CeilingExceeded);
    }
    Ok(amount)
}

// ── Δ (M3): moment policies ──────────────────────────────────────────────
pub struct Delta {
    pub daily_limit: Amount,
    pub whitelist: Vec<CanonAddress>,
    pub confirm_threshold: Amount,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeltaDeny {
    NotWhitelisted,
    WindowExceeded,
}

/// RISK-M3-2: membership only over CanonAddress (no strings here by type).
/// RISK-M5-4: window counts ONLY the log.
pub fn delta_check(
    recipient: &CanonAddress,
    amount_total: Amount,
    d: &Delta,
    log: &Log,
    now: u64,
) -> Result<(), DeltaDeny> {
    if !d.whitelist.contains(recipient) {
        return Err(DeltaDeny::NotWhitelisted);
    }
    let spent = log.window_sum(now);
    let would = spent.checked_add(amount_total).ok_or(DeltaDeny::WindowExceeded)?;
    if would > d.daily_limit {
        return Err(DeltaDeny::WindowExceeded);
    }
    Ok(())
}

/// RISK-M3-3: threshold compared against the same total value as the reserve.
pub fn needs_human(amount_total: Amount, d: &Delta) -> bool {
    amount_total > d.confirm_threshold
}
