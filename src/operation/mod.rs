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
        match self.operator.as_str() {
            "+" => self.left_operand.add(&self.right_operand),
            "-" => self.left_operand.substract(&self.right_operand),
            "*" => self.left_operand.multiply(&self.right_operand),
            "/" => self.left_operand.divide(&self.right_operand),
            _ => panic!("Unsupported operation!") // This will never happen
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
