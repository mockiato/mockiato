use crate::parse::mockable_attr::MockableAttr;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use proc_macro::TokenStream;
use syn::{AttributeArgs, Ident, Item};

#[derive(Default)]
pub(crate) struct Mockable;

impl Mockable {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn expand(&self, attr: AttributeArgs, item: Item) -> TokenStream {
        let trait_decl = match TraitDecl::parse(item).map_err(|err| err.emit(|d| d)) {
            Ok(trait_decl) => trait_decl,
            Err(_) => return TokenStream::new(),
        };

        let mockable_attr = match MockableAttr::parse(attr).map_err(|err| err.emit(|d| d)) {
            Ok(mockable_attr) => mockable_attr,
            Err(_) => return TokenStream::new(),
        };

        let mock_struct_ident = mock_struct_ident(&trait_decl, mockable_attr.name_attr);

        TokenStream::from(quote! {
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
