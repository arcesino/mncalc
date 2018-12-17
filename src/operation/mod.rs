use regex::Regex;
use lazy_static::*;
use crate::mixed_number;
use crate::fraction::Fraction;

// This ensures the regex is compiled only once
lazy_static! {
    static ref OPERATION_RE: Regex = Regex::new(r"(^\S+)\s+([\+\-\*/])\s+(\S+)$").unwrap();
}

/// Models the elements of a simple fraction operation
pub struct Operation {
    pub left_operand: Fraction,
    pub operator: String,
    pub right_operand: Fraction
}

impl Operation {

    /// Parses an operation using a simple regex and delegates specific parsing to downstream modules
    pub fn parse_operation(operation_expression: &str) -> Result<Operation, &str> {
        let trimmed_operation_expression = operation_expression.trim();
        let captures = OPERATION_RE.captures(trimmed_operation_expression)
            .ok_or("Unparseable operation!")?;

        Ok(Operation {
            left_operand: mixed_number::parse_as_fraction(captures.get(1).unwrap().as_str())?,
            operator: captures.get(2).unwrap().as_str().to_string(),
            right_operand: mixed_number::parse_as_fraction(captures.get(3).unwrap().as_str())?,
        })
    }

    /// Triggers computation of the operation. 
    pub fn compute(&self) -> Result<Fraction, &str> {
        match self.operator.as_str() {
            "+" => Ok(self.left_operand.add(&self.right_operand)),
            "-" => Ok(self.left_operand.substract(&self.right_operand)),
            "*" => Ok(self.left_operand.multiply(&self.right_operand)),
            "/" => self.left_operand.divide(&self.right_operand),
            _ => Err("Unsupported operation!") // This will never happen
        }
    }
}

#[cfg(test)]
mod tests;
