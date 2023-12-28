use enum_extract::EnumExtract;

pub mod name_collisions {
    #![allow(dead_code, missing_copy_implementations, missing_docs)]
    pub struct Option;
    pub struct Some;
    pub struct None;
    pub struct Result;
    pub struct Ok;
    pub struct Err;
}
#[allow(unused_imports)]
use name_collisions::*;

#[derive(Debug, PartialEq, Clone, Copy, EnumExtract)]
enum UnitVariants {
    One,
    Two,
    Three,
}

#[test]
fn test_one_unit() {
    let unit = UnitVariants::One;

    assert!(unit.is_one());
    assert!(!unit.is_two());
    assert!(!unit.is_three());
}

#[test]
fn test_two_unit() {
    let unit = UnitVariants::Two;

    assert!(!unit.is_one());
    assert!(unit.is_two());
    assert!(!unit.is_three());
}

#[test]
fn test_three_unit() {
    let unit = UnitVariants::Three;

    assert!(!unit.is_one());
    assert!(!unit.is_two());
    assert!(unit.is_three());
}

#[test]
fn error_should_contain_expected_and_actual() {
    let many = UnitVariants::Three;

    let error = many.into_one().unwrap_err();

    assert_eq!(error.expected, "One");
    assert_eq!(error.actual, "Three");
    assert_eq!(error.value, ::core::option::Option::Some(many));
    assert_eq!(error.to_string(), "expected One, got Three");
}
