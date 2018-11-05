use crate::constant::ATTR_NAME;
use crate::parse::method_decl::MethodDecl;
use crate::spanned::SpannedUnstable;
use crate::{merge_results, Error, Result};
use proc_macro::Span;
use proc_macro::{Diagnostic, Level};
use syn::punctuated::Punctuated;
use syn::{Generics, Ident, ItemTrait, TypeParamBound};

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
    pub(crate) fn parse(item: ItemTrait) -> Result<Self> {
        let span = item.span_unstable();
        let ItemTrait {
            auto_token,
            unsafety,
            generics,
            supertraits,
            items,
            ident,
            ..
        } = item;

        if auto_token.is_some() {
            return Err(Error::Diagnostic(Diagnostic::spanned(
                span,
                Level::Error,
                format!("#[{}] does not work with auto traits", ATTR_NAME),
            )));
        }

        let methods = items.into_iter().map(MethodDecl::parse);

        Ok(TraitDecl {
            ident,
            span,
            unsafety,
            generics: generics.clone(),
            supertraits: supertraits.clone(),
            methods: merge_results!(methods),
        })
    }
}
