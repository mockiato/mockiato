use crate::Result;
use proc_macro::{Diagnostic, Level, Span};
use syn::spanned::Spanned;
use syn::token::Const;
use syn::{Attribute, MethodSig, TraitItem, TraitItemMethod};

#[derive(Debug, Clone)]
pub(crate) struct MethodDecl {
    attrs: Vec<Attribute>,
}

impl MethodDecl {
    pub(crate) fn parse(trait_item: TraitItem) -> Result<Self> {
        match trait_item {
            TraitItem::Method(method) => Self::parse_method(method),
            _ => {
                Diagnostic::spanned(
                    trait_item.span().unstable(),
                    Level::Error,
                    format!("Traits are currently only allowed to contain traits"),
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
            abi,
            ident,
            decl,
        } = sig;

        check_constness(constness, span)?;

        Ok(Self { attrs })
    }
}

fn check_constness(constness: Option<Const>, span: Span) -> Result<()> {
    if constness.is_none() {
        Ok(())
    } else {
        Diagnostic::spanned(
            span,
            Level::Error,
            format!("`const` methods are not supported"),
        )
        .emit();
        Err(())
    }
}
