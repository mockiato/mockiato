use crate::diagnostic::DiagnosticBuilder;
use crate::parse::check_option_is_none;
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::{TraitDecl, TraitDeclParser};
use crate::result::{merge_results, Error, Result};
use proc_macro2::Ident;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{GenericParam, Generics, ItemTrait};

#[derive(Debug)]
pub(crate) struct TraitDeclParserImpl;

impl TraitDeclParserImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl TraitDeclParser for TraitDeclParserImpl {
    fn parse(&self, item: ItemTrait) -> Result<TraitDecl> {
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
            GenericParam::Lifetime(_) => Err(invalid_generic_param_error(
                generic_param,
                "Lifetimes are not supported on mockable traits",
            )),
            GenericParam::Const(_) => Err(invalid_generic_param_error(
                generic_param,
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

fn invalid_generic_param_error(generic_param: &GenericParam, message: &str) -> Error {
    DiagnosticBuilder::error(generic_param.span(), message)
        .build()
        .into()
}
