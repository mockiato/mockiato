use syn::{parse_quote, Visibility};

pub(super) fn raise_visibility_by_one_level(visibility: &Visibility) -> Visibility {
    match visibility {
        Visibility::Public(_) | Visibility::Crate(_) => visibility.clone(),
        Visibility::Inherited => parse_quote!(pub(super)),
        Visibility::Restricted(inner_visibility)
            if inner_visibility.path.segments[0].ident == "crate" =>
        {
            visibility.clone()
        }
        Visibility::Restricted(inner_visibility)
            if inner_visibility.path.segments[0].ident == "self" =>
        {
            let mut inner_visibility = inner_visibility.clone();
            inner_visibility.path.segments[0].ident = parse_quote!(super);
            Visibility::Restricted(inner_visibility)
        }
        Visibility::Restricted(inner_visibility) => {
            let mut inner_visibility = inner_visibility.clone();
            inner_visibility
                .path
                .segments
                .insert(0, parse_quote!(super));

            if inner_visibility.in_token.is_none() {
                inner_visibility.in_token = Some(parse_quote!(in));
            }

            Visibility::Restricted(inner_visibility)
        }
    }
}
