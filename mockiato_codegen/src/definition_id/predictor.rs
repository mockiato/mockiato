use super::Resolver;
use super::{transmute_resolver, DefId};
use crate::context::Context;
use crate::rustc::hir::def_id;
use crate::rustc::hir::lowering::Resolver as LoweringResolver;
use crate::syntax::ast::{Ident, Path};
use std::ops::DerefMut;

pub(crate) trait Predictor {
    fn predict_next_id(&mut self, generated_items: u32) -> DefId;
}

pub(crate) struct ContextPredictor<'a, 'b: 'a> {
    context: Context<'a, 'b>,
    resolver: Box<dyn Resolver + 'a>,
}

impl<'a, 'b: 'a> ContextPredictor<'a, 'b> {
    pub(crate) fn new(context: Context<'a, 'b>, resolver: Box<dyn Resolver + 'a>) -> Self {
        Self { context, resolver }
    }
}

impl<'a, 'b> Predictor for ContextPredictor<'a, 'b> {
    fn predict_next_id(&mut self, generated_items: u32) -> DefId {
        let address_space = {
            let self_id = self
                .resolver
                .resolve_path(&Path::from_ident(Ident::from_str("self")))
                .expect("unable to resolve self");

            self_id.0.index.address_space()
        };

        let mut inner_context = self.context.into_inner();
        let resolver = transmute_resolver(inner_context.resolver.deref_mut());

        let def_index = resolver
            .definitions()
            .def_path_table()
            .next_id(address_space);

        let corrected_def_index =
            def_id::DefIndex::from_raw_u32(def_index.as_raw_u32() + 1 + generated_items);

        DefId(def_id::DefId::local(corrected_def_index))
    }
}
