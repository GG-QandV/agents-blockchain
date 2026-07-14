//! Таблицы валидации §7 спеки: E-* блокируют, W-* предупреждают.
use crate::types::{Delta, OmegaView};
use mu_common::Amount;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VErr {
    ELim01ZeroLimit,
    ELim02AboveCeiling,
    EThr01ThresholdAboveLimit,
    EAdr03ChainUnsupported { chain_id: u64 },
    EAdr04Duplicate,
    ELbl01BadLabel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VWarn {
    WLim01LimitEqualsCeiling,
    WWl01EmptyWhitelist,
    WThr01ZeroThreshold,
    WThr02ThresholdEqualsLimit,
}

#[derive(Clone, Debug, Default)]
pub struct ValidationReport {
    pub errors: Vec<VErr>,
    pub warnings: Vec<VWarn>,
}
impl ValidationReport {
    pub fn ok(&self) -> bool { self.errors.is_empty() }
}

/// bidi/управляющие в label (E-LBL-01, спуфинг diff — RISK-M9-3).
fn label_ok(l: &str) -> bool {
    if l.chars().count() > 64 { return false; }
    !l.chars().any(|c| {
        c.is_control()
            || matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{200E}' | '\u{200F}')
    })
}

pub fn validate(new: &Delta, omega: &OmegaView) -> ValidationReport {
    let mut r = ValidationReport::default();

    // лимиты
    if new.daily_limit == Amount::ZERO {
        r.errors.push(VErr::ELim01ZeroLimit);
    }
    if new.daily_limit > omega.max_ceiling {
        r.errors.push(VErr::ELim02AboveCeiling);
    } else if new.daily_limit == omega.max_ceiling {
        r.warnings.push(VWarn::WLim01LimitEqualsCeiling);
    }
    if new.confirm_threshold > new.daily_limit {
        r.errors.push(VErr::EThr01ThresholdAboveLimit);
    } else if new.confirm_threshold == new.daily_limit && new.daily_limit != Amount::ZERO {
        r.warnings.push(VWarn::WThr02ThresholdEqualsLimit);
    }
    if new.confirm_threshold == Amount::ZERO {
        r.warnings.push(VWarn::WThr01ZeroThreshold);
    }

    // whitelist
    if new.whitelist.is_empty() {
        r.warnings.push(VWarn::WWl01EmptyWhitelist);
    }
    let mut seen: HashSet<([u8; 32], u64)> = HashSet::new();
    for e in &new.whitelist {
        if !omega.supported_chain_ids.contains(&e.address.chain_id()) {
            r.errors.push(VErr::EAdr03ChainUnsupported { chain_id: e.address.chain_id() });
        }
        if !seen.insert((*e.address.bytes(), e.address.chain_id())) {
            r.errors.push(VErr::EAdr04Duplicate);
        }
        if !label_ok(&e.label) {
            r.errors.push(VErr::ELbl01BadLabel);
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::WlEntry;
    use mu_common::CanonAddress;

    fn omega() -> OmegaView {
        OmegaView { max_ceiling: Amount::from_minor(1000), supported_chain_ids: vec![8453] }
    }
    fn wl(a: &str, chain: u64, l: &str) -> WlEntry {
        WlEntry { address: CanonAddress::canon(a, chain).unwrap(), label: l.into() }
    }
    fn base() -> Delta {
        Delta {
            daily_limit: Amount::from_minor(500),
            whitelist: vec![wl("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 8453, "A")],
            confirm_threshold: Amount::from_minor(100),
        }
    }

    #[test]
    fn happy_path_clean() {
        let r = validate(&base(), &omega());
        assert!(r.ok());
        assert!(r.warnings.is_empty());
    }
    #[test]
    fn e_lim_01_02() {
        let mut d = base(); d.daily_limit = Amount::ZERO;
        assert!(validate(&d, &omega()).errors.contains(&VErr::ELim01ZeroLimit));
        let mut d = base(); d.daily_limit = Amount::from_minor(1001);
        assert!(validate(&d, &omega()).errors.contains(&VErr::ELim02AboveCeiling));
    }
    #[test]
    fn e_thr_01() {
        let mut d = base(); d.confirm_threshold = Amount::from_minor(501);
        assert!(validate(&d, &omega()).errors.contains(&VErr::EThr01ThresholdAboveLimit));
    }
    #[test]
    fn e_adr_03_04() {
        let mut d = base();
        d.whitelist.push(wl("0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", 1, "Eth")); // chain 1 не поддержан
        assert!(matches!(validate(&d, &omega()).errors[0], VErr::EAdr03ChainUnsupported { chain_id: 1 }));
        let mut d = base();
        d.whitelist.push(wl("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 8453, "Dup"));
        assert!(validate(&d, &omega()).errors.contains(&VErr::EAdr04Duplicate));
    }
    #[test]
    fn e_lbl_01_bidi_and_control() {
        let mut d = base();
        d.whitelist[0].label = "evil\u{202E}txt".into(); // RLO bidi
        assert!(validate(&d, &omega()).errors.contains(&VErr::ELbl01BadLabel));
        let mut d = base();
        d.whitelist[0].label = "bad\u{0007}bell".into();
        assert!(validate(&d, &omega()).errors.contains(&VErr::ELbl01BadLabel));
    }
    #[test]
    fn warnings_table() {
        let mut d = base(); d.daily_limit = Amount::from_minor(1000); d.confirm_threshold = Amount::from_minor(1000);
        let r = validate(&d, &omega());
        assert!(r.warnings.contains(&VWarn::WLim01LimitEqualsCeiling));
        assert!(r.warnings.contains(&VWarn::WThr02ThresholdEqualsLimit));
        let mut d = base(); d.whitelist.clear(); d.confirm_threshold = Amount::ZERO;
        let r = validate(&d, &omega());
        assert!(r.warnings.contains(&VWarn::WWl01EmptyWhitelist));
        assert!(r.warnings.contains(&VWarn::WThr01ZeroThreshold));
    }
}
