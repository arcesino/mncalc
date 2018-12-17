use spectral::prelude::*;
use super::*;
use crate::mixed_number;

#[test]
fn parse_operation_with_valid_expressions() {
    let operation_expressions = ["2_1/2 * 3_3/4", "  2_1/2 * 3_3/4  ", "2_1/2  *  3_3/4"];
    
    // This is the only way to "parameterize" tests in Rust
    for operation_expression in &operation_expressions {
        println!("Testing expression {}", operation_expression); // To help identify failing test case
        let operation = Operation::parse_operation(operation_expression).unwrap();

        let expected_left_operand = Fraction::new(5, 2).unwrap();
        assert_that!(&operation.left_operand)
            .is_equal_to(&expected_left_operand);
        let expected_operator: &str = &operation.operator;
        assert_that!(expected_operator)
            .is_equal_to(&"*");
        let expected_right_operand = Fraction::new(15, 4).unwrap();
        assert_that!(&operation.right_operand)
            .is_equal_to(&expected_right_operand);
    }
}

#[test]
#[should_panic(expected = "Unparseable operation!")]
fn parse_operation_with_invalid_expression() {
    let operation_expression = "2_1/2 * 2 3_3/4";

    Operation::parse_operation(operation_expression).unwrap();
}

// This function will help test supported computations in tests below
fn test_compute_operations(context: &str, operation_test_cases: &[(&str, &str)]) {
    for (operation_expression, result_expression) in operation_test_cases {
        println!("{} expression {}", context, operation_expression);
        let operation = Operation::parse_operation(operation_expression).unwrap();

        let actual_result = operation.compute().unwrap();

        let expected_result = mixed_number::parse_as_fraction(result_expression).unwrap();
        assert_that!(&actual_result)
            .is_equal_to(&expected_result);
    }
}

#[test]
fn compute_addition() {
    let addition_test_cases = [
        ("2_3/8 + 3_1/5", "5_23/40")
    ];
    
    test_compute_operations("Addition", &addition_test_cases);
}

#[test]
fn compute_substraction() {
    let addition_test_cases = [
        ("2_3/8 - 3_1/5", "-33/40")
    ];
    
    test_compute_operations("Substraction", &addition_test_cases);
}

#[test]
fn compute_multiplication() {
    let addition_test_cases = [
        ("2_3/8 * 3_1/5", "7_3/5")
    ];
    
    test_compute_operations("Multiplication", &addition_test_cases);
}

#[test]
fn compute_division() {
    let addition_test_cases = [
        ("2_3/8 / 3_1/5", "95/128")
    ];
    
    test_compute_operations("Division", &addition_test_cases);
}
