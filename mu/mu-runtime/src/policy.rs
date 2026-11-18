//! Минимальные Ω/Δ для работы конвейера.
//! NB: по модульной карте это M2/M3 (отдельные крейты mu-omega/mu-delta);
//! здесь встроены в runtime как временное размещение — интерфейсы совпадают со спеками.
use mu_common::{Amount, CanonAddress, ConnectorId, Hash32};
use sha2::{Digest, Sha256};
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
    pub resource_allowlist: Vec<mu_policy::ResourceRule>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeltaDeny {
    NotWhitelisted,
    WindowExceeded,
    ResourceDenied,
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

/// Перевірка resource_allowlist для x402 (хост+шлях).
/// Повертає Ok, якщо ресурс дозволений і ціна ≤ max_price_per_call.
pub fn resource_check(
    host: &str,
    path: &str,
    price: Amount,
    d: &Delta,
) -> Result<(), DeltaDeny> {
    if d.resource_allowlist.is_empty() {
        // Пустий список = всі ресурси дозволені (зворотна сумісність)
        return Ok(());
    }
    for rule in &d.resource_allowlist {
        if host == rule.host && path.starts_with(&rule.path_prefix) {
            if price <= rule.max_price_per_call {
                return Ok(());
            } else {
                return Err(DeltaDeny::ResourceDenied);
            }
        }
    }
    Err(DeltaDeny::ResourceDenied)
}

/// RISK-M3-3: порог сравнивается с той же величиной total, что и резерв.
pub fn needs_human(amount_total: Amount, d: &Delta) -> bool {
    amount_total > d.confirm_threshold
}

/// Wire byte of a connector in `Kind::Quoted`.
pub fn connector_byte(c: ConnectorId) -> u8 {
    match c {
        ConnectorId::Crypto => 1,
        ConnectorId::BankStub => 2,
        ConnectorId::CardStub => 3,
        _ => 0,
    }
}

/// Deterministic hash of the Ω/Δ a decision was made under; logged in `Kind::Quoted`
/// so a reader can tell which rules were in force and recompute the verdict.
pub fn policy_hash(o: &Omega, d: &Delta) -> Hash32 {
    let mut h = Sha256::new();
    h.update(b"mu.policy.rt.v1");
    h.update(o.max_ceiling.minor().to_be_bytes());
    h.update((o.connectors.len() as u32).to_be_bytes());
    // order-insensitive: omega_check uses `contains`, so a permutation is the same policy
    let mut cs: Vec<u8> = o.connectors.iter().map(|c| connector_byte(*c)).collect();
    cs.sort_unstable();
    for c in cs { h.update([c]); }
    h.update(d.daily_limit.minor().to_be_bytes());
    h.update(d.confirm_threshold.minor().to_be_bytes());
    h.update((d.whitelist.len() as u32).to_be_bytes());
    for a in &d.whitelist { h.update(a.bytes()); h.update(a.chain_id().to_be_bytes()); }
    h.update((d.resource_allowlist.len() as u32).to_be_bytes());
    for r in &d.resource_allowlist {
        h.update((r.host.len() as u32).to_be_bytes()); h.update(r.host.as_bytes());
        h.update((r.path_prefix.len() as u32).to_be_bytes()); h.update(r.path_prefix.as_bytes());
        h.update(r.max_price_per_call.minor().to_be_bytes());
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&h.finalize());
    Hash32(out)
}

/// The cap a resource rule puts on `host`/`path`, if one matches (for the denial record).
pub fn resource_cap(host: &str, path: &str, d: &Delta) -> u128 {
    d.resource_allowlist
        .iter()
        .find(|r| host == r.host && path.starts_with(&r.path_prefix))
        .map(|r| r.max_price_per_call.minor())
        .unwrap_or(0)
}
