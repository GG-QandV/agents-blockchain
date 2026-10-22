//! P0-7: the policy Δ lives in the journal, `policy.toml` is a cache of it.
//!
//! Order of a change: owner signature → journal record (fsync inside `append`)
//! → atomic file replace → μ advanced to the new log tail. Recovery reads the
//! last `DeltaApplied`:
//!
//! - the file matches it → nothing to do;
//! - the file is older but matches an earlier journal version → rewritten from
//!   the journal (the daemon died between the two writes);
//! - the file is newer, or matches no journal version, or a signature is wrong
//!   → the daemon does not start. A planted `policy.toml` with looser limits
//!   never takes effect, because the journal is what counts.

use std::path::Path;

use mu_common::Hash32;
use mu_core::{owner_sig_valid, Mu};
use mu_log::{Kind, Log};
use mu_policy::{delta_from_bytes, delta_hash, Delta};
use mu_vault::{DomainTag, Vault};
use sha2::{Digest, Sha256};

/// What the daemon runs with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyState {
    pub version: u16,
    pub delta: Delta,
}

#[derive(Debug)]
pub enum PolicyErr {
    /// The journal record is internally inconsistent or unsigned.
    BadRecord,
    /// The owner signature over (version, prev, next) does not verify.
    BadSignature,
    /// `policy.toml` claims a version the journal never produced.
    FileAhead(u16),
    /// `policy.toml` content matches no journal version: planted or edited.
    FileUnknown(u16),
    /// `policy.toml` is unreadable while the journal has a policy.
    File(String),
    Io(String),
    Vault(String),
}

impl From<std::io::Error> for PolicyErr {
    fn from(e: std::io::Error) -> Self {
        PolicyErr::Io(e.to_string())
    }
}

/// The owner signs this: the version the Δ becomes, and both hashes.
pub fn change_digest(version: u16, prev: &Hash32, next: &Hash32) -> Hash32 {
    let mut h = Sha256::new();
    h.update(b"mu.policy.v1");
    h.update(version.to_be_bytes());
    h.update(prev.0);
    h.update(next.0);
    let out = h.finalize();
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    Hash32(a)
}

struct Record {
    version: u16,
    prev: Hash32,
    next: Hash32,
    delta: Delta,
    owner_sig: [u8; 64],
}

/// All policy records in the journal, oldest first.
fn records(log: &Log) -> Result<Vec<Record>, PolicyErr> {
    let mut out = Vec::new();
    for e in log.entries() {
        if let Kind::DeltaApplied { version, prev, next, delta, owner_sig } = &e.kind {
            let delta = delta_from_bytes(delta).ok_or(PolicyErr::BadRecord)?;
            if delta_hash(&delta) != *next {
                return Err(PolicyErr::BadRecord);
            }
            out.push(Record {
                version: *version,
                prev: *prev,
                next: *next,
                delta,
                owner_sig: *owner_sig,
            });
        }
    }
    Ok(out)
}

/// The policy the daemon must run with, verified against the owner key.
pub fn current(log: &Log, owner_pubkey: &[u8]) -> Result<Option<PolicyState>, PolicyErr> {
    let recs = records(log)?;
    let Some(last) = recs.last() else { return Ok(None) };
    if !owner_sig_valid(owner_pubkey, &change_digest(last.version, &last.prev, &last.next), &last.owner_sig)
    {
        return Err(PolicyErr::BadSignature);
    }
    Ok(Some(PolicyState { version: last.version, delta: last.delta.clone() }))
}

