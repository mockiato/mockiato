use crate::derive_resolver::DeriveResolver;
use crate::syntax::ast::Path;

pub(crate) trait TraitBoundResolver {
    fn resolve_trait_bound(&self, path: &Path) -> Option<TraitBoundType>;
}

#[allow(dead_code)]
pub(crate) enum TraitBoundType {
    Derivable(Path),
}

pub(crate) struct TraitBoundResolverImpl<'a> {
    derive_resolver: Box<dyn DeriveResolver + 'a>,
}

impl<'a> TraitBoundResolverImpl<'a> {
    pub(crate) fn new(derive_resolver: Box<dyn DeriveResolver + 'a>) -> Self {
        Self { derive_resolver }
    }
}

impl<'a> TraitBoundResolver for TraitBoundResolverImpl<'a> {
    fn resolve_trait_bound(&self, path: &Path) -> Option<TraitBoundType> {
        let derivable_name = self.derive_resolver.resolve_derivable_name(path)?;

        Some(TraitBoundType::Derivable(derivable_name))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::syntax::ast::Ident;
    use crate::syntax_pos::{Globals, GLOBALS};
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::thread;

    #[test]
    fn resolves_derivable_traits() {
        struct DeriveResolverMock {
            called: RefCell<bool>,
        }

        impl Drop for DeriveResolverMock {
            fn drop(&mut self) {
                if !thread::panicking() {
                    if !self.called.borrow().deref() {}
                }
            }
        }

        impl DeriveResolver for DeriveResolverMock {
            fn resolve_derivable_name(&self, path: &Path) -> Option<Path> {
                if *self.called.borrow().deref() {
                    panic!("resolve_derivable_name was only expected to be called once");
                }

                assert!(path == &"std::fmt::Debug");

                self.called.replace(true);

                Some(Path::from_ident(Ident::from_str("Debug")))
            }
        }

        GLOBALS.set(&Globals::new(), || {
            let derive_resolver = DeriveResolverMock {
                called: RefCell::default(),
            };
            let trait_bound_resolver = TraitBoundResolverImpl::new(Box::new(derive_resolver));

            let trait_bound = trait_bound_resolver
                .resolve_trait_bound(&Path::from_ident(Ident::from_str("std::fmt::Debug")))
                .unwrap();

            match trait_bound {
                TraitBoundType::Derivable(path) => assert!(path == &"Debug"),
            };
        });
    }
}
