use super::util::lifetime_to_generic_param;
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use heck::{CamelCase, SnakeCase};
use proc_macro2::Span;
use syn::{parse_quote, GenericParam, Ident, Lifetime};

/// Generates a lifetime for the given index
pub(super) fn argument_lifetime(index: usize) -> Lifetime {
    const LIFETIME_PREFIX: &str = "'__mockiato_arg";

    Lifetime::new(&format!("{}{}", LIFETIME_PREFIX, index), Span::call_site())
}

/// Generates a generic lifetime
pub(super) fn arguments_lifetime() -> Lifetime {
    parse_quote!('__mockiato_args)
}

pub(super) fn arguments_lifetime_as_generic_param() -> GenericParam {
    lifetime_to_generic_param(arguments_lifetime())
}

/// Generates a mock lifetime
pub(super) fn mock_lifetime() -> Lifetime {
    parse_quote!('mock)
}

pub(super) fn mock_lifetime_as_generic_param() -> GenericParam {
    lifetime_to_generic_param(mock_lifetime())
}

/// Generates the mock identifier
pub(super) fn mock_struct_ident(trait_decl: &TraitDecl) -> Ident {
    const IDENTIFIER_SUFFIX: &str = "Mock";

    Ident::new(
        &format!("{}{}", trait_decl.ident, IDENTIFIER_SUFFIX),
        trait_decl.span.into(),
    )
}

/// Generates a [`struct@Ident`] for the internal sub-mod
/// for `Arguments` and `ArgumentsMatcher` impls for a mock struct.
pub(super) fn mod_ident(mock_ident: &Ident) -> Ident {
    const IDENTIFIER_PREFIX: &str = "__mockiato_";

    Ident::new(
        &format!(
            "{}{}",
            IDENTIFIER_PREFIX,
            mock_ident.to_string().to_snake_case()
        ),
        mock_ident.span(),
    )
}

/// Generates the identifier for an expect method
pub(super) fn expect_method_ident(method_decl_ident: &Ident) -> Ident {
    const IDENTIFIER_PREFIX: &str = "expect_";

    Ident::new(
        &format!("{}{}", IDENTIFIER_PREFIX, method_decl_ident.to_string()),
        method_decl_ident.span(),
    )
}

/// Generates the method identifier for a method configuring calls to be expected sequentially.
pub(super) fn expect_method_calls_in_order_ident(method_decl: &MethodDecl) -> Ident {
    const IDENTIFIER_PREFIX: &str = "expect_";
    const IDENTIFIER_SUFFIX: &str = "_calls_in_order";

    Ident::new(
        &format!(
            "{}{}{}",
            IDENTIFIER_PREFIX,
            method_decl.ident.to_string(),
            IDENTIFIER_SUFFIX
        ),
        method_decl.ident.span(),
    )
}

/// Generates the generic parameter for a given index
pub(super) fn generic_parameter_ident(index: usize) -> Ident {
    const IDENTIFIER_PREFIX: &str = "A";

    Ident::new(
        &format!("{}{}", IDENTIFIER_PREFIX, index),
        Span::call_site(),
    )
}

/// Generates the identifer for an `Arguments` struct
pub(super) fn arguments_ident(method_ident: &Ident) -> Ident {
    const IDENTIFIER_SUFFIX: &str = "Arguments";

    Ident::new(
        &format!(
            "{}{}",
            method_ident.to_string().to_camel_case(),
            IDENTIFIER_SUFFIX
        ),
        method_ident.span(),
    )
}

/// Generates the identifer for an `ArgumentsMatcher` struct
pub(super) fn arguments_matcher_ident(method_ident: &Ident) -> Ident {
    const IDENTIFIER_SUFFIX: &str = "ArgumentsMatcher";

    Ident::new(
        &format!(
            "{}{}",
            method_ident.to_string().to_camel_case(),
            IDENTIFIER_SUFFIX
        ),
        method_ident.span(),
    )
}
