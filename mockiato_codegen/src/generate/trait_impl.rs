use super::bound_lifetimes::rewrite_lifetimes_incrementally;
use super::constant::{arguments_matcher_ident, expect_method_ident, generic_parameter_ident};
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodArg;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::{Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr, ReturnType, Type};

pub(crate) fn generate_trait_impl(
    trait_decl: &TraitDecl,
    mock_struct_ident: &Ident,
    mod_ident: &Ident,
) -> TokenStream {
    let trait_ident = &trait_decl.ident;

    let method_impls: TokenStream = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_method_impl(method_decl, mod_ident))
        .collect();

    quote! {
        impl #trait_ident for #mock_struct_ident {
            #method_impls
        }
    }
}

fn generate_method_impl(method_decl: &MethodDecl, mod_ident: &Ident) -> TokenStream {
    let method_ident = &method_decl.ident;
    let generics = &method_decl.generics;
    let where_clause = &generics.where_clause;
    let self_arg = &method_decl.inputs.self_arg;
    let arguments: Punctuated<_, Token![,]> = method_decl.inputs.args.iter().collect();
    let output = &method_decl.output;

    quote! {
        fn #method_ident#generics(#self_arg, #arguments) #output #where_clause {
            unimplemented!()
        }
    }
}
