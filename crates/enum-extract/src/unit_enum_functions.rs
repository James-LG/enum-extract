use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::function_def::FunctionDef;

pub fn all_unit_functions(enum_name: &Ident, variant_name: &Ident) -> TokenStream {
    unit_enum_is_variant(enum_name, variant_name)
}

fn unit_enum_is_variant(enum_name: &syn::Ident, variant_name: &syn::Ident) -> TokenStream {
    let function_def = FunctionDef::new_is_variant(enum_name, variant_name);

    let function_name = function_def.declaration;
    let docs = function_def.docs;
    let variant_name = variant_name;

    quote!(
        #[doc = #docs]
        #[inline]
        pub fn #function_name(&self) -> bool {
            matches!(self, Self::#variant_name)
        }
    )
}
