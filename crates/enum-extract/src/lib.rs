use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput};

/// returns first the types to return, the match names, and then tokens to the field accesses
fn unit_fields_return(
    variant_name: &syn::Ident,
    (fn_is_variant, doc_is_variant): (&Ident, &str),
) -> TokenStream {
    quote!(
        #[doc = #doc_is_variant]
        #[inline]
        pub fn #fn_is_variant(&self) -> bool {
            matches!(self, Self::#variant_name)
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
#[allow(clippy::too_many_arguments)]
fn unnamed_fields_return(
    variant_name: &syn::Ident,
    err_type: &syn::Type,
    err_type_with_generics: &syn::Type,
    (fn_is_variant, doc_is_variant): (&Ident, &str),
    (fn_as_variant_mut, doc_as_variant_mut): (&Ident, &str),
    (fn_as_variant, doc_as_variant): (&Ident, &str),
    (fn_into_variant, doc_into_variant): (&Ident, &str),
    (fn_extract_as_mut_variant, doc_extract_as_mut_variant): (&Ident, &str),
    (fn_extract_as_variant, doc_extract_as_variant): (&Ident, &str),
    (fn_extract_into_variant, doc_extract_into_variant): (&Ident, &str),
    fields: &syn::FieldsUnnamed,
) -> TokenStream {
    let (returns_mut_ref, returns_ref, returns_val, matches) = match fields.unnamed.len() {
        1 => {
            let field = fields.unnamed.first().expect("no fields on type");

            let returns = &field.ty;
            let returns_mut_ref = quote!(&mut #returns);
            let returns_ref = quote!(&#returns);
            let returns_val = quote!(#returns);
            let matches = quote!(inner);

            (returns_mut_ref, returns_ref, returns_val, matches)
        }
        0 => (quote!(()), quote!(()), quote!(()), quote!()),
        _ => {
            let mut returns_mut_ref = TokenStream::new();
            let mut returns_ref = TokenStream::new();
            let mut returns_val = TokenStream::new();
            let mut matches = TokenStream::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let rt = &field.ty;
                let match_name = Ident::new(&format!("match_{}", i), Span::call_site());
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

    quote!(
        #[doc = #doc_is_variant ]
        #[inline]
        #[allow(unused_variables)]
        pub fn #fn_is_variant(&self) -> bool {
            matches!(self, Self::#variant_name(#matches))
        }

        #[doc = #doc_as_variant_mut ]
        #[inline]
        pub fn #fn_as_variant_mut(&mut self) -> ::core::result::Result<#returns_mut_ref, #err_type_with_generics> {
            match self {
                Self::#variant_name(#matches) => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_as_variant ]
        #[inline]
        pub fn #fn_as_variant(&self) -> ::core::result::Result<#returns_ref, #err_type_with_generics> {
            match self {
                Self::#variant_name(#matches) => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_into_variant ]
        #[inline]
        pub fn #fn_into_variant(self) -> ::core::result::Result<#returns_val, #err_type_with_generics> {
            match self {
                Self::#variant_name(#matches) => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::Some(self),
                    ))
                }
            }
        }

        #[doc = #doc_extract_as_mut_variant ]
        #[inline]
        pub fn #fn_extract_as_mut_variant(&mut self) -> #returns_mut_ref {
            self.#fn_as_variant_mut().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }

        #[doc = #doc_extract_as_variant ]
        #[inline]
        pub fn #fn_extract_as_variant(&self) -> #returns_ref {
            self.#fn_as_variant().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }

        #[doc = #doc_extract_into_variant ]
        #[inline]
        pub fn #fn_extract_into_variant(self) -> #returns_val {
            self.#fn_into_variant().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }
    )
}

/// returns first the types to return, the match names, and then tokens to the field accesses
#[allow(clippy::too_many_arguments)]
fn named_fields_return(
    variant_name: &syn::Ident,
    err_type: &syn::Type,
    err_type_with_generics: &syn::Type,
    (fn_is_variant, doc_is_variant): (&Ident, &str),
    (fn_as_variant_mut, doc_as_variant_mut): (&Ident, &str),
    (fn_as_variant, doc_as_variant): (&Ident, &str),
    (fn_into_variant, doc_into_variant): (&Ident, &str),
    (fn_extract_as_variant_mut, doc_extract_as_variant_mut): (&Ident, &str),
    (fn_extract_as_variant, doc_extract_as_variant): (&Ident, &str),
    (fn_extract_into_variant, doc_extract_into_variant): (&Ident, &str),
    fields: &syn::FieldsNamed,
) -> TokenStream {
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

    quote!(
        #[doc = #doc_is_variant ]
        #[inline]
        #[allow(unused_variables)]
        pub fn #fn_is_variant(&self) -> bool {
            matches!(self, Self::#variant_name{ #matches })
        }

        #[doc = #doc_as_variant_mut ]
        #[inline]
        pub fn #fn_as_variant_mut(&mut self) -> ::core::result::Result<#returns_mut_ref, #err_type_with_generics> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_as_variant ]
        #[inline]
        pub fn #fn_as_variant(&self) -> ::core::result::Result<#returns_ref, #err_type_with_generics> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::None,
                    ))
                }
            }
        }

        #[doc = #doc_into_variant ]
        #[inline]
        pub fn #fn_into_variant(self) -> ::core::result::Result<#returns_val, #err_type_with_generics> {
            match self {
                Self::#variant_name{ #matches } => {
                    ::core::result::Result::Ok((#matches))
                }
                _ => {
                    ::core::result::Result::Err(#err_type::new(
                        stringify!(#variant_name),
                        self.variant_name(),
                        ::core::option::Option::Some(self),
                    ))
                }
            }
        }

        #[doc = #doc_extract_as_variant_mut ]
        #[inline]
        pub fn #fn_extract_as_variant_mut(&mut self) -> #returns_mut_ref {
            self.#fn_as_variant_mut().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }

        #[doc = #doc_extract_as_variant ]
        #[inline]
        pub fn #fn_extract_as_variant(&self) -> #returns_ref {
            self.#fn_as_variant().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }

        #[doc = #doc_extract_into_variant ]
        #[inline]
        pub fn #fn_extract_into_variant(self) -> #returns_val {
            self.#fn_into_variant().unwrap_or_else(|err| panic!("{}", err.to_string()))
        }
    )
}

