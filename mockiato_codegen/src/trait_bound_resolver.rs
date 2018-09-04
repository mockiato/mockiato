use crate::definition_id::DefId;
use crate::definition_id::Resolver;
use crate::parse::trait_decl::TraitDecl;
use syntax::ast::Path;

pub(crate) trait TraitBoundResolver {
    fn register_mocked_trait<'a>(&mut self, identifier: DefId, mocked_trait: &TraitDecl);
    fn resolve_trait_bound<'a>(
        &self,
        resolver: &mut dyn Resolver,
        path: &Path,
    ) -> Option<TraitBoundType<'_>>;
}

pub(crate) enum TraitBoundType<'a> {
    Derivable(String),
    AlreadyMockedTrait(&'a TraitDecl),
}

pub(crate) struct TraitBoundResolverImpl;
impl TraitBoundResolverImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl TraitBoundResolver for TraitBoundResolverImpl {
    fn register_mocked_trait<'a>(&mut self, identifier: DefId, mocked_trait: &TraitDecl) {}

    fn resolve_trait_bound<'a>(
        &self,
        resolver: &mut dyn Resolver,
        path: &Path,
    ) -> Option<TraitBoundType<'_>> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syntax::ast::{Generics, Ident};
    use syntax_pos::{Globals, DUMMY_SP, GLOBALS};

    #[test]
    fn test_registers_mocked_trait() {
        GLOBALS.set(&Globals::new(), || {
            let identifier = DefId::dummy(1234);

            let mocked_trait = TraitDecl {
                span: DUMMY_SP,
                ident: Ident::from_str("Test"),
                generics: Generics::default(),
                generic_bounds: Vec::new(),
                items: Vec::new(),
            };

            let mut resolver = TraitBoundResolverImpl::new();

            resolver.register_mocked_trait(identifier, &mocked_trait);

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
