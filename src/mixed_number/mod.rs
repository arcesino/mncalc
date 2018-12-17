use regex::Regex;
use lazy_static::*;
use crate::fraction::*;

// This ensures the regexes are compiled only once
lazy_static! {
    static ref MIXED_NUMBER_RE: Regex = Regex::new(r"^(\-?\d+)_(\d+/\d+)$").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"^(\-?\d+)$").unwrap();
}

/// Parses the given expression as a Fraction
/// Using fractions directly makes easier to implement the arithmetic
pub fn parse_as_fraction(mixed_number_exp: &str) -> Result<Fraction, &str> {
    if MIXED_NUMBER_RE.is_match(mixed_number_exp) {
        let captures = MIXED_NUMBER_RE.captures(mixed_number_exp).unwrap();
        let fraction = Fraction::parse_fraction(captures.get(2).unwrap().as_str())?;
        Fraction::new_mixed(
            captures.get(1).unwrap().as_str().parse().unwrap(), 
            fraction.numerator,
            fraction.denominator
        )
    } else if Fraction::is_fraction(mixed_number_exp) {
        Fraction::parse_fraction(mixed_number_exp)
    } else if NUMBER_RE.is_match(mixed_number_exp) {
        Fraction::new_whole(mixed_number_exp.parse().unwrap())
    } else {
        Err("Unparseable mixed number!")
    }
}

#[cfg(test)]
mod tests;
