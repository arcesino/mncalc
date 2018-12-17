use spectral::prelude::*;
use super::*;

#[test]
fn is_fraction_with_valid_expression() {
    let fraction_exp = "1/2";

    assert_that!(&Fraction::is_fraction(fraction_exp))
        .is_true();
}

#[test]
fn is_fraction_with_invalid_expression() {
    let fraction_exp = "2";

    assert_that!(&Fraction::is_fraction(fraction_exp))
        .is_false();
}

#[test]
fn parse_fraction_with_valid_expresion() {
    let fraction_exp = "1/2";

    let fraction = Fraction::parse_fraction(fraction_exp).unwrap();

    assert_that!(&fraction.numerator)
        .is_equal_to(&1);
    assert_that!(&fraction.denominator)
        .is_equal_to(&2);
}

#[test]
fn parse_fraction_with_valid_negative_expresion() {
    let fraction_exp = "-1/2";

    let fraction = Fraction::parse_fraction(fraction_exp).unwrap();

    assert_that!(&fraction.numerator)
        .is_equal_to(&-1);
    assert_that!(&fraction.denominator)
        .is_equal_to(&2);
}

#[test]
fn parse_fraction_with_negative_elements() {
    let fraction_exp = "-1/-2";

    let fraction = Fraction::parse_fraction(fraction_exp).unwrap();

    assert_that!(&fraction.numerator)
        .is_equal_to(&1);
    assert_that!(&fraction.denominator)
        .is_equal_to(&2);
}

#[test]
#[should_panic(expected = "Unparseable fraction!")]
fn parse_fraction_with_invalid_expresion() {
    let fraction_exp = "2";

    Fraction::parse_fraction(fraction_exp).unwrap();
}

#[test]
fn add_fractions() {
    let x = Fraction::new(1, 2).unwrap();
    let y = Fraction::new(1, 3).unwrap();

    let actual = x.add(&y);

    let expected = Fraction::new(5, 6).unwrap();
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
fn substract_fractions() {
    let x = Fraction::new(1, 2).unwrap();
    let y = Fraction::new(1, 3).unwrap();

    let actual = x.substract(&y);

    let expected = Fraction::new(1, 6).unwrap();
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
fn multiply_fractions() {
    let x = Fraction::new(1, 2).unwrap();
    let y = Fraction::new(1, 3).unwrap();

    let actual = x.multiply(&y);

    let expected = Fraction::new(1, 6).unwrap();
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
fn divide_fractions() {
    let x = Fraction::new(1, 2).unwrap();
    let y = Fraction::new(1, 3).unwrap();

    let actual = x.divide(&y).unwrap();

    let expected = Fraction::new(3, 2).unwrap();
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
#[should_panic(expected = "Division by zero!")]
fn divide_fractions_by_zero() {
    let x = Fraction::new(1, 2).unwrap();
    let y = Fraction::new(0, 3).unwrap();

    x.divide(&y).unwrap();
}

#[test]
fn new_simplifies_fraction() {
    let actual = Fraction::new(3, 12).unwrap();

    let expected = Fraction::new(1, 4).unwrap();
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
#[should_panic(expected = "Fraction with zero denominator!")]
fn new_with_zero_denominator() {
    Fraction::new(3, 0).unwrap();
}

#[test]
fn display_proper_fraction() {
    let fraction = Fraction::new(1, 2).unwrap();

    let actual = format!("{}", fraction);

    assert_that!(actual.as_str())
        .is_equal_to(&"1/2");
}

#[test]
fn display_improper_fraction_as_mixed_number() {
    let fraction = Fraction::new(5, 2).unwrap();

    let actual = format!("{}", fraction);

    assert_that!(actual.as_str())
        .is_equal_to(&"2_1/2");
}

#[test]
fn display_exact_fraction_as_integer() {
    let fraction = Fraction::new(4, 2).unwrap();

    let actual = format!("{}", fraction);

    assert_that!(actual.as_str())
        .is_equal_to(&"2");
}

#[test]
fn display_zero_fraction() {
    let fraction = Fraction::new(0, 2).unwrap();

    let actual = format!("{}", fraction);

    assert_that!(actual.as_str())
        .is_equal_to(&"0");
}

#[test]
fn display_negative_proper_fraction() {
    let fraction = Fraction::new(-5, 3).unwrap();

    let actual = format!("{}", fraction);

    assert_that!(actual.as_str())
        .is_equal_to(&"-1_2/3");
}
