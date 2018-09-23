use crate::syntax::ast::{self, Ident, Path, VariantData, DUMMY_NODE_ID};
use crate::syntax::ext::base::{Annotatable, ExtCtxt, MultiItemDecorator};
use crate::syntax::ext::build::AstBuilder;
use crate::syntax::ptr::P;
use crate::syntax_pos::Span;

use crate::context::Context;
use crate::definition_id::ContextResolver;
use crate::derive_resolver::DeriveResolverImpl;
use crate::parse::mockable_attr::MockableAttr;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_bounds::TraitBounds;
use crate::parse::trait_decl::TraitDecl;
use crate::trait_bound_resolver::{TraitBoundResolver, TraitBoundResolverImpl, TraitBoundType};
use std::clone::Clone;

pub(crate) struct Mockable {}

impl Mockable {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn mock_trait_bound_impls(
        &self,
        trait_bound_resolver: &'_ dyn TraitBoundResolver,
        cx: &Context,
        trait_decl: &TraitDecl,
    ) {
        let trait_bounds = TraitBounds::parse(trait_decl).0;

        for trait_bound in trait_bounds {
            let identifier = trait_bound.identifier;
            let trait_bound_type = trait_bound_resolver.resolve_trait_bound(&Path::from_ident(
                Ident::from_interned_str(identifier.as_interned_str()),
            ));
            self.mock_trait_bound_type(&cx, trait_bound.span, &trait_bound_type);
        }
    }

    fn mock_trait_bound_type(
        &self,
        cx: &Context,
        sp: Span,
        trait_bound_type: &Option<TraitBoundType>,
    ) {
        if trait_bound_type.is_none() {
            cx.into_inner().parse_sess
                .span_diagnostic
                .mut_span_err(sp, "The referenced trait is not a derivable trait")
                .help("Currently only traits that are automatically derivable are supported as supertrait")
                .emit();
        }
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
        let trait_bound_resolver = TraitBoundResolverImpl::new(Box::new(DeriveResolverImpl::new()));

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
            ).into_inner();

        if let Some(derive_attr) = mockable_attr.derive_attr {
            mock_struct.attrs.push(derive_attr.expand(&cx));
        }

        push(Annotatable::Item(P(mock_struct)));

        self.mock_trait_bound_impls(&trait_bound_resolver, &cx, &trait_decl);
    }
}

fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.expand())
        .unwrap_or_else(|| Ident::from_str(&format!("{}Mock", trait_decl.ident)))
}
