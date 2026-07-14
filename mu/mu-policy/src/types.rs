use mu_common::{Amount, CanonAddress};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WlEntry {
    pub address: CanonAddress,
    pub label: String, // ≤64, без управляющих/bidi (E-LBL-01)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Delta {
    pub daily_limit: Amount,
    pub whitelist: Vec<WlEntry>,
    pub confirm_threshold: Amount,
}

/// Read-only срез Ω для валидации (демон отдаёт по GetPolicy).
#[derive(Clone, Debug)]
pub struct OmegaView {
    pub max_ceiling: Amount,
    pub supported_chain_ids: Vec<u64>,
}
