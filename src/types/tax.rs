/// Taxes applicable to goods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tax {
    /// A 0% tax.
    Tax0,
    /// A 5% tax.
    Tax5,
    /// A 10% tax.
    Tax10,
    /// A 15% tax.
    Tax15,
}

impl Tax {
    /// Apply the current tax to a given amount.
    pub fn apply(&self, amount: u32) -> u32 {
        let value = match self {
            Self::Tax0 => 0.0,
            Self::Tax5 => amount as f64 * 0.05,
            Self::Tax10 => amount as f64 * 0.10,
            Self::Tax15 => amount as f64 * 0.15,
        };
        let tax = value.round() as u32;
        let unit = tax % 10;

        // round to the nearest 0.05 if needed
        if unit > 5 {
            tax + 10 - unit
        } else if unit > 0 && unit < 5 {
            tax + 5 - unit
        } else {
            tax
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax() {
        assert_eq!(0, Tax::Tax0.apply(100));
        assert_eq!(5, Tax::Tax5.apply(100));
        assert_eq!(10, Tax::Tax10.apply(100));
        assert_eq!(15, Tax::Tax15.apply(100));
    }

    #[test]
    fn test_tax_rounding() {
        assert_eq!(150, Tax::Tax10.apply(1499));
        assert_eq!(715, Tax::Tax15.apply(4750));
    }
}
