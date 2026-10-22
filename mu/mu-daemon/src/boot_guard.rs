//! Boot guards: the owner key must match μ, and μ must be a link of the journal.
//!
//! A crash between the journal append and the μ save leaves μ one step behind;
//! that is caught up. Anything else — a shorter journal, a foreign μ, a
//! rollback — stops the boot instead of guessing.

use std::path::Path;

use mu_common::Hash32;
use mu_core::{CoreErr, Mu};
use mu_log::Log;
use mu_vault::Vault;

pub fn owner_keys_agree(mu: &Mu, vault: &dyn Vault) -> bool {
    match vault.owner_pubkey() {
        Some(pk) => pk == mu.owner_pubkey().to_vec(),
        None => true,
    }
}

pub fn catch_up(mu: &Mu, log: &Log, vault: &dyn Vault, mu_path: &Path) -> Result<Mu, CoreErr> {
    if mu.log_head() == log.last_hash() {
        return Ok(mu.clone());
    }
    // Genesis means "issued before the first record": the first save never
    // happened, which is the same crash as being one step behind.
    let genesis = mu.log_head() == Hash32([0u8; 32]);
    let known = genesis || log.entries().iter().any(|e| e.hash() == mu.log_head());
    if !known {
        return Err(CoreErr::LogHeadMismatch);
    }
    let next = mu.advanced(log.last_hash(), vault)?;
    next.save(mu_path)?;
    eprintln!("mu: caught up to the log tail");
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::{catch_up, owner_keys_agree};
    use mu_common::{Amount, Hash32};
    use mu_core::object::Omega;
    use mu_core::Mu;
    use mu_log::{Kind, Log};
    use mu_policy::{delta_hash, Delta};
    use mu_vault::backend::SoftVault;
    use mu_vault::{DomainTag, Vault};

    fn vault() -> SoftVault {
        SoftVault::for_test([1; 32], [2; 32], [3; 32])
    }

    fn owner_pk(v: &SoftVault) -> [u8; 33] {
        v.owner_pubkey().unwrap().try_into().unwrap()
    }

    fn issue(v: &SoftVault, log_head: Hash32) -> Mu {
        let d = Delta {
            daily_limit: Amount::from_minor(1_000_000),
            whitelist: vec![],
            confirm_threshold: Amount::ZERO,
            resource_allowlist: vec![],
        };
        let sig = v.owner_sign(DomainTag::MuDelta, &delta_hash(&d)).unwrap().0;
        Mu::issue(
            [7; 16],
            owner_pk(v),
            1,
            Omega { connectors: vec![0], max_ceiling: 1_000_000 },
            d,
            sig,
            b"ref".to_vec(),
            log_head,
            v,
        )
        .unwrap()
    }

    #[test]
    fn one_step_behind_the_journal_is_caught_up() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = Log::open(&dir.path().join("mu.log"), &v).unwrap();
        let mu = issue(&v, log.last_hash());
        // The daemon died after the journal append, before saving μ.
        log.append(Kind::Alert { code: 1 }, 10, &v).unwrap();
        let mu_path = dir.path().join("mu.bin");
        let next = catch_up(&mu, &log, &v, &mu_path).unwrap();
        assert_eq!(next.log_head(), log.last_hash());
        assert_eq!(Mu::load(&mu_path).unwrap().log_head(), log.last_hash(), "the catch-up is persisted");
    }

    #[test]
    fn mu_from_another_chain_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let mut log = Log::open(&dir.path().join("mu.log"), &v).unwrap();
        let foreign = issue(&v, mu_common::Hash32([9; 32]));
        log.append(Kind::Alert { code: 1 }, 10, &v).unwrap();
        match catch_up(&foreign, &log, &v, &dir.path().join("mu.bin")) {
            Err(mu_core::CoreErr::LogHeadMismatch) => {}
            other => panic!("expected LogHeadMismatch, got {other:?}"),
        }
    }

    #[test]
    fn mu_ahead_of_a_shorter_journal_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let log = Log::open(&dir.path().join("mu.log"), &v).unwrap();
        let ahead = issue(&v, mu_common::Hash32([5; 32]));
        match catch_up(&ahead, &log, &v, &dir.path().join("mu.bin")) {
            Err(mu_core::CoreErr::LogHeadMismatch) => {}
            other => panic!("expected LogHeadMismatch, got {other:?}"),
        }
    }

    #[test]
    fn a_vault_with_another_owner_key_is_refused() {
        let good = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        let other = SoftVault::for_test([1; 32], [9; 32], [3; 32]);
        let mu = issue(&good, mu_common::Hash32([0; 32]));
        assert!(owner_keys_agree(&mu, &good));
        assert!(!owner_keys_agree(&mu, &other), "a different owner key must not sign Δ");
    }

}
