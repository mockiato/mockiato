use crate::parse::method_inputs::MethodInputs;
use std::collections::HashSet;
use syn::punctuated::Punctuated;
use syn::{
    parse_quote, GenericArgument, GenericParam, Generics, Ident, Path, PathArguments,
    PredicateType, Token, Type, TypeParam, WhereClause, WherePredicate,
};
use syn::visit::{Visit, visit_path};

pub(super) fn arguments_struct_path_generics(inputs: &MethodInputs) -> PathArguments {
    let arguments: Punctuated<_, Token![,]> = inputs
        .args
        .iter()
        .map(|arg| GenericArgument::Type(arg.ty.clone()))
        .collect();

    PathArguments::AngleBracketed(parse_quote!(<#arguments>))
}

pub(super) fn get_matching_generics_for_method_inputs(
    inputs: &MethodInputs,
    generics: &Generics,
) -> Generics {
    let matching_generic_types = find_matching_generic_types(inputs, generics);

    let where_clause = generics
        .where_clause
        .as_ref()
        .map(|where_clause| filter_where_clause(where_clause, &matching_generic_types));

    let params = matching_generic_types
        .into_iter()
        .map(|ident| GenericParam::Type(TypeParam::from(ident.clone())))
        .collect();

    Generics {
        lt_token: generics.lt_token.clone(),
        gt_token: generics.gt_token.clone(),
        params: params,
        where_clause,
    }
}

fn filter_where_clause(
    WhereClause {
        predicates,
        where_token,
    }: &WhereClause,
    generic_type_params: &HashSet<&Ident>,
) -> WhereClause {
    let predicates = predicates
        .iter()
        .filter(|predicate| match predicate {
            WherePredicate::Type(PredicateType { bounded_ty, .. }) => {
                match first_path_segment_ident_from_type(bounded_ty) {
                    Some(ident) => generic_type_params.contains(ident),
                    None => false,
                }
            }
            _ => true,
        })
        .cloned()
        .collect();

    WhereClause {
        where_token: where_token.clone(),
        predicates,
    }
}

fn first_path_segment_ident_from_type<'a>(ty: &'a Type) -> Option<&'a Ident> {
    match ty {
        Type::Path(ty) if !ty.path.segments.is_empty() => Some(&ty.path.segments[0].ident),
        _ => None,
    }
}

fn find_matching_generic_types<'a>(
    inputs: &'a MethodInputs,
    generics: &'a Generics,
) -> HashSet<&'a Ident> {
    let mut visitor = FindGenericTypeIdents {
        generic_types_filter: generics.type_params().map(|param| &param.ident).collect(),
        matching_generic_types: HashSet::new(),
    };

    for argument in &inputs.args {
        visitor.visit_type(&argument.ty);
    }

    visitor.matching_generic_types
}

struct FindGenericTypeIdents<'a> {
    generic_types_filter: HashSet<&'a Ident>,
    matching_generic_types: HashSet<&'a Ident>,
}

impl<'a> Visit<'a> for FindGenericTypeIdents<'a> {
    fn visit_path(&mut self, path: &'a Path) {
        visit_path(self, path);

        if !path.segments.is_empty() {
            let first_segment = &path.segments[0];
            let first_segment_ident = &first_segment.ident;

            if self.generic_types_filter.get(first_segment_ident).is_some()
                && first_segment.arguments.is_empty()
            {
                self.matching_generic_types.insert(first_segment_ident);
            }
        }
    }
}
