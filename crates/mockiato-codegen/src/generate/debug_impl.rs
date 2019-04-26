use crate::generate::util::ident_to_string_literal;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident};

pub(crate) struct DebugImplField<'a> {
    pub(crate) ident: &'a Ident,
    pub(crate) expression: TokenStream,
}

pub(crate) fn generate_debug_impl<'a>(
    fields: impl Iterator<Item = DebugImplField<'a>>,
    struct_ident: &'a Ident,
    generics: &'a Generics,
) -> TokenStream {
    let ident_str_literal = ident_to_string_literal(struct_ident);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let debug_fields: TokenStream = fields
        .map(|DebugImplField { ident, expression }| {
            let ident_as_str = ident_to_string_literal(ident);
            quote! { .field(#ident_as_str, &#expression) }
        })
        .collect();

    quote! {
        impl #impl_generics std::fmt::Debug for #struct_ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#ident_str_literal)
                 #debug_fields
                 .finish()
            }
        }
    }
}
