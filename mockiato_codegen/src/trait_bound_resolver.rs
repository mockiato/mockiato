use crate::parse::trait_decl::TraitDecl;
use syntax::symbol::LocalInternedString;

pub(crate) trait TraitBoundResolver {
    fn resolve_trait<'a>(&self, identifier: &'a str) -> Option<TraitBound<'a>>;
}

pub(crate) enum TraitBound<'a> {
    Derivable,
    AlreadyMockedTrait(TraitDecl<'a>)
}


