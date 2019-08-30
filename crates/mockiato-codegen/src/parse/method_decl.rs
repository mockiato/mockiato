use std::collections::HashSet;
use std::fmt::Debug;

use proc_macro2::Span;
use syn::{Attribute, Generics, Ident, ReturnType, Token, TraitItem};

use crate::parse::method_inputs::MethodInputs;
use crate::result::Result;

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
