use spectral::prelude::*;
use super::*;

#[test]
fn parse_operation_with_valid_expression() {
    let valid_expression = "2_1/2 * 3_3/4";
    
    let operation = parse_operation(valid_expression);

    let expected_left_operand = MixedNumber { whole: 2, fraction: Fraction { numerator: 1, denominator: 2} };
    assert_that!(&operation.left_operand)
        .is_equal_to(&expected_left_operand);
    let expected_operator: &str = &operation.operator;
    assert_that!(expected_operator)
        .is_equal_to(&"*");
    assert_that!(&operation.right_operand)
        .is_equal_to(&MixedNumber { whole: 3, fraction: Fraction { numerator: 3, denominator: 4} });
}

#[test]
#[should_panic(expected = "Unparseable expression!")]
fn parse_operation_with_invalid_expression() {
    let valid_expression = "2_1/2 * 2 3_3/4";
    parse_operation(valid_expression);
}
