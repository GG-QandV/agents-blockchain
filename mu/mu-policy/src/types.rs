use mu_common::{Amount, CanonAddress};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WlEntry {
    pub address: CanonAddress,
    pub label: String, // ≤64, без управляющих/bidi (E-LBL-01)
}

/// Правило доступу до ресурсу по URL (хост+шлях) для x402.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceRule {
    pub host: String,
    pub path_prefix: String,
    pub max_price_per_call: Amount,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Delta {
    pub daily_limit: Amount,
    pub whitelist: Vec<WlEntry>,
    pub confirm_threshold: Amount,
    pub resource_allowlist: Vec<ResourceRule>,
}

/// Read-only срез Ω для валидации (демон отдаёт по GetPolicy).
#[derive(Clone, Debug)]
pub struct OmegaView {
    pub max_ceiling: Amount,
    pub supported_chain_ids: Vec<u64>,
}
