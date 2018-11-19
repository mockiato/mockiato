use crate::generate::arguments_matcher::arguments_matcher_ident;
use crate::parse::method_decl::MethodDecl;
use crate::parse::name_attr::NameAttr;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::{Span, TokenStream};
use syn::{Ident, LitStr, ReturnType};

pub(crate) fn generate_mock_struct(
    trait_decl: &TraitDecl,
    mock_struct_ident: &Ident,
    mod_ident: &Ident,
) -> TokenStream {
    let method_fields: TokenStream = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_method_field(method_decl, &mod_ident))
        .collect();

    let initializer_fields: TokenStream = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_initializer_field(&method_decl.ident, &mock_struct_ident))
        .collect();

    quote! {
        #[derive(Debug)]
        struct #mock_struct_ident {
            #method_fields
        }

        impl #mock_struct_ident {
            fn new() -> Self {
                Self {
                    #initializer_fields
                }
            }
        }
    }
}

pub(crate) fn mock_struct_ident(trait_decl: &TraitDecl, name_attr: Option<NameAttr>) -> Ident {
    name_attr
        .map(|attr| attr.ident)
        .unwrap_or_else(|| Ident::new(&format!("{}Mock", trait_decl.ident), trait_decl.span.into()))
}

fn generate_method_field(method_decl: &MethodDecl, mod_ident: &Ident) -> TokenStream {
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

fn generate_initializer_field(method_ident: &Ident, mock_struct_ident: &Ident) -> TokenStream {
    let name = LitStr::new(
        &format!(
            "{}::{}",
            mock_struct_ident.to_string(),
            method_ident.to_string()
        ),
        Span::call_site(),
    );

    quote! {
        #method_ident: mockiato::internal::Method::new(#name),
    }
}
