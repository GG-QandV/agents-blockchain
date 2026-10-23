//! A journal that cannot record must stop the daemon: any failed append
//! poisons it, and every later append is refused until a restart.
use mu_common::Hash32;
use mu_log::{Kind, Log, LogErr};
use mu_vault::backend::SoftVault;
use mu_vault::{DomainTag, P256Sig, TxSigner, Vault, VaultErr, VaultInfo};

fn vault() -> SoftVault {
    SoftVault::for_test([1; 32], [2; 32], [3; 32])
}

/// A vault that refuses to sign: the append cannot be recorded, which is what
/// a disk error looks like from the journal's side.
struct Broken;

impl Vault for Broken {
    fn sign_mu(&self, _t: DomainTag, _d: &Hash32) -> Result<P256Sig, VaultErr> {
        Err(VaultErr::Backend("disk full".into()))
    }
    fn owner_sign(&self, _t: DomainTag, _d: &Hash32) -> Result<P256Sig, VaultErr> {
        Err(VaultErr::Backend("disk full".into()))
    }
    fn tx_signer(&self) -> Result<TxSigner, VaultErr> {
        Err(VaultErr::KeyMissing)
    }
    fn attest(&self) -> VaultInfo {
        VaultInfo { backend: "broken", hw_backed: false }
    }
}

#[test]
fn a_failed_append_poisons_the_journal() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mu.log");
    let v = vault();
    let mut log = Log::open(&path, &v).unwrap();
    log.append(Kind::Alert { code: 1 }, 10, &v).unwrap();
    assert!(!log.is_poisoned());

    assert!(log.append(Kind::Alert { code: 2 }, 11, &Broken).is_err());
    assert!(log.is_poisoned(), "one failed write poisons the journal");

    // Even with a working vault the journal stays refused: only a restart (a
    // fresh Log) clears it.
    assert!(matches!(log.append(Kind::Alert { code: 3 }, 12, &v), Err(LogErr::Poisoned)));
}
