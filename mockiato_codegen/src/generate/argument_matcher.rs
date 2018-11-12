use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use heck::CamelCase;
use proc_macro2::TokenStream;
use syn::Ident;

pub(crate) fn generate_argument_matcher(method_decl: &MethodDecl) -> TokenStream {
    let argument_matcher_ident = argument_matcher_ident(&method_decl.ident);
    let argument_matcher_fields = argument_matcher_fields(&method_decl.inputs);

    quote! {
        pub(super) struct #argument_matcher_ident<'args> {
            #argument_matcher_fields
        }

        impl<'args> std::fmt::Debug for #argument_matcher_ident<'args> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // TODO
                unimplemented!()
            }
        }

        impl<'args> mockiato::internal::Arguments for #argument_matcher_ident<'args> {}
    }
}

fn argument_matcher_ident(method_ident: &Ident) -> Ident {
    Ident::new(
        &format!(
            "{}ArgumentsMatcher",
            method_ident.to_string().to_camel_case()
        ),
        method_ident.span(),
    )
}

fn argument_matcher_fields(method_inputs: &MethodInputs) -> TokenStream {
    method_inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            let ty = &input.ty;

            TokenStream::from(quote!{
                pub(super) #ident: #ty,
            })
        })
        .collect()
}
