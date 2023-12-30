# enum-extract

A deriving proc-macro that generates functions to the inner members of the enum.

This is a heavily modified fork of [enum-as-inner](https://github.com/bluejekyll/enum-as-inner).
Some of the key differences are listed below:

1. `as_[variant]`, `into_[variant]` and `as_[variant]_mut` methods return a Result that can contain a concrete error type `EnumExtractError`.
    - The error is returned when the actual variant does not match the expected variant, and it contains both the name of the expected variant and the name of the actual variant for troubleshooting purposes.
    - The error also implements Display with a message of `expected {expected}, got {actual}`,
      so that you have a place to start troubleshooting rather than calling `unwrap` on an `Option`,
      or repeatedly writing better error messages by hand.
2. Added `extract_as_[variant]`, `extract_into_[variant]` and `extract_as_[variant]_mut` methods,
   which panic if the actual variant does not match the expected variant.
    - These are very useful in tests where panicking is acceptable,
      especially when combined with the better error messages supported by the `EnumExtractError` struct.

## Crates

This project consists of two crates that should be used together.
They are separate because crates the export procedural macros can _only_ export procedural macros.
They should be treated as a single project,
and will therefore be versioned in lock-step.

### enum-extract-macro

[![Dependency Status](https://deps.rs/repo/github/James-LG/enum-extract/status.svg?path=crates%2Fenum-extract-macro)](https://deps.rs/repo/github/James-LG/enum-extract?path=crates%2Fenum-extract-macro)
[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/James-LG/enum-extract/blob/master/LICENSE-MIT)
[![License Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/James-LG/enum-extract/blob/master/LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/enum-extract-macro.svg)](https://crates.io/crates/enum-extract-macro)
[![doc.rs](https://docs.rs/enum-extract-macro/badge.svg)](https://docs.rs/enum-extract-macro)

Provides the `EnumExtract` derive macro that can be used on enums to get `as_[variant]` functions for each variant,
along with other useful functions.

See the [documentation](https://docs.rs/enum-extract-macro) for examples and more details.

### enum-extract-error

[![Dependency Status](https://deps.rs/repo/github/James-LG/enum-extract/status.svg?path=crates%2Fenum-extract-error)](https://deps.rs/repo/github/James-LG/enum-extract?path=crates%2Fenum-extract-error)
[![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/James-LG/enum-extract/blob/master/LICENSE-MIT)
[![License Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/James-LG/enum-extract/blob/master/LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/enum-extract-error.svg)](https://crates.io/crates/enum-extract-error)
[![doc.rs](https://docs.rs/enum-extract-error/badge.svg)](https://docs.rs/enum-extract-error)

Provides the `EnumExtractError` used as a return value for the `as_[variant]` functions.

See the [documentation](https://docs.rs/enum-extract-error) for examples and more details.
