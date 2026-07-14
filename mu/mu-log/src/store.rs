//! Хранилище: append-only файл + fsync (RISK-M5-1), verify_chain (RISK-M5-2),
//! обрезка хвоста (RISK-M5-5), window_sum (RISK-M5-4).
use crate::entry::{entry_hash, Entry, Kind, WINDOW_SECS};
use mu_common::{Amount, Hash32};
use mu_vault::{DomainTag, Vault};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum LogErr {
    Io(std::io::Error),
    ChainBroken { at_seq: u64 },
    SeqNotMonotonic { at_seq: u64 },
    Vault(String),
    Corrupt,
}
impl From<std::io::Error> for LogErr {
    fn from(e: std::io::Error) -> Self { LogErr::Io(e) }
}

const GENESIS: Hash32 = Hash32([0u8; 32]);

pub struct Log {
    file: File,
    path: PathBuf,
    entries: Vec<Entry>, // MVP: рабочая копия в памяти (сегментация — фаза 2)
    last_hash: Hash32,
    next_seq: u64,
}

impl Log {
    /// Открытие: чтение всех записей, обрезка битого хвоста (RISK-M5-5).
    /// Подпись новой TailTruncated-записи требует vault → передаётся сюда.
    pub fn open(path: &Path, vault: &dyn Vault) -> Result<Self, LogErr> {
        let mut file = OpenOptions::new().create(true).read(true).append(true).open(path)?;
        let mut buf = Vec::new();
        file.seek(SeekFrom::Start(0))?;
        file.read_to_end(&mut buf)?;

        let (entries, valid_bytes, truncated) = decode_all(&buf);
        if truncated {
            // усечь файл до последней валидной записи
            file.set_len(valid_bytes as u64)?;
            file.sync_all()?;
        }
        let last_hash = entries.last().map(Entry::hash).unwrap_or(GENESIS);
        let next_seq = entries.last().map(|e| e.seq.saturating_add(1)).unwrap_or(0);

        let mut log = Log { file, path: path.to_path_buf(), entries, last_hash, next_seq };
        if truncated {
            let lost = log.next_seq; // всё, что было бы дальше
            log.append_inner(Kind::TailTruncated { lost_from_seq: lost }, 0, vault)?;
        }
        Ok(log)
    }

    /// RISK-M5-1: durable append. Возврат Ok(hash) — только после flush+sync.
    pub fn append(&mut self, kind: Kind, ts: u64, vault: &dyn Vault) -> Result<Hash32, LogErr> {
        self.append_inner(kind, ts, vault)
    }

    fn append_inner(&mut self, kind: Kind, ts: u64, vault: &dyn Vault) -> Result<Hash32, LogErr> {
        let seq = self.next_seq;
        let h = entry_hash(seq, &self.last_hash, ts, &kind);
        let sig = vault
            .sign_mu(DomainTag::MuLog, &h)
            .map_err(|e| LogErr::Vault(format!("{e:?}")))?;
        let e = Entry { seq, prev_hash: self.last_hash, ts, kind, sig: sig.0 };

        let rec = encode_entry(&e);
        self.file.write_all(&rec)?;
        self.file.flush()?;
        self.file.sync_all()?; // fsync до возврата (RISK-M5-1)

        self.last_hash = h;
        self.next_seq = seq.saturating_add(1);
        self.entries.push(e);
        Ok(h)
    }

    /// RISK-M5-2: полная проверка цепи (hash-chain + монотонность seq).
    /// Проверка подписей — verify_signatures() с pubkey (отделена: ключ приходит извне).
    pub fn verify_chain(&self) -> Result<(), LogErr> {
        let mut prev = GENESIS;
        let mut expect_seq = 0u64;
        for e in &self.entries {
            if e.seq != expect_seq {
                return Err(LogErr::SeqNotMonotonic { at_seq: e.seq });
            }
            if e.prev_hash != prev {
                return Err(LogErr::ChainBroken { at_seq: e.seq });
            }
            prev = e.hash();
            expect_seq = expect_seq.saturating_add(1);
        }
        Ok(())
    }

    /// Pending без парного Settled/Failed (по intent_hash).
    pub fn pending(&self) -> Vec<&Entry> {
        let mut closed: HashSet<[u8; 32]> = HashSet::new();
        for e in &self.entries {
            match &e.kind {
                Kind::Settled { intent_hash, .. } | Kind::Failed { intent_hash } => {
                    closed.insert(intent_hash.0);
                }
                _ => {}
            }
        }
        self.entries
            .iter()
            .filter(|e| matches!(&e.kind, Kind::Pending { intent_hash, .. } if !closed.contains(&intent_hash.0)))
            .collect()
    }

    /// RISK-M5-4: окно 24ч. Settled ∪ открытые Pending; Simulated/Failed/Denied исключены.
    pub fn window_sum(&self, now: u64) -> Amount {
        let from = now.saturating_sub(WINDOW_SECS);
        let open_pending: Vec<&Entry> = self.pending();
        let open_set: HashSet<u64> = open_pending.iter().map(|e| e.seq).collect();
        let mut acc = Amount::ZERO;
        for e in &self.entries {
            if e.ts < from { continue; }
            let add = match &e.kind {
                Kind::Settled { effective_gas: _, .. } => {
                    // сумма Settled = amount_total его Pending; ищем парный Pending
                    self.pending_total_for(&e.kind)
                }
                Kind::Pending { amount_total, .. } if open_set.contains(&e.seq) => Some(*amount_total),
                _ => None,
            };
            if let Some(v) = add {
                acc = acc.checked_add(Amount::from_minor(v)).unwrap_or(Amount::from_minor(u128::MAX));
            }
        }
        acc
    }

