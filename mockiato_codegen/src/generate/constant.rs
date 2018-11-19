use crate::parse::method_decl::MethodDecl;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use heck::{CamelCase, SnakeCase};
use proc_macro2::Span;
use syn::{Ident, Lifetime};

/// Generates a lifetime for the given index
pub(super) fn argument_lifetime(index: usize) -> Lifetime {
    Lifetime::new(&format!("'__mockiato_arg{}", index), Span::call_site())
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
    name_attr
        .map(|attr| attr.ident)
        .unwrap_or_else(|| Ident::new(&format!("{}Mock", trait_decl.ident), trait_decl.span.into()))
}

/// Generates a [`struct@Ident`] for the internal sub-mod
/// for `Arguments` and `ArgumentsMatcher` impls for a mock struct.
pub(super) fn mod_ident(mock_ident: &Ident) -> Ident {
    Ident::new(
        &format!("__mockiato_{}", mock_ident.to_string().to_snake_case()),
        mock_ident.span(),
    )
}

/// Generates the identifier for an expect method
pub(super) fn expect_method_ident(method_decl: &MethodDecl) -> Ident {
    Ident::new(
        &format!("expect_{}", method_decl.ident.to_string()),
        Span::call_site(),
    )
}

/// Generates the generic parameter for a given index
pub(super) fn generic_argument_parameter_ident(index: usize) -> Ident {
    Ident::new(&format!("A{}", index), Span::call_site())
}

/// Generates the identifer for an arguments struct
pub(super) fn arguments_ident(method_ident: &Ident) -> Ident {
    Ident::new(
        &format!("{}Arguments", method_ident.to_string().to_camel_case()),
        method_ident.span(),
    )
}

/// Generates the identifer for an arguments matcher struct
pub(super) fn arguments_matcher_ident(method_ident: &Ident) -> Ident {
    Ident::new(
        &format!(
            "{}ArgumentsMatcher",
            method_ident.to_string().to_camel_case()
        ),
        method_ident.span(),
    )
}
