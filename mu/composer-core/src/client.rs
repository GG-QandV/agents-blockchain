//! C4: клиент транспорта. Кадрирование len:u32 LE ‖ body поверх любого Read+Write —
//! unix socket в проде, in-memory пара в тестах.
use std::io::{Read, Write};

pub const FRAME_MAX: usize = 8192 + 16;

#[derive(Debug)]
pub enum TransportErr {
    Io(std::io::Error),
    FrameTooLarge,
}
impl From<std::io::Error> for TransportErr {
    fn from(e: std::io::Error) -> Self { TransportErr::Io(e) }
}

pub fn write_frame<W: Write>(w: &mut W, body: &[u8]) -> Result<(), TransportErr> {
    if body.len() > FRAME_MAX { return Err(TransportErr::FrameTooLarge); }
    w.write_all(&(body.len() as u32).to_le_bytes())?;
    w.write_all(body)?;
    w.flush()?;
    Ok(())
}

pub fn read_frame<R: Read>(r: &mut R) -> Result<Vec<u8>, TransportErr> {
    let mut lb = [0u8; 4];
    r.read_exact(&mut lb)?;
    let len = u32::from_le_bytes(lb) as usize;
    if len > FRAME_MAX { return Err(TransportErr::FrameTooLarge); }
    let mut body = vec![0u8; len];
    r.read_exact(&mut body)?;
    Ok(body)
}
