use super::{MockableAttr, MockableAttrParser, RemoteTraitPath};
use crate::constant::{
    ATTR_NAME, MOCK_STRUCT_NAME_ATTR_PARAM_NAME, REMOTE_ATTR_PARAM_NAME,
    STATIC_REFERENCES_ATTR_PARAM_NAME,
};
use crate::diagnostic::DiagnosticBuilder;
use crate::result::{merge_results, Error, Result};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{AttributeArgs, Ident, Lit, Meta, MetaNameValue, NestedMeta};

#[derive(Default, Debug)]
pub(crate) struct MockableAttrParserImpl;

impl MockableAttrParserImpl {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl MockableAttrParser for MockableAttrParserImpl {
    fn parse(&self, args: AttributeArgs) -> Result<MockableAttr> {
        get_meta_items(args)?.try_fold(MockableAttr::default(), |mockable_attr, item| {
            parse_meta_item(mockable_attr, item)
        })
    }
}

fn parse_meta_item(mockable_attr: MockableAttr, item: Meta) -> Result<MockableAttr> {
    match item.name() {
        ref item_name if item_name == MOCK_STRUCT_NAME_ATTR_PARAM_NAME => {
            parse_name_meta_item(mockable_attr, item)
        }
        ref item_name if item_name == STATIC_REFERENCES_ATTR_PARAM_NAME => {
            parse_static_references_meta_item(mockable_attr, item)
        }
        ref item_name if item_name == REMOTE_ATTR_PARAM_NAME => {
            parse_remote_meta_item(mockable_attr, item)
        }
        _ => Err(attribute_property_not_supported_error(&item)),
    }
}

fn parse_name_meta_item(mockable_attr: MockableAttr, item: Meta) -> Result<MockableAttr> {
    if mockable_attr.name.is_some() {
        Err(name_specified_more_than_once_error(&item))
    } else {
        let name = Some(parse_name_property(item)?);
        Ok(MockableAttr {
            name,
            ..mockable_attr
        })
    }
}

fn parse_static_references_meta_item(
    mockable_attr: MockableAttr,
    item: Meta,
) -> Result<MockableAttr> {
    if mockable_attr.force_static_lifetimes {
        Err(static_references_specified_more_than_once_error(&item))
    } else {
        validate_static_references_property(&item)?;
        Ok(MockableAttr {
            force_static_lifetimes: true,
            ..mockable_attr
        })
    }
}

fn parse_remote_meta_item(mockable_attr: MockableAttr, item: Meta) -> Result<MockableAttr> {
    if mockable_attr.remote_trait_path.is_some() {
        Err(parameter_specified_more_than_once_error(
            REMOTE_ATTR_PARAM_NAME,
            &item,
        ))
    } else {
        let remote_trait_path = Some(parse_remote_property(item)?);
        Ok(MockableAttr {
            remote_trait_path,
            ..mockable_attr
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

fn parse_remote_property(meta_item: Meta) -> Result<RemoteTraitPath> {
    let meta_item_span = meta_item.span();

    match meta_item {
        Meta::Word(_) => Ok(RemoteTraitPath::SameAsLocalIdent),
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(str_lit),
            ..
        }) => {
            let path = str_lit
                .parse()
                .map_err(|err| invalid_remote_property_syntax_error(err.span()))?;
            Ok(RemoteTraitPath::Path(path))
        }
        _ => Err(invalid_remote_property_syntax_error(meta_item_span)),
    }
}

fn invalid_remote_property_syntax_error(span: Span) -> Error {
    let error_message = format!(
        "#[{attr}({param} = \"...\") must be a valid path",
        attr = ATTR_NAME,
        param = REMOTE_ATTR_PARAM_NAME
    );
    let help_message = format!(
        "Example usage: #[{attr}({param} = \"io::Write\")]",
        attr = ATTR_NAME,
        param = REMOTE_ATTR_PARAM_NAME
    );
    DiagnosticBuilder::error(span, error_message)
        .help(help_message)
        .build()
        .into()
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
