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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, EnumExtract)]
enum WithGenerics<T: Clone + Copy + std::fmt::Debug> {
    A(T),
    B(T),
}

#[test]
fn with_generics() {
    let mut with_generics = WithGenerics::A(100);

    assert!(with_generics.is_a());
    assert!(!with_generics.is_b());

    assert!(with_generics.as_a().is_ok());
    assert!(with_generics.as_b().is_err());

    assert_eq!(with_generics.into_a().unwrap(), 100);
    assert_eq!(*with_generics.as_a().unwrap(), 100);
    assert_eq!(*with_generics.as_a_mut().unwrap(), 100);

    assert!(with_generics.into_b().is_err());
    assert!(with_generics.as_b().is_err());
    assert!(with_generics.as_b_mut().is_err());
}
