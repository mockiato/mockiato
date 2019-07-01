use crate::diagnostic::DiagnosticBuilder;
use crate::generate::{generate_mock, GenerateMockOptions};
use crate::parse::mockable_attr_parser::{
    MockableAttrParser, MockableAttrParserImpl, RemoteTraitPath,
};
use crate::parse::trait_decl::TraitDecl;
use crate::result::{Error, Result};
use crate::Controller;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{AttributeArgs, Item, ItemTrait};

#[derive(Debug, Default)]
pub(crate) struct ControllerImpl;

impl ControllerImpl {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Controller for ControllerImpl {
    fn expand_mockable_trait(&self, attr: AttributeArgs, item: Item) -> Result<TokenStream> {
        let mockable_attr_parser = MockableAttrParserImpl::new();
        let mockable_attr = mockable_attr_parser.parse(attr)?;
        let item_trait = extract_item_trait(item)?;
        let trait_decl = TraitDecl::parse(item_trait.clone()).map_err(add_note_to_error)?;

        let emit_item_trait = match mockable_attr.remote_trait_path {
            Some(_) => None,
            None => Some(item_trait),
        };

        let generated_mock = generate_mock(
            &trait_decl,
            GenerateMockOptions {
                custom_struct_ident: mockable_attr.name,
                force_static_lifetimes: mockable_attr.force_static_lifetimes,
                custom_trait_path: match mockable_attr.remote_trait_path {
                    Some(RemoteTraitPath::Path(path)) => Some(path),
                    _ => None,
                },
            },
        );

        Ok(quote! {
            #emit_item_trait
            #generated_mock
        })
    }
}

fn extract_item_trait(item: Item) -> Result<ItemTrait> {
    match item {
        Item::Trait(item_trait) => Ok(item_trait),
        _ => Err(only_traits_can_be_made_mockable_error(&item)),
    }
}

fn add_note_to_error(error: Error) -> Error {
    error
        .diagnostics
        .into_iter()
        .map(|diagnostic| {
            DiagnosticBuilder::from(diagnostic)
                .note_with_span(Span::call_site(), "Required for mockable traits")
                .build()
        })
        .collect()
}

fn only_traits_can_be_made_mockable_error(item: &Item) -> Error {
    DiagnosticBuilder::error(item.span(), "Only traits can be made mockable")
        .note_with_span(Span::call_site(), "Required because of this attribute")
        .build()
        .into()
}
