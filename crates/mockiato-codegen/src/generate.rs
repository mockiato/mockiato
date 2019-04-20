use self::arguments::generate_arguments;
use self::arguments_matcher::generate_arguments_matcher;
use self::constant::{mock_struct_ident, mod_ident};
use self::drop_impl::generate_drop_impl;
use self::mock_struct::{generate_mock_struct};
use self::trait_impl::{generate_trait_impl};
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_quote, Generics, WherePredicate};

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

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct GenerateMockParameters {
    pub(crate) mock_struct_ident: Ident,
    pub(crate) mod_ident: Ident,
    pub(crate) generics: Generics,
}

pub(crate) fn generate_mock(trait_decl: &TraitDecl, options: GenerateMockOptions) -> TokenStream {
    let mock_struct_ident = options
        .custom_struct_ident
        .unwrap_or_else(|| mock_struct_ident(trait_decl));

    let static_lifetime_restriction = if options.force_static_lifetimes {
        Some(get_static_lifetime_restriction())
    } else {
        None
    };

    let parameters = GenerateMockParameters {
        mock_struct_ident: mock_struct_ident.clone(),
        mod_ident: mod_ident(&mock_struct_ident),
        generics: generics_for_trait_decl(trait_decl, static_lifetime_restriction),
    };

    let mock_struct = generate_mock_struct(
        &trait_decl,
        &parameters,
    );

    let trait_impl = generate_trait_impl(
        &trait_decl,
        &parameters,
    );

    let arguments: TokenStream = trait_decl
        .methods
        .iter()
        .map(generate_argument_structs)
        .collect();

    let drop_impl = generate_drop_impl(&trait_decl, &parameters);
    let mod_ident = &parameters.mod_ident;

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

fn generics_for_trait_decl(
    trait_decl: &TraitDecl,
    static_lifetime_restriction: Option<WherePredicate>,
) -> Generics {
    let mut generics = trait_decl.generics.clone();
    generics.params.push(parse_quote!('mock));

    {
        let where_clause = generics.make_where_clause();

        if let Some(static_lifetime_restriction) = static_lifetime_restriction {
            where_clause
                .predicates
                .push(static_lifetime_restriction.clone());
        }
    }

    generics
}

fn get_static_lifetime_restriction() -> WherePredicate {
    parse_quote!('mock: 'static)
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
