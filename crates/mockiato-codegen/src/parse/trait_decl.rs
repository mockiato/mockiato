use super::check_option_is_none;
use crate::parse::method_decl::MethodDecl;
use crate::spanned::SpannedUnstable;
use crate::{merge_results, Result};
use proc_macro::Span;
use syn::punctuated::Punctuated;
use syn::{Generics, Ident, ItemTrait, Token, TypeParamBound, Visibility};

#[derive(Clone)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct TraitDecl {
    pub(crate) visibility: Visibility,
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
            vis: visibility,
            ..
        } = item;

        check_option_is_none(&auto_token, span, "Auto traits are not supported")?;

        let methods = items.into_iter().map(MethodDecl::parse);

        Ok(TraitDecl {
            visibility,
            ident,
            span,
            unsafety,
            generics: generics.clone(),
            supertraits: supertraits.clone(),
            methods: merge_results(methods)?.collect(),
        })
    }
}
