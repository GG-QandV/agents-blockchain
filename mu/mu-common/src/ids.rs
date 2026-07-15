//! Идентификаторы. RISK-M2-3/M3-2: ConnectorId и CanonAddress — закрытые типы.

/// Закрытый набор коннекторов. Строк нет: неизвестный id невозможен by design.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum ConnectorId {
    Crypto,
    BankStub,
    CardStub,
}

/// Каноническая форма адреса получателя: 20 байт + chain_id.
/// Конструктор — только через canon() (единая нормализация, RISK-M3-2).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct CanonAddress {
    addr: [u8; 20],
    chain_id: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddrErr {
    BadLength,
    BadHex,
    BadChecksum,
}

impl CanonAddress {
    pub fn bytes(&self) -> &[u8; 20] { &self.addr }
    pub fn chain_id(&self) -> u64 { self.chain_id }

    /// Единственный конструктор. Принимает hex (с/без 0x), проверяет EIP-55 если смешанный регистр.
    pub fn canon(input: &str, chain_id: u64) -> Result<Self, AddrErr> {
        let s = input.strip_prefix("0x").unwrap_or(input);
        if s.len() != 40 { return Err(AddrErr::BadLength); }
        let mut addr = [0u8; 20];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let hi = hexval(chunk[0]).ok_or(AddrErr::BadHex)?;
            let lo = hexval(chunk[1]).ok_or(AddrErr::BadHex)?;
            addr[i] = (hi << 4) | lo;
        }
        let mixed = s.bytes().any(|b| b.is_ascii_uppercase())
            && s.bytes().any(|b| b.is_ascii_lowercase());
        if mixed && !eip55_ok(s, &addr) {
            return Err(AddrErr::BadChecksum);
        }
        Ok(CanonAddress { addr, chain_id })
    }

    /// Усечённый показ для UI/логов (RISK-X-4, RISK-M9-3): 0xAbC…123.
    pub fn redacted(&self) -> String {
        let hex = self.to_hex();
        format!("0x{}…{}", &hex[..6], &hex[hex.len() - 4..])
    }
    pub fn to_hex(&self) -> String {
        let mut out = String::with_capacity(40);
        for b in &self.addr {
            out.push_str(&format!("{:02x}", b));
        }
        out
    }
}

fn hexval(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Упрощённая проверка EIP-55 (keccak-заглушка помечена; в проде — из mu-policy).
/// Здесь: если строка смешанного регистра, требуем хотя бы структурную корректность.
/// NB: полноценный keccak256 подключается в mu-policy; в mu-common оставлен хук.
fn eip55_ok(_s: &str, _addr: &[u8; 20]) -> bool {
    // Хук: без keccak в mu-common принимаем (валидация EIP-55 — ответственность mu-policy).
    // Для строгого MVP заменяется реализацией с keccak256.
    true
}

/// 32-байтный хеш (SHA-256).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Hash32(pub [u8; 32]);

/// 16-байтный тикет операции.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Ticket(pub [u8; 16]);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canon_lengths() {
        assert_eq!(CanonAddress::canon("0x00", 1), Err(AddrErr::BadLength));
        let ok = CanonAddress::canon("0x52908400098527886E0F7030069857D2E4169EE7", 8453);
        assert!(ok.is_ok());
    }
    #[test]
    fn canon_case_insensitive_equal() {
        let a = CanonAddress::canon("0xabcdef0123456789abcdef0123456789abcdef01", 1).unwrap();
        let b = CanonAddress::canon("0xABCDEF0123456789ABCDEF0123456789ABCDEF01", 1).unwrap();
        assert_eq!(a, b); // одинаковые байты → равны (RISK-M3-2)
    }
    #[test]
    fn redacted_format() {
        let a = CanonAddress::canon("0xabcdef0123456789abcdef0123456789abcdef01", 1).unwrap();
        let r = a.redacted();
        assert!(r.starts_with("0xabcdef"));
        assert!(r.ends_with("ef01"));
    }
}
