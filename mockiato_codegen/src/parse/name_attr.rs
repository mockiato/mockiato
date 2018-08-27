use syntax::ast::{Ident, LitKind, MetaItem, MetaItemKind};
use syntax::ext::base::ExtCtxt;
use syntax::source_map::Spanned;
use syntax::symbol::Symbol;
use syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct NameAttr {
    symbol_span: Span,
    symbol: Symbol,
}

impl NameAttr {
    pub(crate) fn parse(cx: &mut ExtCtxt, meta_item: MetaItem) -> Option<Self> {
        macro emit_error($cx: expr, $span:expr) {
            $cx.parse_sess
                .span_diagnostic
                .mut_span_err($span, "`name` should be a string literal")
                .help("Use something like #[mocked(name = \"FooMock\")]")
                .emit();
        };

        match meta_item.node {
            MetaItemKind::NameValue(Spanned { node, span }) => match node {
                LitKind::Str(symbol, _) => Some(Self {
                    symbol,
                    symbol_span: span,
                }),
                _ => {
                    emit_error!(cx, meta_item.span);
                    None
                }
            },
            _ => {
                emit_error!(cx, meta_item.span);
                None
            }
        }
    }

    pub(crate) fn expand(self) -> Ident {
        Ident::new(self.symbol, self.symbol_span)
    }
}
