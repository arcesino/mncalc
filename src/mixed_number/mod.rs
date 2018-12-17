use regex::Regex;
use std::fmt;
use lazy_static::*;
use crate::fraction::*;

// This ensures the regexes are compiled only once
lazy_static! {
    static ref MIXED_NUMBER_RE: Regex = Regex::new(r"^(\d+)_(\d+/\d+)$").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"^(\d+)$").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct MixedNumber {
    pub whole: Option<i32>,
    pub fraction: Option<Fraction>
}

impl fmt::Display for MixedNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.whole.unwrap())
    }
}

pub fn parse_mixed_number(mixed_number_exp: &str) -> MixedNumber {
    if MIXED_NUMBER_RE.is_match(mixed_number_exp) {
        let captures = MIXED_NUMBER_RE.captures(mixed_number_exp).unwrap();
        MixedNumber {
            whole: captures.get(1).unwrap().as_str().parse().ok(),
            fraction: Some(parse_fraction(captures.get(2).unwrap().as_str()))
        }
    } else if is_fraction(mixed_number_exp) {
        MixedNumber {
            whole: None,
            fraction: Some(parse_fraction(mixed_number_exp))
        }
    } else if NUMBER_RE.is_match(mixed_number_exp) {
        MixedNumber {
            whole: mixed_number_exp.parse().ok(),
            fraction: None
        }
    } else {
        panic!("Unparseable mixed number! <{}>", mixed_number_exp);
    }
}

#[cfg(test)]
mod tests;