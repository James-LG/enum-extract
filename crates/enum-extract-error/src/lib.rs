use thiserror::Error;

#[derive(Error, Debug)]
#[error("expected {expected}, got {actual}")]
pub struct EnumExtractError<T> {
    pub expected: &'static str,
    pub actual: &'static str,
    pub value: Option<T>,
}

impl<T> EnumExtractError<T> {
    pub fn new(expected: &'static str, actual: &'static str, value: Option<T>) -> Self {
        Self {
            expected,
            actual,
            value,
        }
    }
}
