use super::bound_lifetimes::rewrite_lifetimes_incrementally;
use super::constant::{
    arguments_lifetime, arguments_lifetime_as_generic_param, arguments_matcher_ident,
    mock_lifetime, mock_lifetime_as_generic_param,
};
use super::debug_impl::{generate_debug_impl, DebugImplField};
use super::MethodDeclMetadata;
use crate::code_generator::util::ident_to_string_literal;
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::{MethodArg, MethodInputs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Generics, Token, Visibility};

pub(crate) fn generate_arguments_matcher(
    method: &MethodDeclMetadata,
    visibility: &Visibility,
) -> TokenStream {
    let MethodDeclMetadata {
        method_decl,
        generics,
        ..
    } = method;
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);

    let mut generics = generics.clone();
    generics.params.push(mock_lifetime_as_generic_param());

    let arguments_matcher_fields = arguments_matcher_fields(&method_decl.inputs);
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    let display_impl = generate_display_impl(method_decl, &generics);
    let arguments_matcher_impl = generate_arguments_matcher_impl(method, &generics);

    let debug_impl = generate_debug_impl(
        debug_impl_fields(method_decl),
        &arguments_matcher_ident,
        &generics,
    );

    let mock_lifetime = mock_lifetime();

    quote! {
        #[doc(hidden)]
        #visibility struct #arguments_matcher_ident #ty_generics #where_clause {
            #arguments_matcher_fields
            pub(super) phantom_data: std::marker::PhantomData<&#mock_lifetime ()>,
        }

        #display_impl
        #debug_impl
        #arguments_matcher_impl
    }
}

/// Generates a `Display` implementation for an argument matcher.
fn generate_display_impl(method_decl: &MethodDecl, generics: &Generics) -> TokenStream {
    let method_name_str = ident_to_string_literal(&method_decl.ident);
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let debug_fields: TokenStream = method_decl
        .inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            quote! { format!("{}", &self.#ident), }
        })
        .collect();

    quote! {
        impl #impl_generics std::fmt::Display for #arguments_matcher_ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let arguments: Vec<String> = vec![
                    #debug_fields
                ];

                write!(f, "{}({})", #method_name_str, arguments.join(", "))
            }
        }
    }
}

fn generate_arguments_matcher_impl(
    method: &MethodDeclMetadata,
    generics_with_mock_lifetime: &Generics,
) -> TokenStream {
    let MethodDeclMetadata {
        method_decl,
        arguments_struct_ident,
        generics,
        ..
    } = method;
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);

    let mut arguments_struct_generics = generics.clone();
    arguments_struct_generics
        .params
        .push(arguments_lifetime_as_generic_param());

    let mut generics_with_arguments_lifetime = generics_with_mock_lifetime.clone();
    generics_with_arguments_lifetime
        .params
        .push(arguments_lifetime_as_generic_param());

    let (impl_generics, _, _) = generics_with_arguments_lifetime.split_for_impl();
    let (_, ty_generics, where_clause) = generics_with_mock_lifetime.split_for_impl();
    let (_, arguments_ty_generics, _) = arguments_struct_generics.split_for_impl();

    let matches_argument_method = generate_matches_arguments_method_impl(method_decl);
    let arguments_lifetime = arguments_lifetime();

    quote! {
        impl #impl_generics mockiato::internal::ArgumentsMatcher<#arguments_lifetime> for #arguments_matcher_ident #ty_generics #where_clause {
            type Arguments = #arguments_struct_ident #arguments_ty_generics;

            #matches_argument_method
        }
    }
}

fn generate_matches_arguments_method_impl(method_decl: &MethodDecl) -> TokenStream {
    let args = &method_decl.inputs.args;

    let matches_argument_calls = if args.is_empty() {
        quote!(true)
    } else {
        generate_matches_argument_calls(args)
    };

    quote! {
        fn matches_arguments(&self, args: &Self::Arguments) -> bool {
            #matches_argument_calls
        }
    }
}

fn generate_matches_argument_calls(args: &[MethodArg]) -> TokenStream {
    let matches_argument_calls: Punctuated<_, Token![&&]> = args
        .iter()
        .map(|arg| {
            let ident = &arg.ident;
            quote! { self.#ident.matches_argument(&args.#ident) }
        })
        .collect();
    quote!(#matches_argument_calls)
}

fn arguments_matcher_fields(method_inputs: &MethodInputs) -> TokenStream {
    let mock_lifetime = mock_lifetime();
    method_inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            let mut ty = input.ty.clone();
            let bound_lifetimes = rewrite_lifetimes_incrementally(&mut ty);

            quote! {
                pub(super) #ident: std::boxed::Box<dyn #bound_lifetimes mockiato::internal::ArgumentMatcher<#ty> + #mock_lifetime>,
            }
        })
        .collect()
}

fn debug_impl_fields<'a>(
    method_decl: &'a MethodDecl,
) -> impl Iterator<Item = DebugImplField<'a>> + 'a {
    method_decl.inputs.args.iter().map(|input| {
        let ident = &input.ident;
        DebugImplField {
            ident,
            expression: quote! { self.#ident },
        }
    })
}
