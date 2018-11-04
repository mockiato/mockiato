use crate::parse::mockable_attr::MockableAttr;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use crate::Error;
use proc_macro::{Diagnostic, Level, Span, TokenStream};
use syn::spanned::Spanned;
use syn::{AttributeArgs, Ident, Item, ItemTrait};

#[derive(Default)]
pub(crate) struct Mockable;

impl Mockable {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn expand(&self, attr: AttributeArgs, item: Item) -> TokenStream {
        macro early_return() {
            return TokenStream::new();
        }

        let mockable_attr = match MockableAttr::parse(attr).map_err(|err| err.emit(|d| d)) {
            Ok(mockable_attr) => mockable_attr,
            Err(_) => return TokenStream::new(),
        };

        let item_trait = match expect_item_trait(item).map_err(|err| err.emit(|d| d)) {
            Ok(trait_decl) => trait_decl,
            Err(_) => early_return!(),
        };

        let trait_decl = match TraitDecl::parse(item_trait.clone()).map_err(|err| {
            err.emit(|d| d.span_note(Span::call_site(), "Required for mockable traits"))
        }) {
            Ok(trait_decl) => trait_decl,
            Err(_) => early_return!(),
        };

        let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);

        TokenStream::from(quote! {
            #item_trait

            #[derive(Debug)]
            struct #mock_struct_ident;
        })
    }
}

fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.ident)
        .unwrap_or_else(|| Ident::new(&format!("{}Mock", trait_decl.ident), trait_decl.span.into()))
}

fn expect_item_trait(item: Item) -> Result<ItemTrait, Error> {
    match item {
        Item::Trait(item_trait) => Ok(item_trait),
        _ => Err(Error::Diagnostic(
            Diagnostic::spanned(
                item.span().unstable(),
                Level::Error,
                "Only traits can be made mockable",
            )
            .span_note(Span::call_site(), "Required because of this attribute"),
        )),
    }
}
