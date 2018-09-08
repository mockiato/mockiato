use super::{transmute_resolver, DefId};
use crate::context::Context;
use std::ops::DerefMut;
use syntax::ast::Path;
use syntax_pos::DUMMY_SP;

pub(crate) trait Resolver {
    fn resolve_path(&mut self, path: Path) -> Option<DefId> {
        self.resolve_str_path(&path.to_string())
    }

    fn resolve_str_path(&mut self, path: &str) -> Option<DefId>;
}

#[derive(Clone)]
pub(crate) struct ContextResolver<'a, 'b: 'a> {
    context: Context<'a, 'b>,
}

impl<'a, 'b: 'a> ContextResolver<'a, 'b> {
    pub(crate) fn new(context: Context<'a, 'b>) -> Self {
        Self { context }
    }
}

impl<'a, 'b: 'a> Resolver for ContextResolver<'a, 'b> {
    fn resolve_str_path(&mut self, path: &str) -> Option<DefId> {
        let mut context = self.context.into_inner();
        let resolver = transmute_resolver(context.deref_mut().resolver);

        let path = resolver
            .resolve_str_path_error(DUMMY_SP, path, false)
            .ok()?;

        let def_id = path.def.def_id();

        Some(DefId(def_id))
    }
}
