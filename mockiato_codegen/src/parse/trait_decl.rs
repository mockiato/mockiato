use syntax::ast::{GenericBounds, Generics, Ident, IsAuto, ItemKind, TraitItem, Unsafety};
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct TraitDecl {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) generic_bounds: GenericBounds,
    pub(crate) items: Vec<TraitItem>,
}

impl TraitDecl {
    pub(crate) fn parse(cx: &mut ExtCtxt, annotated: &Annotatable) -> Result<Self, ()> {
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

                if is_auto == &IsAuto::Yes {
                    cx.span_err(span, "#[mockable] does not work with auto traits");
                    return Err(());
                }

                return Ok(TraitDecl {
                    ident,
                    span,
                    generics: generics.clone(),
                    generic_bounds: generic_bounds.clone(),
                    items: items.clone(),
                });
            }
        }

        cx.span_err(annotated.span(), "#[mockable] can only be used with traits");
        Err(())
    }
}
