use syntax::ast::{Attribute, Ident, MetaItem, MetaItemKind, NestedMetaItem, Path};
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct DeriveAttr {
    span: Span,
    list: Vec<NestedMetaItem>,
}

impl DeriveAttr {
    pub(crate) fn parse(cx: &mut ExtCtxt, meta_item: MetaItem) -> Option<Self> {
        if let MetaItemKind::List(list) = meta_item.node {
            Some(DeriveAttr {
                span: meta_item.span,
                list,
            })
        } else {
            cx.parse_sess
                .span_diagnostic
                .mut_span_err(meta_item.span, "#[mockable(derive(..)] must contain a list")
                .help("Example usage: #[mockable(derive(Eq, PartialEq))]")
                .emit();
            None
        }
    }

    pub(crate) fn expand(self, cx: &mut ExtCtxt) -> Attribute {
        cx.attribute(
            self.span,
            MetaItem {
                ident: Path::from_ident(Ident::from_str("derive")),
                node: MetaItemKind::List(self.list),
                span: self.span,
            },
        )
    }
}
