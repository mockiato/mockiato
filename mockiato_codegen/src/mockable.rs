use crate::syntax::ast::{self, Ident, VariantData, DUMMY_NODE_ID};
use crate::syntax::ext::base::{Annotatable, ExtCtxt, MultiItemDecorator};
use crate::syntax::ext::build::AstBuilder;
use crate::syntax::ptr::P;
use crate::syntax_pos::Span;

use crate::context::Context;
use crate::definition_id::ContextResolver;
use crate::derive_resolver::DeriveResolverImpl;
use crate::generate::DeriveAttributeGenerator;
use crate::parse::mockable_attr::MockableAttr;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use crate::trait_bound_resolver::TraitBoundResolverImpl;
use std::clone::Clone;

#[derive(Default)]
pub(crate) struct Mockable;

impl Mockable {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl<'a> MultiItemDecorator for Mockable {
    fn expand(
        &self,
        cx: &mut ExtCtxt,
        _sp: Span,
        meta_item: &ast::MetaItem,
        item: &Annotatable,
        push: &mut dyn FnMut(Annotatable),
    ) {
        let cx = Context::new(cx);
        let resolver = ContextResolver::new(cx.clone());
        let trait_bound_resolver =
            TraitBoundResolverImpl::new(Box::new(DeriveResolverImpl::new(Box::new(resolver))));
        let derive_attr_generator =
            DeriveAttributeGenerator::new(cx.clone(), Box::new(trait_bound_resolver));

        let trait_decl = match TraitDecl::parse(&cx, item) {
            Ok(trait_decl) => trait_decl,
            Err(_) => return,
        };

        let mockable_attr = match MockableAttr::parse(&cx, meta_item) {
            Some(mockable_attr) => mockable_attr,
            None => return,
        };

        let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);

        let mut mock_struct = cx
            .into_inner()
            .item_struct(
                meta_item.span,
                mock_struct_ident,
                VariantData::Unit(DUMMY_NODE_ID),
            )
            .into_inner();

        match derive_attr_generator.generate_for_trait(&trait_decl) {
            Some(attr) => mock_struct.attrs.push(attr),
            None => return,
        }

        push(Annotatable::Item(P(mock_struct)));
    }
}

fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.expand())
        .unwrap_or_else(|| Ident::from_str(&format!("{}Mock", trait_decl.ident)))
}
