pub(crate) use self::builder::*;
use proc_macro2::Span;

mod builder;

#[derive(Debug)]
pub(crate) enum DiagnosticLevel {
    Error,
}

#[derive(Debug)]
pub(crate) struct Diagnostic {
    pub(crate) span: Span,
    pub(crate) message: String,
    pub(crate) level: DiagnosticLevel,
    pub(crate) notes: Vec<DiagnosticMessage>,
    pub(crate) help: Vec<DiagnosticMessage>,
}

#[derive(Debug)]
pub(crate) struct DiagnosticMessage {
    pub(crate) span: Option<Span>,
    pub(crate) message: String,
}
