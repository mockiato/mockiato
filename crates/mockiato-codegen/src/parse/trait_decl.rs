use super::check_option_is_none;
use crate::diagnostic::DiagnosticBuilder;
use crate::parse::method_decl::MethodDecl;
use crate::result::{merge_results, Error, Result};
use proc_macro2::Span;
use std::collections::HashSet;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
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
        let span = item.span();
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

        let generic_types = collect_generic_type_idents(&generics);
        let methods = items
            .into_iter()
            .map(move |method| MethodDecl::parse(method, &generic_types));

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
            GenericParam::Lifetime(_) => Err(create_spanned_error(
                generic_param.span(),
                "Lifetimes are not supported on mockable traits",
            )),
            GenericParam::Const(_) => Err(create_spanned_error(
                generic_param.span(),
                "Const generics are not supported on mockable traits",
            )),
        });

    merge_results(results).map(|_| ())
}

fn collect_generic_type_idents(generics: &Generics) -> HashSet<Ident> {
    generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(param_ty) => Some(param_ty.ident.clone()),
            _ => None,
        })
        .collect()
}

fn create_spanned_error(span: Span, message: &str) -> Error {
    DiagnosticBuilder::error(span, message).build().into()
}
