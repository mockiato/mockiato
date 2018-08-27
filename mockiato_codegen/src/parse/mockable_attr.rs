use syntax::ast::{self, MetaItemKind, NestedMetaItemKind};
use syntax::ext::base::ExtCtxt;

use super::derive_attr::DeriveAttr;
use super::name_attr::NameAttr;

#[derive(Debug)]
pub(crate) struct MockableAttr {
    pub(crate) derive_attr: Option<DeriveAttr>,
    pub(crate) name_attr: Option<NameAttr>,
}

impl MockableAttr {
    pub(crate) fn parse(cx: &mut ExtCtxt, meta_item: &ast::MetaItem) -> Option<Self> {
        let mut derive_attr = None;
        let mut name_attr = None;

        match meta_item.node {
            MetaItemKind::Word => {}
            MetaItemKind::NameValue(..) => unreachable!(),
            MetaItemKind::List(ref list) => {
                let meta_items: Vec<_> = list
                    .iter()
                    .map(|nested| {
                        match nested.node {
                            NestedMetaItemKind::MetaItem(ref meta_item) => Some(meta_item),
                            NestedMetaItemKind::Literal(_) => {
                                // TODO: make more helpful
                                cx.parse_sess
                                    .span_diagnostic
                                    .mut_span_err(nested.span(), "Unsupported syntax for #[mockable]")
                                    .help("Example usage: #[mockable(name = \"FooMock\", derive(Debug))]")
                                    .emit();
                                None
                            }
                        }
                    }).collect();

                for item in meta_items {
                    match item {
                        Some(item) => if item.ident == "derive" {
                            if derive_attr.is_some() {
                                cx.span_warn(item.span(), "`derive` is specified more than once. The latter definition will take precedence.");
                            }
                            derive_attr = DeriveAttr::parse(cx, item.clone());
                        } else if item.ident == "name" {
                            if name_attr.is_some() {
                                cx.span_warn(item.span(), "`name` is specified more than once. The latter definition will take precedence.");
                            }
                            name_attr = NameAttr::parse(cx, item.clone());
                        } else {
                            cx.span_err(
                                item.span(),
                                "This attribute property is not supported by #[mockable]",
                            );
                            return None;
                        },
                        None => return None,
                    }
                }
            }
        }

        Some(Self {
            derive_attr,
            name_attr,
        })
    }
}
