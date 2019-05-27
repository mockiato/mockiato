use crate::result::Error;
use proc_macro2::TokenStream;
use quote::quote_spanned;

pub(crate) fn emit_diagnostics(error: Error) -> TokenStream {
    error
        .diagnostics
        .into_iter()
        .map(|diagnostic| {
            let message = diagnostic.message;
            quote_spanned!(diagnostic.span => compile_error!(#message);)
        })
        .collect()
}
