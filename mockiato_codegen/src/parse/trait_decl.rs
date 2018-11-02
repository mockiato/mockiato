use crate::constant::ATTR_NAME;
use crate::Result;
use proc_macro::Span;
use proc_macro::{Diagnostic, Level};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Add;
use syn::{Generics, Ident, Item, ItemTrait, TraitItem, TypeParamBound};

#[derive(Debug, Clone)]
pub(crate) struct TraitDecl {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) supertraits: Punctuated<TypeParamBound, Add>,
    pub(crate) items: Vec<TraitItem>,
}

impl TraitDecl {
    pub(crate) fn parse(item: Item) -> Result<Self> {
        if let Item::Trait(item_trait) = item {
            let span = item_trait.span().unstable();
            let ItemTrait {
                auto_token,
                unsafety,
                generics,
                supertraits,
                items,
                ident,
                ..
            } = item_trait;

            if unsafety.is_some() {
                Diagnostic::spanned(
                    span,
                    Level::Error,
                    format!("#[{}] does not work with unsafe traits", ATTR_NAME),
                )
                .emit();
                return Err(());
            }

            if auto_token.is_some() {
                Diagnostic::spanned(
                    span,
                    Level::Error,
                    format!("#[{}] does not work with auto traits", ATTR_NAME),
                )
                .emit();
                return Err(());
            }

            return Ok(TraitDecl {
                ident,
                span,
                generics: generics.clone(),
                supertraits: supertraits.clone(),
                items: items.clone(),
            });
        }

        Diagnostic::spanned(
            item.span().unstable(),
            Level::Error,
            format!("#[{}] can only be used with traits", ATTR_NAME),
        )
        .emit();

        Err(())
    }
}
