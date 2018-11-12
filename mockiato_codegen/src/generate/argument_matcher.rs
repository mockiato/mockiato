use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodInputs;
use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use syn::visit_mut::{visit_type_mut, visit_type_reference_mut, VisitMut};
use syn::{BoundLifetimes, Ident, Lifetime, LifetimeDef, TypeReference};

pub(crate) fn generate_argument_matcher(method_decl: &MethodDecl) -> TokenStream {
    let argument_matcher_ident = argument_matcher_ident(&method_decl.ident);
    let argument_matcher_fields = argument_matcher_fields(&method_decl.inputs);

    quote! {
        pub(super) struct #argument_matcher_ident {
            #argument_matcher_fields
        }

        impl std::fmt::Debug for #argument_matcher_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // TODO
                unimplemented!()
            }
        }

        impl mockiato::internal::Arguments for #argument_matcher_ident {}
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
            let mut ty = input.ty.clone();

            let mut lifetime_rewriter = LifetimeRewriter::default();
            visit_type_mut(&mut lifetime_rewriter, &mut ty);

            let bound_lifetimes = bound_lifetimes(lifetime_rewriter.lifetimes);

            quote! {
                pub(super) #ident: std::boxed::Box<dyn #bound_lifetimes mockiato::internal::ArgumentMatcher<#ty>>,
            }
        })
        .collect()
}

fn bound_lifetimes(lifetimes: Vec<Lifetime>) -> Option<BoundLifetimes> {
    if lifetimes.is_empty() {
        return None;
    }

    Some(BoundLifetimes {
        lifetimes: lifetimes.into_iter().map(LifetimeDef::new).collect(),
        ..Default::default()
    })
}

#[derive(Default)]
struct LifetimeRewriter {
    lifetimes: Vec<Lifetime>,
}

impl LifetimeRewriter {
    fn create_new_lifetime(&mut self) -> Lifetime {
        let lifetime = Lifetime::new(&format!("'arg{}", self.lifetimes.len()), Span::call_site());
        self.lifetimes.push(lifetime.clone());
        lifetime
    }
}

impl VisitMut for LifetimeRewriter {
    fn visit_lifetime_mut(&mut self, lifetime: &mut Lifetime) {
        *lifetime = self.create_new_lifetime();
    }

    fn visit_type_reference_mut(&mut self, type_reference: &mut TypeReference) {
        visit_type_reference_mut(self, type_reference);

        if type_reference.lifetime.is_none() {
            type_reference.lifetime = Some(self.create_new_lifetime());
        }
    }
}
