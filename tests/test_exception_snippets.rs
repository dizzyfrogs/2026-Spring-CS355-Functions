#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use std::num::ParseIntError;

use functions_exceptions::*;

#[rstest]
fn test_parse_an_int_ok() {
    assert_that!(parse_an_int("7"), is(equal_to(Ok(7))));
    assert_that!(parse_an_int("12"), is(equal_to(Ok(12))));

}

#[rstest]
fn test_parse_an_int_err() {
    assert!(matches!(parse_an_int("20.0"), Err(ParseIntError)));
    assert!(matches!(parse_an_int("The number 337"), Err(ParseIntError)));
    assert!(matches!(parse_an_int("Twenty"), Err(ParseIntError)));
}

#[rstest]
fn test_divide_some() {
    assert_that!(divide(20, 5), is(equal_to(Some(4))));
    assert_that!(divide(7, 5), is(equal_to(Some(1))));
    assert_that!(divide(70, 5), is(equal_to(Some(14))));
}

#[rstest]
fn test_divide_none() {
    assert_that!(divide(72, 0), is(none()));
}
