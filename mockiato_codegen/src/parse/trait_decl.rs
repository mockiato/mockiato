use crate::constant::ATTR_NAME;
use crate::parse::method_decl::MethodDecl;
use crate::{merge_results, Error, Result};
use proc_macro::Span;
use proc_macro::{Diagnostic, Level};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Generics, Ident, Item, ItemTrait, TypeParamBound};

#[derive(Debug, Clone)]
pub(crate) struct TraitDecl {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) unsafety: Option<Token![unsafe]>,
    pub(crate) supertraits: Punctuated<TypeParamBound, Token![+]>,
    pub(crate) methods: Vec<MethodDecl>,
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

            if auto_token.is_some() {
                return Err(Error::Diagnostic(Diagnostic::spanned(
                    span,
                    Level::Error,
                    format!("#[{}] does not work with auto traits", ATTR_NAME),
                )));
            }

            let methods = items.into_iter().map(MethodDecl::parse);

            return Ok(TraitDecl {
                ident,
                span,
                unsafety,
                generics: generics.clone(),
                supertraits: supertraits.clone(),
                methods: merge_results!(methods),
            });
        }

        Err(Error::Diagnostic(Diagnostic::spanned(
            item.span().unstable(),
            Level::Error,
            format!("#[{}] can only be used with traits", ATTR_NAME),
        )))
    }
}
