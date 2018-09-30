use crate::constant::ATTR_NAME;
use crate::context::Context;
use crate::syntax::ast::{Ident, LitKind, MetaItem, MetaItemKind};
use crate::syntax::source_map::Spanned;
use crate::syntax::symbol::Symbol;
use crate::syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct NameAttr {
    symbol_span: Span,
    symbol: Symbol,
}

impl NameAttr {
    pub(crate) fn parse(cx: &Context, meta_item: MetaItem) -> Option<Self> {
        if let MetaItemKind::NameValue(Spanned { node, span }) = meta_item.node {
            if let LitKind::Str(symbol, _) = node {
                return Some(Self {
                    symbol,
                    symbol_span: span,
                });
            }
        }

        cx.into_inner()
            .parse_sess
            .span_diagnostic
            .mut_span_err(
                meta_item.span,
                &format!("#[{}(name = \"...\") expects a string literal", ATTR_NAME),
            )
            .help(&format!(
                "Example usage: #[{}(name = \"FooMock\")]",
                ATTR_NAME,
            ))
            .emit();

        None
    }

    pub(crate) fn expand(self) -> Ident {
        Ident::new(self.symbol, self.symbol_span)
    }
}
