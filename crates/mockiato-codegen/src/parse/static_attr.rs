use crate::constant::ATTR_NAME;
use crate::{DiagnosticBuilder, Result};
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
            "#[{}(static_references) does not take any parameters",
            ATTR_NAME
        );
        let help_message = format!("Example usage: #[{}(static_references)]", ATTR_NAME);
        let error = DiagnosticBuilder::error(meta_item_span, error_message)
            .help(help_message)
            .build()
            .into();
        Err(error)
    }
}
