//! Codegen for `mockiato`. Do not use this crate directly.

#![feature(
    proc_macro_diagnostic,
    proc_macro_span,
    proc_macro_hygiene,
    bind_by_move_pattern_guards,
    decl_macro,
    box_syntax,
    box_patterns
)]
#![warn(clippy::dbg_macro, clippy::unimplemented)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

extern crate proc_macro;

mod constant;
mod generate;
mod mockable;
mod parse;
mod result;
mod syn_ext;

use self::mockable::Mockable;
pub(crate) use self::result::*;
use proc_macro::{
    Diagnostic as ProcMacroDiagnostic, Level as ProcMacroLevel, Span as ProcMacroSpan,
    TokenStream as ProcMacroTokenStream,
};
use proc_macro2::Span;
use syn::{parse_macro_input, AttributeArgs, Item};

#[doc(hidden)]
#[proc_macro_attribute]
pub fn mockable(args: ProcMacroTokenStream, input: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let original_input = input.clone();

    let attr = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as Item);

    let mockable = Mockable::new();

    match mockable.expand(attr, item) {
        Ok(output) => ProcMacroTokenStream::from(output),
        Err(error) => {
            emit_diagnostics(error);
            original_input
        }
    }
}

fn emit_diagnostics(error: Error) {
    error
        .diagnostics
        .into_iter()
        .map(to_proc_macro_diagnostic)
        .for_each(ProcMacroDiagnostic::emit);
}

fn to_proc_macro_diagnostic(source: Diagnostic) -> ProcMacroDiagnostic {
    let level = to_proc_macro_level(source.level);
    let span = to_proc_macro_span(source.span);
    let diagnostic = ProcMacroDiagnostic::spanned(span, level, source.message);
    let diagnostic = add_notes_to_proc_macro_diagnostic(diagnostic, source.notes);
    let diagnostic = add_help_to_proc_macro_diagnostic(diagnostic, source.help);
    diagnostic
}

fn add_help_to_proc_macro_diagnostic(
    diagnostic: ProcMacroDiagnostic,
    help: Vec<DiagnosticMessage>,
) -> ProcMacroDiagnostic {
    help.into_iter()
        .fold(diagnostic, |diagnostic, help| match help.span {
            Some(span) => diagnostic.span_help(to_proc_macro_span(span), help.message),
            None => diagnostic.help(help.message),
        })
}

fn add_notes_to_proc_macro_diagnostic(
    diagnostic: ProcMacroDiagnostic,
    notes: Vec<DiagnosticMessage>,
) -> ProcMacroDiagnostic {
    notes
        .into_iter()
        .fold(diagnostic, |diagnostic, note| match note.span {
            Some(span) => diagnostic.span_note(to_proc_macro_span(span), note.message),
            None => diagnostic.note(note.message),
        })
}

fn to_proc_macro_span(span: Span) -> ProcMacroSpan {
    span.unstable()
}

fn to_proc_macro_level(level: DiagnosticLevel) -> ProcMacroLevel {
    match level {
        DiagnosticLevel::Error => ProcMacroLevel::Error,
    }
}
