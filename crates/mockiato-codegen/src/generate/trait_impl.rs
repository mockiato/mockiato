use super::GenerateMockParameters;
use super::MethodDeclMetadata;
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

    let method_impls: TokenStream = parameters
        .methods
        .iter()
        .map(|method| generate_method_impl(method, &parameters.mod_ident))
        .collect();

    let (impl_generics, ty_generics, where_clause) = parameters.generics.split_for_impl();
    let (_, trait_ty_generics, _) = trait_decl.generics.split_for_impl();

    quote! {
        #unsafety impl #impl_generics #trait_ident #trait_ty_generics for #mock_struct_ident #ty_generics #where_clause {
            #method_impls
        }
    }
}

fn generate_method_impl(
    MethodDeclMetadata {
        arguments_struct_ident,
        method_decl:
            MethodDecl {
                ident,
                unsafety,
                generics,
                inputs,
                output,
                ..
            },
        ..
    }: &MethodDeclMetadata,
    mod_ident: &Ident,
) -> TokenStream {
    let self_arg = &inputs.self_arg;
    let arguments: Punctuated<_, Token![,]> = inputs.args.iter().collect();

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let arguments_struct_fields: TokenStream = inputs
        .args
        .iter()
        .map(|argument| {
            let ident = &argument.ident;
            quote! { #ident, }
        })
        .collect();

    let output = quote! {
        #unsafety fn #ident#impl_generics(#self_arg, #arguments) #output #where_clause {
            self.#ident.call_unwrap(
                self::#mod_ident::#arguments_struct_ident {
                    #arguments_struct_fields
                    phantom_data: std::marker::PhantomData,
                }
            )
        }
    };

    output
}
