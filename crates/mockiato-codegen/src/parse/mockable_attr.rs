use super::name_attr::NameAttr;
use super::static_attr::StaticAttr;
use crate::constant::{
    ATTR_NAME, MOCK_STRUCT_NAME_ATTR_PARAM_NAME, STATIC_REFERENCES_ATTR_PARAM_NAME,
};
use crate::diagnostic::DiagnosticBuilder;
use crate::result::{merge_results, Error, Result};
use syn::spanned::Spanned;
use syn::{AttributeArgs, Lit, Meta, NestedMeta};

/// The `#[mockable]` attribute, which is placed on a trait.
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MockableAttr {
    /// The name sub-attribute. Example: `#[name = "FooMock"]`
    /// This customizes the name of the generated mock struct.
    pub(crate) name_attr: Option<NameAttr>,
    /// The static sub-attribute. Example: `#[mockable(static)]`.
    /// Enforces that only static lifetimes are used within the mock.
    pub(crate) static_attr: Option<StaticAttr>,
}

impl MockableAttr {
    pub(crate) fn parse(args: AttributeArgs) -> Result<Self> {
        let meta_items = get_meta_items(args)?;

        let mut name_attr = None;
        let mut static_attr = None;

        for item in meta_items {
            let item_name = item.name();

            if item_name == MOCK_STRUCT_NAME_ATTR_PARAM_NAME {
                if name_attr.is_some() {
                    return Err(name_specified_more_than_once_error(&item));
                }
                name_attr = Some(NameAttr::parse(item)?);
            } else if item_name == STATIC_REFERENCES_ATTR_PARAM_NAME {
                if static_attr.is_some() {
                    return Err(static_references_specified_more_than_once_error(&item));
                }
                static_attr = Some(StaticAttr::parse(item)?);
            } else {
                return Err(attribute_property_not_supported_error(&item));
            }
        }

        Ok(Self {
            name_attr,
            static_attr,
        })
    }
}

fn get_meta_items(args: AttributeArgs) -> Result<impl Iterator<Item = Meta>> {
    let meta_items = args.into_iter().map(|nested| match nested {
        NestedMeta::Meta(meta) => Ok(meta),
        NestedMeta::Literal(literal) => Err(unsupported_syntax_error(&literal)),
    });
    merge_results(meta_items)
}

fn attribute_property_not_supported_error(meta_item: &Meta) -> Error {
    let error_message = format!(
        "This attribute property is not supported by #[{}]",
        ATTR_NAME
    );
    DiagnosticBuilder::error(meta_item.span(), error_message)
        .build()
        .into()
}

fn static_references_specified_more_than_once_error(meta_item: &Meta) -> Error {
    parameter_specified_more_than_once_error(STATIC_REFERENCES_ATTR_PARAM_NAME, meta_item)
}

fn name_specified_more_than_once_error(meta_item: &Meta) -> Error {
    parameter_specified_more_than_once_error(MOCK_STRUCT_NAME_ATTR_PARAM_NAME, meta_item)
}

fn parameter_specified_more_than_once_error(name: &str, meta_item: &Meta) -> Error {
    let error_message = format!("`{}` is specified more than once.", name);
    DiagnosticBuilder::error(meta_item.span(), error_message)
        .build()
        .into()
}

fn unsupported_syntax_error(literal: &Lit) -> Error {
    let error_message = format!("Unsupported syntax for #[{}]", ATTR_NAME);
    let help_message = format!(
        "Example usage: #[{attr}({param} = \"FooMock\")]",
        attr = ATTR_NAME,
        param = MOCK_STRUCT_NAME_ATTR_PARAM_NAME
    );
    DiagnosticBuilder::error(literal.span(), error_message)
        .help(help_message)
        .build()
        .into()
}
