//! Key files: 32 bytes as 64 hex characters, owner-only (mode 600).
//! A key file readable by others is refused; a new one is created with `create_new`
//! (never overwrites) and mode 600 from the first byte.
use std::io::Write;
use std::path::Path;
use zeroize::{Zeroize, Zeroizing};

#[derive(Debug, PartialEq, Eq)]
pub enum KeyFileErr {
    Unreadable,
    /// Readable by group/others (mode & 077 != 0).
    Permissions,
    /// Not 64 hex characters, or all zero.
    Format,
    Exists,
    Io(String),
}

impl std::fmt::Display for KeyFileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyFileErr::Unreadable => write!(f, "cannot read"),
            KeyFileErr::Permissions => write!(f, "must be mode 600 (readable by the owner only)"),
            KeyFileErr::Format => write!(f, "expected 32 bytes as 64 hex characters"),
            KeyFileErr::Exists => write!(f, "already exists (refusing to overwrite)"),
            KeyFileErr::Io(e) => write!(f, "{e}"),
        }
    }
}

pub fn parse_hex32(text: &str) -> Option<[u8; 32]> {
    let t = text.trim();
    let t = t.strip_prefix("0x").unwrap_or(t).as_bytes();
    if t.len() != 64 {
        return None;
    }
    let nib = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut key = [0u8; 32];
    for (i, b) in key.iter_mut().enumerate() {
        *b = (nib(t[2 * i])? << 4) | nib(t[2 * i + 1])?;
    }
    if key == [0; 32] {
        return None;
    }
    Some(key)
}

pub fn to_hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Read a secret key file. Refuses files readable by group/others.
pub fn read_seed(path: &Path) -> Result<Zeroizing<[u8; 32]>, KeyFileErr> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(path).map_err(|_| KeyFileErr::Unreadable)?.permissions().mode();
        if mode & 0o077 != 0 {
            return Err(KeyFileErr::Permissions);
        }
    }
    let mut text = std::fs::read_to_string(path).map_err(|_| KeyFileErr::Unreadable)?;
    let key = parse_hex32(&text);
    text.zeroize();
    key.map(Zeroizing::new).ok_or(KeyFileErr::Format)
}

/// Create a new secret key file (mode 600). Fails if it exists.
pub fn write_new_seed(path: &Path, seed: &[u8; 32]) -> Result<(), KeyFileErr> {
    write_new(path, &Zeroizing::new(format!("{}\n", to_hex(seed))), 0o600)
}

/// Create a new public/plain file (mode 644). Fails if it exists.
pub fn write_new_public(path: &Path, text: &str) -> Result<(), KeyFileErr> {
    write_new(path, text, 0o644)
}

fn write_new(path: &Path, text: &str, mode: u32) -> Result<(), KeyFileErr> {
    let mut o = std::fs::OpenOptions::new();
    o.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(mode);
    }
    let _ = mode;
    let mut f = o.open(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists { KeyFileErr::Exists } else { KeyFileErr::Io(e.to_string()) }
    })?;
    f.write_all(text.as_bytes()).map_err(|e| KeyFileErr::Io(e.to_string()))?;
    f.sync_all().map_err(|e| KeyFileErr::Io(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn roundtrip_mode_600_and_no_overwrite() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("k");
        write_new_seed(&p, &[7; 32]).unwrap();
        assert_eq!(std::fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o600);
        assert_eq!(*read_seed(&p).unwrap(), [7; 32]);
        assert_eq!(write_new_seed(&p, &[8; 32]), Err(KeyFileErr::Exists));
        assert_eq!(*read_seed(&p).unwrap(), [7; 32]); // untouched
    }

    #[test]
    fn refuses_group_readable_bad_format_and_zero() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("k");
        write_new_seed(&p, &[7; 32]).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o640)).unwrap();
        assert_eq!(read_seed(&p).unwrap_err(), KeyFileErr::Permissions);
        for bad in ["", "abc", &"zz".repeat(32), &"00".repeat(32), &"11".repeat(33)] {
            std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o600)).unwrap();
            std::fs::write(&p, bad).unwrap();
            assert_eq!(read_seed(&p).unwrap_err(), KeyFileErr::Format, "{bad}");
        }
        assert_eq!(read_seed(&d.path().join("none")).unwrap_err(), KeyFileErr::Unreadable);
    }
}
