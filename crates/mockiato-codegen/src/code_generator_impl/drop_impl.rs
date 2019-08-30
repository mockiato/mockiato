use super::GenerateMockParameters;
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_drop_impl(
    trait_decl: &TraitDecl,
    parameters: &'_ GenerateMockParameters,
) -> TokenStream {
    let verify_calls: TokenStream = trait_decl
        .methods
        .iter()
        .map(generate_verify_call)
        .collect();

    let mock_ident = &parameters.mock_struct_ident;
    let (impl_generics, ty_generics, where_clause) = parameters.generics.split_for_impl();

    quote! {
        impl #impl_generics Drop for #mock_ident #ty_generics #where_clause {
            fn drop(&mut self) {
                if !std::thread::panicking() {
                    #verify_calls
                }
            }
        }
    }
}

fn generate_verify_call(method_decl: &MethodDecl) -> TokenStream {
    let ident = &method_decl.ident;

    quote! {
        self.#ident.verify_unwrap();
    }
}
