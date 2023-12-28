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
