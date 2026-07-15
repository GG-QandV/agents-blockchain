//! EIP-55: чексумма Ethereum-адреса регистром hex-букв (keccak256).
//! Закрывает хук из mu-common: канонизация + строгая проверка смешанного регистра.
use mu_common::{ids::AddrErr, CanonAddress};
use sha3::{Digest, Keccak256};

/// Полная канонизация с проверкой EIP-55 (E-ADR-01/02 из спеки §7.1).
/// - не hex / не 40 символов → Err
/// - смешанный регистр и чексумма не сходится → Err(BadChecksum)
/// - весь lower/upper → Ok, но с флагом needs_warning (W-ADR-01)
pub fn canon_address_checked(input: &str, chain_id: u64) -> Result<(CanonAddress, bool), AddrErr> {
    let s = input.strip_prefix("0x").unwrap_or(input);
    if s.len() != 40 {
        return Err(AddrErr::BadLength);
    }
    let mixed = s.bytes().any(|b| b.is_ascii_uppercase()) && s.bytes().any(|b| b.is_ascii_lowercase());
    if mixed && !eip55_valid(s) {
        return Err(AddrErr::BadChecksum);
    }
    let canon = CanonAddress::canon(input, chain_id)?;
    let needs_warning = !mixed && s.bytes().any(|b| b.is_ascii_alphabetic());
    Ok((canon, needs_warning))
}

fn eip55_valid(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    let mut h = Keccak256::new();
    h.update(lower.as_bytes());
    let digest = h.finalize();
    for (i, c) in s.bytes().enumerate() {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        let nibble = if i % 2 == 0 { digest[i / 2] >> 4 } else { digest[i / 2] & 0x0f };
        let should_upper = nibble >= 8;
        if should_upper != c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    // официальные векторы EIP-55
    const VALID: &[&str] = &[
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
        "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
        "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
    ];
    #[test]
    fn official_vectors_pass() {
        for a in VALID {
            assert!(canon_address_checked(a, 1).is_ok(), "{a}");
        }
    }
    #[test]
    fn broken_checksum_rejected() {
        // портим регистр одной буквы валидного адреса
        let bad = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeD"; // последняя d → D
        assert_eq!(canon_address_checked(bad, 1), Err(AddrErr::BadChecksum));
    }
    #[test]
    fn all_lower_ok_with_warning() {
        let (_, warn) = canon_address_checked("0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed", 1).unwrap();
        assert!(warn); // W-ADR-01
    }
}
