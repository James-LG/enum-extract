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

#[derive(Debug, EnumExtract)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
enum MixedCaseVariants {
    XMLIsNotCool,
    Rust_IsCoolThough(u32),
    YMCA { named: i16 },
}

#[test]
fn test_xml_unit() {
    let mixed = MixedCaseVariants::XMLIsNotCool;

    assert!(mixed.is_xml_is_not_cool());
    assert!(mixed.as_rust_is_cool_though().is_err());
    assert!(mixed.as_ymca().is_err());
}

#[test]
fn test_rust_unnamed() {
    let mixed = MixedCaseVariants::Rust_IsCoolThough(42);

    assert!(!mixed.is_xml_is_not_cool());
    assert!(mixed.as_rust_is_cool_though().is_ok());
    assert!(mixed.as_ymca().is_err());

    assert_eq!(*mixed.as_rust_is_cool_though().unwrap(), 42);
    assert_eq!(mixed.into_rust_is_cool_though().unwrap(), 42);
}

#[test]
fn test_ymca_named() {
    let mixed = MixedCaseVariants::YMCA { named: -32_768 };

    assert!(!mixed.is_xml_is_not_cool());
    assert!(mixed.as_rust_is_cool_though().is_err());
    assert!(mixed.as_ymca().is_ok());

    assert_eq!(*mixed.as_ymca().unwrap(), (-32_768));
    assert_eq!(mixed.into_ymca().unwrap(), (-32_768));
}
