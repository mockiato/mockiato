use super::name_attr::NameAttr;
use super::static_attr::StaticAttr;
use crate::constant::ATTR_NAME;
use crate::spanned::SpannedUnstable;
use crate::{Error, Result};
use proc_macro::{Diagnostic, Level};
use syn::{AttributeArgs, NestedMeta};

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
                NestedMeta::Literal(lit) => Err(Error::Diagnostic(
                    Diagnostic::spanned(
                        lit.span_unstable(),
                        Level::Error,
                        format!("Unsupported syntax for #[{}]", ATTR_NAME),
                    )
                    .help(format!(
                        "Example usage: #[{}(name = \"FooMock\")]",
                        ATTR_NAME
                    )),
                )),
            })
            .collect();

        for item in meta_items {
            let item = item?;
            let item_name = item.name();

            if item_name == "name" {
                if name_attr.is_some() {
                    Diagnostic::spanned(item.span_unstable(), Level::Warning, "`name` is specified more than once. The latter definition will take precedence.").emit();
                }
                name_attr = Some(NameAttr::parse(item)?);
            } else if item_name == "static_references" {
                if static_attr.is_some() {
                    Diagnostic::spanned(
                        item.span_unstable(),
                        Level::Warning,
                        "`static_references` is specified more than once.",
                    )
                    .emit();
                }
                static_attr = Some(StaticAttr::parse(item)?);
            } else {
                return Err(Error::Diagnostic(Diagnostic::spanned(
                    item.span_unstable(),
                    Level::Error,
                    format!(
                        "This attribute property is not supported by #[{}]",
                        ATTR_NAME
                    ),
                )));
            }
        }

        Ok(Self {
            name_attr,
            static_attr,
        })
    }
}
