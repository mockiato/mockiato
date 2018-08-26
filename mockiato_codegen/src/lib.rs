#![crate_type = "dylib"]
#![feature(quote, concat_idents, plugin_registrar, rustc_private)]
#![feature(custom_attribute)]

extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_pos;

use rustc_plugin::Registry;
use syntax::ast::{
    self, GenericBounds, Generics, Ident, IsAuto, Item, ItemKind, TraitItem, Unsafety,
};
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemDecorator, SyntaxExtension};
use syntax::symbol::Symbol;
use syntax_pos::Span;

#[derive(Debug)]
struct TraitDecl<'a> {
    span: Span,
    ident: Ident,
    is_auto: &'a IsAuto,
    unsafety: &'a Unsafety,
    generics: &'a Generics,
    generic_bounds: &'a GenericBounds,
    items: &'a [TraitItem],
}

impl<'a> TraitDecl<'a> {
    fn parse(annotated: &'a Annotatable) -> Result<Self, Span> {
        if let Annotatable::Item(ref item) = annotated {
            let span = item.span;
            let ident = item.ident;

            if let ItemKind::Trait(
                ref is_auto,
                ref unsafety,
                ref generics,
                ref generic_bounds,
                ref items,
            ) = item.node
            {
                return Ok(TraitDecl {
                    ident,
                    span,
                    is_auto,
                    unsafety,
                    generics,
                    generic_bounds,
                    items,
                });
            }
        }

        return Err(annotated.span());
    }
}

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
        let trait_decl = match TraitDecl::parse(item) {
            Ok(trait_decl) => trait_decl,
            Err(span) => {
                cx.span_err(span, "#[mockable] can only be used on traits");
                return;
            }
        };

        cx.parse_sess
            .span_diagnostic
            .span_note_without_error(item.span(), "Let's brew you some macchiato");
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(Box::new(Mockable)),
    );
}
