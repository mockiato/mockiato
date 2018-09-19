use crate::definition_id::Resolver as DefIdResolver;
use crate::derive_resolver::DeriveResolver;
use crate::mocked_trait_registry::MockedTraitRegistry;
use crate::parse::trait_decl::TraitDecl;
use crate::syntax::ast::Path;

pub(crate) trait TraitBoundResolver {
    fn resolve_trait_bound(&self, path: &Path) -> Option<TraitBoundType>;
}

#[allow(dead_code)]
pub(crate) enum TraitBoundType {
    Derivable(String),
    AlreadyMockedTrait(TraitDecl),
}

pub(crate) struct TraitBoundResolverImpl<'a> {
    mocked_trait_registry: Box<dyn MockedTraitRegistry + 'a>,
    derive_resolver: Box<dyn DeriveResolver + 'a>,
    def_id_resolver: Box<dyn DefIdResolver + 'a>,
}

impl<'a> TraitBoundResolverImpl<'a> {
    pub(crate) fn new(
        mocked_trait_registry: Box<dyn MockedTraitRegistry + 'a>,
        derive_resolver: Box<dyn DeriveResolver + 'a>,
        def_id_resolver: Box<dyn DefIdResolver + 'a>,
    ) -> Self {
        Self {
            mocked_trait_registry,
            derive_resolver,
            def_id_resolver,
        }
    }
}

impl<'a> TraitBoundResolver for TraitBoundResolverImpl<'a> {
    fn resolve_trait_bound(&self, path: &Path) -> Option<TraitBoundType> {
        let def_id = self.def_id_resolver.resolve_path(path)?;
        let mocked_trait = self.mocked_trait_registry.get_mocked_trait(def_id);

        if let Some(mocked_trait) = mocked_trait {
            return Some(TraitBoundType::AlreadyMockedTrait(mocked_trait));
        }

        None
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

            #[derive(Default)]
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
                    assert_eq!(identifier, DefId::dummy(1234));

                    self.called.replace(true);

                    Some(TraitDecl {
                        span: DUMMY_SP,
                        ident: Ident::from_str("Test"),
                        generics: Generics::default(),
                        generic_bounds: Vec::new(),
                        items: Vec::new(),
                    })
                }
            }

            #[derive(Default)]
            struct DefIdResolverMock {
                called: RefCell<bool>,
            }

            impl Drop for DefIdResolverMock {
                fn drop(&mut self) {
                    if !std::thread::panicking() {
                        assert!(*self.called.borrow())
                    }
                }
            }

            impl DefIdResolver for DefIdResolverMock {
                fn resolve_path(&self, path: &Path) -> Option<DefId> {
                    assert!(path == &"Test");

                    self.called.replace(true);

                    Some(DefId::dummy(1234))
                }

                fn resolve_str_path(&self, _path: &str) -> Option<DefId> {
                    panic!("unexpected call to resolve_str_path()");
                }
            }

            struct DeriveResolverMock;

            impl DeriveResolver for DeriveResolverMock {
                fn resolve_derivable_name(
                    &self,
                    _resolver: &mut dyn DefIdResolver,
                    _path: &Path,
                ) -> Option<Path> {
                    panic!("unexpected call to resolve_derivable_name()");
                }

                fn is_automatically_derivable(&self, _path: &Path) -> bool {
                    panic!("unexpected call to is_automatically_derivable()");
                }
            }

            let resolver = TraitBoundResolverImpl::new(
                Box::new(MockedTraitRegistryMock::default()),
                Box::new(DeriveResolverMock),
                Box::new(DefIdResolverMock::default()),
            );

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
