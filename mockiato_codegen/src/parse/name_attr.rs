use crate::constant::ATTR_NAME;
use crate::spanned::SpannedUnstable;
use crate::{Error, Result};
use proc_macro::Span;
use proc_macro::{Diagnostic, Level};
use syn::{Ident, Lit, Meta, MetaNameValue};

#[derive(Debug)]
pub(crate) struct NameAttr {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

impl NameAttr {
    pub(crate) fn parse(meta_item: Meta) -> Result<Self> {
        let meta_item_span = meta_item.span_unstable();

        if let Meta::NameValue(MetaNameValue { lit, .. }) = meta_item {
            if let Lit::Str(str_lit) = lit {
                return Ok(Self {
                    ident: Ident::new(&str_lit.value(), str_lit.span()),
                    span: str_lit.span().unstable(),
                });
            }
        }

        Err(Error::Diagnostic(
            Diagnostic::spanned(
                meta_item_span,
                Level::Error,
                format!("#[{}(name = \"...\") expects a string literal", ATTR_NAME),
            )
            .help(format!(
                "Example usage: #[{}(name = \"FooMock\")]",
                ATTR_NAME,
            )),
        ))
    }
}
