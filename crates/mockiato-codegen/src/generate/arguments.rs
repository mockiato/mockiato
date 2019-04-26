use super::constant::{arguments_ident, arguments_lifetime, arguments_lifetime_as_generic_param};
use super::debug_impl::{generate_debug_impl, DebugImplField};
use super::lifetime_rewriter::{LifetimeRewriter, UniformLifetimeGenerator};
use super::MethodDeclMetadata;
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use proc_macro2::TokenStream;
use quote::quote;
use syn::visit_mut::visit_type_mut;
use syn::{Generics, Visibility};

pub(crate) fn generate_arguments(
    method: &MethodDeclMetadata,
    visibility: &Visibility,
) -> TokenStream {
    let MethodDeclMetadata {
        method_decl,
        arguments_struct_ident,
        generics,
        ..
    } = method;

    let mut lifetime_rewriter =
        LifetimeRewriter::new(UniformLifetimeGenerator::new(arguments_lifetime()));
    let arguments_fields = generate_arguments_fields(&mut lifetime_rewriter, &method_decl.inputs);

    let mut generics = generics.clone();
    generics.params.push(arguments_lifetime_as_generic_param());

    let debug_impl = generate_debug_impl(
        debug_impl_fields(method_decl),
        arguments_struct_ident,
        &generics,
    );
    let display_impl = generate_display_impl(method_decl, &generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let arguments_lifetime = arguments_lifetime();

    quote! {
        #[doc(hidden)]
        #visibility struct #arguments_struct_ident #ty_generics #where_clause {
            #arguments_fields
            pub(super) phantom_data: std::marker::PhantomData<&#arguments_lifetime ()>,
        }

        #display_impl
        #debug_impl

        impl #impl_generics mockiato::internal::Arguments for #arguments_struct_ident #ty_generics #where_clause {}
    }
}

/// Generates a `Display` implementation for an arguments struct.
fn generate_display_impl(method_decl: &MethodDecl, generics: &Generics) -> TokenStream {
    let arguments_ident = arguments_ident(&method_decl.ident);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
        impl #impl_generics std::fmt::Display for #arguments_ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let arguments: Vec<String> = vec![
                    #display_fields
                ];

                write!(f, "({})", arguments.join(", "))
            }
        }
    }
}

fn debug_impl_fields<'a>(
    method_decl: &'a MethodDecl,
) -> impl Iterator<Item = DebugImplField<'a>> + 'a {
    method_decl.inputs.args.iter().map(|input| {
        let ident = &input.ident;
        DebugImplField {
            ident,
            expression: quote! { mockiato::internal::MaybeDebugWrapper(&self.#ident) }
        }
    })
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

            quote! { pub(super) #ident: #ty, }
        })
        .collect()
}
