#![crate_type = "dylib"]
#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_pos;

use rustc_plugin::Registry;
use syntax::ast;
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemDecorator, SyntaxExtension};
use syntax::symbol::Symbol;
use syntax_pos::Span;

struct Mockable;

impl MultiItemDecorator for Mockable {
    fn expand(
        &self,
        cx: &mut ExtCtxt,
        sp: Span,
        meta_item: &ast::MetaItem,
        item: &Annotatable,
        push: &mut dyn FnMut(Annotatable),
    ) {
        cx.parse_sess.span_diagnostic.span_note_without_error(sp, "Let's brew you some macchiato");
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(Box::new(Mockable)),
    );
}
