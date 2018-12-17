use spectral::prelude::*;
use super::*;

#[test]
fn is_fraction_with_valid_expression() {
    let fraction_exp = "1/2";

    assert_that!(&is_fraction(fraction_exp))
        .is_true();
}

#[test]
fn is_fraction_with_invalid_expression() {
    let fraction_exp = "2";

    assert_that!(&is_fraction(fraction_exp))
        .is_false();
}

#[test]
fn parse_fraction_with_valid_expresion() {
    let fraction_exp = "1/2";

    let fraction = parse_fraction(fraction_exp);

    assert_that!(&fraction.numerator)
        .is_equal_to(&1);
    assert_that!(&fraction.denominator)
        .is_equal_to(&2);
}

#[test]
#[should_panic(expected = "Unparseable fraction! <2>")]
fn parse_fraction_with_invalid_expresion() {
    let fraction_exp = "2";

    parse_fraction(fraction_exp);
}