/// Startup: take the policy from the journal and make `policy.toml` agree.
pub fn load(log: &Log, owner_pubkey: &[u8], policy_path: &Path) -> Result<PolicyState, PolicyErr> {
    let recs = records(log)?;
    let Some(last) = recs.last() else {
        // No journal record yet: the file written by the operator is the base.
        let text = std::fs::read_to_string(policy_path).map_err(|e| PolicyErr::File(e.to_string()))?;
        let delta = parse(&text)?;
        return Ok(PolicyState { version: 0, delta });
    };
    if !owner_sig_valid(owner_pubkey, &change_digest(last.version, &last.prev, &last.next), &last.owner_sig)
    {
        return Err(PolicyErr::BadSignature);
    }

    let text = std::fs::read_to_string(policy_path).map_err(|e| PolicyErr::File(e.to_string()))?;
    let file_version = mu_policy::config::parse_toml_version(&text)
        .map_err(|e| PolicyErr::File(format!("version: {e}")))?;
    let file_delta = parse(&text)?;
    let file_hash = delta_hash(&file_delta);

    if file_version > last.version {
        return Err(PolicyErr::FileAhead(file_version));
    }
    if file_version == last.version {
        if file_hash != last.next {
            return Err(PolicyErr::FileUnknown(file_version));
        }
        return Ok(PolicyState { version: last.version, delta: last.delta.clone() });
    }
    // The file lags. It is trustworthy only if it is a version the journal
    // really produced; then it is a crash between the two writes.
    // The file is trustworthy if it is a version the journal produced, or if it
    // is still the base the first record was built on (the crash case: the
    // daemon died before the first cache write).
    let known = recs.iter().any(|r| r.version == file_version && r.next == file_hash)
        || (file_version == 0 && recs[0].prev == file_hash);
    if !known {
        return Err(PolicyErr::FileUnknown(file_version));
    }
    write_file(policy_path, &last.delta, last.version)?;
    Ok(PolicyState { version: last.version, delta: last.delta.clone() })
}

fn parse(text: &str) -> Result<Delta, PolicyErr> {
    // 8453 is the base chain; addresses in the file carry their own chain id.
    mu_policy::config::parse_toml(text, 8453).map_err(PolicyErr::File)
}

/// Atomic replace: temp file, fsync, rename, fsync the directory.
fn write_file(path: &Path, delta: &Delta, version: u16) -> Result<(), PolicyErr> {
    let body = mu_policy::config::to_toml(delta, version);
    let tmp = path.with_extension("tmp");
    {
        use std::io::Write;
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(body.as_bytes())?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp, path)?;
    if let Some(dir) = path.parent() {
        if let Ok(d) = std::fs::File::open(dir) {
            let _ = d.sync_all();
        }
    }
    Ok(())
}

/// The result of a confirmed change.
#[derive(Debug)]
pub struct Applied {
    pub old_hash: Hash32,
    pub new_hash: Hash32,
    pub state: PolicyState,
}

/// Apply a Δ the owner confirmed: journal first, then the cache, then μ.
pub fn commit(
    log: &mut Log,
    mu: &Mu,
    vault: &dyn Vault,
    policy_path: &Path,
    next: Delta,
    now: u64,
) -> Result<(Applied, Mu), PolicyErr> {
    let prev = current(log, mu.owner_pubkey())?;
    let prev_hash = prev
        .as_ref()
        .map(|p| delta_hash(&p.delta))
        .unwrap_or_else(|| mu_policy::delta_hash(mu_delta_or_empty(mu)));
    // The journal version keeps counting from the last record.
    let version = match prev.as_ref() {
        Some(p) => p.version.checked_add(1).ok_or(PolicyErr::BadRecord)?,
        None => 1,
    };
    let next_hash = delta_hash(&next);
    let digest = change_digest(version, &prev_hash, &next_hash);

    let sig = vault
        .owner_sign(DomainTag::MuDelta, &digest)
        .map_err(|e| PolicyErr::Vault(format!("{e:?}")))?;
    if !owner_sig_valid(mu.owner_pubkey(), &digest, &sig.0) {
        return Err(PolicyErr::BadSignature);
    }

    // 1. journal (fsync inside append)
    log.append(
        Kind::DeltaApplied {
            version,
            prev: prev_hash,
            next: next_hash,
            delta: mu_policy::delta_bytes(&next),
            owner_sig: sig.0,
        },
        now,
        vault,
    )
    .map_err(|e| PolicyErr::Io(format!("{e:?}")))?;
    let log_head = log.last_hash();

    // 2. the cache
    write_file(policy_path, &next, version)?;

    // 3. μ: Δ and the new tail, signed by the owner and the μ key.
    let mu_delta_sig = vault
        .owner_sign(DomainTag::MuDelta, &delta_hash(&next))
        .map_err(|e| PolicyErr::Vault(format!("{e:?}")))?;
    let next_mu = mu
        .apply_delta(next.clone(), mu_delta_sig.0, log_head, vault)
        .map_err(|e| PolicyErr::Vault(format!("{e:?}")))?;

    Ok((
        Applied { old_hash: prev_hash, new_hash: next_hash, state: PolicyState { version, delta: next } },
        next_mu,
    ))
}

