use super::constant::{arguments_ident, arguments_lifetime};
use super::lifetime_rewriter::{LifetimeRewriter, UniformLifetimeGenerator};
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use proc_macro2::TokenStream;
use syn::visit_mut::visit_type_mut;
use syn::Ident;

pub(crate) struct GeneratedArguments {
    pub(crate) output: TokenStream,
    pub(crate) generics: TokenStream,
    pub(crate) ident: Ident,
}

pub(crate) fn generate_arguments(method_decl: &MethodDecl) -> GeneratedArguments {
    let arguments_ident = arguments_ident(&method_decl.ident);

    let mut lifetime_rewriter = LifetimeRewriter::new(UniformLifetimeGenerator::default());
    let arguments_fields = generate_arguments_fields(&mut lifetime_rewriter, &method_decl.inputs);

    let generics = if lifetime_rewriter.generator.has_lifetimes {
        generics()
    } else {
        TokenStream::new()
    };

    let debug_impl = generate_debug_impl(method_decl, &generics);

    GeneratedArguments {
        generics: generics.clone(),
        ident: arguments_ident.clone(),
        output: quote! {
            #[doc(hidden)]
            pub struct #arguments_ident #generics {
                #arguments_fields
            }

            #debug_impl

            impl #generics mockiato::internal::Arguments for #arguments_ident #generics {}
        },
    }
}

/// Generates the generics clause (including angled brackets) for the arguments struct.
fn generics() -> TokenStream {
    let lifetime = arguments_lifetime();

    quote! {
        <#lifetime>
    }
}

/// Generates a `Debug` implementation for an arguments struct.
fn generate_debug_impl(method_decl: &MethodDecl, generics: &TokenStream) -> TokenStream {
    let arguments_ident = arguments_ident(&method_decl.ident);

    let debug_fields: TokenStream = method_decl
        .inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            quote! { .field(&mockiato::internal::MaybeDebugWrapper(&self.#ident)) }
        })
        .collect();

    quote! {
        impl #generics std::fmt::Debug for #arguments_ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("")
                  #debug_fields
                 .finish()
            }
        }
    }
}

fn generate_arguments_fields(
    lifetime_rewriter: &mut LifetimeRewriter<UniformLifetimeGenerator>,
    method_inputs: &MethodInputs,
) -> TokenStream {
    method_inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            let mut ty = input.ty.clone();

            visit_type_mut(lifetime_rewriter, &mut ty);

            quote! {
                pub(super) #ident: #ty,
            }
        })
        .collect()
}
