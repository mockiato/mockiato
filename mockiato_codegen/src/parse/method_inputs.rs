use crate::spanned::SpannedUnstable;
use crate::{merge_results, Error, Result};
use proc_macro::{Diagnostic, Level, Span};
use syn::punctuated::Punctuated;
use syn::{ArgCaptured, ArgSelf, ArgSelfRef, FnArg, Ident, Pat, PatIdent, Type};

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

#[derive(Debug, Clone)]
pub(crate) struct MethodArg {
    ident: Ident,
    ty: Type,
    span: Span,
}

impl MethodArg {
    pub(crate) fn parse(arg: FnArg) -> Result<Self> {
        let span = arg.span_unstable();

        match arg {
            FnArg::Captured(captured) => {
                let span = captured.span_unstable();

                match captured.pat {
                    Pat::Ident(pat_ident) => {

                        if pat_ident.subpat.is_some() {
                            panic!("Sub-pattern should not appear within method declaration");
                        }

                        Ok(MethodArg {
                            ident: pat_ident.ident,
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
