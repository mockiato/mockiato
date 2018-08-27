use syntax::ast::{GenericBounds, Generics, Ident, IsAuto, ItemKind, TraitItem, Unsafety};
use syntax::ext::base::Annotatable;
use syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct TraitDecl<'a> {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) is_auto: &'a IsAuto,
    pub(crate) unsafety: &'a Unsafety,
    pub(crate) generics: &'a Generics,
    pub(crate) generic_bounds: &'a GenericBounds,
    pub(crate) items: &'a [TraitItem],
}

impl<'a> TraitDecl<'a> {
    pub(crate) fn parse(annotated: &'a Annotatable) -> Result<Self, Span> {
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

        Err(annotated.span())
    }
}
