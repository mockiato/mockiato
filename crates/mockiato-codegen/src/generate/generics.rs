use crate::parse::method_inputs::MethodInputs;
use std::collections::HashSet;
use syn::visit::{visit_path, Visit};
use syn::{
    GenericParam, Generics, Ident, Path, PredicateType, Type, TypeParam, WhereClause,
    WherePredicate,
};

pub(super) fn get_matching_generics_for_method_inputs(
    inputs: &MethodInputs,
    generics: &Generics,
) -> Generics {
    let matching_generic_types = find_overlapping_generic_types(inputs, generics);

    let where_clause = generics
        .where_clause
        .as_ref()
        .map(|where_clause| filter_where_clause(where_clause, &matching_generic_types));

    let params = matching_generic_types
        .into_iter()
        .map(|ident| GenericParam::Type(TypeParam::from(ident.clone())))
        .collect();

    Generics {
        lt_token: generics.lt_token,
        gt_token: generics.gt_token,
        params,
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
        where_token: *where_token,
        predicates,
    }
}

fn first_path_segment_ident_from_type(ty: &Type) -> Option<&'_ Ident> {
    match ty {
        Type::Path(ty) if !ty.path.segments.is_empty() => Some(&ty.path.segments[0].ident),
        _ => None,
    }
}

fn find_overlapping_generic_types<'a>(
    inputs: &'a MethodInputs,
    generics: &'a Generics,
) -> HashSet<&'a Ident> {
    let mut visitor = FindOverlappingGenericTypeIdents {
        generic_type_idents_filter: generics.type_params().map(|param| &param.ident).collect(),
        overlapping_generic_type_idents: HashSet::new(),
    };

    for argument in &inputs.args {
        visitor.visit_type(&argument.ty);
    }

    visitor.overlapping_generic_type_idents
}

struct FindOverlappingGenericTypeIdents<'a> {
    generic_type_idents_filter: HashSet<&'a Ident>,
    overlapping_generic_type_idents: HashSet<&'a Ident>,
}

impl<'a> Visit<'a> for FindOverlappingGenericTypeIdents<'a> {
    fn visit_path(&mut self, path: &'a Path) {
        visit_path(self, path);

        if !path.segments.is_empty() {
            let first_segment = &path.segments[0];
            let first_segment_ident = &first_segment.ident;

            if self
                .generic_type_idents_filter
                .get(first_segment_ident)
                .is_some()
                && first_segment.arguments.is_empty()
            {
                self.overlapping_generic_type_idents
                    .insert(first_segment_ident);
            }
        }
    }
}
