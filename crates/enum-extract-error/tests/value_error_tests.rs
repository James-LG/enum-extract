use enum_extract_error::{EnumExtractError, EnumExtractValueError};

#[test]
fn data_error_should_convert_to_plain_error() {
    // arrange
    let plain_error = EnumExtractError::new("One", "Two");
    let data_error = EnumExtractValueError::from_plain_error(plain_error.clone(), Some(1));

    // act
    let converted_error: EnumExtractError = data_error.into();

    // assert
    assert_eq!(converted_error.expected, plain_error.expected);
    assert_eq!(converted_error.actual, plain_error.actual);
}

#[test]
fn data_error_should_convert_to_plain_error_with_question_mark_operator(
) -> Result<(), EnumExtractError> {
    // arrange
    let result: Result<(), EnumExtractValueError<i32>> = Ok(());

    _ = result?;

    Ok(())
}
