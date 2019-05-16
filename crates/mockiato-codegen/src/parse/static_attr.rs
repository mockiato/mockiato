use crate::constant::{ATTR_NAME, STATIC_REFERENCES_ATTR_PARAM_NAME};
use crate::diagnostic::DiagnosticBuilder;
use crate::result::Result;
use syn::spanned::Spanned;
use syn::Meta;

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct StaticAttr;

impl StaticAttr {
    pub(crate) fn parse(meta_item: Meta) -> Result<Self> {
        let meta_item_span = meta_item.span();

        if let Meta::Word(_ident) = meta_item {
            return Ok(Self);
        }

        let error_message = format!(
            "#[{}({}) does not take any parameters",
            ATTR_NAME, STATIC_REFERENCES_ATTR_PARAM_NAME
        );
        let help_message = format!(
            "Correct usage: #[{}({})]",
            ATTR_NAME, STATIC_REFERENCES_ATTR_PARAM_NAME
        );
        let error = DiagnosticBuilder::error(meta_item_span, error_message)
            .help(help_message)
            .build()
            .into();
        Err(error)
    }
}
