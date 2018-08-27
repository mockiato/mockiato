#![crate_type = "dylib"]
#![feature(
    quote,
    concat_idents,
    plugin_registrar,
    rustc_private,
    decl_macro
)]
#![feature(custom_attribute)]

use rustc_plugin::Registry;
use syntax::ast::{self, Ident, Unsafety, VariantData, DUMMY_NODE_ID};
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemDecorator, SyntaxExtension};
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax::symbol::Symbol;
use syntax_pos::Span;

pub mod parse;

use self::parse::mockable_attr::MockableAttr;
use self::parse::name_attr::NameAttr;
use self::parse::trait_decl::TraitDecl;

fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.expand())
        .unwrap_or_else(|| Ident::from_str(&format!("{}Mock", trait_decl.ident)))
}

struct Mockable;

impl MultiItemDecorator for Mockable {
    fn expand(
        &self,
        cx: &mut ExtCtxt,
        _sp: Span,
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

        if trait_decl.unsafety == &Unsafety::Unsafe {
            cx.span_err(item.span(), "#[mockable] does not support unsafe traits");
            return;
        }

        let mockable_attr = match MockableAttr::parse(cx, meta_item) {
            Some(mockable_attr) => mockable_attr,
            None => return,
        };

        let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);

        let mut mock_struct = cx
            .item_struct(
                meta_item.span,
                mock_struct_ident,
                VariantData::Unit(DUMMY_NODE_ID),
            ).into_inner();

        if let Some(derive_attr) = mockable_attr.derive_attr {
            mock_struct.attrs.push(derive_attr.expand(cx));
        }

        push(Annotatable::Item(P(mock_struct)));
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(Box::new(Mockable)),
    );
}
