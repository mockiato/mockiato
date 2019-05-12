use crate::diagnostic::DiagnosticBuilder;
use crate::result::Result;
use proc_macro2::Span;
pub(crate) mod method_decl;
pub(crate) mod method_inputs;
pub(crate) mod mockable_attr;
pub(crate) mod name_attr;
pub(crate) mod static_attr;
pub(crate) mod trait_decl;

fn check_option_is_none<T>(value: &Option<T>, span: Span, error_message: &str) -> Result<()> {
    match value {
        None => Ok(()),
        Some(_) => Err(DiagnosticBuilder::error(span, error_message).build().into()),
    }
}
