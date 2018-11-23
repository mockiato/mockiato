use self::arguments::generate_arguments;
use self::arguments_matcher::generate_arguments_matcher;
use self::constant::{mock_struct_ident, mod_ident};
use self::mock_struct::generate_mock_struct;
use crate::parse::method_decl::MethodDecl;
use crate::parse::mockable_attr::MockableAttr;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use syn::ItemTrait;

pub(crate) mod arguments;
pub(crate) mod arguments_matcher;
mod bound_lifetimes;
mod constant;
mod lifetime_rewriter;
mod mock_struct;

pub(crate) fn generate_mock(
    mockable_attr: MockableAttr,
    item_trait: &ItemTrait,
    trait_decl: &TraitDecl,
) -> TokenStream {
    let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);
    let mod_ident = mod_ident(&mock_struct_ident);

    let mock_struct = generate_mock_struct(&trait_decl, &mock_struct_ident, &mod_ident);

    let arguments: TokenStream = trait_decl
        .methods
        .iter()
        .map(generate_argument_structs)
        .collect();

    // The sub-mod is used to hide implementation details from the user
    // and to prevent cluttering of the namespace of the trait's mod.
    quote! {
        #item_trait

        #mock_struct

        mod #mod_ident {
            use super::*;

            #arguments
        }
    }
}

fn generate_argument_structs(method_decl: &MethodDecl) -> proc_macro2::TokenStream {
    let arguments = generate_arguments(method_decl);
    let arguments_matcher = generate_arguments_matcher(&method_decl, &arguments);
    let arguments_output = arguments.output;

    quote! {
        #arguments_output
        #arguments_matcher
    }
}
