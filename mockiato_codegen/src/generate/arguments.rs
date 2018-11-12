use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use syn::visit_mut::{visit_type_mut, visit_type_reference_mut, VisitMut};
use syn::{Ident, Lifetime, TypeReference};

pub(crate) fn generate_arguments(method_decl: &MethodDecl) -> TokenStream {
    let arguments_ident = arguments_ident(&method_decl.ident);

    let mut lifetime_rewriter = LifetimeRewriter::default();
    let arguments_fields = arguments_fields(&mut lifetime_rewriter, &method_decl.inputs);

    let generics = generics(lifetime_rewriter.has_lifetimes);
    let debug_impl = generate_debug_impl(method_decl, &generics);

    quote! {
        pub(super) struct #arguments_ident #generics {
            #arguments_fields
        }

        #debug_impl

        impl #generics mockiato::internal::Arguments for #arguments_ident #generics {}
    }
}

/// Generates the generics clause (including angled brackets) for the arguments struct.
fn generics(has_lifetimes: bool) -> TokenStream {
    if has_lifetimes {
        let lifetime = args_lifetime();

        quote! {
            <#lifetime>
        }
    } else {
        TokenStream::new()
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
            quote!{ .field(&mockiato::internal::MaybeDebugWrapper(&self.#ident)) }
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

fn arguments_ident(method_ident: &Ident) -> Ident {
    Ident::new(
        &format!("{}Arguments", method_ident.to_string().to_camel_case()),
        method_ident.span(),
    )
}

fn arguments_fields(
    lifetime_rewriter: &mut LifetimeRewriter,
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

fn args_lifetime() -> Lifetime {
    // The fixed prefix is arbitrary.
    Lifetime::new("'__mockiato_args", Span::call_site())
}

/// Replaces all lifetimes in the given AST with the same lifetime
/// It also gives explicit lifetimes to references without lifetimes
#[derive(Default)]
struct LifetimeRewriter {
    // Indicates that the rewriter found at least one lifetime
    has_lifetimes: bool,
}

impl VisitMut for LifetimeRewriter {
    fn visit_lifetime_mut(&mut self, lifetime: &mut Lifetime) {
        *lifetime = args_lifetime();
        self.has_lifetimes = true;
    }

    fn visit_type_reference_mut(&mut self, type_reference: &mut TypeReference) {
        visit_type_reference_mut(self, type_reference);

        if type_reference.lifetime.is_none() {
            type_reference.lifetime = Some(args_lifetime());
            self.has_lifetimes = true;
        }
    }
}
