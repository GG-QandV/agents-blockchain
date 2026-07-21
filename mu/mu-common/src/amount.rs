//! RISK-X-1: money — u128 minor units only, only checked arithmetic.
//! No impl Add/Sub: `a + b` on Amount does not compile by design.

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Amount(u128);

#[derive(Debug, PartialEq, Eq)]
pub enum AmtErr {
    Overflow,
    BadDecimal,
    TooManyFractionDigits,
}

impl Amount {
    pub const ZERO: Amount = Amount(0);
    pub const fn from_minor(v: u128) -> Self { Amount(v) }
    pub const fn minor(self) -> u128 { self.0 }

    #[must_use]
    pub fn checked_add(self, rhs: Amount) -> Option<Amount> {
        self.0.checked_add(rhs.0).map(Amount)
    }
    #[must_use]
    pub fn checked_sub(self, rhs: Amount) -> Option<Amount> {
        self.0.checked_sub(rhs.0).map(Amount)
    }
    #[must_use]
    pub fn checked_mul_u32(self, k: u32) -> Option<Amount> {
        self.0.checked_mul(u128::from(k)).map(Amount)
    }
    #[must_use]
    pub fn div_u32(self, k: u32) -> Option<Amount> {
        if k == 0 { return None; }
        Some(Amount(self.0 / u128::from(k)))
    }
}

/// Parse decimal string to minor units WITHOUT float.
pub fn parse_decimal(s: &str, decimals: u8) -> Result<Amount, AmtErr> {
    let mut acc: u128 = 0;
    let mut frac_digits: u8 = 0;
    let mut seen_dot = false;
    let mut seen_digit = false;
    for b in s.as_bytes() {
        match b {
            b'0'..=b'9' => {
                seen_digit = true;
                if seen_dot {
                    if frac_digits >= decimals {
                        return Err(AmtErr::TooManyFractionDigits);
                    }
                    frac_digits = frac_digits.saturating_add(1);
                }
                let d = u128::from(b - b'0');
                acc = acc.checked_mul(10).and_then(|v| v.checked_add(d)).ok_or(AmtErr::Overflow)?;
            }
            b'.' if !seen_dot => seen_dot = true,
            b'_' => {}
            _ => return Err(AmtErr::BadDecimal),
        }
    }
    if !seen_digit { return Err(AmtErr::BadDecimal); }
    let mut i = frac_digits;
    while i < decimals {
        acc = acc.checked_mul(10).ok_or(AmtErr::Overflow)?;
        i = i.saturating_add(1);
    }
    Ok(Amount(acc))
}

/// Human-readable output (RISK-X-4: truncation, no float).
pub fn display_minor(a: Amount, decimals: u8) -> String {
    let v = a.minor();
    if decimals == 0 { return v.to_string(); }
    let div = 10u128.pow(u32::from(decimals));
    let int = v / div;
    let frac = v % div;
    format!("{}.{:0width$}", int, frac, width = usize::from(decimals))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_basic() {
        assert_eq!(parse_decimal("20", 6), Ok(Amount::from_minor(20_000_000)));
        assert_eq!(parse_decimal("0.5", 6), Ok(Amount::from_minor(500_000)));
        assert_eq!(parse_decimal("1_000.25", 2), Ok(Amount::from_minor(100_025)));
    }
    #[test]
    fn parse_rejects() {
        assert_eq!(parse_decimal("1.2345678", 6), Err(AmtErr::TooManyFractionDigits));
        assert_eq!(parse_decimal("12a", 6), Err(AmtErr::BadDecimal));
        assert_eq!(parse_decimal("", 6), Err(AmtErr::BadDecimal));
    }
    #[test]
    fn checked_bounds() {
        let max = Amount::from_minor(u128::MAX);
        assert_eq!(max.checked_add(Amount::from_minor(1)), None);
        assert_eq!(Amount::ZERO.checked_sub(Amount::from_minor(1)), None);
    }
    #[test]
    fn display_roundtrip() {
        let a = parse_decimal("12.34", 6).unwrap();
        assert_eq!(display_minor(a, 6), "12.340000");
    }
}
