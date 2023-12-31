#![allow(unused)]

use enum_extract_macro::EnumExtract;

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
enum ManyVariants {
    One { one: u32 },
    Two { one: u32, two: i32 },
    Three { one: bool, two: u32, three: i64 },
}

#[test]
fn test_one_named() {
    let mut many = ManyVariants::One { one: 1 };

    assert!(many.is_one());
    assert!(!many.is_two());
    assert!(!many.is_three());

    assert!(many.as_one().is_ok());
    assert!(many.as_two().is_err());
    assert!(many.as_three().is_err());

    assert!(many.as_one_mut().is_ok());
    assert!(many.as_two_mut().is_err());
    assert!(many.as_three_mut().is_err());

    assert_eq!(*many.as_one().unwrap(), 1_u32);
    assert_eq!(*many.as_one_mut().unwrap(), 1_u32);
}

#[test]
fn test_two_named() {
    let mut many = ManyVariants::Two { one: 1, two: 2 };

    assert!(!many.is_one());
    assert!(many.is_two());
    assert!(!many.is_three());

    assert!(many.as_one().is_err());
    assert!(many.as_two().is_ok());
    assert!(many.as_three().is_err());

    assert!(many.as_one_mut().is_err());
    assert!(many.as_two_mut().is_ok());
    assert!(many.as_three_mut().is_err());

    assert_eq!(many.as_two().unwrap(), (&1_u32, &2_i32));
    assert_eq!(many.as_two_mut().unwrap(), (&mut 1_u32, &mut 2_i32));
    assert_eq!(many.into_two().unwrap(), (1_u32, 2_i32));
}

#[test]
fn test_three_named() {
    let mut many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    assert!(!many.is_one());
    assert!(!many.is_two());
    assert!(many.is_three());

    assert!(many.as_one().is_err());
    assert!(many.as_two().is_err());
    assert!(many.as_three().is_ok());

    assert!(many.as_one_mut().is_err());
    assert!(many.as_two_mut().is_err());
    assert!(many.as_three_mut().is_ok());

    assert_eq!(many.as_three().unwrap(), (&true, &1_u32, &2_i64));
    assert_eq!(
        many.as_three_mut().unwrap(),
        (&mut true, &mut 1_u32, &mut 2_i64)
    );
    assert_eq!(many.into_three().unwrap(), (true, 1_u32, 2_i64));
}

#[test]
fn error_should_contain_expected_and_actual() {
    let many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    let error = many.into_one().unwrap_err();

    assert_eq!(error.source.expected, "One");
    assert_eq!(error.source.actual, "Three");
    assert_eq!(error.value, many);
    assert_eq!(error.to_string(), "expected One, got Three");
}

#[test]
fn extract_as_should_not_panic_when_expected_variant_matches_actual() {
    let many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    assert_eq!(many.extract_as_three(), (&true, &1_u32, &2_i64));
}

#[test]
#[should_panic(expected = "expected One, got Three")]
fn extract_as_should_panic_when_expected_variant_does_not_match_actual() {
    let many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    many.extract_as_one();
}

#[test]
fn extract_as_mut_should_not_panic_when_expected_variant_matches_actual() {
    let mut many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    assert_eq!(
        many.extract_as_three_mut(),
        (&mut true, &mut 1_u32, &mut 2_i64)
    );
}

#[test]
#[should_panic(expected = "expected One, got Three")]
fn extract_as_mut_should_panic_when_expected_variant_does_not_match_actual() {
    let mut many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    many.extract_as_one_mut();
}

#[test]
fn extract_into_should_not_panic_when_expected_variant_matches_actual() {
    let many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    assert_eq!(many.extract_into_three(), (true, 1_u32, 2_i64));
}

#[test]
#[should_panic(expected = "expected One, got Three")]
fn extract_into_should_panic_when_expected_variant_does_not_match_actual() {
    let many = ManyVariants::Three {
        one: true,
        two: 1,
        three: 2,
    };

    many.extract_into_one();
}
