//! Sui-адрес: 0x + 64 hex, без чексумми регістра (Sui не має EIP-55).
//! W-ADR-01: всі lowercase/uppercase → попередження про ручну звірку identicon.
use mu_common::{ids::AddrErr, CanonAddress};

/// Канонізація адреси Sui: довжина/hex, без чексумми.
/// - не hex / не 64 символи → Err(BadLength/BadHex)
/// - весь lower/upper → Ok, з флагом warn (W-ADR-01: звірити identicon)
pub fn canon_address_checked(input: &str, chain_id: u64) -> Result<(CanonAddress, bool), AddrErr> {
    let s = input.strip_prefix("0x").unwrap_or(input);
    if s.len() != 64 {
        return Err(AddrErr::BadLength);
    }
    // Sui не має чексумми — будь-який регістр проходить.
    // Але якщо всі букви в одному регістрі (lowercase або uppercase) —
    // попереджаємо, що це не захищає від плутанини (W-ADR-01).
    let all_lower = s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
    let all_upper = s.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit());
    let needs_warning = all_lower || all_upper;
    let canon = CanonAddress::canon(input, chain_id)?;
    Ok((canon, needs_warning))
}

#[cfg(test)]
mod tests {
    use super::*;
    // Еталонний адрес з sui keytool (dev ключ [3;32], P-256)
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
        assert!(warn); // W-ADR-01 (теж не має чексумми)
    }
}
