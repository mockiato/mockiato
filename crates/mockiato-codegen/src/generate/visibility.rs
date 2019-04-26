use std::iter;
use syn::{parse_quote, PathSegment, VisRestricted, Visibility};

pub(super) fn raise_visibility_by_one_level(visibility: &Visibility) -> Visibility {
    match visibility {
        Visibility::Inherited => parse_quote!(pub(super)),
        Visibility::Public(_) | Visibility::Crate(_) => visibility.clone(),
        Visibility::Restricted(restricted) => {
            Visibility::Restricted(if has_leading_colon(restricted) {
                restricted.clone()
            } else {
                raise_restricted_visibility_by_one_level(restricted)
            })
        }
    }
}

fn has_leading_colon(visibility: &VisRestricted) -> bool {
    visibility.path.leading_colon.is_some()
}

fn raise_restricted_visibility_by_one_level(visibility: &VisRestricted) -> VisRestricted {
    let mut visibility = visibility.clone();

    match &visibility.path.segments[0].ident {
        ident if ident == "crate" => {}
        ident if ident == "self" => replace_self_with_super_in_visibility(&mut visibility),
        _ => prefix_visibility_with_super(&mut visibility),
    }

    visibility
}

fn replace_self_with_super_in_visibility(visibility: &mut VisRestricted) {
    visibility.path.segments[0] = super_path_segment();
}

fn prefix_visibility_with_super(visibility: &mut VisRestricted) {
    visibility.in_token = Some(parse_quote!(in));
    visibility.path.segments = iter::once(super_path_segment())
        .chain(visibility.path.segments.iter().cloned())
        .collect();
}

fn super_path_segment() -> PathSegment {
    parse_quote!(super)
}
