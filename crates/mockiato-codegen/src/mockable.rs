use crate::generate::{generate_mock, GenerateMockOptions};
use crate::parse::mockable_attr::MockableAttr;
use crate::parse::trait_decl::TraitDecl;
use crate::spanned::SpannedUnstable;
use crate::Error;
use proc_macro::{Diagnostic, Level, Span, TokenStream};
use quote::quote;
use syn::{AttributeArgs, Item, ItemTrait};

#[derive(Default)]
pub(crate) struct Mockable;

impl Mockable {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn expand(&self, attr: AttributeArgs, item: Item) -> TokenStream {
        let item_backup = item.clone();

        #[doc(hidden)]
        macro try_or_return($expr: expr) {
            match $expr {
                Ok(value) => value,
                Err(_) => return TokenStream::from(quote! { #item_backup }),
            }
        }

        let mockable_attr = try_or_return!(MockableAttr::parse(attr).map_err(Error::emit));
        let item_trait = try_or_return!(extract_item_trait(item).map_err(Error::emit));
        let trait_decl = try_or_return!(TraitDecl::parse(item_trait.clone())
            .map_err(|err| err
                .emit_with(|d| d.span_note(Span::call_site(), "Required for mockable traits"))));

        let generated_mock = generate_mock(
            &trait_decl,
            GenerateMockOptions {
                custom_struct_ident: mockable_attr.name_attr.map(|attr| attr.ident),
                force_static_lifetimes: mockable_attr.static_attr.is_some(),
            },
        );

        TokenStream::from(quote! {
            #item_trait
            #generated_mock
        })
    }
}

fn extract_item_trait(item: Item) -> Result<ItemTrait, Error> {
    match item {
        Item::Trait(item_trait) => Ok(item_trait),
        _ => Err(Error::Diagnostic(
            Diagnostic::spanned(
                item.span_unstable(),
                Level::Error,
                "Only traits can be made mockable",
            )
            .span_note(Span::call_site(), "Required because of this attribute"),
        )),
    }
}
