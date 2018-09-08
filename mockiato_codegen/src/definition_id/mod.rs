use rustc::hir::def_id;
use rustc_resolve::Resolver as ResolverImpl;
use syntax::ext::base::Resolver as SyntaxResolver;

mod predictor;
mod resolver;

pub(crate) use self::predictor::{ContextPredictor, Predictor};
pub(crate) use self::resolver::{ContextResolver, Resolver};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub(crate) struct DefId(pub(crate) def_id::DefId);

#[cfg(test)]
impl DefId {
    pub(crate) fn dummy(value: u32) -> Self {
        DefId(def_id::DefId::local(def_id::DefIndex::from_raw_u32(22)))
    }
}

fn transmute_resolver(mut resolver: &mut SyntaxResolver) -> &mut &mut ResolverImpl {
    // Behold — The mighty transmutation
    unsafe { std::mem::transmute(&mut resolver) }
}
