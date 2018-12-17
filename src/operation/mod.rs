use regex::Regex;
use lazy_static::*;
use crate::mixed_number::*;
use crate::fraction::Fraction;

// This ensures the regex is compiled only once
lazy_static! {
    static ref RE: Regex = Regex::new(r"^\S+\s+[\+\-\*/]\s+\S+$").unwrap();
}

pub struct Operation {
    pub left_operand: MixedNumber,
    pub operator: String,
    pub right_operand: MixedNumber
}

impl Operation {
    pub fn compute(&self) -> MixedNumber {
        MixedNumber {
            whole: Some(1),
            fraction: Some(Fraction { numerator: 1, denominator: 2})
        }
    }
}

pub fn parse_operation(operation_expression: &str) -> Operation {
    let trimmed_operation_expression = operation_expression.trim();
    check_parseable_expression(trimmed_operation_expression);
    let elements: Vec<&str> = trimmed_operation_expression.split(' ').collect();

    Operation {
        left_operand: parse_mixed_number(elements[0]),
        operator: elements[1].to_string(),
        right_operand: parse_mixed_number(elements[2]),
    }
}

pub fn check_parseable_expression(operation_expression: &str) {
    if !RE.is_match(operation_expression) {
        panic!("Unparseable operation! <{}>", operation_expression)
    }
}

#[cfg(test)]
mod tests;
