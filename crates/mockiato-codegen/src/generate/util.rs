use proc_macro2::Span;
use syn::{parse_quote, Attribute, GenericParam, Ident, Lifetime, LifetimeDef, LitStr};

pub(super) fn doc_attribute(content: String) -> Attribute {
    let string_literal = LitStr::new(&content, Span::call_site());

    parse_quote! {
        #[doc = #string_literal]
    }
}

pub(super) fn ident_to_string_literal(ident: &Ident) -> LitStr {
    LitStr::new(&ident.to_string(), ident.span())
}

pub(super) fn lifetime_to_generic_param(lifetime: Lifetime) -> GenericParam {
    GenericParam::Lifetime(LifetimeDef::new(lifetime))
}
