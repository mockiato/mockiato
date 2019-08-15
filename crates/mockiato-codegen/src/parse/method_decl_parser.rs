use std::collections::hash_map::RandomState;
use std::collections::HashSet;

use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::visit::{visit_type, Visit};
use syn::{GenericParam, Generics, Path, Signature, TraitItem, TraitItemMethod, Type, TypePath};

use crate::diagnostic::DiagnosticBuilder;
use crate::parse::check_option_is_none;
use crate::parse::method_decl::{MethodDecl, MethodDeclParser};
use crate::parse::method_inputs::MethodInputsParser;
use crate::result::{merge_results, Error, Result};
use crate::syn_ext::PathExt;

#[derive(Debug)]
pub(crate) struct MethodDeclParserImpl {
    method_inputs_parser: Box<dyn MethodInputsParser>,
}

impl MethodDeclParserImpl {
    pub(crate) fn new(method_inputs_parser: Box<dyn MethodInputsParser>) -> Self {
        Self {
            method_inputs_parser,
        }
    }
}

impl MethodDeclParser for MethodDeclParserImpl {
    fn parse(
        &self,
        trait_item: TraitItem,
        generic_types_on_trait: &HashSet<Ident, RandomState>,
    ) -> Result<MethodDecl> {
        match trait_item {
            TraitItem::Method(method) => self.parse_method(method, generic_types_on_trait),
            trait_item => Err(invalid_trait_item_error(&trait_item)),
        }
    }
}

impl MethodDeclParserImpl {
    fn parse_method(
        &self,
        method: TraitItemMethod,
        generic_types_on_trait: &HashSet<Ident>,
    ) -> Result<MethodDecl> {
        let span = method.span();

        let TraitItemMethod {
            attrs,
            sig: signature,
            ..
        } = method;

        validate_usage_of_generic_types(&signature, generic_types_on_trait)?;

        let Signature {
            constness,
            unsafety,
            asyncness,
            ident,
            generics,
            inputs,
            output,
            ..
        } = signature;

        validate_generic_type_parameters(&generics)?;

        check_option_is_none(&constness, span, "`const` methods are not supported")?;
        check_option_is_none(&asyncness, span, "`async` methods are not supported")?;

        Ok(MethodDecl {
            attrs,
            unsafety,
            ident,
            generics,
            span,
            inputs: self.method_inputs_parser.parse(inputs)?,
            output,
        })
    }
}

fn invalid_trait_item_error(trait_item: &TraitItem) -> Error {
    DiagnosticBuilder::error(
        trait_item.span(),
        "Traits are only allowed to contain methods",
    )
    .build()
    .into()
}

fn validate_generic_type_parameters(generics: &Generics) -> Result<()> {
    let results = generics
        .params
        .iter()
        .map(|generic_param| match generic_param {
            GenericParam::Lifetime(_) => Ok(()),
            generic_param => Err(invalid_generic_param(generic_param)),
        });

    merge_results(results).map(|_| ())
}

fn invalid_generic_param(generic_param: &GenericParam) -> Error {
    let error_message = "Only lifetimes are supported as generic parameters on methods";
    DiagnosticBuilder::error(generic_param.span(), error_message)
        .build()
        .into()
}

fn validate_usage_of_generic_types(
    signature: &Signature,
    generic_types_on_trait: &HashSet<Ident>,
) -> Result<()> {
    let references_to_generic_types =
        find_references_to_generic_types(signature, generic_types_on_trait);

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
    signature: &'a Signature,
    generic_types_on_trait: &'a HashSet<Ident>,
) -> Vec<&'a Type> {
    let mut visitor = TypeVisitor {
        generic_types_on_trait,
        references_to_generic_types: Vec::new(),
        state: TypeVisitorState::Initial,
    };
    visitor.visit_signature(signature);
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
