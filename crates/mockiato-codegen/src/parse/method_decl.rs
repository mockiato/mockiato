use super::check_option_is_none;
use crate::diagnostic::DiagnosticBuilder;
use crate::parse::method_inputs::{MethodInputs, MethodInputsParser};
use crate::result::{merge_results, Error, Result};
use crate::syn_ext::PathExt;
use proc_macro2::Span;
use std::collections::HashSet;
use std::fmt::Debug;
use syn::spanned::Spanned;
use syn::visit::{visit_type, Visit};
use syn::{
    Attribute, FnDecl, GenericParam, Generics, Ident, MethodSig, Path, ReturnType, Token,
    TraitItem, TraitItemMethod, Type, TypePath,
};

/// Holds everything required to generate a mock struct
/// from a trait declaration.
#[derive(Clone)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MethodDecl {
    /// A list of attributes decorating this method. (E.g. `#[inline(always)]`)
    pub(crate) attrs: Vec<Attribute>,
    /// Whether this method is unsafe or not
    pub(crate) unsafety: Option<Token![unsafe]>,
    /// The name of this method. (E.g. `greet`)
    pub(crate) ident: Ident,
    /// The generic type params (including lifetimes)
    pub(crate) generics: Generics,
    /// The [`Span`] of the entire method
    pub(crate) span: Span,
    /// The inputs (arguments) of this method
    pub(crate) inputs: MethodInputs,
    /// Return type of this method.
    pub(crate) output: ReturnType,
}

pub(crate) trait MethodDeclParser: Debug {
    fn parse(
        &self,
        trait_item: TraitItem,
        generic_types_on_trait: &HashSet<Ident>,
    ) -> Result<MethodDecl>;
}
