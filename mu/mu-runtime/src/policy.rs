//! Минимальные Ω/Δ для работы конвейера.
//! NB: по модульной карте это M2/M3 (отдельные крейты mu-omega/mu-delta);
//! здесь встроены в runtime как временное размещение — интерфейсы совпадают со спеками.
use mu_common::{Amount, CanonAddress, ConnectorId};
use mu_log::Log;

// ── Ω (M2): статичный фильтр возможностей ─────────────────────────────────
pub struct Omega {
    pub connectors: Vec<ConnectorId>,
    pub max_ceiling: Amount,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OmegaDeny {
    ConnectorUnknown,
    CeilingExceeded,
}

/// RISK-M2-1: checked-арифметика; переполнение = Deny, не паника.
pub fn omega_check(
    amount: Amount,
    gas_estimate: Amount,
    connector: ConnectorId,
    o: &Omega,
) -> Result<Amount, OmegaDeny> {
    if !o.connectors.contains(&connector) {
        return Err(OmegaDeny::ConnectorUnknown);
    }
    let total = amount.checked_add(gas_estimate).ok_or(OmegaDeny::CeilingExceeded)?;
    if total > o.max_ceiling {
        return Err(OmegaDeny::CeilingExceeded);
    }
    Ok(total)
}

// ── Δ (M3): политики момента ──────────────────────────────────────────────
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

/// RISK-M3-2: membership только над CanonAddress (строк здесь нет по типам).
/// RISK-M5-4: окно считает ТОЛЬКО лог.
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

/// RISK-M3-3: порог сравнивается с той же величиной total, что и резерв.
pub fn needs_human(amount_total: Amount, d: &Delta) -> bool {
    amount_total > d.confirm_threshold
}
