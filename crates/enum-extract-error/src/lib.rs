//! Companion crate for [`enum-extract`](https://crates.io/crates/enum-extract).
//!
//! This crate provides the `EnumExtractError` type, which is used by `enum-extract` to report errors.
//! It must be a separate crate because `enum-extract` is a `proc-macro` crate,
//! which are only allowed to export procedural macros.
//!
//! # Example Message
//!
//! ```rust
//! use enum_extract_error::EnumExtractError;
//!
//! let error: EnumExtractError = EnumExtractError::new("One", "Three");
//! assert_eq!(error.to_string(), "expected One, got Three");
//! ```

#![warn(missing_docs)]

use thiserror::Error;

/// An error that occurs when the actual variant does not match the expected variant.
#[derive(Error, Debug, Clone)]
#[error("expected {expected}, got {actual}")]
pub struct EnumExtractError {
    /// The name of the expected variant.
    pub expected: &'static str,
    /// The name of the actual variant.
    pub actual: &'static str,
}

impl EnumExtractError {
    /// Create a new [`EnumExtractError`].
    pub fn new(expected: &'static str, actual: &'static str) -> Self {
        Self { expected, actual }
    }
}

/// An error that occurs when the actual variant does not match the expected variant.
///
/// This error is only produced by functions that consume the value,
/// and therefore holds on to the value in case it is needed.
#[derive(Error, Debug)]
#[error("{source}")]
pub struct EnumExtractValueError<T> {
    /// The inner extraction error.
    #[source]
    pub source: EnumExtractError,

    /// The value of the actual variant.
    pub value: T,
}

impl<T> EnumExtractValueError<T> {
    /// Create a new [`EnumExtractError`].
    pub fn from_plain_error(extract_error: EnumExtractError, value: T) -> Self {
        Self {
            source: extract_error,
            value,
        }
    }

    /// Create a new [`EnumExtractError`].
    pub fn new(expected: &'static str, actual: &'static str, value: T) -> Self {
        Self {
            source: EnumExtractError::new(expected, actual),
            value,
        }
    }
}

impl<T> From<EnumExtractValueError<T>> for EnumExtractError {
    fn from(value: EnumExtractValueError<T>) -> Self {
        value.source
    }
}
