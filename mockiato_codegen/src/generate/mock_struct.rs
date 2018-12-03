use super::bound_lifetimes::rewrite_lifetimes_incrementally;
use super::constant::{arguments_matcher_ident, expect_method_ident, generic_parameter_ident};
use crate::parse::method_decl::MethodDecl;
use crate::parse::method_inputs::MethodArg;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::{Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitStr, ReturnType, Type};

type ArgumentsWithGenerics<'a> = &'a [(Ident, &'a MethodArg)];

pub(crate) fn generate_mock_struct(
    trait_decl: &TraitDecl,
    mock_struct_ident: &Ident,
    mod_ident: &Ident,
) -> TokenStream {
    let method_fields: Punctuated<_, Token![,]> = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_method_field(method_decl, &mod_ident))
        .collect();

    let initializer_fields: Punctuated<_, Token![,]> = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_initializer_field(&method_decl.ident, &mock_struct_ident))
        .collect();

    let expect_methods: TokenStream = trait_decl
        .methods
        .iter()
        .map(|method_decl| generate_expect_method(trait_decl, method_decl, &mod_ident))
        .collect();

    let visibility = &trait_decl.visibility;

    quote! {
        #[derive(Debug, Clone)]
        #visibility struct #mock_struct_ident {
            #method_fields
        }

        impl #mock_struct_ident {
            #visibility fn new() -> Self {
                Self { #initializer_fields }
            }

            #expect_methods
        }
    }
}

fn generate_method_field(method_decl: &MethodDecl, mod_ident: &Ident) -> TokenStream {
    let ident = &method_decl.ident;
    let arguments_matcher_ident = arguments_matcher_ident(ident);
    let return_type = return_type(method_decl);

    quote! {
        #ident: mockiato::internal::Method<self::#mod_ident::#arguments_matcher_ident, #return_type>
    }
}

fn return_type(method_decl: &MethodDecl) -> TokenStream {
    match &method_decl.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
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
        #method_ident: mockiato::internal::Method::new(#name)
    }
}

fn generate_expect_method(
    trait_decl: &TraitDecl,
    method_decl: &MethodDecl,
    mod_ident: &Ident,
) -> TokenStream {
    let method_ident = &method_decl.ident;
    let visibility = &trait_decl.visibility;
    let expect_method_ident = expect_method_ident(method_decl);

    let arguments_with_generics: Vec<_> = method_decl
        .inputs
        .args
        .iter()
        .enumerate()
        .map(|(index, argument)| (generic_parameter_ident(index), argument))
        .collect();

    let generics = generics(&arguments_with_generics);
    let arguments: TokenStream = arguments_with_generics
        .iter()
        .map(generate_argument)
        .collect();

    let arguments_matcher_ident = arguments_matcher_ident(&method_decl.ident);
    let return_type = return_type(method_decl);

    let where_clause = where_clause(&arguments_with_generics);

    let expected_parameters: TokenStream = arguments_with_generics
        .iter()
        .map(|(_, argument)| &argument.ident)
        .map(|argument_ident| quote! { #argument_ident: Box::new(#argument_ident), })
        .collect();

    let requires_must_use_annotation = !is_empty_return_value(&method_decl.output);

    let must_use_annotation = if requires_must_use_annotation {
        quote! { #[must_use] }
    } else {
        TokenStream::new()
    };

    quote! {
        #must_use_annotation
        #visibility fn #expect_method_ident#generics(
            &mut self,
            #arguments
        ) -> mockiato::internal::MethodCallBuilder<
            '_,
            self::#mod_ident::#arguments_matcher_ident,
            #return_type
        > #where_clause
        {
            self.#method_ident.add_expected_call(
                self::#mod_ident::#arguments_matcher_ident {
                    #expected_parameters
                }
            )
        }
    }
}

fn is_empty_return_value(return_value: &ReturnType) -> bool {
    match return_value {
        ReturnType::Default => true,
        ReturnType::Type(_, ty) => match ty {
            box Type::Tuple(tuple) => tuple.elems.is_empty(),
            _ => false,
        },
    }
}

fn where_clause(arguments: ArgumentsWithGenerics<'_>) -> TokenStream {
    if arguments.is_empty() {
        TokenStream::new()
    } else {
        let predicates: Punctuated<_, Token![,]> = arguments
            .iter()
            .map(|(generic_type_ident, method_argument)| {
                where_clause_predicate(generic_type_ident, method_argument)
            })
            .collect();

        quote! {
            where #predicates
        }
    }
}

fn where_clause_predicate(generic_type_ident: &Ident, method_argument: &MethodArg) -> TokenStream {
    let mut ty = method_argument.ty.clone();
    let bound_lifetimes = rewrite_lifetimes_incrementally(&mut ty);

    quote! {
        #generic_type_ident: #bound_lifetimes mockiato::internal::ArgumentMatcher<#ty> + 'static
    }
}

fn generics(arguments: ArgumentsWithGenerics<'_>) -> TokenStream {
    if arguments.is_empty() {
        TokenStream::new()
    } else {
        let parameters: Punctuated<_, Token![,]> = arguments
            .iter()
            .map(|(generic_type_ident, _)| generic_type_ident)
            .collect();

        quote! { <#parameters> }
    }
}

fn generate_argument((generic_type_ident, method_argument): &(Ident, &MethodArg)) -> TokenStream {
    let argument_ident = &method_argument.ident;

    quote! {
        #argument_ident: #generic_type_ident,
    }
}
