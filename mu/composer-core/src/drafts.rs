//! C3: черновики. Δ не секрет — plaintext-файл; encrypt_drafts — опция фазы 2.
//! Атомарная запись tmp+rename (та же дисциплина, что M1).
use crate::proposal::{decode_proposal, encode_proposal, DeltaProposal, ProposalErr};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub enum DraftErr {
    Io(std::io::Error),
    Corrupt,
}
impl From<std::io::Error> for DraftErr {
    fn from(e: std::io::Error) -> Self { DraftErr::Io(e) }
}
impl From<ProposalErr> for DraftErr {
    fn from(_: ProposalErr) -> Self { DraftErr::Corrupt }
}

pub fn save_draft(p: &DeltaProposal, path: &Path) -> Result<(), DraftErr> {
    let bytes = encode_proposal(p)?;
    let tmp = path.with_extension("tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(&bytes)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Повреждённый черновик отбрасывается (спека §10), не роняет приложение.
pub fn load_draft(path: &Path) -> Result<Option<DeltaProposal>, DraftErr> {
    match fs::read(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(DraftErr::Io(e)),
        Ok(bytes) => match decode_proposal(&bytes) {
            Ok(p) => Ok(Some(p)),
            Err(_) => Ok(None), // corrupt → отбросить с None, вызвавший покажет сообщение
        },
    }
}
