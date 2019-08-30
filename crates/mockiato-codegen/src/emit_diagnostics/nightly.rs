use crate::diagnostic::{Diagnostic, DiagnosticLevel, DiagnosticMessage};
use crate::result::Error;
use proc_macro::{
    Diagnostic as ProcMacroDiagnostic, Level as ProcMacroLevel, Span as ProcMacroSpan,
};
use proc_macro2::Span;
use proc_macro2::TokenStream;

pub(crate) fn emit_diagnostics(error: Error) -> TokenStream {
    error
        .diagnostics
        .into_iter()
        .map(to_proc_macro_diagnostic)
        .for_each(ProcMacroDiagnostic::emit);
    TokenStream::new()
}

fn to_proc_macro_diagnostic(source: Diagnostic) -> ProcMacroDiagnostic {
    let level = to_proc_macro_level(source.level);
    let span = to_proc_macro_span(source.span);
    let diagnostic = ProcMacroDiagnostic::spanned(span, level, source.message);
    let diagnostic = add_notes_to_proc_macro_diagnostic(diagnostic, source.notes);
    add_help_to_proc_macro_diagnostic(diagnostic, source.help)
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
