# enum-extract

A deriving proc-macro that generates functions to the inner members of the enum.

Based on [enum-as-inner](https://github.com/bluejekyll/enum-as-inner) with a few key differences:

1. `as_[variant]`, `into_[variant]` and `as_[variant]_mut` methods return a Result that can contain a concrete error type `EnumExtractError`.
    - The error is returned when the actual variant does not match the expected variant, and the error contains both of these details for easier troubleshooting.
    - The error also implements Display with a message of `expected {expected}, got {actual}`,
      so that you have a place to start troubleshooting rather than calling `unwrap` on an `Option`,
      or repeatedly writing better error messages by hand.
2. Added `extract_as_[variant]`, `extract_into_[variant]` and `extract_as_[variant]_mut` methods,
   which panic if the actual variant does not match the expected variant.
    - These are very useful in tests where panicking is acceptable,
      especially when combined with the better error messages supported by the `EnumExtractError` struct.
