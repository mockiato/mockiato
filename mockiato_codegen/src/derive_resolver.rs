use crate::definition_id::Resolver;
use syntax::ast::Path;

pub(crate) trait DeriveResolver {
    /// Resolves a derivable name for a in-code `Path`
    fn resolve_derivable_name(&self, resolver: &mut dyn Resolver, path: &Path) -> Path;
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
    fn resolve_derivable_name(&self, _resolver: &mut dyn Resolver, _path: &Path) -> Path {
        unimplemented!();
    }

    fn is_automatically_derivable(&self, _path: &Path) -> bool {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syntax::ast::Ident;
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
}
