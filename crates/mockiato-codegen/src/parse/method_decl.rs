use super::check_option_is_none;
use crate::parse::method_inputs::MethodInputs;
use crate::spanned::SpannedUnstable;
use crate::{merge_results, Error, Result};
use proc_macro::{Diagnostic, Level, Span};
use syn::{
    Attribute, FnDecl, GenericParam, Generics, Ident, MethodSig, ReturnType, Token, TraitItem,
    TraitItemMethod,
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

impl MethodDecl {
    pub(crate) fn parse(trait_item: TraitItem) -> Result<Self> {
        match trait_item {
            TraitItem::Method(method) => Self::parse_method(method),
            _ => Err(Error::Diagnostic(Diagnostic::spanned(
                trait_item.span_unstable(),
                Level::Error,
                "Traits are only allowed to contain methods",
            ))),
        }
    }

    fn parse_method(method: TraitItemMethod) -> Result<Self> {
        let span = method.span_unstable();
        let TraitItemMethod {
            attrs,
            sig: signature,
            ..
        } = method;
        let MethodSig {
            constness,
            unsafety,
            asyncness,
            ident,
            decl,
            ..
        } = signature;
        let FnDecl {
            generics,
            inputs,
            output,
            ..
        } = decl;

        check_option_is_none(&constness, span, "`const` methods are not supported")?;
        check_option_is_none(&asyncness, span, "`async` methods are not supported")?;

        validate_generic_type_parameters(&generics)?;

        Ok(Self {
            attrs,
            unsafety,
            ident,
            generics,
            span,
            inputs: MethodInputs::parse(inputs)?,
            output,
        })
    }
}

fn validate_generic_type_parameters(generics: &Generics) -> Result<()> {
    let results = generics
        .params
        .iter()
        .map(|generic_param| match generic_param {
            GenericParam::Lifetime(_) => Ok(()),
            generic_param => Err(Error::Diagnostic(Diagnostic::spanned(
                generic_param.span_unstable(),
                Level::Error,
                "Only lifetimes are supported as generic parameters on methods",
            ))),
        });

    merge_results(results).map(|_| ())
}
