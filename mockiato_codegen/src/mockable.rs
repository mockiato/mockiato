use syntax::ast::{self, Ident, VariantData, DUMMY_NODE_ID};
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemDecorator};
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax_pos::Span;

use crate::parse::mockable_attr::MockableAttr;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_bounds::TraitBounds;
use crate::parse::trait_decl::TraitDecl;

pub(crate) struct Mockable;

impl MultiItemDecorator for Mockable {
    fn expand(
        &self,
        cx: &mut ExtCtxt,
        _sp: Span,
        meta_item: &ast::MetaItem,
        item: &Annotatable,
        push: &mut dyn FnMut(Annotatable),
    ) {
        let trait_decl = match TraitDecl::parse(cx, item) {
            Ok(trait_decl) => trait_decl,
            Err(_) => return,
        };

        let mockable_attr = match MockableAttr::parse(cx, meta_item) {
            Some(mockable_attr) => mockable_attr,
            None => return,
        };

        let trait_bounds = TraitBounds::parse(&trait_decl);

        let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);
        println!("{:#?}", trait_decl.generic_bounds);

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

fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.expand())
        .unwrap_or_else(|| Ident::from_str(&format!("{}Mock", trait_decl.ident)))
}
