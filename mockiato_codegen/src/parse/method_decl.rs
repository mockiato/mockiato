use crate::parse::method_inputs::MethodInputs;
use crate::spanned::SpannedUnstable;
use crate::{Error, Result};
use proc_macro::{Diagnostic, Level, Span};
use syn::{Attribute, FnDecl, Generics, Ident, MethodSig, ReturnType, TraitItem, TraitItemMethod};

/// Holds everything required to generate a mock struct
/// from a trait declaration.
#[derive(Debug, Clone)]
pub(crate) struct MethodDecl {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) unsafety: Option<Token![unsafe]>,
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) span: Span,
    pub(crate) inputs: MethodInputs,
    pub(crate) output: ReturnType,
}

impl MethodDecl {
    pub(crate) fn parse(trait_item: TraitItem) -> Result<Self> {
        match trait_item {
            TraitItem::Method(method) => Self::parse_method(method),
            _ => Err(Error::Diagnostic(Diagnostic::spanned(
                trait_item.span_unstable(),
                Level::Error,
                "Traits are currently only allowed to contain traits",
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

fn check_option_is_none<T>(value: &Option<T>, span: Span, error_message: &str) -> Result<()> {
    match value {
        None => Ok(()),
        Some(_) => Err(Error::Diagnostic(Diagnostic::spanned(
            span,
            Level::Error,
            error_message,
        ))),
    }
}
