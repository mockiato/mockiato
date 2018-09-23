use crate::derive_resolver::DeriveResolver;
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
    derive_resolver: Box<dyn DeriveResolver + 'a>,
}

impl<'a> TraitBoundResolverImpl<'a> {
    pub(crate) fn new(derive_resolver: Box<dyn DeriveResolver + 'a>) -> Self {
        Self { derive_resolver }
    }
}

impl<'a> TraitBoundResolver for TraitBoundResolverImpl<'a> {
    fn resolve_trait_bound(&self, path: &Path) -> Option<TraitBoundType> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
