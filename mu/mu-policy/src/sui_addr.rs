//! Sui address: 0x + 64 hex, no register checksum (Sui doesn't have EIP-55).
//! W-ADR-01: all lowercase/uppercase → warning to manually verify identicon.
use mu_common::{ids::AddrErr, CanonAddress};

/// Canonicalize Sui address: length/hex, no checksum.
/// - not hex / not 64 chars → Err(BadLength/BadHex)
/// - all lower/upper → Ok, with warn flag (W-ADR-01: verify identicon)
pub fn canon_address_checked(input: &str, chain_id: u64) -> Result<(CanonAddress, bool), AddrErr> {
    let s = input.strip_prefix("0x").unwrap_or(input);
    if s.len() != 64 {
        return Err(AddrErr::BadLength);
    }
    // Sui has no checksum — any register passes.
    // But if all letters are in the same case (lowercase or uppercase) —
    // we warn that this doesn't protect against confusion (W-ADR-01).
    let all_lower = s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
    let all_upper = s.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit());
    let needs_warning = all_lower || all_upper;
    let canon = CanonAddress::canon(input, chain_id)?;
    Ok((canon, needs_warning))
}

#[cfg(test)]
mod tests {
    use super::*;
    // Reference address from sui keytool (dev key [3;32], P-256)
    const DEV_ADDR: &str = "0x64a32d2f8b9ce1c87c71a7868adc02e4b07a28e1318fd66651f14800279fd6fb";
    #[test]
    fn dev_address_passes() {
        assert!(canon_address_checked(DEV_ADDR, 1).is_ok());
    }
    #[test]
    fn mixed_case_passes() {
        let upper = "0x64A32D2F8B9CE1C87C71A7868ADC02E4B07A28E1318FD66651F14800279FD6FB";
        assert!(canon_address_checked(upper, 1).is_ok());
    }
    #[test]
    fn short_address_rejected() {
        assert_eq!(canon_address_checked("0x1234", 1), Err(AddrErr::BadLength));
    }
    #[test]
    fn bad_hex_rejected() {
        assert_eq!(canon_address_checked(&format!("0x{}z{}", "a".repeat(63), "b".repeat(0)), 1), Err(AddrErr::BadHex));
    }
    #[test]
    fn all_lowercase_warns() {
        let (_, warn) = canon_address_checked(
            "0x64a32d2f8b9ce1c87c71a7868adc02e4b07a28e1318fd66651f14800279fd6fb", 1
        ).unwrap();
        assert!(warn); // W-ADR-01
    }
    #[test]
    fn all_uppercase_warns() {
        let (_, warn) = canon_address_checked(
            "0x64A32D2F8B9CE1C87C71A7868ADC02E4B07A28E1318FD66651F14800279FD6FB", 1
        ).unwrap();
        assert!(warn); // W-ADR-01 (also no checksum)
    }
}
