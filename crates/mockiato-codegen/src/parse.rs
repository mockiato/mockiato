use proc_macro2::Span;

use crate::diagnostic::DiagnosticBuilder;
use crate::result::Result;

pub(crate) mod method_decl;
pub(crate) mod method_decl_parser;
pub(crate) mod method_inputs;
pub(crate) mod method_inputs_parser;
pub(crate) mod mockable_attr;
pub(crate) mod mockable_attr_parser;
pub(crate) mod trait_decl;
pub(crate) mod trait_decl_parser;

fn check_option_is_none<T>(value: &Option<T>, span: Span, error_message: &str) -> Result<()> {
    match value {
        None => Ok(()),
        Some(_) => Err(DiagnosticBuilder::error(span, error_message).build().into()),
    }
}
