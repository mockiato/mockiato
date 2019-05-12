use crate::constant::{ATTR_NAME, MOCK_STRUCT_NAME_ATTR_PARAM_NAME};
use crate::diagnostic::DiagnosticBuilder;
use crate::result::Result;
use syn::spanned::Spanned;
use syn::{Ident, Lit, Meta, MetaNameValue};

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct NameAttr {
    pub(crate) ident: Ident,
}

impl NameAttr {
    pub(crate) fn parse(meta_item: Meta) -> Result<Self> {
        let meta_item_span = meta_item.span();

        if let Meta::NameValue(MetaNameValue { lit, .. }) = meta_item {
            if let Lit::Str(str_lit) = lit {
                return Ok(Self {
                    ident: Ident::new(&str_lit.value(), str_lit.span()),
                });
            }
        }

        let error_message = format!(
            "#[{attr}({param} = \"...\") expects a string literal",
            attr = ATTR_NAME,
            param = MOCK_STRUCT_NAME_ATTR_PARAM_NAME
        );
        let help_message = format!(
            "Example usage: #[{attr}({param} = \"FooMock\")]",
            attr = ATTR_NAME,
            param = MOCK_STRUCT_NAME_ATTR_PARAM_NAME
        );
        let error = DiagnosticBuilder::error(meta_item_span, error_message)
            .help(help_message)
            .build()
            .into();
        Err(error)
    }
}
