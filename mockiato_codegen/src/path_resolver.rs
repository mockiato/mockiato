use rustc;
use rustc_resolve::Resolver;
use syntax::ast::Path;
use syntax::ext::base::ExtCtxt;
use syntax_pos::DUMMY_SP;

#[derive(Eq, PartialEq, Hash, Debug)]
pub(crate) struct DefId(rustc::hir::def_id::DefId);

pub(crate) trait PathResolver {
    fn resolve_path(&mut self, path: Path) -> Result<DefId, ()>;
}

impl<'a> PathResolver for ExtCtxt<'a> {
    fn resolve_path(&mut self, path: Path) -> Result<DefId, ()> {
        // Behold — The mighty transmutation
        let resolver: &mut &mut Resolver = unsafe { std::mem::transmute(&mut self.resolver) };

        let path = resolver.resolve_str_path_error(DUMMY_SP, &path.to_string(), false)?;

        Ok(DefId(path.def.def_id()))
    }
}
