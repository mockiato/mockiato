use crate::constant::ATTR_NAME;
use crate::spanned::SpannedUnstable;
use crate::{Error, Result};
use proc_macro::Span;
use proc_macro::{Diagnostic, Level};
use syn::Meta;

#[derive(Debug)]
pub(crate) struct StaticAttr {
    pub(crate) span: Span,
}

impl StaticAttr {
    pub(crate) fn parse(meta_item: Meta) -> Result<Self> {
        let meta_item_span = meta_item.span_unstable();

        if let Meta::Word(_ident) = meta_item {
            return Ok(Self {
                span: meta_item_span,
            });
        }

        Err(Error::Diagnostic(
            Diagnostic::spanned(
                meta_item_span,
                Level::Error,
                format!("#[{}(static_references) does not take any parameters", ATTR_NAME),
            )
            .help(format!("Example usage: #[{}(static_references)]", ATTR_NAME)),
        ))
    }
}
