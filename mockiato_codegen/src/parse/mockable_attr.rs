use crate::context::Context;
use crate::syntax::ast::{self, MetaItemKind, NestedMetaItemKind};

use super::name_attr::NameAttr;

#[derive(Debug)]
pub(crate) struct MockableAttr {
    pub(crate) name_attr: Option<NameAttr>,
}

impl MockableAttr {
    pub(crate) fn parse(cx: &Context, meta_item: &ast::MetaItem) -> Option<Self> {
        let mut name_attr = None;

        match meta_item.node {
            MetaItemKind::Word => {}
            MetaItemKind::NameValue(..) => unreachable!(),
            MetaItemKind::List(ref list) => {
                let meta_items: Vec<_> = list
                    .iter()
                    .map(|nested| match nested.node {
                        NestedMetaItemKind::MetaItem(ref meta_item) => Some(meta_item),
                        NestedMetaItemKind::Literal(_) => {
                            cx.into_inner()
                                .parse_sess
                                .span_diagnostic
                                .mut_span_err(nested.span(), "Unsupported syntax for #[mockable]")
                                .help("Example usage: #[mockable(name = \"FooMock\")]")
                                .emit();
                            None
                        }
                    })
                    .collect();

                for item in meta_items {
                    match item {
                        Some(item) => {
                            if item.ident == "name" {
                                if name_attr.is_some() {
                                    cx.into_inner().span_warn(item.span(), "`name` is specified more than once. The latter definition will take precedence.");
                                }
                                name_attr = NameAttr::parse(&cx, item.clone());
                            } else {
                                cx.into_inner().span_err(
                                    item.span(),
                                    "This attribute property is not supported by #[mockable]",
                                );
                                return None;
                            }
                        }
                        None => return None,
                    }
                }
            }
        }

        Some(Self { name_attr })
    }
}
