use rustc::hir::def_id;
use rustc::hir::lowering::Resolver as LoweringResolver;
use rustc_resolve::Resolver as ResolverImpl;
use syntax::ast::{Ident, Path};
use syntax::ext::base::{ExtCtxt, Resolver as SyntaxResolver};
use syntax_pos::DUMMY_SP;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub(crate) struct DefId(pub(crate) def_id::DefId);

#[cfg(test)]
impl DefId {
    pub(crate) fn dummy(value: u32) -> Self {
        DefId(def_id::DefId::local(def_id::DefIndex::from_raw_u32(22)))
    }
}

pub(crate) trait Resolver {
    fn resolve_path(&mut self, path: Path) -> Option<DefId> {
        self.resolve_str_path(&path.to_string())
    }

    fn resolve_str_path(&mut self, path: &str) -> Option<DefId>;
}

pub(crate) trait Predictor {
    fn predict_next_id(&mut self, generated_items: u32) -> DefId;
}

fn transmute_resolver(mut resolver: &mut SyntaxResolver) -> &mut &mut ResolverImpl {
    // Behold — The mighty transmutation
    unsafe { std::mem::transmute(&mut resolver) }
}

impl<'a> Resolver for ExtCtxt<'a> {
    fn resolve_str_path(&mut self, path: &str) -> Option<DefId> {
        let resolver = transmute_resolver(self.resolver);

        let path = resolver
            .resolve_str_path_error(DUMMY_SP, path, false)
            .ok()?;

        let def_id = path.def.def_id();

        Some(DefId(def_id))
    }
}

impl<'a> Predictor for ExtCtxt<'a> {
    fn predict_next_id(&mut self, generated_items: u32) -> DefId {
        let address_space = {
            let self_id = self
                .resolve_path(Path::from_ident(Ident::from_str("self")))
                .expect("unable to resolve self");

            self_id.0.index.address_space()
        };

        let resolver = transmute_resolver(self.resolver);

        let def_index = resolver
            .definitions()
            .def_path_table()
            .next_id(address_space);

        let corrected_def_index =
            def_id::DefIndex::from_raw_u32(def_index.as_raw_u32() + 1 + generated_items);

        DefId(def_id::DefId::local(corrected_def_index))
    }
}
