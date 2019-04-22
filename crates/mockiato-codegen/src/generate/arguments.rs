use super::constant::{arguments_ident, arguments_lifetime};
use super::lifetime_rewriter::{LifetimeRewriter, UniformLifetimeGenerator};
use crate::generate::util::ident_to_string_literal;
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use proc_macro2::TokenStream;
use quote::quote;
use syn::visit_mut::visit_type_mut;
use syn::Ident;

pub(crate) struct GeneratedArguments {
    pub(crate) output: TokenStream,
    pub(crate) generics: TokenStream,
    pub(crate) ident: Ident,
}

pub(crate) fn generate_arguments(method_decl: &MethodDecl) -> GeneratedArguments {
    let arguments_ident = arguments_ident(&method_decl.ident);

    let mut lifetime_rewriter =
        LifetimeRewriter::new(UniformLifetimeGenerator::new(arguments_lifetime()));
    let arguments_fields = generate_arguments_fields(&mut lifetime_rewriter, &method_decl.inputs);

    let generics = if lifetime_rewriter.generator.has_lifetimes() {
        generics()
    } else {
        TokenStream::new()
    };

    let display_impl = generate_display_impl(method_decl, &generics);
    let debug_impl = generate_debug_impl(method_decl, &generics);

    GeneratedArguments {
        generics: generics.clone(),
        ident: arguments_ident.clone(),
        output: quote! {
            #[doc(hidden)]
            pub struct #arguments_ident #generics {
                #arguments_fields
            }

            #display_impl
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

/// Generates a `Display` implementation for an arguments struct.
fn generate_display_impl(method_decl: &MethodDecl, generics: &TokenStream) -> TokenStream {
    let arguments_ident = arguments_ident(&method_decl.ident);

    let display_fields: TokenStream = method_decl
        .inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            quote! { format!("{:?}", &mockiato::internal::MaybeDebugWrapper(&self.#ident)), }
        })
        .collect();

    quote! {
        impl #generics std::fmt::Display for #arguments_ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let arguments: Vec<String> = vec![
                    #display_fields
                ];

                write!(f, "({})", arguments.join(", "))
            }
        }
    }
}

/// Generates a `Debug` implementation for an arguments struct.
fn generate_debug_impl(method_decl: &MethodDecl, generics: &TokenStream) -> TokenStream {
    let arguments_ident = arguments_ident(&method_decl.ident);
    let arguments_ident_str_literal = ident_to_string_literal(&arguments_ident);

    let debug_fields: TokenStream = method_decl
        .inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            let ident_as_str = ident_to_string_literal(ident);
            quote! {
                .field(#ident_as_str, &mockiato::internal::MaybeDebugWrapper(&self.#ident))
            }
        })
        .collect();

    quote! {
        impl #generics std::fmt::Debug for #arguments_ident #generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(#arguments_ident_str_literal)
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
