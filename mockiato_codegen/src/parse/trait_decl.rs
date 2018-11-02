use crate::constant::ATTR_NAME;
use crate::parse::method_decl::MethodDecl;
use crate::Result;
use proc_macro::Span;
use proc_macro::{Diagnostic, Level};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Add, Unsafe};
use syn::{Generics, Ident, Item, ItemTrait, TypeParamBound};

#[derive(Debug, Clone)]
pub(crate) struct TraitDecl {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) unsafety: Option<Unsafe>,
    pub(crate) supertraits: Punctuated<TypeParamBound, Add>,
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
                Diagnostic::spanned(
                    span,
                    Level::Error,
                    format!("#[{}] does not work with auto traits", ATTR_NAME),
                )
                .emit();
                return Err(());
            }

            let methods: Vec<_> = items.into_iter().map(MethodDecl::parse).collect();

            if methods.iter().any(Result::is_err) {
                return Err(());
            }

            return Ok(TraitDecl {
                ident,
                span,
                unsafety,
                generics: generics.clone(),
                supertraits: supertraits.clone(),
                methods: methods.into_iter().map(Result::unwrap).collect(),
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
