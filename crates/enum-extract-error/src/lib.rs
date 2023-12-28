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
//! let error: EnumExtractError<()> = EnumExtractError::new("One", "Three", None);
//! assert_eq!(error.to_string(), "expected One, got Three");
//! ```

#![warn(missing_docs)]

use thiserror::Error;

/// An error that occurs when the actual variant does not match the expected variant.
#[derive(Error, Debug)]
#[error("expected {expected}, got {actual}")]
pub struct EnumExtractError<T> {
    /// The name of the expected variant.
    pub expected: &'static str,
    /// The name of the actual variant.
    pub actual: &'static str,
    /// The value of the actual variant.
    ///
    /// Only present when the value was consumed. For example, `into_[variant]` consumes the value.
    pub value: Option<T>,
}

impl<T> EnumExtractError<T> {
    /// Create a new [`EnumExtractError`].
    pub fn new(expected: &'static str, actual: &'static str, value: Option<T>) -> Self {
        Self {
            expected,
            actual,
            value,
        }
    }
}
