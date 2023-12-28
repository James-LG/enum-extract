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

/// returns first the types to return, the match names, and then tokens to the field accesses

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
