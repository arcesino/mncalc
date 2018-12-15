use std::fmt;
use regex::Regex;
use lazy_static::*;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+_\d+/\d+\s+[\+\-\*/]\s+\d+_\d+/\d+").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32
}

#[derive(Debug, PartialEq)]
pub struct MixedNumber {
    pub whole: i32,
    pub fraction: Fraction
}

impl fmt::Display for MixedNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.whole)
    }
}

pub struct Operation {
    pub left_operand: MixedNumber,
    pub operator: String,
    pub right_operand: MixedNumber
}

impl Operation {
    pub fn compute(&self) -> MixedNumber {
        MixedNumber {
            whole: 1,
            fraction: Fraction { numerator: 1, denominator: 2}
        }
    }
}

pub fn parse_mixed_number(expression: &str) -> MixedNumber {
    let elements: Vec<&str> = expression.split('_').collect();
    let fraction_elements: Vec<&str> = elements[1].split('/').collect();

    MixedNumber {
        whole: elements[0].parse().unwrap(),
        fraction: Fraction { 
            numerator: fraction_elements[0].parse().unwrap(), 
            denominator: fraction_elements[1].parse().unwrap()
        }
    }
}

pub fn parse_operation(expression: &str) -> Operation {
    check_parseable_expression(expression);
    let elements: Vec<&str> = expression.split(' ').collect();

    Operation {
        left_operand: parse_mixed_number(elements[0]),
        operator: elements[1].to_string(),
        right_operand: parse_mixed_number(elements[2]),
    }
}

pub fn check_parseable_expression(expression: &str) {
    if !RE.is_match(expression) {
        panic!("Unparseable expression! {}", expression)
    }
}

#[cfg(test)]
mod operation_tests;
