use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use syn::Ident;

pub(crate) fn generate_drop_impl(mock_ident: &Ident, trait_decl: &TraitDecl) -> TokenStream {
    let verify_calls: TokenStream = trait_decl
        .methods
        .iter()
        .map(generate_verify_call)
        .collect();

    quote! {
        impl<'mock> Drop for #mock_ident<'mock> {
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
