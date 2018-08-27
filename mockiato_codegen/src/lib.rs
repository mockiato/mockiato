#![crate_type = "dylib"]
#![feature(
    quote,
    concat_idents,
    plugin_registrar,
    rustc_private,
    decl_macro
)]
#![feature(custom_attribute)]

extern crate rustc_plugin;
extern crate rustc_resolve;
extern crate syntax;
extern crate syntax_pos;

use rustc_plugin::Registry;
use syntax::ast::{
    self, Attribute, GenericBounds, Generics, Ident, IsAuto, ItemKind, LitKind, MetaItem,
    MetaItemKind, NestedMetaItem, NestedMetaItemKind, Path, TraitItem, Unsafety, VariantData,
    DUMMY_NODE_ID,
};
use syntax::ext::base::{Annotatable, ExtCtxt, MultiItemDecorator, SyntaxExtension};
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;
use syntax::source_map::Spanned;
use syntax::symbol::Symbol;
use syntax_pos::Span;

#[derive(Debug)]
struct TraitDecl<'a> {
    span: Span,
    ident: Ident,
    is_auto: &'a IsAuto,
    unsafety: &'a Unsafety,
    generics: &'a Generics,
    generic_bounds: &'a GenericBounds,
    items: &'a [TraitItem],
}

#[derive(Debug)]
struct MockableAttr {
    derive_attr: Option<DeriveAttr>,
    name_attr: Option<NameAttr>,
}

#[derive(Debug)]
struct DeriveAttr {
    span: Span,
    list: Vec<NestedMetaItem>,
}

#[derive(Debug)]
struct NameAttr {
    symbol_span: Span,
    symbol: Symbol,
}

impl<'a> TraitDecl<'a> {
    fn parse(annotated: &'a Annotatable) -> Result<Self, Span> {
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

        return Err(annotated.span());
    }
}

impl MockableAttr {
    fn parse(cx: &mut ExtCtxt, meta_item: &ast::MetaItem) -> Option<Self> {
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
                                    .mut_span_err(nested.span(), "Unsupported syntax for #[mocked]")
                                    .help("Use something like #[mocked(name = \"FooMock\", derive(Debug))]")
                                    .emit();
                                None
                            }
                        }
                    }).collect();

                for item in meta_items {
                    match item {
                        Some(item) => if item.ident == "derive" {
                            if derive_attr.is_some() {
                                cx.span_warn(item.span(), "`derive` is specified more than once. The latter definition will overwrite the former.");
                            }
                            derive_attr = DeriveAttr::parse(cx, item.clone());
                        } else if item.ident == "name" {
                            if name_attr.is_some() {
                                cx.span_warn(item.span(), "`name` is specified more than once. The latter definition will overwrite the former.");
                            }
                            name_attr = NameAttr::parse(cx, item.clone());
                        } else {
                            cx.span_err(item.span(), "Syntax not supported");
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

impl DeriveAttr {
    fn parse(cx: &mut ExtCtxt, meta_item: MetaItem) -> Option<Self> {
        if let MetaItemKind::List(list) = meta_item.node {
            Some(DeriveAttr {
                span: meta_item.span,
                list,
            })
        } else {
            // TODO: make more helpful
            cx.span_err(meta_item.span(), "Syntax not supported");
            None
        }
    }

    fn expand(self, cx: &mut ExtCtxt) -> Attribute {
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

impl NameAttr {
    fn parse(cx: &mut ExtCtxt, meta_item: MetaItem) -> Option<Self> {
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
}

struct Mockable;

impl MultiItemDecorator for Mockable {
    fn expand(
        &self,
        cx: &mut ExtCtxt,
        _sp: Span,
        meta_item: &ast::MetaItem,
        item: &Annotatable,
        push: &mut dyn FnMut(Annotatable),
    ) {
        let trait_decl = match TraitDecl::parse(item) {
            Ok(trait_decl) => trait_decl,
            Err(span) => {
                cx.span_err(span, "#[mockable] can only be used on traits");
                return;
            }
        };

        if trait_decl.unsafety == &Unsafety::Unsafe {
            cx.span_err(item.span(), "#[mockable] does not support unsafe traits");
            return;
        }

        let mockable_attr = match MockableAttr::parse(cx, meta_item) {
            Some(mockable_attr) => mockable_attr,
            None => return,
        };

        let mock_struct_ident = mockable_attr
            .name_attr
            .map(|attr| Ident::new(attr.symbol, attr.symbol_span))
            .unwrap_or_else(|| Ident::from_str(&format!("{}Mock", trait_decl.ident)));

        let mut mock_struct = cx
            .item_struct(
                meta_item.span,
                mock_struct_ident,
                VariantData::Unit(DUMMY_NODE_ID),
            ).into_inner();

        if let Some(derive_attr) = mockable_attr.derive_attr {
            mock_struct.attrs.push(derive_attr.expand(cx));
        }

        push(Annotatable::Item(P(mock_struct)));
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Symbol::intern("mockable"),
        SyntaxExtension::MultiDecorator(Box::new(Mockable)),
    );
}
