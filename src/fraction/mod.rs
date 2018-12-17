use std::fmt;
use regex::Regex;
use lazy_static::*;
use crate::math;

// This ensures the regexes are compiled only once
lazy_static! {
    static ref FRACTION_RE: Regex = Regex::new(r"^(\-?\d+)/(\-?\d+)$").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32
}

impl Fraction {
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

    fn simplify(fraction: &Fraction) -> Fraction {
        let gcd = math::gcd(math::abs(fraction.numerator), math::abs(fraction.denominator));

        Fraction {
            numerator: fraction.numerator / gcd,
            denominator: fraction.denominator / gcd
        }
    }

    pub fn new_mixed(whole: i32, numerator: i32, denominator: i32) -> Result<Fraction, &'static str> {
        let whole_abs = math::abs(whole);
        let numerator = (whole_abs * denominator) + numerator;

        if whole < 0 {
            Fraction::new(-numerator, denominator)
        } else {
            Fraction::new(numerator, denominator)
        }
    }

    pub fn new_whole(numerator: i32) -> Result<Fraction, &'static str> {
        Fraction::new(numerator, 1)
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

    fn is_proper(&self) -> bool {
        math::abs(self.numerator) < math::abs(self.denominator)
    }

    fn is_exact(&self) -> bool {
        (self.numerator % self.denominator) == 0
    }

    fn is_zero(&self) -> bool {
        self.numerator == 0
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_exact() {
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

pub fn is_fraction(fraction_exp: &str) -> bool {
    FRACTION_RE.is_match(fraction_exp)
}

pub fn parse_fraction(fraction_exp: &str) -> Result<Fraction, &str> {
    if !is_fraction(fraction_exp) {
        Err("Unparseable fraction!")
    } else {
        let captures = FRACTION_RE.captures(fraction_exp).unwrap();
        Fraction::new(
            captures.get(1).unwrap().as_str().parse().unwrap(), 
            captures.get(2).unwrap().as_str().parse().unwrap()
        )
    }
}

#[cfg(test)]
mod tests;