    fn pending_total_for(&self, settled: &Kind) -> Option<u128> {
        let target = match settled {
            Kind::Settled { intent_hash, .. } => intent_hash.0,
            _ => return None,
        };
        self.entries.iter().find_map(|e| match &e.kind {
            Kind::Pending { intent_hash, amount_total, .. } if intent_hash.0 == target => Some(*amount_total),
            _ => None,
        })
    }

    pub fn entries(&self) -> &[Entry] { self.entries.len().checked_sub(0).map(|_| &self.entries[..]).unwrap_or(&[]) }
    pub fn last_hash(&self) -> Hash32 { self.last_hash }
    pub fn path(&self) -> &Path { &self.path }
}

// ── сериализация записи: len:u32 LE ‖ body ────────────────────────────────
fn encode_entry(e: &Entry) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend(e.seq.to_be_bytes());
    body.extend(e.prev_hash.0);
    body.extend(e.ts.to_be_bytes());
    let k = e.kind.encode();
    body.extend((k.len() as u32).to_be_bytes());
    body.extend(k);
    body.extend(e.sig);
    let mut out = Vec::with_capacity(body.len() + 4);
    out.extend((body.len() as u32).to_le_bytes());
    out.extend(body);
    out
}

/// Декод всех записей; возвращает (записи, валидные_байты, был_ли_битый_хвост).
fn decode_all(buf: &[u8]) -> (Vec<Entry>, usize, bool) {
    let mut entries = Vec::new();
    let mut pos = 0usize;
    loop {
        if pos == buf.len() { return (entries, pos, false); }
        let Some(rec) = try_decode_at(buf, pos) else {
            return (entries, pos, true); // битый хвост (RISK-M5-5)
        };
        let (e, next) = rec;
        entries.push(e);
        pos = next;
    }
}

fn try_decode_at(buf: &[u8], pos: usize) -> Option<(Entry, usize)> {
    if pos.checked_add(4)? > buf.len() { return None; }
    let len = u32::from_le_bytes(buf[pos..pos + 4].try_into().ok()?) as usize;
    let start = pos.checked_add(4)?;
    let end = start.checked_add(len)?;
    if end > buf.len() || len < 8 + 32 + 8 + 4 + 64 { return None; }
    let b = &buf[start..end];
    let seq = u64::from_be_bytes(b[0..8].try_into().ok()?);
    let mut prev = [0u8; 32];
    prev.copy_from_slice(&b[8..40]);
    let ts = u64::from_be_bytes(b[40..48].try_into().ok()?);
    let klen = u32::from_be_bytes(b[48..52].try_into().ok()?) as usize;
    let kstart = 52usize;
    let kend = kstart.checked_add(klen)?;
    if kend.checked_add(64)? != b.len() { return None; }
    let kind = decode_kind(&b[kstart..kend])?;
    let mut sig = [0u8; 64];
    sig.copy_from_slice(&b[kend..]);
    Some((Entry { seq, prev_hash: Hash32(prev), ts, kind, sig }, end))
}

fn decode_kind(b: &[u8]) -> Option<Kind> {
    let tag = *b.first()?;
    let r = &b[1..];
    let h32 = |s: &[u8]| -> Option<Hash32> {
        let mut a = [0u8; 32]; a.copy_from_slice(s.get(..32)?); Some(Hash32(a))
    };
    match tag {
        1 => Some(Kind::DeniedOmega { intent_hash: h32(r)? }),
        2 => Some(Kind::DeniedDelta { intent_hash: h32(r)? }),
        3 => {
            let ih = h32(r)?;
            let amt = u128::from_be_bytes(r.get(32..48)?.try_into().ok()?);
            let n = u64::from_be_bytes(r.get(48..56)?.try_into().ok()?);
            Some(Kind::Pending { intent_hash: ih, amount_total: amt, chain_nonce: n })
        }
        4 => {
            let ih = h32(r)?;
            let mut tx = [0u8; 32]; tx.copy_from_slice(r.get(32..64)?);
            let g = u128::from_be_bytes(r.get(64..80)?.try_into().ok()?);
            Some(Kind::Settled { intent_hash: ih, tx_hash: tx, effective_gas: g })
        }
        5 => Some(Kind::Failed { intent_hash: h32(r)? }),
        6 => {
            let ih = h32(r)?;
            let l = *r.get(32)? as usize;
            let s = std::str::from_utf8(r.get(33..33 + l)?).ok()?;
            let connector: &'static str = match s { "bank_stub" => "bank_stub", "card_stub" => "card_stub", _ => "stub" };
            Some(Kind::Simulated { intent_hash: ih, connector })
        }
        7 => Some(Kind::DeltaChanged { old_hash: h32(r)?, new_hash: h32(r.get(32..)?)? }),
        8 => Some(Kind::HumanDecision { intent_hash: h32(r)?, approved: *r.get(32)? == 1 }),
        9 => Some(Kind::TailTruncated { lost_from_seq: u64::from_be_bytes(r.get(..8)?.try_into().ok()?) }),
        10 => {
            let l = *r.first()? as usize;
            let id = std::str::from_utf8(r.get(1..1 + l)?).ok()?.to_string();
            let n = u64::from_be_bytes(r.get(1 + l..9 + l)?.try_into().ok()?);
            Some(Kind::NonceSnapshot { agent_id: id, nonce: n })
        }
        11 => Some(Kind::Alert { code: u16::from_be_bytes(r.get(..2)?.try_into().ok()?) }),
        _ => None,
    }
}