fn mu_delta_or_empty(mu: &Mu) -> &Delta {
    mu.delta()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mu_common::Amount;
    use mu_core::object::Omega;
    use mu_policy::WlEntry;
    use mu_vault::backend::SoftVault;
    use std::path::{Path, PathBuf};

    fn vault() -> SoftVault {
        SoftVault::for_test([1; 32], [2; 32], [3; 32])
    }

    fn owner_pubkey() -> [u8; 33] {
        let v = vault().owner_pubkey().expect("owner key");
        let mut a = [0u8; 33];
        a.copy_from_slice(&v);
        a
    }

    fn owner_pk_vec() -> Vec<u8> {
        owner_pubkey().to_vec()
    }

    fn d(limit: u128) -> Delta {
        Delta {
            daily_limit: Amount::from_minor(limit),
            whitelist: vec![WlEntry {
                address: mu_common::CanonAddress::canon(
                    "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    8453,
                )
                .unwrap(),
                label: "issuer".into(),
            }],
            confirm_threshold: Amount::ZERO,
            resource_allowlist: vec![],
        }
    }

    struct Env {
        _dir: tempfile::TempDir,
        log_path: PathBuf,
        mu_path: PathBuf,
        policy_path: PathBuf,
        vault: SoftVault,
        mu: Mu,
    }

    fn env() -> Env {
        let dir = tempfile::tempdir().unwrap();
        let v = vault();
        let log_path = dir.path().join("mu.log");
        let log = Log::open(&log_path, &v).unwrap();
        let base = d(5_000_000);
        let sig = v.owner_sign(DomainTag::MuDelta, &delta_hash(&base)).unwrap().0;
        let mu = Mu::issue(
            [7; 16],
            owner_pubkey(),
            1_000,
            Omega { connectors: vec![0], max_ceiling: 1_000_000_000 },
            base.clone(),
            sig,
            b"ref".to_vec(),
            log.last_hash(),
            &v,
        )
        .unwrap();
        let policy_path = dir.path().join("policy.toml");
        std::fs::write(&policy_path, mu_policy::config::to_toml(&base, 0)).unwrap();
        let mu_path = dir.path().join("mu.bin");
        Env { _dir: dir, log_path, mu_path, policy_path, vault: v, mu }
    }

    fn apply(env: &mut Env, next: Delta) -> Result<u16, PolicyErr> {
        let mut log = Log::open(&env.log_path, &env.vault).unwrap();
        let (applied, next_mu) = self::commit(&mut log, &env.mu, &env.vault, &env.policy_path, next, 2_000)?;
        next_mu.save(&env.mu_path).unwrap();
        env.mu = next_mu;
        Ok(applied.state.version)
    }

    #[test]
    fn a_confirmed_change_survives_a_restart() {
        let mut e = env();
        let v = apply(&mut e, d(7_000_000)).unwrap();
        assert_eq!(v, 1);
        // A restart: a fresh Log and a fresh read of the cache.
        let log = Log::open(&e.log_path, &e.vault).unwrap();
        let st = load(&log, &owner_pk_vec(), &e.policy_path).unwrap();
        assert_eq!(st.version, 1);
        assert_eq!(st.delta.daily_limit.minor(), 7_000_000);
        let text = std::fs::read_to_string(&e.policy_path).unwrap();
        assert!(text.contains("version = 1"), "{text}");
        assert!(text.contains("daily_limit_minor = 7000000"), "{text}");
    }

    #[test]
    fn a_crash_between_the_journal_and_the_file_is_repaired() {
        let mut e = env();
        apply(&mut e, d(7_000_000)).unwrap();
        // The daemon died before replacing the cache: the file is still v0.
        std::fs::write(&e.policy_path, mu_policy::config::to_toml(&d(5_000_000), 0)).unwrap();
        let log = Log::open(&e.log_path, &e.vault).unwrap();
        let st = load(&log, &owner_pk_vec(), &e.policy_path).unwrap();
        assert_eq!(st.version, 1, "the journal wins");
        assert_eq!(st.delta.daily_limit.minor(), 7_000_000);
        let text = std::fs::read_to_string(&e.policy_path).unwrap();
        assert!(text.contains("version = 1"), "the cache was rewritten: {text}");
    }

    #[test]
    fn a_planted_looser_file_is_refused() {
        let mut e = env();
        apply(&mut e, d(7_000_000)).unwrap();
        // Someone drops in a file with a bigger limit, old version, no journal.
        std::fs::write(&e.policy_path, mu_policy::config::to_toml(&d(9_000_000), 0)).unwrap();
        let log = Log::open(&e.log_path, &e.vault).unwrap();
        match load(&log, &owner_pk_vec(), &e.policy_path) {
            Err(PolicyErr::FileUnknown(0)) => {}
            other => panic!("expected FileUnknown(0), got {other:?}"),
        }
    }

    #[test]
    fn a_file_newer_than_the_journal_is_refused() {
        let mut e = env();
        apply(&mut e, d(7_000_000)).unwrap();
        std::fs::write(&e.policy_path, mu_policy::config::to_toml(&d(9_000_000), 5)).unwrap();
        let log = Log::open(&e.log_path, &e.vault).unwrap();
        match load(&log, &owner_pk_vec(), &e.policy_path) {
            Err(PolicyErr::FileAhead(5)) => {}
            other => panic!("expected FileAhead(5), got {other:?}"),
        }
    }

    #[test]
    fn a_tampered_record_is_refused() {
        let e = env();
        let mut log = Log::open(&e.log_path, &e.vault).unwrap();
        let next = d(9_000_000);
        log.append(
            Kind::DeltaApplied {
                version: 1,
                prev: delta_hash(&d(5_000_000)),
                next: delta_hash(&next),
                delta: mu_policy::delta_bytes(&next),
                owner_sig: [0u8; 64], // not a signature
            },
            2_000,
            &e.vault,
        )
        .unwrap();
        match load(&log, &owner_pk_vec(), &e.policy_path) {
            Err(PolicyErr::BadSignature) => {}
            other => panic!("expected BadSignature, got {other:?}"),
        }
    }

    #[test]
    fn a_change_signed_by_a_foreign_key_is_not_applied() {
        let mut e = env();
        // A vault whose owner key is not the one μ was issued with.
        let foreign = SoftVault::for_test([1; 32], [9; 32], [3; 32]);
        let mut log = Log::open(&e.log_path, &e.vault).unwrap();
        match commit(&mut log, &e.mu, &foreign, &e.policy_path, d(9_000_000), 2_000) {
            Err(PolicyErr::BadSignature) => {}
            other => panic!("expected BadSignature, got {other:?}"),
        }
        // Nothing was written: the log has no record, the file is still v0.
        let log = Log::open(&e.log_path, &e.vault).unwrap();
        assert!(current(&log, &owner_pk_vec()).unwrap().is_none());
        let text = std::fs::read_to_string(&e.policy_path).unwrap();
        assert!(text.contains("version = 0"), "{text}");
    }

    #[test]
    fn versions_climb_and_the_previous_hash_is_the_last_one() {
        let mut e = env();
        assert_eq!(apply(&mut e, d(6_000_000)).unwrap(), 1);
        assert_eq!(apply(&mut e, d(7_000_000)).unwrap(), 2);
        let log = Log::open(&e.log_path, &e.vault).unwrap();
        let recs = records(&log).unwrap();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs[1].prev, recs[0].next);
        assert_eq!(recs[1].version, 2);
    }
}
