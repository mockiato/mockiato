use crate::generate::arguments_matcher::arguments_matcher_ident;
use crate::parse::method_decl::MethodDecl;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use syn::{Ident, ReturnType};

pub(crate) fn generate_mock_struct(
    trait_decl: &TraitDecl,
    mock_struct_ident: &Ident,
    mod_ident: &Ident,
) -> proc_macro2::TokenStream {
    let mock_method_fields: proc_macro2::TokenStream = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_method_field(method_decl, &mod_ident))
        .collect();

    quote! {
        #[derive(Debug)]
        struct #mock_struct_ident {
            #mock_method_fields
        }
    }
}

pub(crate) fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.ident)
        .unwrap_or_else(|| Ident::new(&format!("{}Mock", trait_decl.ident), trait_decl.span.into()))
}

fn generate_method_field(method_decl: &MethodDecl, mod_ident: &Ident) -> proc_macro2::TokenStream {
    let ident = &method_decl.ident;
    let arguments_matcher_ident = arguments_matcher_ident(ident);
    let return_type = match &method_decl.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    quote! {
        #ident: mockiato::internal::Method<self::#mod_ident::#arguments_matcher_ident, #return_type>,
    }
}
