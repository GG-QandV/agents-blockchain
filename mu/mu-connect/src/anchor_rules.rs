//! P11c (Spec S5 §3.1 step 4): the owner's rules for anchors.
//!
//! The rules are the owner's, set once: a cap per anchor, a budget per capsule and
//! in all per day, the list of the capsules allowed to be paid for. One wallet pays
//! for all of them (decision 07.11, variant B: no sub-wallet per capsule, no refill).
//! Everything here is a pure decision over numbers the caller hands in — no
//! network, no key, no clock (the day is a number the caller names).
//!
//! A denial is **not** a failure of the page: the page is written all the same and
//! its anchor waits in the reader's queue of debts (S5 §3.3); the denial says why.

use std::collections::BTreeMap;

/// The owner's rules for anchors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchorRules {
    /// The most one anchor may cost, lamports.
    pub per_anchor_cap_lamports: u64,
    /// The most one capsule's anchors may cost in a day, lamports.
    pub daily_budget_per_capsule_lamports: u64,
    /// The most all capsules together may cost in a day (anchors), lamports.
    pub daily_budget_total_lamports: u64,
    /// The capsules (container multihashes) allowed to pay; any other pays nothing.
    pub allowed_capsules: Vec<String>,
}

/// Why an anchor is not paid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Denied {
    /// The capsule is not on the owner's list.
    NotAllowed,
    /// The fee passes the cap per anchor.
    Cap {
        /// The fee, lamports.
        fee: u64,
        /// The cap, lamports.
        cap: u64,
    },
    /// The capsule's budget for the day is spent.
    BudgetCapsule {
        /// Spent today by this capsule, lamports.
        spent: u64,
        /// The budget, lamports.
        budget: u64,
    },
    /// The budget in all for the day is spent.
    BudgetTotal {
        /// Spent today by all, lamports.
        spent: u64,
        /// The budget, lamports.
        budget: u64,
    },
}

impl core::fmt::Display for Denied {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAllowed => f.write_str("the capsule is not on the owner's list"),
            Self::Cap { fee, cap } => write!(f, "the fee {fee} passes the cap {cap} per anchor"),
            Self::BudgetCapsule { spent, budget } => {
                write!(f, "the capsule's budget for the day is spent ({spent} of {budget})")
            }
            Self::BudgetTotal { spent, budget } => {
                write!(f, "the budget in all for the day is spent ({spent} of {budget})")
            }
        }
    }
}

impl std::error::Error for Denied {}

/// What was spent, by day: anchors per capsule.
#[derive(Clone, Debug, Default)]
pub struct Ledger {
    anchors: BTreeMap<(u64, String), u64>,
}

impl Ledger {
    /// Notes an anchor paid for `capsule` on `day`.
    pub fn record(&mut self, capsule: &str, day: u64, fee_lamports: u64) {
        let e = self.anchors.entry((day, capsule.to_owned())).or_insert(0);
        *e = e.saturating_add(fee_lamports);
    }

    fn spent_by(&self, capsule: &str, day: u64) -> u64 {
        self.anchors.get(&(day, capsule.to_owned())).copied().unwrap_or(0)
    }

    fn spent_in_all(&self, day: u64) -> u64 {
        self.anchors
            .iter()
            .filter(|((d, _), _)| *d == day)
            .map(|(_, v)| *v)
            .fold(0u64, u64::saturating_add)
    }
}

/// Whether an anchor of `fee_lamports` for `capsule` may be paid on `day`.
///
/// # Errors
///
/// The first rule that does not hold, as [`Denied`].
pub fn decide(
    rules: &AnchorRules,
    ledger: &Ledger,
    capsule: &str,
    fee_lamports: u64,
    day: u64,
) -> Result<(), Denied> {
    if !rules.allowed_capsules.iter().any(|c| c == capsule) {
        return Err(Denied::NotAllowed);
    }
    if fee_lamports > rules.per_anchor_cap_lamports {
        return Err(Denied::Cap { fee: fee_lamports, cap: rules.per_anchor_cap_lamports });
    }
    let by_capsule = ledger.spent_by(capsule, day);
    if by_capsule.saturating_add(fee_lamports) > rules.daily_budget_per_capsule_lamports {
        return Err(Denied::BudgetCapsule {
            spent: by_capsule,
            budget: rules.daily_budget_per_capsule_lamports,
        });
    }
    let in_all = ledger.spent_in_all(day);
    if in_all.saturating_add(fee_lamports) > rules.daily_budget_total_lamports {
        return Err(Denied::BudgetTotal { spent: in_all, budget: rules.daily_budget_total_lamports });
    }
    Ok(())
}
