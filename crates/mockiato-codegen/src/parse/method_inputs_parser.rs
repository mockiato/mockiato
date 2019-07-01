use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{ArgCaptured, FnArg, Ident, Pat, PatIdent, Token};

use crate::constant::CREATE_ISSUE_LINK;
use crate::diagnostic::DiagnosticBuilder;
use crate::parse::method_inputs::{
    MethodArg, MethodArgParser, MethodInputs, MethodInputsParser, MethodSelfArg,
    MethodSelfArgParser,
};
use crate::result::{merge_results, Error, Result};

#[derive(Debug)]
pub(crate) struct MethodInputsParserImpl {
    method_self_arg_parser: Box<dyn MethodSelfArgParser>,
    method_arg_parser: Box<dyn MethodArgParser>,
}

impl MethodInputsParserImpl {
    pub(crate) fn new(
        method_self_arg_parser: Box<dyn MethodSelfArgParser>,
        method_arg_parser: Box<dyn MethodArgParser>,
    ) -> Self {
        Self {
            method_self_arg_parser,
            method_arg_parser,
        }
    }
}

impl MethodInputsParser for MethodInputsParserImpl {
    fn parse(&self, inputs: Punctuated<FnArg, Token![,]>) -> Result<MethodInputs> {
        let span = inputs.span();
        let mut inputs_iter = inputs.into_iter();

        let self_arg = inputs_iter
            .next()
            .ok_or(())
            .and_then(|arg| self.method_self_arg_parser.parse(arg))
            .map_err(|_| first_argument_is_not_self_error(span))?;

        let args = inputs_iter.map(|arg| self.method_arg_parser.parse(arg));

        Ok(MethodInputs {
            self_arg,
            args: merge_results(args)?.collect(),
        })
    }
}

fn first_argument_is_not_self_error(span: Span) -> Error {
    let error_message =
        "The first parameter of a method must be self, so that the trait is object-safe";
    DiagnosticBuilder::error(span, error_message).build().into()
}

#[derive(Debug)]
pub(crate) struct MethodSelfArgParserImpl;

impl MethodSelfArgParserImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl MethodSelfArgParser for MethodSelfArgParserImpl {
    fn parse(&self, arg: FnArg) -> std::result::Result<MethodSelfArg, ()> {
        match arg {
            FnArg::SelfRef(self_ref) => Ok(MethodSelfArg::Ref(self_ref)),
            FnArg::SelfValue(self_value) => Ok(MethodSelfArg::Value(self_value)),
            FnArg::Captured(captured_arg) => parse_captured_arg(captured_arg),
            _ => Err(()),
        }
    }
}

fn parse_captured_arg(arg: ArgCaptured) -> std::result::Result<MethodSelfArg, ()> {
    const SELF_ARG_NAME: &str = "self";

    match arg.pat {
        Pat::Ident(PatIdent { ref ident, .. }) if ident == SELF_ARG_NAME => {
            Ok(MethodSelfArg::Captured(Box::new(arg)))
        }
        _ => Err(()),
    }
}

#[derive(Debug)]
pub(crate) struct MethodArgParserImpl;

impl MethodArgParserImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl MethodArgParser for MethodArgParserImpl {
    fn parse(&self, arg: FnArg) -> Result<MethodArg> {
        let span = arg.span();

        match arg {
            // A "captured" argument is the "normal" way of specifying an argument
            // with an explicit name and type.
            // E.g. `name: &str`
            FnArg::Captured(captured) => {
                let span = captured.span();

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
                            span,
                        })
                    }
                    _ => Err(
                        DiagnosticBuilder::error(span, "Ignored arguments are not supported")
                            .build()
                            .into(),
                    ),
                }
            }
            _ => Err(
                DiagnosticBuilder::error(span, "Only captured arguments are supported")
                    .note(format!(
                        "This error should never appear, because rustc already enforces these \
                         requirements. Please report this error using the following link: {}",
                        CREATE_ISSUE_LINK
                    ))
                    .build()
                    .into(),
            ),
        }
    }
}

/// Sanitizes a method identifier by removing all leading underscores
fn sanitize_method_ident(ident: &Ident) -> Ident {
    let ident_string = ident.to_string();
    let sanitized_ident_str = ident_string.trim_start_matches('_');

    Ident::new(sanitized_ident_str, ident.span())
}
