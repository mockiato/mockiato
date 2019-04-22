use super::bound_lifetimes::rewrite_lifetimes_incrementally;
use super::constant::{
    arguments_lifetime, arguments_lifetime_as_generic_param, arguments_matcher_ident,
};
use super::debug_impl::{generate_debug_impl, DebugImplField};
use super::generics::get_matching_generics_for_method_inputs;
use super::visibility::raise_visibility_by_one_level;
use crate::generate::arguments::GeneratedArguments;
use crate::generate::util::ident_to_string_literal;
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Generics, Token};

pub(crate) fn generate_arguments_matcher(
    method_decl: &MethodDecl,
    trait_decl: &TraitDecl,
    arguments: &GeneratedArguments,
) -> TokenStream {
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);

    let mut generics =
        get_matching_generics_for_method_inputs(&method_decl.inputs, &trait_decl.generics);
    generics.params.push(parse_quote!('mock));

    let arguments_matcher_fields = arguments_matcher_fields(&method_decl.inputs);
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let visibility = raise_visibility_by_one_level(&trait_decl.visibility);

    let display_impl = generate_display_impl(method_decl, &generics);
    let arguments_matcher_impl = generate_arguments_matcher_impl(method_decl, arguments, &generics);

    let debug_impl = generate_debug_impl(
        debug_impl_fields(method_decl),
        &arguments_matcher_ident,
        &generics,
    );

    quote! {
        #[doc(hidden)]
        #visibility struct #arguments_matcher_ident #ty_generics #where_clause {
            #arguments_matcher_fields
            pub(super) phantom_data: std::marker::PhantomData<&'mock ()>,
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
    method_decl: &MethodDecl,
    arguments: &GeneratedArguments,
    generics: &Generics,
) -> TokenStream {
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);
    let arguments_ident = &arguments.ident;

    let mut generics_with_arguments_lifetime = generics.clone();
    generics_with_arguments_lifetime
        .params
        .push(arguments_lifetime_as_generic_param());

    let (impl_generics, _, _) = generics_with_arguments_lifetime.split_for_impl();
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let (_, arguments_ty_generics, _) = arguments.generics.split_for_impl();

    let matches_argument_method = generate_matches_arguments_method_impl(method_decl);
    let arguments_lifetime = arguments_lifetime();

    quote! {
        impl #impl_generics mockiato::internal::ArgumentsMatcher<#arguments_lifetime> for #arguments_matcher_ident #ty_generics #where_clause {
            type Arguments = #arguments_ident #arguments_ty_generics;

            #matches_argument_method
        }
    }
}

fn generate_matches_arguments_method_impl(method_decl: &MethodDecl) -> TokenStream {
    let args = &method_decl.inputs.args;

    // Since argument matchers for methods without any arguments should always match, we can
    // fall back to the default impl on the trait `ArgumentsMatcher`.
    if args.is_empty() {
        return TokenStream::new();
    }

    let matches_argument_calls: Punctuated<_, Token![&&]> = args
        .iter()
        .map(|arg| {
            let ident = &arg.ident;
            quote! { self.#ident.matches_argument(&args.#ident) }
        })
        .collect();

    quote! {
        fn matches_arguments(&self, args: &Self::Arguments) -> bool {
            #matches_argument_calls
        }
    }
}

fn arguments_matcher_fields(method_inputs: &MethodInputs) -> TokenStream {
    method_inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            let mut ty = input.ty.clone();
            let bound_lifetimes = rewrite_lifetimes_incrementally(&mut ty);

            quote! {
                pub(super) #ident: std::boxed::Box<dyn #bound_lifetimes mockiato::internal::ArgumentMatcher<#ty> + 'mock>,
            }
        })
        .collect()
}

fn debug_impl_fields<'a>(
    method_decl: &'a MethodDecl,
) -> impl Iterator<Item = DebugImplField<'a>> + 'a {
    method_decl.inputs.args.iter().map(|input| {
        let ident = &input.ident;
        (ident, quote! { self.#ident })
    })
}
