use super::check_option_is_none;
use crate::parse::method_decl::MethodDecl;
use crate::spanned::SpannedUnstable;
use crate::{merge_results, Error, Result};
use proc_macro::{Diagnostic, Level, Span};
use syn::punctuated::Punctuated;
use syn::{GenericParam, Generics, Ident, ItemTrait, Token, TypeParamBound, Visibility};

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
        validate_generic_type_parameters(&generics)?;

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

fn validate_generic_type_parameters(generics: &Generics) -> Result<()> {
    let results = generics
        .params
        .iter()
        .map(|generic_param| match generic_param {
            GenericParam::Type(_) => Ok(()),
            GenericParam::Lifetime(_) => Err(create_spanned_error(generic_param.span_unstable(), "Lifetimes are not supported on mockable traits")),
            GenericParam::Const(_) => Err(create_spanned_error(generic_param.span_unstable(), "Const generics are not supported on mockable traits")),
        });

    merge_results(results).map(|_| ())
}

fn create_spanned_error(span: Span, message: &str) -> Error {
    Error::Diagnostic(Diagnostic::spanned(span, Level::Error, message))
}
