#![crate_type = "dylib"]
#![feature(
    quote,
    concat_idents,
    plugin_registrar,
    rustc_private,
    decl_macro,
    custom_attribute,
    underscore_imports,
    tool_lints
)]

extern crate rustc;
extern crate rustc_plugin;
extern crate rustc_resolve;
extern crate syntax;
extern crate syntax_pos;

use crate::rustc_plugin::Registry;
use crate::syntax::ext::base::SyntaxExtension;
use crate::syntax::symbol::Symbol;

mod context;
mod definition_id;
mod derive_resolver;
mod generate;
mod mockable;
mod parse;
mod trait_bound_resolver;

use self::mockable::Mockable;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(Box::new(Mockable::new())),
    );
}
