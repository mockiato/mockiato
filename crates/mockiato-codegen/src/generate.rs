use self::arguments::generate_arguments;
use self::arguments_matcher::generate_arguments_matcher;
use self::constant::{arguments_ident, arguments_matcher_ident};
use self::constant::{mock_lifetime, mock_lifetime_as_generic_param, mock_struct_ident, mod_ident};
use self::drop_impl::generate_drop_impl;
use self::generics::get_matching_generics_for_method_inputs;
use self::mock_struct::generate_mock_struct;
use self::trait_impl::generate_trait_impl;
use self::visibility::raise_visibility_by_one_level;
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Generics, Ident, Path, ReturnType, Type, WherePredicate};

pub(crate) mod arguments;
pub(crate) mod arguments_matcher;
mod bound_lifetimes;
mod constant;
mod debug_impl;
mod drop_impl;
mod generics;
mod lifetime_rewriter;
mod mock_struct;
mod trait_impl;
mod util;
mod visibility;

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct GenerateMockOptions {
    pub(crate) custom_struct_ident: Option<Ident>,
    pub(crate) force_static_lifetimes: bool,
    pub(crate) custom_trait_path: Option<Path>,
}

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct GenerateMockParameters {
    pub(crate) mock_struct_ident: Ident,
    pub(crate) mod_ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) trait_path: Path,
    pub(crate) methods: Vec<MethodDeclMetadata>,
}

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MethodDeclMetadata {
    pub(crate) method_decl: MethodDecl,
    pub(crate) arguments_struct_ident: Ident,
    pub(crate) arguments_matcher_struct_ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) return_type: Type,
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

    let trait_path = options
        .custom_trait_path
        .unwrap_or_else(|| ident_to_path(&trait_decl.ident));

    let methods = trait_decl
        .methods
        .iter()
        .cloned()
        .map(|method_decl| map_method_decl_to_method_decl_metadata(method_decl, trait_decl))
        .collect();

    let parameters = GenerateMockParameters {
        mock_struct_ident: mock_struct_ident.clone(),
        mod_ident: mod_ident(&mock_struct_ident),
        generics: generics_for_trait_decl(trait_decl, static_lifetime_restriction),
        methods,
        trait_path,
    };

    let mock_struct = generate_mock_struct(trait_decl, &parameters);

    let trait_impl = generate_trait_impl(trait_decl, &parameters);

    let arguments: TokenStream = parameters
        .methods
        .iter()
        .map(|method| generate_argument_structs(method, trait_decl))
        .collect();

    let drop_impl = generate_drop_impl(trait_decl, &parameters);
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

fn ident_to_path(ident: &Ident) -> Path {
    parse_quote!(#ident)
}

fn map_method_decl_to_method_decl_metadata(
    method_decl: MethodDecl,
    trait_decl: &TraitDecl,
) -> MethodDeclMetadata {
    let generics =
        get_matching_generics_for_method_inputs(&method_decl.inputs, &trait_decl.generics);
    let arguments_struct_ident = arguments_ident(&method_decl.ident);
    let arguments_matcher_struct_ident = arguments_matcher_ident(&method_decl.ident);
    let return_type = return_type(&method_decl);

    MethodDeclMetadata {
        method_decl,
        generics,
        arguments_struct_ident,
        arguments_matcher_struct_ident,
        return_type,
    }
}

fn generics_for_trait_decl(
    trait_decl: &TraitDecl,
    static_lifetime_restriction: Option<WherePredicate>,
) -> Generics {
    let mut generics = trait_decl.generics.clone();
    generics.params.push(mock_lifetime_as_generic_param());

    if let Some(static_lifetime_restriction) = static_lifetime_restriction {
        generics
            .make_where_clause()
            .predicates
            .push(static_lifetime_restriction);
    }

    generics
}

fn get_static_lifetime_restriction() -> WherePredicate {
    let mock_lifetime = mock_lifetime();
    parse_quote!(#mock_lifetime: 'static)
}

fn generate_argument_structs(
    method: &MethodDeclMetadata,
    trait_decl: &TraitDecl,
) -> proc_macro2::TokenStream {
    let visibility = raise_visibility_by_one_level(&trait_decl.visibility);
    let arguments = generate_arguments(method, &visibility);
    let arguments_matcher = generate_arguments_matcher(method, &visibility);

    quote! {
        #arguments
        #arguments_matcher
    }
}

fn return_type(method_decl: &MethodDecl) -> Type {
    match &method_decl.output {
        ReturnType::Default => parse_quote! { () },
        ReturnType::Type(_, ty) => ty.as_ref().clone(),
    }
}
