use super::check_option_is_none;
use crate::parse::method_inputs::MethodInputs;
use crate::spanned::SpannedUnstable;
use crate::syn_ext::PathExt;
use crate::{merge_results, Error, Result};
use proc_macro::{Diagnostic, Level, Span};
use std::collections::HashSet;
use syn::visit::{visit_type, Visit};
use syn::{
    Attribute, FnDecl, GenericParam, Generics, Ident, MethodSig, ReturnType, Token, TraitItem,
    TraitItemMethod, Type, TypePath,
};

/// Holds everything required to generate a mock struct
/// from a trait declaration.
#[derive(Clone)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MethodDecl {
    /// A list of attributes decorating this method. (E.g. `#[inline(always)]`)
    pub(crate) attrs: Vec<Attribute>,
    /// Whether this method is unsafe or not
    pub(crate) unsafety: Option<Token![unsafe]>,
    /// The name of this method. (E.g. `greet`)
    pub(crate) ident: Ident,
    /// The generic type params (including lifetimes)
    pub(crate) generics: Generics,
    /// The [`Span`] of the entire method
    pub(crate) span: Span,
    /// The inputs (arguments) of this method
    pub(crate) inputs: MethodInputs,
    /// Return type of this method.
    pub(crate) output: ReturnType,
}

impl MethodDecl {
    pub(crate) fn parse(
        trait_item: TraitItem,
        generic_types_on_trait: &HashSet<Ident>,
    ) -> Result<Self> {
        match trait_item {
            TraitItem::Method(method) => Self::parse_method(method, generic_types_on_trait),
            _ => Err(Error::Diagnostic(Diagnostic::spanned(
                trait_item.span_unstable(),
                Level::Error,
                "Traits are only allowed to contain methods",
            ))),
        }
    }

    fn parse_method(
        method: TraitItemMethod,
        generic_types_on_trait: &HashSet<Ident>,
    ) -> Result<Self> {
        let span = method.span_unstable();

        let TraitItemMethod {
            attrs,
            sig: signature,
            ..
        } = method;
        let MethodSig {
            constness,
            unsafety,
            asyncness,
            ident,
            decl,
            ..
        } = signature;

        validate_usage_of_generic_types(&decl, &generic_types_on_trait)?;

        let FnDecl {
            generics,
            inputs,
            output,
            ..
        } = decl;

        validate_generic_type_parameters(&generics)?;

        check_option_is_none(&constness, span, "`const` methods are not supported")?;
        check_option_is_none(&asyncness, span, "`async` methods are not supported")?;

        Ok(Self {
            attrs,
            unsafety,
            ident,
            generics,
            span,
            inputs: MethodInputs::parse(inputs)?,
            output,
        })
    }
}

fn validate_generic_type_parameters(generics: &Generics) -> Result<()> {
    let results = generics
        .params
        .iter()
        .map(|generic_param| match generic_param {
            GenericParam::Lifetime(_) => Ok(()),
            generic_param => Err(Error::Diagnostic(Diagnostic::spanned(
                generic_param.span_unstable(),
                Level::Error,
                "Only lifetimes are supported as generic parameters on methods",
            ))),
        });

    merge_results(results).map(|_| ())
}

fn validate_usage_of_generic_types(
    fn_decl: &FnDecl,
    generic_types_on_trait: &HashSet<Ident>,
) -> Result<()> {
    let references_to_generic_types =
        find_references_to_generic_types(fn_decl, generic_types_on_trait);

    if references_to_generic_types.is_empty() {
        Ok(())
    } else {
        Err(Error::merge(references_to_generic_types.into_iter().map(
            |ty| {
                Error::Diagnostic(Diagnostic::spanned(
                    ty.span_unstable(),
                    Level::Error,
                    "References to generic types are not supported",
                ))
            },
        )))
    }
}

fn find_references_to_generic_types<'a>(
    fn_decl: &'a FnDecl,
    generic_types_on_trait: &'a HashSet<Ident>,
) -> Vec<&'a Type> {
    let mut visitor = TypeVisitor {
        generic_types_on_trait,
        references_to_generic_types: Vec::new(),
        state: TypeVisitorState::Initial,
    };
    visitor.visit_fn_decl(fn_decl);
    visitor.references_to_generic_types
}

struct TypeVisitor<'a> {
    generic_types_on_trait: &'a HashSet<Ident>,
    references_to_generic_types: Vec<&'a Type>,
    state: TypeVisitorState,
}

#[derive(Copy, Clone)]
enum TypeVisitorState {
    Initial,
    CheckingReferenceInner,
    FoundReferenceToGenericType,
}

impl<'a> Visit<'a> for TypeVisitor<'a> {
    fn visit_type(&mut self, ty: &'a Type) {
        match (self.state, ty) {
            (TypeVisitorState::Initial, Type::Reference(_)) => {
                self.state = TypeVisitorState::CheckingReferenceInner;

                visit_type(self, ty);
                if let TypeVisitorState::FoundReferenceToGenericType = self.state {
                    self.references_to_generic_types.push(ty);
                }

                self.state = TypeVisitorState::Initial;
            }
            (TypeVisitorState::CheckingReferenceInner, Type::Path(TypePath { path, .. })) => {
                if self
                    .generic_types_on_trait
                    .contains(path.first_segment_as_ident().unwrap())
                {
                    self.state = TypeVisitorState::FoundReferenceToGenericType;
                } else {
                    visit_type(self, ty)
                }
            }
            _ => visit_type(self, ty),
        }
    }
}
