use super::check_option_is_none;
use crate::diagnostic::DiagnosticBuilder;
use crate::parse::method_inputs::MethodInputs;
use crate::result::{merge_results, Error, Result};
use crate::syn_ext::PathExt;
use proc_macro2::Span;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::{visit_type, Visit};
use syn::{
    Attribute, FnDecl, GenericParam, Generics, Ident, MethodSig, Path, ReturnType, Token,
    TraitItem, TraitItemMethod, Type, TypePath,
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
            _ => Err(DiagnosticBuilder::error(
                trait_item.span(),
                "Traits are only allowed to contain methods",
            )
            .build()
            .into()),
        }
    }

    fn parse_method(
        method: TraitItemMethod,
        generic_types_on_trait: &HashSet<Ident>,
    ) -> Result<Self> {
        let span = method.span();

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

        validate_usage_of_generic_types(&decl, generic_types_on_trait)?;

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
            generic_param => Err(DiagnosticBuilder::error(
                generic_param.span(),
                "Only lifetimes are supported as generic parameters on methods",
            )
            .build()
            .into()),
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
        Err(references_to_generic_types
            .into_iter()
            .map(error_for_reference_to_generic_type)
            .collect())
    }
}

fn error_for_reference_to_generic_type(ty: &Type) -> Error {
    DiagnosticBuilder::error(ty.span(), "References to generic types are not supported")
        .build()
        .into()
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
                self.visit_reference_in_initial_state(ty);
            }
            (TypeVisitorState::CheckingReferenceInner, Type::Path(TypePath { path, .. })) => {
                self.visit_path_when_checking_reference_inner(ty, path)
            }
            _ => visit_type(self, ty),
        }
    }
}

impl<'a> TypeVisitor<'a> {
    fn visit_reference_in_initial_state(&mut self, ty: &'a Type) {
        self.state = TypeVisitorState::CheckingReferenceInner;

        visit_type(self, ty);
        if let TypeVisitorState::FoundReferenceToGenericType = self.state {
            self.references_to_generic_types.push(ty);
        }

        self.state = TypeVisitorState::Initial;
    }

    fn visit_path_when_checking_reference_inner(&mut self, ty: &'a Type, path: &Path) {
        if self
            .generic_types_on_trait
            .contains(path.first_segment_as_ident().unwrap())
        {
            self.state = TypeVisitorState::FoundReferenceToGenericType;
        } else {
            visit_type(self, ty)
        }
    }
}
