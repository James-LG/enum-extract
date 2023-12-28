// Copyright 2015-2018 Benjamin Fry <benjaminfry@me.com>
// Copyright 2023 James La Novara-Gsell <james.lanovara.gsell@gmail.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Derive functions on an Enum for easily accessing individual items in the Enum.
//! This crate is intended to be used with the [enum-extract-error](https://crates.io/crates/enum-extract-error) crate.
//!
//! # Examples
//!
//! ## Unit Variants
//!
//! Check if the variant is the expected variant:
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum UnitVariants {
//!     One,
//!     Two,
//! }
//!
//! let unit = UnitVariants::One;
//! assert!(unit.is_one());
//! assert!(!unit.is_two());
//! ```
//!
//! ## Unnamed Variants
//!
//! Check if the variant is the expected variant:
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum UnnamedVariants {
//!     One(u32),
//!     Two(u32, i32),
//! }
//!
//! let unnamed = UnnamedVariants::One(1);
//! assert!(unnamed.is_one());
//! assert!(!unnamed.is_two());
//! ```
//!
//! Get the variant's value:
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum UnnamedVariants {
//!     One(u32),
//!     Two(u32, i32),
//! }
//!
//! fn main() -> Result<(), enum_extract_error::EnumExtractError<UnnamedVariants>> {
//!     let mut unnamed = UnnamedVariants::One(1);
//!
//!     // returns a reference to the value
//!     let one = unnamed.as_one()?;
//!     assert_eq!(*one, 1);
//!
//!     // returns a mutable reference to the value
//!     let one = unnamed.as_one_mut()?;
//!     assert_eq!(*one, 1);
//!
//!     // returns the value by consuming the enum
//!     let one = unnamed.into_one()?;
//!     assert_eq!(one, 1);
//!
//!     Ok(())
//! }
//! ```
//!
//! If the variant has multiple values, a tuple will be returned:
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum UnnamedVariants {
//!     One(u32),
//!     Two(u32, i32),
//! }
//!
//! fn main() -> Result<(), enum_extract_error::EnumExtractError<UnnamedVariants>> {
//!     let mut unnamed = UnnamedVariants::Two(1, 2);
//!
//!     // returns a reference to the value
//!     let two = unnamed.as_two()?;
//!     assert_eq!(two, (&1, &2));
//!
//!     // returns a mutable reference to the value
//!     let two = unnamed.as_two_mut()?;
//!     assert_eq!(two, (&mut 1, &mut 2));
//!
//!     // returns the value by consuming the enum
//!     let two = unnamed.into_two()?;
//!     assert_eq!(two, (1, 2));
//!
//!     Ok(())
//! }
//! ```
//!
//! Extract variants of all of the above methods will panic with a decent message if the variant is not the expected variant.
//! Very useful for testing, but not recommended for production code.
//!
//! See the [enum-extract-error](https://crates.io/crates/enum-extract-error) crate for more information on the error type.
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum UnnamedVariants {
//!     One(u32),
//!     Two(u32, i32),
//! }
//!
//! let mut unnamed = UnnamedVariants::One(1);
//!
//! // returns a reference to the value
//! let one = unnamed.extract_as_one();
//! assert_eq!(*one, 1);
//!
//! // returns a mutable reference to the value
//! let one = unnamed.extract_as_one_mut();
//! assert_eq!(*one, 1);
//!
//! // returns the value by consuming the enum
//! let one = unnamed.extract_into_one();
//! assert_eq!(one, 1);
//! ```
//!
//! ```should_panic
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum UnnamedVariants {
//!     One(u32),
//!     Two(u32, i32),
//! }
//!
//! let unnamed = UnnamedVariants::One(1);
//!
//! // panics with a decent message
//! let one = unnamed.extract_as_two();
//! ```
//!
//! ## Named Variants
//!
//! Check if the variant is the expected variant:
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum NamedVariants {
//!     One {
//!         first: u32
//!     },
//!     Two {
//!         first: u32,
//!         second: i32
//!     },
//! }
//!
//! let named = NamedVariants::One { first: 1 };
//! assert!(named.is_one());
//! assert!(!named.is_two());
//! ```
//!
//! Get the variant's value:
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum NamedVariants {
//!     One {
//!         first: u32
//!     },
//!     Two {
//!         first: u32,
//!         second: i32
//!     },
//! }
//!
//! fn main() -> Result<(), enum_extract_error::EnumExtractError<NamedVariants>> {
//!     let mut named = NamedVariants::One { first: 1 };
//!
//!     // returns a reference to the value
//!     let one = named.as_one()?;
//!     assert_eq!(*one, 1);
//!
//!     // returns a mutable reference to the value
//!     let one = named.as_one_mut()?;
//!     assert_eq!(*one, 1);
//!
//!     // returns the value by consuming the enum
//!     let one = named.into_one()?;
//!     assert_eq!(one, 1);
//!
//!     Ok(())
//! }
//! ```
//!
//! If the variant has multiple values, a tuple will be returned:
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum NamedVariants {
//!     One {
//!         first: u32
//!     },
//!     Two {
//!         first: u32,
//!         second: i32
//!     },
//! }
//!
//! fn main() -> Result<(), enum_extract_error::EnumExtractError<NamedVariants>> {
//!     let mut unnamed = NamedVariants::Two { first: 1, second: 2 };
//!
//!     // returns a reference to the value
//!     let two = unnamed.as_two()?;
//!     assert_eq!(two, (&1, &2));
//!
//!     // returns a mutable reference to the value
//!     let two = unnamed.as_two_mut()?;
//!     assert_eq!(two, (&mut 1, &mut 2));
//!
//!     // returns the value by consuming the enum
//!     let two = unnamed.into_two()?;
//!     assert_eq!(two, (1, 2));
//!
//!     Ok(())
//! }
//! ```
//!
//! Extract variants of all of the above methods will panic with a decent message if the variant is not the expected variant.
//! Very useful for testing, but not recommended for production code.
//!
//! See the [enum-extract-error](https://crates.io/crates/enum-extract-error) crate for more information on the error type.
//!
//! ```rust
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum NamedVariants {
//!     One {
//!         first: u32
//!     },
//!     Two {
//!         first: u32,
//!         second: i32
//!     },
//! }
//!
//! let mut named = NamedVariants::One { first: 1 };
//!
//! // returns a reference to the value
//! let one = named.extract_as_one();
//! assert_eq!(*one, 1);
//!
//! // returns a mutable reference to the value
//! let one = named.extract_as_one_mut();
//! assert_eq!(*one, 1);
//!
//! // returns the value by consuming the enum
//! let one = named.extract_into_one();
//! assert_eq!(one, 1);
//! ```
//!
//! ```should_panic
//! use enum_extract::EnumExtract;
//!
//! #[derive(Debug, EnumExtract)]
//! enum NamedVariants {
//!     One {
//!         first: u32
//!     },
//!     Two {
//!         first: u32,
//!         second: i32
//!     },
//! }
//!
//! let named = NamedVariants::One { first: 1 };
//!
//! // panics with a decent message
//! let one = named.extract_as_two();
//! ```

#![warn(missing_docs)]

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput};

mod function_def;
mod named_enum_functions;
mod unit_enum_functions;
mod unnamed_enum_functions;

/// Derive functions on an Enum for easily accessing individual items in the Enum
#[proc_macro_derive(EnumExtract, attributes(derive_err))]
pub fn enum_extract(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // get a usable token stream
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let generics = &ast.generics;

    let enum_data = if let syn::Data::Enum(data) = &ast.data {
        data
    } else {
        panic!("{} is not an enum", name);
    };

    let mut expanded = TokenStream::new();

    // Build the impl
    let fns = impl_all_as_fns(name, generics, enum_data);

    expanded.extend(fns);

    proc_macro::TokenStream::from(expanded)
}

/// Returns an impl block for all of the enum's functions.
fn impl_all_as_fns(enum_name: &Ident, generics: &syn::Generics, data: &DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let err_name = syn::Ident::new("EnumExtractError", Span::call_site());
    let err_path = syn::Path::from(syn::PathSegment::from(syn::Ident::new(
        "enum_extract_error",
        Span::call_site(),
    )));
    let err_type = get_error_type(&err_name, &err_path);
    let err_type_with_generics =
        get_error_type_with_generics(err_name, err_path, enum_name, generics);

    let mut stream = TokenStream::new();
    let mut variant_names = TokenStream::new();
    for variant_data in &data.variants {
        let variant_name = &variant_data.ident;

        let tokens = match &variant_data.fields {
            syn::Fields::Unit => unit_enum_functions::all_unit_functions(enum_name, variant_name),
            syn::Fields::Unnamed(unnamed) => unnamed_enum_functions::all_unnamed_functions(
                enum_name,
                variant_name,
                &err_type,
                &err_type_with_generics,
                unnamed,
            ),
            syn::Fields::Named(named) => named_enum_functions::all_named_functions(
                enum_name,
                variant_name,
                &err_type,
                &err_type_with_generics,
                named,
            ),
        };

        stream.extend(tokens);

        let variant_name = match &variant_data.fields {
            syn::Fields::Unit => quote!(Self::#variant_name => stringify!(#variant_name),),
            syn::Fields::Unnamed(_) => {
                quote!(Self::#variant_name(..) => stringify!(#variant_name),)
            }
            syn::Fields::Named(_) => quote!(Self::#variant_name{..} => stringify!(#variant_name),),
        };

        variant_names.extend(variant_name);
    }

    quote!(
        impl #impl_generics #enum_name #ty_generics #where_clause {
            #stream

            /// Returns the name of the variant.
            fn variant_name(&self) -> &'static str {
                match self {
                    #variant_names
                    _ => unreachable!(),
                }
            }
        }
    )
}

/// Returns the error type. ex: `EnumExtractError`
fn get_error_type(err_name: &Ident, err_path: &syn::Path) -> syn::Type {
    let err_type = {
        let last_segment = syn::PathSegment::from(err_name.clone());
        let mut path = err_path.clone();
        path.segments.push(last_segment);
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: path,
        })
    };
    err_type
}

/// Returns the error type with generics. ex: `EnumExtractError<T>`
fn get_error_type_with_generics(
    err_name: Ident,
    err_path: syn::Path,
    enum_name: &Ident,
    generics: &syn::Generics,
) -> syn::Type {
    let err_type_with_generics = {
        let mut last_segment = syn::PathSegment::from(err_name.clone());
        let mut path = err_path.clone();

        let mut inner_type_path = syn::Path::from(syn::PathSegment::from(enum_name.clone()));
        let inner_type_segment = inner_type_path.segments.last_mut().unwrap();
        let mut generic_args = syn::punctuated::Punctuated::new();
        for param in generics.params.iter() {
            match param {
                syn::GenericParam::Lifetime(lifetime_param) => {
                    generic_args.push(syn::GenericArgument::Lifetime(syn::Lifetime::new(
                        &format!("'{}", lifetime_param.lifetime.ident),
                        Span::call_site(),
                    )));
                }
                syn::GenericParam::Const(const_param) => {
                    generic_args.push(syn::GenericArgument::Const(syn::Expr::Path(
                        syn::ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: syn::Path::from(syn::PathSegment::from(
                                const_param.ident.clone(),
                            )),
                        },
                    )));
                }
                syn::GenericParam::Type(type_param) => {
                    generic_args.push(syn::GenericArgument::Type(syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path::from(syn::PathSegment::from(type_param.ident.clone())),
                    })));
                }
            }
        }
        inner_type_segment.arguments =
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: syn::token::Lt::default(),
                args: generic_args,
                gt_token: syn::token::Gt::default(),
            });

        last_segment.arguments =
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: syn::token::Lt::default(),
                args: syn::punctuated::Punctuated::from_iter(vec![syn::GenericArgument::Type(
                    syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: inner_type_path,
                    }),
                )]),
                gt_token: syn::token::Gt::default(),
            });
        path.segments.push(last_segment);
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: path,
        })
    };
    err_type_with_generics
}
