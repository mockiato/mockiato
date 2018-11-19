use super::bound_lifetimes::bound_lifetimes;
use super::constant::{arguments_lifetime, arguments_matcher_ident};
use super::lifetime_rewriter::{IncrementalLifetimeGenerator, LifetimeRewriter};
use crate::generate::arguments::GeneratedArguments;
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::visit_mut::visit_type_mut;
use syn::LitStr;

pub(crate) fn generate_arguments_matcher(
    method_decl: &MethodDecl,
    arguments: &GeneratedArguments,
) -> TokenStream {
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);
    let arguments_matcher_fields = arguments_matcher_fields(&method_decl.inputs);
    let debug_impl = generate_debug_impl(method_decl);
    let arguments_matcher_impl = generate_arguments_matcher_impl(method_decl, arguments);

    quote! {
        pub(super) struct #arguments_matcher_ident {
            #arguments_matcher_fields
        }

        #debug_impl
        #arguments_matcher_impl
    }
}

/// Generates a `Debug` implementation for an argument matcher.
fn generate_debug_impl(method_decl: &MethodDecl) -> TokenStream {
    let method_name_str = LitStr::new(&method_decl.ident.to_string(), method_decl.ident.span());
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);

    let debug_fields: TokenStream = method_decl
        .inputs
        .args
        .iter()
        .map(|input| {
            let ident = &input.ident;
            quote!{ .field(&mockiato::internal::MaybeDebugExtWrapper(&self.#ident)) }
        })
        .collect();

    quote! {
        impl std::fmt::Debug for #arguments_matcher_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(#method_name_str)
                  #debug_fields
                 .finish()
            }
        }
    }
}

fn generate_arguments_matcher_impl(
    method_decl: &MethodDecl,
    arguments: &GeneratedArguments,
) -> TokenStream {
    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);
    let arguments_ident = &arguments.ident;
    let arguments_generics = &arguments.generics;

    let matches_argument_method = generate_matches_arguments_method_impl(method_decl);
    let arguments_lifetime = arguments_lifetime();

    quote! {
        impl<#arguments_lifetime> mockiato::internal::ArgumentsMatcher<#arguments_lifetime> for #arguments_matcher_ident {
            type Arguments = #arguments_ident #arguments_generics;

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

            let mut lifetime_rewriter = LifetimeRewriter::new(IncrementalLifetimeGenerator::default());
            visit_type_mut(&mut lifetime_rewriter, &mut ty);

            let bound_lifetimes = bound_lifetimes(lifetime_rewriter.generator.lifetimes);

            quote! {
                pub(super) #ident: std::boxed::Box<dyn #bound_lifetimes mockiato::internal::ArgumentMatcher<#ty>>,
            }
        })
        .collect()
}
