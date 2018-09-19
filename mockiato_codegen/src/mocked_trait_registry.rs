use crate::definition_id::DefId;
use crate::parse::trait_decl::TraitDecl;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub(crate) trait MockedTraitRegistry {
    fn register_mocked_trait(&self, identifier: DefId, mocked_trait: TraitDecl);
    fn get_mocked_trait(&self, identifier: DefId) -> Option<TraitDecl>;
}

#[derive(Default, Clone)]
pub(crate) struct MockedTraitRegistryImpl {
    mocked_traits: Arc<RwLock<HashMap<DefId, TraitDecl>>>,
}

impl MockedTraitRegistryImpl {
    pub(crate) fn new() -> Self {
        Default::default()
    }
}

const LOCK_RESOLVER_ERR: &str = "Internal Error: Mocked Trait Registry is poisoned";

impl MockedTraitRegistry for MockedTraitRegistryImpl {
    fn register_mocked_trait(&self, identifier: DefId, mocked_trait: TraitDecl) {
        self.mocked_traits
            .write()
            .expect(LOCK_RESOLVER_ERR)
            .insert(identifier, mocked_trait);
    }

    fn get_mocked_trait(&self, identifier: DefId) -> Option<TraitDecl> {
        self.mocked_traits
            .read()
            .expect(LOCK_RESOLVER_ERR)
            .get(&identifier)
            .map(Clone::clone)
    }
}
