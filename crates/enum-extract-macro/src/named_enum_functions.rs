use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{FieldsNamed, Type};

use crate::function_def::FunctionDef;

pub fn all_named_functions(
    enum_name: &Ident,
    variant_name: &Ident,
    err_type: &Type,
    err_value_type: &Type,
    err_value_type_with_generics: &Type,
    fields: &FieldsNamed,
) -> TokenStream {
    let context = NamedEnumFunctionContext::new(
        enum_name,
        variant_name,
        err_type,
        err_value_type,
        err_value_type_with_generics,
        fields,
    );

    let mut tokens = TokenStream::new();
    tokens.append_all(named_enum_is_variant(&context));
    tokens.append_all(named_enum_as_variant(&context));
    tokens.append_all(named_enum_as_variant_mut(&context));
    tokens.append_all(named_enum_into_variant(&context));
    tokens.append_all(named_enum_extract_as_variant(&context));
    tokens.append_all(named_enum_extract_as_variant_mut(&context));
    tokens.append_all(named_enum_extract_into_variant(&context));

    tokens
}
pub struct NamedEnumFunctionContext<'a> {
    pub enum_name: &'a syn::Ident,
    pub variant_name: &'a syn::Ident,
    pub matches: TokenStream,
    pub returns_ref: TokenStream,
    pub returns_mut_ref: TokenStream,
    pub returns_val: TokenStream,
    pub err_type: &'a syn::Type,
    pub err_value_type: &'a syn::Type,
    pub err_value_type_with_generics: &'a syn::Type,
}

impl<'a> NamedEnumFunctionContext<'a> {
    pub fn new(
        enum_name: &'a Ident,
        variant_name: &'a Ident,
        err_type: &'a syn::Type,
        err_value_type: &'a syn::Type,
        err_value_type_with_generics: &'a syn::Type,
        fields: &'a syn::FieldsNamed,
    ) -> Self {
        let (returns_mut_ref, returns_ref, returns_val, matches) = match fields.named.len() {
            1 => {
                let field = fields.named.first().expect("no fields on type");
                let match_name = field.ident.as_ref().expect("expected a named field");

                let returns = &field.ty;
                let returns_mut_ref = quote!(&mut #returns);
                let returns_ref = quote!(&#returns);
                let returns_val = quote!(#returns);
                let matches = quote!(#match_name);

                (returns_mut_ref, returns_ref, returns_val, matches)
            }
            0 => (quote!(()), quote!(()), quote!(()), quote!(())),
            _ => {
                let mut returns_mut_ref = TokenStream::new();
                let mut returns_ref = TokenStream::new();
                let mut returns_val = TokenStream::new();
                let mut matches = TokenStream::new();

                for field in fields.named.iter() {
                    let rt = &field.ty;
                    let match_name = field.ident.as_ref().expect("expected a named field");

                    returns_mut_ref.extend(quote!(&mut #rt,));
                    returns_ref.extend(quote!(&#rt,));
                    returns_val.extend(quote!(#rt,));
                    matches.extend(quote!(#match_name,));
                }

                (
                    quote!((#returns_mut_ref)),
                    quote!((#returns_ref)),
                    quote!((#returns_val)),
                    quote!(#matches),
                )
            }
        };

        Self {
            enum_name,
            variant_name,
            matches,
            returns_ref,
            returns_mut_ref,
            returns_val,
            err_type,
            err_value_type,
            err_value_type_with_generics,
        }
    }
}

pub fn named_enum_is_variant(context: &NamedEnumFunctionContext) -> TokenStream {
    let function_def = FunctionDef::new_is_variant(context.enum_name, context.variant_name);
    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let variant_name = context.variant_name;
    let matches = &context.matches;

    quote!(
        #[doc = #docs]
        #[inline]
        pub fn #function_name(&self) -> bool {
            matches!(self, Self::#variant_name{ #matches })
        }
    )
}

pub fn named_enum_as_variant(context: &NamedEnumFunctionContext) -> TokenStream {
    let function_def = FunctionDef::new_as_variant(context.enum_name, context.variant_name);
    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let variant_name = context.variant_name;
    let matches = &context.matches;
    let returns_ref = &context.returns_ref;
    let err_type = context.err_type;

    quote!(
        #[doc = #docs ]
        #[inline]
        pub fn #function_name(&self) -> ::core::result::Result<#returns_ref, #err_type> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                    ))
                }
            }
        }
    )
}

pub fn named_enum_as_variant_mut(context: &NamedEnumFunctionContext) -> TokenStream {
    let function_def = FunctionDef::new_as_variant_mut(context.enum_name, context.variant_name);
    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let variant_name = context.variant_name;
    let matches = &context.matches;
    let returns_mut_ref = &context.returns_mut_ref;
    let err_type = context.err_type;

    quote!(
        #[doc = #docs ]
        #[inline]
        pub fn #function_name(&mut self) -> ::core::result::Result<#returns_mut_ref, #err_type> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                    ))
                }
            }
        }
    )
}

pub fn named_enum_into_variant(context: &NamedEnumFunctionContext) -> TokenStream {
    let function_def = FunctionDef::new_into_variant(context.enum_name, context.variant_name);
    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let variant_name = context.variant_name;
    let matches = &context.matches;
    let returns_val = &context.returns_val;
    let err_type_with_generics = context.err_value_type_with_generics;
    let err_type = context.err_value_type;

    quote!(
        #[doc = #docs ]
        #[inline]
        pub fn #function_name(self) -> ::core::result::Result<#returns_val, #err_type_with_generics> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        self,
                    ))
                }
            }
        }
    )
}

pub fn named_enum_extract_as_variant(context: &NamedEnumFunctionContext) -> TokenStream {
    let function_def = FunctionDef::new_extract_as_variant(context.enum_name, context.variant_name);
    let alt_function = FunctionDef::new_as_variant(context.enum_name, context.variant_name);

    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let alt_function_name = alt_function.declaration;
    let returns_ref = &context.returns_ref;

    quote!(
        #[doc = #docs ]
        #[inline]
        pub fn #function_name(&self) -> #returns_ref {
            self.#alt_function_name().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }
    )
}

pub fn named_enum_extract_as_variant_mut(context: &NamedEnumFunctionContext) -> TokenStream {
    let function_def =
        FunctionDef::new_extract_as_variant_mut(context.enum_name, context.variant_name);
    let alt_function = FunctionDef::new_as_variant_mut(context.enum_name, context.variant_name);

    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let alt_function_name = alt_function.declaration;
    let returns_mut_ref = &context.returns_mut_ref;

    quote!(
        #[doc = #docs ]
        #[inline]
        pub fn #function_name(&mut self) -> #returns_mut_ref {
            self.#alt_function_name().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }
    )
}

pub fn named_enum_extract_into_variant(context: &NamedEnumFunctionContext) -> TokenStream {
    let function_def =
        FunctionDef::new_extract_into_variant(context.enum_name, context.variant_name);
    let alt_function = FunctionDef::new_into_variant(context.enum_name, context.variant_name);

    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let alt_function_name = alt_function.declaration;
    let returns_val = &context.returns_val;

    quote!(
        #[doc = #docs ]
        #[inline]
        pub fn #function_name(self) -> #returns_val {
            self.#alt_function_name().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }
    )
}
