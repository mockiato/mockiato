use super::{MockableAttr, MockableAttrParser};
use crate::constant::{
    ATTR_NAME, MOCK_STRUCT_NAME_ATTR_PARAM_NAME, STATIC_REFERENCES_ATTR_PARAM_NAME,
};
use crate::diagnostic::DiagnosticBuilder;
use crate::result::{merge_results, Error, Result};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{AttributeArgs, Ident, Lit, Meta, MetaNameValue, NestedMeta};

#[derive(Default)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MockableAttrParserImpl;

impl MockableAttrParserImpl {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl MockableAttrParser for MockableAttrParserImpl {
    fn parse(&self, args: AttributeArgs) -> Result<MockableAttr> {
        let meta_items = get_meta_items(args)?;

        let mut name = None;
        let mut force_static_lifetimes = false;

        for item in meta_items {
            let item_name = item.name();

            if item_name == MOCK_STRUCT_NAME_ATTR_PARAM_NAME {
                if name.is_some() {
                    return Err(name_specified_more_than_once_error(&item));
                }
                name = Some(parse_name_property(item)?);
            } else if item_name == STATIC_REFERENCES_ATTR_PARAM_NAME {
                if force_static_lifetimes {
                    return Err(static_references_specified_more_than_once_error(&item));
                }
                validate_static_references_property(&item)?;
                force_static_lifetimes = true;
            } else {
                return Err(attribute_property_not_supported_error(&item));
            }
        }

        Ok(MockableAttr {
            name,
            force_static_lifetimes,
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

fn parse_name_property(meta_item: Meta) -> Result<Ident> {
    let meta_item_span = meta_item.span();

    if let Meta::NameValue(MetaNameValue { lit, .. }) = meta_item {
        if let Lit::Str(str_lit) = lit {
            return Ok(Ident::new(&str_lit.value(), str_lit.span()));
        }
    }

    Err(invalid_name_property_syntax_error(meta_item_span))
}

fn invalid_name_property_syntax_error(span: Span) -> Error {
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
    DiagnosticBuilder::error(span, error_message)
        .help(help_message)
        .build()
        .into()
}

fn validate_static_references_property(meta_item: &Meta) -> Result<()> {
    let meta_item_span = meta_item.span();

    if let Meta::Word(_ident) = meta_item {
        Ok(())
    } else {
        Err(invalid_static_references_property_syntax_error(
            meta_item_span,
        ))
    }
}

fn invalid_static_references_property_syntax_error(span: Span) -> Error {
    let error_message = format!(
        "#[{}({}) does not take any parameters",
        ATTR_NAME, STATIC_REFERENCES_ATTR_PARAM_NAME
    );
    let help_message = format!(
        "Correct usage: #[{}({})]",
        ATTR_NAME, STATIC_REFERENCES_ATTR_PARAM_NAME
    );
    DiagnosticBuilder::error(span, error_message)
        .help(help_message)
        .build()
        .into()
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
