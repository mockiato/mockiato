use crate::mocked_trait_registry::MockedTraitRegistry;
use crate::parse::trait_decl::TraitDecl;
use crate::syntax::ast::Path;

pub(crate) trait TraitBoundResolver {
    fn resolve_trait_bound(&self, path: &Path) -> Option<TraitBoundType<'_>>;
}

#[allow(dead_code)]
pub(crate) enum TraitBoundType<'a> {
    Derivable(String),
    AlreadyMockedTrait(&'a TraitDecl),
}

pub(crate) struct TraitBoundResolverImpl {
    mocked_trait_registry: Box<dyn MockedTraitRegistry>,
}

impl TraitBoundResolverImpl {
    pub(crate) fn new(mocked_trait_registry: Box<dyn MockedTraitRegistry>) -> Self {
        Self {
            mocked_trait_registry,
        }
    }
}

impl TraitBoundResolver for TraitBoundResolverImpl {
    fn resolve_trait_bound(&self, _path: &Path) -> Option<TraitBoundType<'_>> {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::definition_id::DefId;
    use crate::syntax::ast::{Generics, Ident};
    use crate::syntax_pos::{Globals, DUMMY_SP, GLOBALS};
    use std::cell::RefCell;

    #[test]
    fn test_resolves_mocked_trait() {
        GLOBALS.set(&Globals::new(), || {
            let mocked_trait = TraitDecl {
                span: DUMMY_SP,
                ident: Ident::from_str("Test"),
                generics: Generics::default(),
                generic_bounds: Vec::new(),
                items: Vec::new(),
            };

            struct MockedTraitRegistryMock {
                called: RefCell<bool>,
            }

            impl Drop for MockedTraitRegistryMock {
                fn drop(&mut self) {
                    if !std::thread::panicking() {
                        assert!(*self.called.borrow())
                    }
                }
            }

            impl MockedTraitRegistry for MockedTraitRegistryMock {
                fn register_mocked_trait(&self, _identifier: DefId, _mocked_trait: TraitDecl) {
                    panic!("Unexpected call to register_mocked_trait()");
                }
                fn get_mocked_trait(&self, identifier: DefId) -> Option<TraitDecl> {
                    self.called.replace(true);

                    assert_eq!(identifier, DefId::dummy(1234));

                    Some(TraitDecl {
                        span: DUMMY_SP,
                        ident: Ident::from_str("Test"),
                        generics: Generics::default(),
                        generic_bounds: Vec::new(),
                        items: Vec::new(),
                    })
                }
            }

            let resolver = TraitBoundResolverImpl::new(Box::new(MockedTraitRegistryMock {
                called: RefCell::default(),
            }));

            match resolver
                .resolve_trait_bound(&Path::from_ident(Ident::from_str("Test")))
                .unwrap()
            {
                TraitBoundType::AlreadyMockedTrait(already_mocked_trait) => {
                    assert_eq!(mocked_trait.ident, already_mocked_trait.ident);
                }
                TraitBoundType::Derivable(_) => panic!("Exected an already mocked trait"),
            };
        });
    }
}
