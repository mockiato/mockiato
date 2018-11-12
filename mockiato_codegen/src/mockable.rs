use crate::generate::argument_matcher::generate_argument_matcher;
use crate::parse::mockable_attr::MockableAttr;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use crate::spanned::SpannedUnstable;
use crate::Error;
use heck::SnakeCase;
use proc_macro::{Diagnostic, Level, Span, TokenStream};
use syn::{AttributeArgs, Ident, Item, ItemTrait};

#[derive(Default)]
pub(crate) struct Mockable;

impl Mockable {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn expand(&self, attr: AttributeArgs, item: Item) -> TokenStream {
        macro try_or_return($expr: expr) {
            match $expr {
                Ok(value) => value,
                Err(_) => return TokenStream::new(),
            }
        }

        let mockable_attr = try_or_return!(MockableAttr::parse(attr).map_err(|err| err.emit()));
        let item_trait = try_or_return!(extract_item_trait(item).map_err(|err| err.emit()));
        let trait_decl = try_or_return!(TraitDecl::parse(item_trait.clone())
            .map_err(|err| err
                .emit_with(|d| d.span_note(Span::call_site(), "Required for mockable traits"))));

        let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);
        let mod_ident = mod_ident(&mock_struct_ident);

        let argument_matchers: proc_macro2::TokenStream = trait_decl
            .methods
            .iter()
            .map(generate_argument_matcher)
            .collect();

        TokenStream::from(quote! {
            #item_trait

            #[derive(Debug)]
            struct #mock_struct_ident;

            mod #mod_ident {
                use super::*;

                #argument_matchers
            }
        })
    }
}

fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.ident)
        .unwrap_or_else(|| Ident::new(&format!("{}Mock", trait_decl.ident), trait_decl.span.into()))
}

fn mod_ident(ident: &Ident) -> Ident {
    Ident::new(&ident.to_string().to_snake_case(), ident.span())
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
