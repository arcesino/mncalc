use spectral::prelude::*;
use super::*;

#[test]
fn parse_operation_with_valid_expressions() {
    let operation_expressions = ["2_1/2 * 3_3/4", "  2_1/2 * 3_3/4  "];
    
    // This is the only way to "parameterize" tests in Rust
    for operation_expression in &operation_expressions {
        println!("Testing expression {}", operation_expression); // To help identify failing case
        let operation = parse_operation(operation_expression);

        let expected_left_operand = MixedNumber { 
            whole: Some(2), 
            fraction: Some(Fraction { numerator: 1, denominator: 2}) 
        };
        assert_that!(&operation.left_operand)
            .is_equal_to(&expected_left_operand);
        let expected_operator: &str = &operation.operator;
        assert_that!(expected_operator)
            .is_equal_to(&"*");
        let expected_right_operand = MixedNumber { 
            whole: Some(3), 
            fraction: Some(Fraction { numerator: 3, denominator: 4}) 
        };
        assert_that!(&operation.right_operand)
            .is_equal_to(&expected_right_operand);
    }
}

#[test]
#[should_panic(expected = "Unparseable operation! <2_1/2 * 2 3_3/4>")]
fn parse_operation_with_invalid_expression() {
    let operation_expression = "2_1/2 * 2 3_3/4";

    parse_operation(operation_expression);
}
