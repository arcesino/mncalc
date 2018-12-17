use spectral::prelude::*;
use super::*;

#[test]
fn parse_full_mixed_number() {
    let mixed_number_exp = "2_1/2";

    let mixed_number = parse_mixed_number(mixed_number_exp);

    assert_that!(&mixed_number.whole)
        .is_some()
        .is_equal_to(&2);
    assert_that!(&mixed_number.fraction)
        .is_some()
        .is_equal_to(&Fraction { numerator: 1, denominator: 2 });
}

#[test]
fn parse_fraction_only_mixed_number() {
    let mixed_number_exp = "1/2";

    let mixed_number = parse_mixed_number(mixed_number_exp);

    assert_that!(&mixed_number.whole)
        .is_none();
    assert_that!(&mixed_number.fraction)
        .is_some()
        .is_equal_to(&Fraction { numerator: 1, denominator: 2 });
}

#[test]
fn parse_whole_only_mixed_number() {
    let mixed_number_exp = "2";

    let mixed_number = parse_mixed_number(mixed_number_exp);

    assert_that!(&mixed_number.whole)
        .is_some()
        .is_equal_to(&2);
    assert_that!(&mixed_number.fraction)
        .is_none();
}

#[test]
#[should_panic(expected = "Unparseable mixed number! <2__1/2>")]
fn parse_mixed_number_with_invalid_expresion() {
    let mixed_number_exp = "2__1/2";

    parse_mixed_number(mixed_number_exp);
}
