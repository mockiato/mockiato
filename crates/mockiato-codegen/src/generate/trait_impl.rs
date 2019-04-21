use super::constant::arguments_ident;
use super::GenerateMockParameters;
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

pub(crate) fn generate_trait_impl(
    trait_decl: &TraitDecl,
    parameters: &'_ GenerateMockParameters,
) -> TokenStream {
    let trait_ident = &trait_decl.ident;
    let unsafety = &trait_decl.unsafety;
    let mock_struct_ident = &parameters.mock_struct_ident;

    let method_impls: TokenStream = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_method_impl(method_decl, &parameters.mod_ident))
        .collect();

    let (impl_generics, ty_generics, where_clause) = parameters.generics.split_for_impl();

    quote! {
        #unsafety impl #impl_generics #trait_ident for #mock_struct_ident #ty_generics #where_clause {
            #method_impls
        }
    }
}

fn generate_method_impl(method_decl: &MethodDecl, mod_ident: &Ident) -> TokenStream {
    let MethodDecl {
        ident,
        unsafety,
        generics,
        inputs,
        output,
        ..
    } = method_decl;

    let self_arg = &inputs.self_arg;
    let arguments: Punctuated<_, Token![,]> = method_decl.inputs.args.iter().collect();

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let arguments_struct_ident = arguments_ident(ident);
    let arguments_struct_fields: Punctuated<_, Token![,]> = method_decl
        .inputs
        .args
        .iter()
        .map(|argument| &argument.ident)
        .collect();

    quote! {
        #unsafety fn #ident#impl_generics(#self_arg, #arguments) #output #where_clause {
            self.#ident.call_unwrap(self::#mod_ident::#arguments_struct_ident { #arguments_struct_fields })
        }
    }
}
