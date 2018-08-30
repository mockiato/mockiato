use crate::parse::trait_decl::TraitDecl;

pub(crate) trait TraitBoundResolver {
    fn register_mocked_trait(&mut self, identifier: &str, mocked_trait: &TraitDecl);
    fn resolve_trait_bound(&self, identifier: &str) -> Option<TraitBound<'_>>;
}

pub(crate) enum TraitBound<'a> {
    Derivable(String),
    AlreadyMockedTrait(&'a TraitDecl),
}
