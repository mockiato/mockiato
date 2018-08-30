#![crate_type = "dylib"]
#![feature(
    quote,
    concat_idents,
    plugin_registrar,
    rustc_private,
    decl_macro,
    custom_attribute
)]

use rustc_plugin::Registry;
use syntax::ext::base::SyntaxExtension;
use syntax::symbol::Symbol;

mod mockable;
mod parse;
mod path_resolver;
mod trait_bound_resolver;

use self::mockable::Mockable;
use self::trait_bound_resolver::TraitBoundResolverImpl;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(Box::new(Mockable::new(Box::new(
            TraitBoundResolverImpl::new(),
        )))),
    );
}
