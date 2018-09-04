#![crate_type = "dylib"]
#![feature(
    quote,
    concat_idents,
    plugin_registrar,
    rustc_private,
    decl_macro,
    custom_attribute,
    underscore_imports,
)]

use rustc_plugin::Registry;
use syntax::ext::base::SyntaxExtension;
use syntax::symbol::Symbol;

mod definition_id;
mod derive_resolver;
mod mockable;
mod parse;
mod trait_bound_resolver;

use self::definition_id::ContextPredictorFactory;
use self::mockable::Mockable;
use self::trait_bound_resolver::TraitBoundResolverImpl;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(Box::new(Mockable::new(
            Box::new(TraitBoundResolverImpl::new()),
            Box::new(ContextPredictorFactory::default()),
        ))),
    );
}
