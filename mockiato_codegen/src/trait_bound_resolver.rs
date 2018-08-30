use crate::parse::trait_decl::TraitDecl;
use crate::path_resolver::DefId;

pub(crate) trait TraitBoundResolver {
    fn register_mocked_trait<'a>(&mut self, identifier: DefId, mocked_trait: &TraitDecl);
    fn resolve_trait_bound<'a>(&self, identifier: &str) -> Option<TraitBound<'_>>;
}

pub(crate) enum TraitBound<'a> {
    Derivable(String),
    AlreadyMockedTrait(&'a TraitDecl),
}
