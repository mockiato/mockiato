use syn::{Ident, Path};

pub(crate) trait PathExt {
    fn first_segment_as_ident(&self) -> Option<&Ident>;
}

impl PathExt for Path {
    fn first_segment_as_ident(&self) -> Option<&Ident> {
        Some(&self.segments.iter().nth(0)?.ident)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::punctuated::Punctuated;
    use syn::{parse_quote, Ident, Path};

    #[test]
    fn returns_none_if_path_is_empty() {
        let path = Path {
            leading_colon: None,
            segments: Punctuated::new(),
        };

        assert!(path.first_segment_as_ident().is_none());
    }

    #[test]
    fn returns_first_segment_without_params() {
        let path: Path = parse_quote!(Foo<'a>::Bar::Baz);
        let expected_ident: Ident = parse_quote!(Foo);

        assert_eq!(&expected_ident, path.first_segment_as_ident().unwrap(),);
    }
}
