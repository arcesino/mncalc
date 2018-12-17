use spectral::prelude::*;
use super::*;
use crate::fraction::Fraction;

#[test]
fn parse_full_mixed_number() {
    let mixed_number_exp = "2_1/2";

    let actual = parse_as_fraction(mixed_number_exp);

    let expected = Fraction::new(5, 2);
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
fn parse_full_negative_mixed_number() {
    let mixed_number_exp = "-2_1/2";

    let actual = parse_as_fraction(mixed_number_exp);

    let expected = Fraction::new(-5, 2);
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
fn parse_fraction_only_mixed_number() {
    let mixed_number_exp = "1/2";

    let actual = parse_as_fraction(mixed_number_exp);

    let expected = Fraction::new(1, 2);
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
fn parse_whole_only_mixed_number() {
    let mixed_number_exp = "2";

    let actual = parse_as_fraction(mixed_number_exp);

    let expected = Fraction::new(2, 1);
    assert_that!(&actual)
        .is_equal_to(&expected);
}

#[test]
#[should_panic(expected = "Unparseable mixed number!")]
fn parse_as_fraction_with_invalid_expresion() {
    let mixed_number_exp = "2__1/2";

    parse_as_fraction(mixed_number_exp).unwrap();
}
