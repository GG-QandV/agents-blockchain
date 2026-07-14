//! Сквозные тесты Composer: TOCTOU-гонка, отказ владельца, троян-подмена (критерий приёмки §14.5).
use composer_core::{encode_proposal, handle_propose, DaemonPolicy, DeltaProposal, ProposeOutcome, RejectCode};
use composer_core::drafts::{load_draft, save_draft};
use mu_common::{Amount, CanonAddress, Hash32};
use mu_policy::{delta_hash, Delta, OmegaView, WlEntry};

fn addr(x: &str) -> CanonAddress { CanonAddress::canon(x, 8453).unwrap() }
fn base_delta() -> Delta {
    Delta {
        daily_limit: Amount::from_minor(500),
        whitelist: vec![WlEntry { address: addr("0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), label: "API".into() }],
        confirm_threshold: Amount::from_minor(100),
    }
}
fn daemon() -> DaemonPolicy {
    DaemonPolicy {
        current: base_delta(),
        omega: OmegaView { max_ceiling: Amount::from_minor(1000), supported_chain_ids: vec![8453] },
    }
}
fn proposal_from(current: &Delta, new_limit: u128) -> DeltaProposal {
    let mut d = current.clone();
    d.daily_limit = Amount::from_minor(new_limit);
    DeltaProposal { new_delta: d, base_delta_hash: delta_hash(current), ts: 1 }
}

#[test]
fn happy_path_applied() {
    let mut st = daemon();
    let raw = encode_proposal(&proposal_from(&st.current, 700)).unwrap();
    let out = handle_propose(&mut st, &raw, |_old, _new| true);
    assert!(matches!(out, ProposeOutcome::Applied { .. }));
    assert_eq!(st.current.daily_limit, Amount::from_minor(700));
}

#[test]
fn toctou_race_one_applied_one_stale() {
    // Критерий приёмки §14.3: два Composer'а от одной базы → ровно один Applied, второй Stale
    let mut st = daemon();
    let p1 = encode_proposal(&proposal_from(&st.current, 600)).unwrap();
    let p2 = encode_proposal(&proposal_from(&st.current, 800)).unwrap(); // от той же базы
    assert!(matches!(handle_propose(&mut st, &p1, |_, _| true), ProposeOutcome::Applied { .. }));
    assert_eq!(handle_propose(&mut st, &p2, |_, _| true), ProposeOutcome::Rejected(RejectCode::Stale));
    assert_eq!(st.current.daily_limit, Amount::from_minor(600)); // вторая не применилась
}

#[test]
fn human_denied_no_change() {
    let mut st = daemon();
    let raw = encode_proposal(&proposal_from(&st.current, 700)).unwrap();
    let out = handle_propose(&mut st, &raw, |_, _| false);
    assert_eq!(out, ProposeOutcome::Rejected(RejectCode::HumanDenied));
    assert_eq!(st.current.daily_limit, Amount::from_minor(500));
}

#[test]
fn invalid_rejected_by_daemon_side_validation() {
    // §4.2: даже если UI Composer сломан и пропустил ошибку — демон валидирует тем же mu-policy
    let mut st = daemon();
    let mut bad = proposal_from(&st.current, 5000); // > ceiling 1000
    bad.base_delta_hash = delta_hash(&st.current);
    let raw = encode_proposal(&bad).unwrap();
    let out = handle_propose(&mut st, &raw, |_, _| true);
    assert!(matches!(out, ProposeOutcome::Rejected(RejectCode::Invalid(_))));
}

#[test]
fn trojan_composer_swap_visible_in_diff() {
    // Критерий приёмки §14.5: троян подменяет адрес ПОСЛЕ показа в UI Composer'а.
    // Демон рендерит diff из proposal → подмена видна владельцу в approve-замыкании.
    let mut st = daemon();
    let mut d = st.current.clone();
    d.whitelist.push(WlEntry { address: addr("0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"), label: "Evil".into() });
    let prop = DeltaProposal { new_delta: d, base_delta_hash: delta_hash(&st.current), ts: 1 };
    let raw = encode_proposal(&prop).unwrap();
    let mut owner_saw_evil = false;
    let out = handle_propose(&mut st, &raw, |old, new| {
        // диалог демона строится ИЗ proposal: владелец видит фактический новый адрес
        owner_saw_evil = new.whitelist.len() > old.whitelist.len()
            && new.whitelist.iter().any(|e| e.label == "Evil");
        false // владелец, увидев подмену, отклоняет
    });
    assert!(owner_saw_evil);
    assert_eq!(out, ProposeOutcome::Rejected(RejectCode::HumanDenied));
}

#[test]
fn draft_roundtrip_and_corrupt_dropped() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("draft.bin");
    let p = proposal_from(&base_delta(), 700);
    save_draft(&p, &path).unwrap();
    assert_eq!(load_draft(&path).unwrap(), Some(p));
    // порча файла → None, не паника (§10)
    std::fs::write(&path, b"garbage").unwrap();
    assert_eq!(load_draft(&path).unwrap(), None);
}
