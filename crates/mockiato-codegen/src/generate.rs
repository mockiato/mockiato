use self::arguments::generate_arguments;
use self::arguments_matcher::generate_arguments_matcher;
use self::constant::{mock_struct_ident, mod_ident};
use self::drop_impl::generate_drop_impl;
use self::mock_struct::{generate_mock_struct, GenerateMockStructOptions};
use self::trait_impl::{generate_trait_impl, GenerateTraitImplOptions};
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub(crate) mod arguments;
pub(crate) mod arguments_matcher;
mod bound_lifetimes;
mod constant;
mod drop_impl;
mod lifetime_rewriter;
mod mock_struct;
mod trait_impl;

#[derive(Default)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct GenerateMockOptions {
    pub(crate) custom_struct_ident: Option<Ident>,
    pub(crate) force_static_lifetimes: bool,
}

pub(crate) fn generate_mock(trait_decl: &TraitDecl, options: GenerateMockOptions) -> TokenStream {
    let mock_struct_ident = options
        .custom_struct_ident
        .unwrap_or_else(|| mock_struct_ident(trait_decl));

    let mod_ident = mod_ident(&mock_struct_ident);

    let static_lifetime_restriction = if options.force_static_lifetimes {
        Some(get_static_lifetime_restriction())
    } else {
        None
    };

    let mock_struct = generate_mock_struct(
        trait_decl,
        GenerateMockStructOptions {
            mod_ident: &mod_ident,
            mock_struct_ident: &mock_struct_ident,
            static_lifetime_restriction: static_lifetime_restriction.as_ref(),
        },
    );

    let trait_impl = generate_trait_impl(
        trait_decl,
        GenerateTraitImplOptions {
            mod_ident: &mod_ident,
            mock_struct_ident: &mock_struct_ident,
        },
    );

    let arguments: TokenStream = trait_decl
        .methods
        .iter()
        .map(generate_argument_structs)
        .collect();

    let drop_impl = generate_drop_impl(&mock_struct_ident, trait_decl);

    // The sub-mod is used to hide implementation details from the user
    // and to prevent cluttering of the namespace of the trait's mod.
    quote! {
        #mock_struct

        #trait_impl

        #drop_impl

        mod #mod_ident {
            use super::*;

            #arguments
        }
    }
}

fn get_static_lifetime_restriction() -> TokenStream {
    quote! { where 'mock: 'static }
}

fn generate_argument_structs(method_decl: &MethodDecl) -> proc_macro2::TokenStream {
    let arguments = generate_arguments(method_decl);
    let arguments_matcher = generate_arguments_matcher(method_decl, &arguments);
    let arguments_output = arguments.output;

    quote! {
        #arguments_output
        #arguments_matcher
    }
}
