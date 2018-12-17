use regex::Regex;
use lazy_static::*;

// This ensures the regexes are compiled only once
lazy_static! {
    static ref FRACTION_RE: Regex = Regex::new(r"^(\d+)/(\d+)$").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Fraction {
        Fraction {
            numerator: numerator,
            denominator: denominator
        }
    }
}

pub fn is_fraction(fraction_exp: &str) -> bool {
    return FRACTION_RE.is_match(fraction_exp);
}

pub fn parse_fraction(fraction_exp: &str) -> Fraction {
    if !is_fraction(fraction_exp) {
        panic!("Unparseable fraction! <{}>", fraction_exp);
    }

    let captures = FRACTION_RE.captures(fraction_exp).unwrap();
    Fraction::new(
        captures.get(1).unwrap().as_str().parse().unwrap(), 
        captures.get(2).unwrap().as_str().parse().unwrap()
    )
}

#[cfg(test)]
mod tests;