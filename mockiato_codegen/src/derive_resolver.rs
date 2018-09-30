use crate::definition_id::Resolver;
use crate::syntax::ast::{Ident, Path};

const DERIVABLE_TRAITS: [(&str, &str); 9] = [
    ("Clone", "std::clone::Clone"),
    ("Hash", "std::hash::Hash"),
    ("PartialEq", "std::cmp::PartialEq"),
    ("Eq", "std::cmp::Eq"),
    ("PartialOrd", "std::cmp::PartialOrd"),
    ("Ord", "std::cmp::Ord"),
    ("Debug", "std::fmt::Debug"),
    ("Default", "std::default::Default"),
    ("Copy", "std::marker::Copy"),
];

pub(crate) trait DeriveResolver {
    /// Resolves an in-code [`Path`] into a derivable name
    /// which can be used in a `#[derive(..)]` attribute
    fn resolve_derivable_name(&self, path: &Path) -> Option<Path>;
}

#[allow(dead_code)]
pub(crate) struct DeriveResolverImpl<'a> {
    resolver: Box<dyn Resolver + 'a>,
}

impl<'a> DeriveResolverImpl<'a> {
    pub(crate) fn new(resolver: Box<dyn Resolver + 'a>) -> Self {
        DeriveResolverImpl { resolver }
    }
}

impl<'a> DeriveResolver for DeriveResolverImpl<'a> {
    fn resolve_derivable_name(&self, path: &Path) -> Option<Path> {
        let def_id = self.resolver.resolve_path(path)?;

        let derivable_trait = DERIVABLE_TRAITS.iter().find(|(_, path)| {
            let comp_dev_id = match self.resolver.resolve_str_path(path) {
                Some(value) => value,
                None => return false,
            };

            comp_dev_id == def_id
        })?;

        Some(Path::from_ident(Ident::from_str(derivable_trait.0)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::definition_id::DefId;
    use crate::syntax_pos::{Globals, GLOBALS};

    #[test]
    fn test_resolve_derivable_name_works() {
        struct ResolverMock;

        impl Resolver for ResolverMock {
            fn resolve_str_path(&self, path: &str) -> Option<DefId> {
                if path == "Debug1234" || path == "std::fmt::Debug" {
                    return Some(DefId::dummy(22));
                }

                None
            }
        }

        let resolver = ResolverMock;
        let derive_resolver = DeriveResolverImpl::new(Box::new(resolver));

        GLOBALS.set(&Globals::new(), || {
            assert_eq!(
                derive_resolver
                    .resolve_derivable_name(&Path::from_ident(Ident::from_str("Debug1234")))
                    .unwrap(),
                "Debug"
            );
        });
    }
}
