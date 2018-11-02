use crate::Result;
use proc_macro::{Diagnostic, Level};
use syn::spanned::Spanned;
use syn::{Attribute, TraitItem, TraitItemMethod};

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
        let TraitItemMethod { attrs, sig, .. } = method;

        Ok(Self { attrs })
    }
}