fn impl_all_as_fns(name: &Ident, generics: &syn::Generics, data: &DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let err_name = syn::Ident::new("EnumExtractError", Span::call_site());
    let err_path = syn::Path::from(syn::PathSegment::from(syn::Ident::new(
        "enum_extract_error",
        Span::call_site(),
    )));
    let err_type = {
        let last_segment = syn::PathSegment::from(err_name.clone());
        let mut path = err_path.clone();
        path.segments.push(last_segment);
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: path,
        })
    };
    let err_type_with_generics = {
        let mut last_segment = syn::PathSegment::from(err_name.clone());
        let mut path = err_path.clone();

        let mut inner_type_path = syn::Path::from(syn::PathSegment::from(name.clone()));
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

    let mut stream = TokenStream::new();
    let mut variant_names = TokenStream::new();
    for variant_data in &data.variants {
        let variant_name = &variant_data.ident;

        let fn_is_variant = Ident::new(
            &format!("is_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_is_variant = format!(
            "Returns true if this is a `{}::{}`, otherwise false",
            name, variant_name,
        );

        let fn_as_variant = Ident::new(
            &format!("as_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_as_variant = format!(
            "Returns references to the inner fields if this is a `{}::{}`, otherwise an `{}`",
            name, variant_name, "EnumExtractError",
        );

        let fn_extract_as_variant = Ident::new(
            &format!("extract_as_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_extract_as_variant = format!(
            "Returns references to the inner fields if this is a `{}::{}`, otherwise panics.",
            name, variant_name,
        );

        let fn_as_variant_mut = Ident::new(
            &format!("as_{}_mut", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_as_variant_mut = format!(
            "Returns mutable references to the inner fields if this is a `{}::{}`, otherwise an `{}`",
            name,
            variant_name,
            "EnumExtractError",
        );

        let fn_extract_as_variant_mut = Ident::new(
            &format!("extract_as_{}_mut", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_extract_as_variant_mut = format!(
            "Returns mutable references to the inner fields if this is a `{}::{}`, otherwise panics.",
            name,
            variant_name,
        );

        let fn_into_variant = Ident::new(
            &format!("into_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_into_variant = format!(
            "Returns the inner fields if this is a `{}::{}`, otherwise otherwise an `{}`",
            name, variant_name, "EnumExtractError",
        );

        let fn_extract_into_variant = Ident::new(
            &format!("extract_into_{}", variant_name).to_snake_case(),
            Span::call_site(),
        );
        let doc_extract_into_variant = format!(
            "Returns the inner fields if this is a `{}::{}`, otherwise panics",
            name, variant_name,
        );

        let tokens = match &variant_data.fields {
            syn::Fields::Unit => {
                unit_fields_return(variant_name, (&fn_is_variant, &doc_is_variant))
            }
            syn::Fields::Unnamed(unnamed) => unnamed_fields_return(
                variant_name,
                &err_type,
                &err_type_with_generics,
                (&fn_is_variant, &doc_is_variant),
                (&fn_as_variant_mut, &doc_as_variant_mut),
                (&fn_as_variant, &doc_as_variant),
                (&fn_into_variant, &doc_into_variant),
                (&fn_extract_as_variant_mut, &doc_extract_as_variant_mut),
                (&fn_extract_as_variant, &doc_extract_as_variant),
                (&fn_extract_into_variant, &doc_extract_into_variant),
                unnamed,
            ),
            syn::Fields::Named(named) => named_fields_return(
                variant_name,
                &err_type,
                &err_type_with_generics,
                (&fn_is_variant, &doc_is_variant),
                (&fn_as_variant_mut, &doc_as_variant_mut),
                (&fn_as_variant, &doc_as_variant),
                (&fn_into_variant, &doc_into_variant),
                (&fn_extract_as_variant_mut, &doc_extract_as_variant_mut),
                (&fn_extract_as_variant, &doc_extract_as_variant),
                (&fn_extract_into_variant, &doc_extract_into_variant),
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
        impl #impl_generics #name #ty_generics #where_clause {
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
