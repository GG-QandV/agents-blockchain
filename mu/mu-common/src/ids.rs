//! Identifiers. RISK-M2-3/M3-2: ConnectorId and CanonAddress — closed types.

/// Closed set of connectors. No strings: unknown id is impossible by design.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum ConnectorId {
    Crypto,
    BankStub,
    CardStub,
}

/// Canonical address form: 32 bytes + chain_id.
/// Sui: 0x + 64 hex, without register checksum (EIP-55 removed).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct CanonAddress {
    addr: [u8; 32],
    chain_id: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddrErr {
    BadLength,
    BadHex,
}

impl CanonAddress {
    pub fn bytes(&self) -> &[u8; 32] { &self.addr }
    pub fn chain_id(&self) -> u64 { self.chain_id }

    /// Single constructor. Accepts hex (with/without 0x), length 64 characters,
    /// without register checksum (Sui does not have EIP-55).
    pub fn canon(input: &str, chain_id: u64) -> Result<Self, AddrErr> {
        let s = input.strip_prefix("0x").unwrap_or(input);
        if s.len() != 64 { return Err(AddrErr::BadLength); }
        let mut addr = [0u8; 32];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let hi = hexval(chunk[0]).ok_or(AddrErr::BadHex)?;
            let lo = hexval(chunk[1]).ok_or(AddrErr::BadHex)?;
            addr[i] = (hi << 4) | lo;
        }
        Ok(CanonAddress { addr, chain_id })
    }

    /// Truncated display for UI/logs: 0xAbC…123.
    pub fn redacted(&self) -> String {
        let hex = self.to_hex();
        format!("0x{}…{}", &hex[..6], &hex[hex.len() - 4..])
    }
    pub fn to_hex(&self) -> String {
        let mut out = String::with_capacity(64);
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

/// 32-byte hash (SHA-256 or blake2b).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Hash32(pub [u8; 32]);

/// 16-byte operation ticket.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Ticket(pub [u8; 16]);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canon_lengths() {
        assert_eq!(CanonAddress::canon("0x00", 1), Err(AddrErr::BadLength));
        let ok = CanonAddress::canon(
            "0x64a32d2f8b9ce1c87c71a7868adc02e4b07a28e1318fd66651f14800279fd6fb", 1
        );
        assert!(ok.is_ok());
    }
    #[test]
    fn canon_case_insensitive_equal() {
        let a = CanonAddress::canon(
            "0x64a32d2f8b9ce1c87c71a7868adc02e4b07a28e1318fd66651f14800279fd6fb", 1
        ).unwrap();
        let b = CanonAddress::canon(
            "0x64A32D2F8B9CE1C87C71A7868ADC02E4B07A28E1318FD66651F14800279FD6FB", 1
        ).unwrap();
        assert_eq!(a, b);
    }
    #[test]
    fn redacted_format() {
        let a = CanonAddress::canon(
            "0x64a32d2f8b9ce1c87c71a7868adc02e4b07a28e1318fd66651f14800279fd6fb", 1
        ).unwrap();
        let r = a.redacted();
        assert!(r.starts_with("0x64a32d"));
        assert!(r.ends_with("d6fb"));
    }
}
