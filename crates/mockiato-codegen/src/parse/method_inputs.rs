use crate::spanned::SpannedUnstable;
use crate::{merge_results, Error, Result};
use proc_macro::{Diagnostic, Level, Span};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{ArgCaptured, ArgSelf, ArgSelfRef, FnArg, Ident, Pat, PatIdent, Token, Type};

#[derive(Debug, Clone)]
pub(crate) struct MethodInputs {
    pub(crate) self_arg: MethodSelfArg,
    pub(crate) args: Vec<MethodArg>,
}

impl MethodInputs {
    pub(crate) fn parse(inputs: Punctuated<FnArg, Token![,]>) -> Result<Self> {
        let span = inputs.span_unstable();
        let mut inputs_iter = inputs.into_iter();

        let self_arg = inputs_iter
            .next()
            .ok_or(())
            .and_then(MethodSelfArg::parse)
            .map_err(|_| {
                Error::Diagnostic(Diagnostic::spanned(
                    span,
                    Level::Error,
                    "The first parameter of a method must be self, so that the trait is object-safe",
                ))
            })?;

        let args = inputs_iter.map(MethodArg::parse);

        Ok(Self {
            self_arg,
            args: merge_results(args)?.collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum MethodSelfArg {
    /// `self` is taken by reference: `&self` or `&mut self`
    Ref(ArgSelfRef),
    /// `self` is consumed: `self`
    Value(ArgSelf),
    /// `self` has a type. Example: `self: Box<Self>`
    Captured(Box<ArgCaptured>),
}

impl MethodSelfArg {
    fn parse(arg: FnArg) -> std::result::Result<Self, ()> {
        match arg {
            FnArg::SelfRef(self_ref) => Ok(MethodSelfArg::Ref(self_ref)),
            FnArg::SelfValue(self_value) => Ok(MethodSelfArg::Value(self_value)),
            FnArg::Captured(captured_arg) => Self::parse_captured_arg(captured_arg),
            _ => Err(()),
        }
    }

    fn parse_captured_arg(arg: ArgCaptured) -> std::result::Result<Self, ()> {
        const SELF_ARG_NAME: &str = "self";

        match arg.pat {
            Pat::Ident(PatIdent { ref ident, .. }) if ident == SELF_ARG_NAME => {
                Ok(MethodSelfArg::Captured(box arg))
            }
            _ => Err(()),
        }
    }
}

impl ToTokens for MethodSelfArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner: &ToTokens = match self {
            MethodSelfArg::Ref(ref arg_self_ref) => arg_self_ref,
            MethodSelfArg::Value(ref arg_self) => arg_self,
            MethodSelfArg::Captured(ref arg_captured) => arg_captured,
        };

        inner.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MethodArg {
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) span: Span,
}

impl MethodArg {
    pub(crate) fn parse(arg: FnArg) -> Result<Self> {
        let span = arg.span_unstable();

        match arg {
            // A "captured" argument is the "normal" way of specifying an argument
            // with an explicit name and type.
            // E.g. `name: &str`
            FnArg::Captured(captured) => {
                let span = captured.span_unstable();

                match captured.pat {
                    Pat::Ident(pat_ident) => {
                        // Subpat is the part behind the @ in a pattern match.
                        // See: https://docs.rs/syn/0.15.20/syn/struct.PatIdent.html#structfield.subpat
                        if pat_ident.subpat.is_some() {
                            panic!("Sub-pattern should not appear within method declaration");
                        }

                        Ok(MethodArg {
                            ident: sanitize_method_ident(&pat_ident.ident),
                            ty: captured.ty,
                            span
                        })
                    },
                    _ => {
                        Err(Error::Diagnostic(Diagnostic::spanned(
                        span,
                        Level::Error,
                        "Ignored arguments are not supported")))
                    },
                }
            }
            _ => Err(Error::Diagnostic(Diagnostic::spanned(
                span,
                Level::Error,
                "Only captured arguments are supported",
            ).note("This error should never appear, because rustc already enforces these requirements"))),
        }
    }
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

/// Sanitizes a method identifier by removing all leading underscores
fn sanitize_method_ident(ident: &Ident) -> Ident {
    let ident_string = ident.to_string();
    let sanitized_ident_str = ident_string.trim_start_matches('_');

    Ident::new(sanitized_ident_str, ident.span())
}
