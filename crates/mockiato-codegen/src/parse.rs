use crate::result::{Error, Result};
use proc_macro::{Diagnostic, Level, Span};
pub(crate) mod method_decl;
pub(crate) mod method_inputs;
pub(crate) mod mockable_attr;
pub(crate) mod name_attr;
pub(crate) mod static_attr;
pub(crate) mod trait_decl;

fn check_option_is_none<T>(value: &Option<T>, span: Span, error_message: &str) -> Result<()> {
    match value {
        None => Ok(()),
        Some(_) => Err(Error::Diagnostic(Diagnostic::spanned(
            span,
            Level::Error,
            error_message,
        ))),
    }
}
