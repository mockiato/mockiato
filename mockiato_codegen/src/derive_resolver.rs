use crate::definition_id::Resolver;
use syntax::ast::{Ident, Path};

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
    /// Resolves a derivable name for a in-code `Path`
    fn resolve_derivable_name(&self, resolver: &mut dyn Resolver, path: &Path) -> Option<Path>;
    /// `Path` must be a name of a derive() value (e.g. Debug)
    fn is_automatically_derivable(&self, path: &Path) -> bool;
}

pub(crate) struct DeriveResolverImpl;

impl DeriveResolverImpl {
    pub(crate) fn new() -> Self {
        DeriveResolverImpl
    }
}

impl DeriveResolver for DeriveResolverImpl {
    fn resolve_derivable_name(&self, resolver: &mut dyn Resolver, path: &Path) -> Option<Path> {
        let def_id = resolver.resolve_path(path.clone())?;

        let derivable_trait = DERIVABLE_TRAITS.iter().find(|(_, path)| {
            let comp_dev_id = match resolver.resolve_str_path(path) {
                Some(value) => value,
                None => return false,
            };

            comp_dev_id == def_id
        })?;

        Some(Path::from_ident(Ident::from_str(derivable_trait.0)))
    }

    fn is_automatically_derivable(&self, path: &Path) -> bool {
        let name = path.segments.first().unwrap().ident;

        DERIVABLE_TRAITS
            .iter()
            .any(|(derive, _)| name.as_str() == *derive)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::definition_id::DefId;
    use rustc::hir::def_id::{self, DefIndex};
    use syntax_pos::{Globals, GLOBALS};

    #[test]
    fn test_is_automatically_derivable_works() {
        let derive_resolver = DeriveResolverImpl::new();

        GLOBALS.set(&Globals::new(), || {
            assert_eq!(
                true,
                derive_resolver
                    .is_automatically_derivable(&Path::from_ident(Ident::from_str("Clone")))
            );

            assert_eq!(
                true,
                derive_resolver
                    .is_automatically_derivable(&Path::from_ident(Ident::from_str("Debug")))
            );

            assert_eq!(
                false,
                derive_resolver
                    .is_automatically_derivable(&Path::from_ident(Ident::from_str("Display")))
            );
        });
    }

    #[test]
    fn test_resolve_derivable_name_works() {
        struct ResolverMock;

        impl Resolver for ResolverMock {
            fn resolve_str_path(&mut self, path: &str) -> Option<DefId> {
                if path == "Debug1234" || path == "std::fmt::Debug" {
                    return Some(DefId(def_id::DefId::local(DefIndex::from_raw_u32(22))));
                }

                None
            }
        }

        let mut resolver = ResolverMock;
        let derive_resolver = DeriveResolverImpl;

        GLOBALS.set(&Globals::new(), || {
            assert_eq!(
                derive_resolver
                    .resolve_derivable_name(
                        &mut resolver,
                        &Path::from_ident(Ident::from_str("Debug1234"))
                    ).unwrap(),
                "Debug"
            );
        });
    }
}
