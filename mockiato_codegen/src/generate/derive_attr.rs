use crate::syntax::ast::Attribute;
use crate::syntax::ext::build::AstBuilder;
use crate::syntax::source_map::symbol::Symbol;
use crate::syntax_pos::DUMMY_SP;

use crate::context::Context;
use crate::parse::trait_bounds::TraitBounds;
use crate::parse::trait_decl::TraitDecl;
use crate::trait_bound_resolver::{TraitBoundResolver, TraitBoundType};

const DERIVE_ATTRIBUTE_NAME: &str = "derive";

pub(crate) struct DeriveAttributeGenerator<'a, 'ext> {
    context: Context<'a, 'ext>,
    trait_bound_resolver: Box<TraitBoundResolver + 'a>,
}

impl<'a, 'ext> DeriveAttributeGenerator<'a, 'ext> {
    pub(crate) fn new(
        context: Context<'a, 'ext>,
        trait_bound_resolver: Box<TraitBoundResolver + 'a>,
    ) -> Self {
        Self {
            context,
            trait_bound_resolver,
        }
    }

    pub(crate) fn generate_for_trait(&self, trait_decl: &TraitDecl) -> Result<Attribute, ()> {
        let trait_bounds = self.resolve_trait_bounds(trait_decl);
        let inner_context = self.context.into_inner();

        if trait_bounds.iter().any(Option::is_none) {
            return Err(());
        }

        let list_items = trait_bounds
            .into_iter()
            .map(Option::unwrap)
            .map(|resolved| {
                let TraitBoundType::Derivable(path) = resolved;
                let ident = path.segments.first().unwrap().ident;

                inner_context.meta_list_item_word(ident.span, ident.name)
            })
            .collect();

        let list =
            inner_context.meta_list(DUMMY_SP, Symbol::intern(DERIVE_ATTRIBUTE_NAME), list_items);

        Ok(inner_context.attribute(DUMMY_SP, list))
    }

    fn resolve_trait_bounds(&self, trait_decl: &TraitDecl) -> Vec<Option<TraitBoundType>> {
        let trait_bounds = TraitBounds::parse(trait_decl).0;

        trait_bounds
            .iter()
            .map(|trait_bound| {
                let resolved = self.trait_bound_resolver.resolve_trait_bound(&trait_bound.path);

                if resolved.is_none() {
                    self.context.into_inner().parse_sess
                      .span_diagnostic
                      .mut_span_err(trait_bound.span, "The referenced trait is not a derivable trait")
                      .help("Only traits that are automatically derivable are supported as supertrait")
                      .emit();
                }

                resolved
            })
            .collect()
    }
}
