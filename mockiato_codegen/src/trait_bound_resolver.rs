use crate::parse::trait_decl::TraitDecl;
use crate::definition_id::DefId;

pub(crate) trait TraitBoundResolver {
    fn register_mocked_trait<'a>(&mut self, identifier: DefId, mocked_trait: &TraitDecl);
    fn resolve_trait_bound<'a>(&self, identifier: &str) -> Option<TraitBoundType<'_>>;
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

    fn resolve_trait_bound<'a>(&self, identifier: &str) -> Option<TraitBoundType<'_>> {
        None
    }
}
