use heck::ToSnakeCase;
use proc_macro2::{Ident, Span};
use quote::ToTokens;

static DOCS_ERROR_TYPE: &'static str = "enum_extract_error::EnumExtractError";

pub struct FunctionDef {
    pub declaration: syn::Ident,
    pub docs: String,
}

impl ToTokens for FunctionDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.declaration.to_tokens(tokens)
    }
}

impl FunctionDef {
    pub fn new_is_variant(enum_name: &Ident, variant_name: &Ident) -> Self {
        FunctionDef {
            docs: format!(
                "Returns true if this is a `{}::{}`, otherwise false",
                enum_name, variant_name,
            ),
            declaration: Ident::new(
                &format!("is_{}", variant_name).to_snake_case(),
                Span::call_site(),
            ),
        }
    }

    pub fn new_as_variant(enum_name: &Ident, variant_name: &Ident) -> Self {
        FunctionDef {
            docs: format!(
                "Returns references to the inner fields if this is a `{}::{}`, otherwise an [`{}`]",
                enum_name, variant_name, DOCS_ERROR_TYPE,
            ),
            declaration: Ident::new(
                &format!("as_{}", variant_name).to_snake_case(),
                Span::call_site(),
            ),
        }
    }

    pub fn new_extract_as_variant(enum_name: &Ident, variant_name: &Ident) -> Self {
        let alt_fn = Self::new_as_variant(enum_name, variant_name);

        FunctionDef {
            docs: format!(
                r#"
                Returns references to the inner fields if this is a `{enum_name}::{variant}`, otherwise panics.
    
                # Panics
    
                Panics when the value is not a `{enum_name}::{variant}`.
                Prefer using a `match` or [`{enum_name}::{alt_fn}`]; only use this if you *know* this is a `{enum_name}::{variant}`,
                or if panicking is ok, such as during testing.
                "#,
                enum_name = enum_name,
                variant = variant_name,
                alt_fn = alt_fn.declaration,
            ),
            declaration: Ident::new(
                &format!("extract_as_{}", variant_name).to_snake_case(),
                Span::call_site(),
            ),
        }
    }

    pub fn new_as_variant_mut(enum_name: &Ident, variant_name: &Ident) -> Self {
        FunctionDef {
            docs: format!(
                "Returns mutable references to the inner fields if this is a `{}::{}`, otherwise an [`{}`].",
                enum_name,
                variant_name,
                DOCS_ERROR_TYPE,
            ),
            declaration: Ident::new(
                &format!("as_{}_mut", variant_name).to_snake_case(),
                Span::call_site(),
            )
        }
    }

    pub fn new_extract_as_variant_mut(enum_name: &Ident, variant_name: &Ident) -> Self {
        let alt_fn = Self::new_as_variant_mut(enum_name, variant_name);

        FunctionDef {
            docs: format!(
                r#"
                Returns mutable references to the inner fields if this is a `{enum_name}::{variant}`, otherwise panics.
    
                # Panics
    
                Panics when the value is not a `{enum_name}::{variant}`.
                Prefer using a `match` or [`{enum_name}::{alt_fn}`]; only use this if you *know* this is a `{enum_name}::{variant}`,
                or if panicking is ok, such as during testing.
                "#,
                enum_name = enum_name,
                variant = variant_name,
                alt_fn = alt_fn.declaration,
            ),
            declaration: Ident::new(
                &format!("extract_as_{}_mut", variant_name).to_snake_case(),
                Span::call_site(),
            ),
        }
    }

    pub fn new_into_variant(enum_name: &Ident, variant_name: &Ident) -> Self {
        FunctionDef {
            docs: format!(
                "Returns the inner fields if this is a `{}::{}`, otherwise otherwise an [`{}`].",
                enum_name, variant_name, DOCS_ERROR_TYPE,
            ),
            declaration: Ident::new(
                &format!("into_{}", variant_name).to_snake_case(),
                Span::call_site(),
            ),
        }
    }

    pub fn new_extract_into_variant(enum_name: &Ident, variant_name: &Ident) -> Self {
        let alt_fn = Self::new_into_variant(enum_name, variant_name);

        FunctionDef {
            docs: format!(
                r#"
                Returns the inner fields if this is a `{enum_name}::{variant}`, otherwise panics.
                
                # Panics
    
                Panics when the value is not a `{enum_name}::{variant}`.
                Prefer using a `match` or [`{enum_name}::{alt_fn}`]; only use this if you *know* this is a `{enum_name}::{variant}`,
                or if panicking is ok, such as during testing.
                "#,
                enum_name = enum_name,
                variant = variant_name,
                alt_fn = alt_fn.declaration,
            ),
            declaration: Ident::new(
                &format!("extract_into_{}", variant_name).to_snake_case(),
                Span::call_site(),
            ),
        }
    }
}
