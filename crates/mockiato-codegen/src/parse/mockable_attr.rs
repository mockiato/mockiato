use super::name_attr::NameAttr;
use super::static_attr::StaticAttr;
use crate::constant::ATTR_NAME;
use crate::diagnostic::DiagnosticBuilder;
use crate::result::{Error, Result};
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
        let mut name_attr = None;
        let mut static_attr = None;

        let meta_items: Vec<_> = args
            .into_iter()
            .map(|nested| match nested {
                NestedMeta::Meta(meta) => Ok(meta),
                NestedMeta::Literal(literal) => Err(unsupported_syntax_error(&literal)),
            })
            .collect();

        for item in meta_items {
            let item = item?;
            let item_name = item.name();

            if item_name == "name" {
                if name_attr.is_some() {
                    return Err(name_specified_more_than_once_error(&item));
                }
                name_attr = Some(NameAttr::parse(item)?);
            } else if item_name == "static_references" {
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
    let error_message = format!("`static_references` is specified more than once.");
    DiagnosticBuilder::error(meta_item.span(), error_message)
        .build()
        .into()
}

fn name_specified_more_than_once_error(meta_item: &Meta) -> Error {
    let error_message = format!("`name` should only be specified once");
    DiagnosticBuilder::error(meta_item.span(), error_message)
        .build()
        .into()
}

fn unsupported_syntax_error(literal: &Lit) -> Error {
    let error_message = format!("Unsupported syntax for #[{}]", ATTR_NAME);
    let help_message = format!("Example usage: #[{}(name = \"FooMock\")]", ATTR_NAME);
    DiagnosticBuilder::error(literal.span(), error_message)
        .help(help_message)
        .build()
        .into()
}
