use crate::parse::method_inputs::MethodInputs;
use crate::Result;
use proc_macro::{Diagnostic, Level, Span};
use syn::spanned::Spanned;
use syn::{Attribute, FnDecl, Generics, Ident, MethodSig, TraitItem, TraitItemMethod};

#[derive(Debug, Clone)]
pub(crate) struct MethodDecl {
    attrs: Vec<Attribute>,
    unsafety: Option<Token![unsafe]>,
    ident: Ident,
    generics: Generics,
    span: Span,
    inputs: MethodInputs,
}

impl MethodDecl {
    pub(crate) fn parse(trait_item: TraitItem) -> Result<Self> {
        match trait_item {
            TraitItem::Method(method) => Self::parse_method(method),
            _ => {
                Diagnostic::spanned(
                    trait_item.span().unstable(),
                    Level::Error,
                    "Traits are currently only allowed to contain traits",
                )
                .emit();
                Err(())
            }
        }
    }

    fn parse_method(method: TraitItemMethod) -> Result<Self> {
        let span = method.span().unstable();
        let TraitItemMethod { attrs, sig, .. } = method;
        let MethodSig {
            constness,
            unsafety,
            asyncness,
            ident,
            decl,
            ..
        } = sig;
        let FnDecl {
            generics,
            inputs,
            output,
            ..
        } = decl;

        check_constness(constness, span)?;
        check_asyncness(asyncness, span)?;

        Ok(Self {
            attrs,
            unsafety,
            ident,
            generics,
            span,
            inputs: MethodInputs::parse(inputs)?,
        })
    }
}

fn check_constness(constness: Option<Token![const]>, span: Span) -> Result<()> {
    if constness.is_none() {
        Ok(())
    } else {
        Diagnostic::spanned(span, Level::Error, "`const` methods are not supported").emit();
        Err(())
    }
}

fn check_asyncness(asyncness: Option<Token![async]>, span: Span) -> Result<()> {
    if asyncness.is_none() {
        Ok(())
    } else {
        Diagnostic::spanned(span, Level::Error, "`async` methods are not yet supported").emit();
        Err(())
    }
}
