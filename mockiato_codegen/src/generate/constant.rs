use crate::parse::method_decl::MethodDecl;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use heck::{CamelCase, SnakeCase};
use proc_macro2::Span;
use syn::{Ident, Lifetime};

/// Generates a lifetime for the given index
pub(super) fn argument_lifetime(index: usize) -> Lifetime {
    const LIFETIME_PREFIX: &str = "'__mockiato_arg";

    Lifetime::new(&format!("{}{}", LIFETIME_PREFIX, index), Span::call_site())
}

/// Generates a generic lifetime
pub(super) fn arguments_lifetime() -> Lifetime {
    const LIFETIME_NAME: &str = "'__mockiato_args";

    Lifetime::new(LIFETIME_NAME, Span::call_site())
}

/// Generates the mock identifier
///
/// The name_attr is used if it is supplied
pub(super) fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    const IDENTIFIER_POSTFIX: &str = "Mock";

    name_attr.map(|attr| attr.ident).unwrap_or_else(|| {
        Ident::new(
            &format!("{}{}", trait_decl.ident, IDENTIFIER_POSTFIX),
            trait_decl.span.into(),
        )
    })
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
pub(super) fn expect_method_ident(method_decl: &MethodDecl) -> Ident {
    const IDENTIFIER_PREFIX: &str = "expect_";

    Ident::new(
        &format!("{}{}", IDENTIFIER_PREFIX, method_decl.ident.to_string()),
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

/// Generates the identifer for an arguments struct
pub(super) fn arguments_ident(method_ident: &Ident) -> Ident {
    const IDENTIFIER_POSTFIX: &str = "Arguments";

    Ident::new(
        &format!(
            "{}{}",
            method_ident.to_string().to_camel_case(),
            IDENTIFIER_POSTFIX
        ),
        method_ident.span(),
    )
}

/// Generates the identifer for an arguments matcher struct
pub(super) fn arguments_matcher_ident(method_ident: &Ident) -> Ident {
    const IDENTIFIER_POSTFIX: &str = "ArgumentsMatcher";

    Ident::new(
        &format!(
            "{}{}",
            method_ident.to_string().to_camel_case(),
            IDENTIFIER_POSTFIX
        ),
        method_ident.span(),
    )
}
