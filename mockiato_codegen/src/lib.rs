#![crate_type = "dylib"]
#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::ext::base::{MultiItemDecorator, SyntaxExtension};
use syntax::symbol::Symbol;

struct Mockable {}

impl MultiItemDecorator for Mockable {
    fn expand(
        &self,
        ecx: &mut ExtCtxt,
        sp: Span,
        meta_item: &ast::MetaItem,
        item: &Annotatable,
        push: &mut dyn FnMut(Annotatable),
    ) {
        unimplemented!()
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    registry.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(),
    );
}
