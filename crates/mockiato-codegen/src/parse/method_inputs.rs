use crate::constant::CREATE_ISSUE_LINK;
use crate::diagnostic::DiagnosticBuilder;
use crate::result::{merge_results, Error, Result};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::fmt::Debug;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{ArgCaptured, ArgSelf, ArgSelfRef, FnArg, Ident, Pat, PatIdent, Token, Type};

#[derive(Clone)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MethodInputs {
    pub(crate) self_arg: MethodSelfArg,
    pub(crate) args: Vec<MethodArg>,
}

pub(crate) trait MethodInputsParser: Debug {
    fn parse(&self, inputs: Punctuated<FnArg, Token![,]>) -> Result<MethodInputs>;
}

#[derive(Clone)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) enum MethodSelfArg {
    /// `self` is taken by reference: `&self` or `&mut self`
    Ref(ArgSelfRef),
    /// `self` is consumed: `self`
    Value(ArgSelf),
    /// `self` has a type. Example: `self: Box<Self>`
    Captured(Box<ArgCaptured>),
}

impl ToTokens for MethodSelfArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner: &dyn ToTokens = match self {
            MethodSelfArg::Ref(ref arg_self_ref) => arg_self_ref,
            MethodSelfArg::Value(ref arg_self) => arg_self,
            MethodSelfArg::Captured(ref arg_captured) => arg_captured,
        };

        inner.to_tokens(tokens);
    }
}

pub(crate) trait MethodSelfArgParser: Debug {
    fn parse(&self, arg: FnArg) -> std::result::Result<MethodSelfArg, ()>;
}

#[derive(Clone)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MethodArg {
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) span: Span,
}

pub(crate) trait MethodArgParser: Debug {
    fn parse(&self, arg: FnArg) -> Result<MethodArg>;
}

impl ToTokens for MethodArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ref ident, ref ty, ..
        } = self;

        tokens.extend(quote! {
            #ident: #ty
        });
    }
}
