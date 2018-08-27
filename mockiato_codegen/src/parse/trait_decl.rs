use syntax::ast::{GenericBounds, Generics, Ident, IsAuto, ItemKind, TraitItem, Unsafety};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct TraitDecl<'a> {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) is_auto: &'a IsAuto,
    pub(crate) generics: &'a Generics,
    pub(crate) generic_bounds: &'a GenericBounds,
    pub(crate) items: &'a [TraitItem],
}

impl<'a> TraitDecl<'a> {
    pub(crate) fn parse(cx: &mut ExtCtxt, annotated: &'a Annotatable) -> Result<Self, ()> {
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
                if unsafety == &Unsafety::Unsafe {
                    cx.span_err(span, "#[mockable] does not work with unsafe traits");
                    return Err(());
                }

                return Ok(TraitDecl {
                    ident,
                    span,
                    is_auto,
                    generics,
                    generic_bounds,
                    items,
                });
            }
        }

        cx.span_err(annotated.span(), "#[mockable] can only be used with traits");
        Err(())
    }
}
