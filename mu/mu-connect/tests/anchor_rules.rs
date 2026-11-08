//! P11c (Spec S5 §3.1 step 4 and §3.2, T7): the owner's rules for anchors — a
//! budget per capsule and in all per day, a cap per anchor, a list of the capsules
//! allowed to pay (one wallet pays for all; no refill).
//! Pure decisions: no network, no key.

use mu_connect::anchor_rules::{decide, AnchorRules, Denied, Ledger};

fn rules() -> AnchorRules {
    AnchorRules {
        per_anchor_cap_lamports: 10_000,
        daily_budget_per_capsule_lamports: 30_000,
        daily_budget_total_lamports: 50_000,
        allowed_capsules: vec!["1220aa".into(), "1220bb".into()],
    }
}

#[test]
fn a_capsule_not_on_the_list_pays_nothing() {
    let l = Ledger::default();
    assert_eq!(decide(&rules(), &l, "1220cc", 5_000, 1), Err(Denied::NotAllowed));
}

#[test]
fn the_cap_per_anchor_is_the_rules_too() {
    let l = Ledger::default();
    assert_eq!(decide(&rules(), &l, "1220aa", 10_000, 1), Ok(()));
    assert_eq!(decide(&rules(), &l, "1220aa", 10_001, 1), Err(Denied::Cap { fee: 10_001, cap: 10_000 }));
}

#[test]
fn the_daily_budget_per_capsule_and_in_all_runs_out() {
    let mut l = Ledger::default();
    // Three anchors of 10 000 use up the capsule's 30 000; the fourth is denied.
    for _ in 0..3 {
        decide(&rules(), &l, "1220aa", 10_000, 1).expect("within the budget");
        l.record("1220aa", 1, 10_000);
    }
    assert_eq!(
        decide(&rules(), &l, "1220aa", 1, 1),
        Err(Denied::BudgetCapsule { spent: 30_000, budget: 30_000 })
    );
    // Another capsule still has its own budget...
    decide(&rules(), &l, "1220bb", 10_000, 1).expect("capsule b is untouched");
    l.record("1220bb", 1, 10_000);
    l.record("1220bb", 1, 10_000);
    // ...until the budget in all is spent: 30 000 + 20 000 = 50 000.
    assert_eq!(
        decide(&rules(), &l, "1220bb", 1, 1),
        Err(Denied::BudgetTotal { spent: 50_000, budget: 50_000 })
    );
    // A new day opens new budgets.
    assert_eq!(decide(&rules(), &l, "1220aa", 10_000, 2), Ok(()));
}

