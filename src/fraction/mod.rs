use std::fmt;
use regex::Regex;
use lazy_static::*;
use crate::math;

// This ensures the regexes are compiled only once
lazy_static! {
    static ref FRACTION_RE: Regex = Regex::new(r"^(\-?\d+)/(\-?\d+)$").unwrap();
}

/// Models the elements of a fraction
#[derive(Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32
}

impl Fraction {

    /// Returns `true` if given expression can be parsed as a fraction
    pub fn is_fraction(fraction_exp: &str) -> bool {
        FRACTION_RE.is_match(fraction_exp)
    }

    /// Parses a fraction in the form `1/2` with support for negative numbers
    pub fn parse_fraction(fraction_exp: &str) -> Result<Fraction, &str> {
        if !Fraction::is_fraction(fraction_exp) {
            Err("Unparseable fraction!")
        } else {
            let captures = FRACTION_RE.captures(fraction_exp).unwrap();
            Fraction::new(
                captures.get(1).unwrap().as_str().parse().unwrap(), 
                captures.get(2).unwrap().as_str().parse().unwrap()
            )
        }
    }

    /// Factory method that MUST be used to build a `Fraction` instance.
    /// Fractions created by this method will ALWAYS be simplified and signs correctly managed
    pub fn new(numerator: i32, denominator: i32) -> Result<Fraction, &'static str> {
        if denominator == 0 {
            Err("Fraction with zero denominator!")
        } else {
            let (signed_numerator, signed_denominator) = if numerator < 0 && denominator < 0 {
                (math::abs(numerator), math::abs(denominator))
            } else {
                (numerator, denominator)
            };
            Ok(Fraction::simplify(&Fraction {
                numerator: signed_numerator,
                denominator: signed_denominator
            }))
        }
    }

    /// Factory method that MUST be used to build a `Fraction` from a mixed number.
    /// The resulting `Fraction` will be the improper fraction equivalent to the mixed number.
    /// It leverages to `Fraction::new` to get simplification and sign handling.
    pub fn new_mixed(whole: i32, numerator: i32, denominator: i32) -> Result<Fraction, &'static str> {
        let whole_abs = math::abs(whole);
        let numerator = (whole_abs * denominator) + numerator;

        if whole < 0 {
            Fraction::new(-numerator, denominator)
        } else {
            Fraction::new(numerator, denominator)
        }
    }

    /// Factory method that MUST be used to build a `Fraction` from a whole number.
    /// The resulting `Fraction` will be the given number divided by 1.
    /// It leverages to `Fraction::new` to get simplification and sign handling.
    pub fn new_whole(whole: i32) -> Result<Fraction, &'static str> {
        Fraction::new(whole, 1)
    }

    pub fn add(&self, fraction: &Fraction) -> Fraction {
        let numerator = (self.numerator * fraction.denominator) + (self.denominator * fraction.numerator);
        let denominator = self.denominator * fraction.denominator;

        Fraction::new(numerator, denominator).unwrap()
    }

    pub fn substract(&self, fraction: &Fraction) -> Fraction {
        let negative_fraction = Fraction::new(fraction.numerator * -1, fraction.denominator).unwrap();

        self.add(&negative_fraction)
    }

    pub fn multiply(&self, fraction: &Fraction) -> Fraction {
        let numerator = self.numerator * fraction.numerator;
        let denominator = self.denominator * fraction.denominator;

        Fraction::new(numerator, denominator).unwrap()
    }

    pub fn divide(&self, fraction: &Fraction) -> Result<Fraction, &str> {
        if fraction.is_zero() {
            Err("Division by zero!")
        } else {
            let numerator = self.numerator * fraction.denominator;
            let denominator = self.denominator * fraction.numerator;

            Fraction::new(numerator, denominator)
        }
    }

    fn simplify(fraction: &Fraction) -> Fraction {
        let gcd = math::gcd(math::abs(fraction.numerator), math::abs(fraction.denominator));

        Fraction {
            numerator: fraction.numerator / gcd,
            denominator: fraction.denominator / gcd
        }
    }

    fn is_proper(&self) -> bool {
        math::abs(self.numerator) < math::abs(self.denominator)
    }

    fn is_whole(&self) -> bool {
        (self.numerator % self.denominator) == 0
    }

    fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

/// Implementation to make `Fraction` displayable.
impl fmt::Display for Fraction {

    /// Formats a `Fraction` to a whole number, proper fraction or mixed number.
    /// Improper fractions are ALWAYS formatted as either whole or mixed number.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_whole() {
            write!(f, "{}", self.numerator / self.denominator)
        } else if self.is_proper() {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            let numerator = math::abs(self.numerator);
            let whole = numerator / self.denominator;
            let signed_whole = if self.numerator < 0 { -whole } else { whole };
            write!(f, "{}_{}/{}",  signed_whole, numerator % self.denominator,  self.denominator)
        }
    }
}

#[cfg(test)]
mod tests;
