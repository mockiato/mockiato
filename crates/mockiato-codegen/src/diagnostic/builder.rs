use super::{Diagnostic, DiagnosticLevel, DiagnosticMessage};
use proc_macro2::Span;

#[derive(Debug)]
pub(crate) struct DiagnosticBuilder {
    diagnostic: Diagnostic,
}

impl DiagnosticBuilder {
    pub(crate) fn error(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, message.into(), DiagnosticLevel::Error)
    }

    pub(crate) fn note_with_span(mut self, span: Span, message: impl Into<String>) -> Self {
        self.diagnostic.notes.push(DiagnosticMessage {
            span: Some(span),
            message: message.into(),
        });
        self
    }

    pub(crate) fn note(mut self, message: impl Into<String>) -> Self {
        self.diagnostic.notes.push(DiagnosticMessage {
            span: None,
            message: message.into(),
        });
        self
    }

    pub(crate) fn help(mut self, message: impl Into<String>) -> Self {
        self.diagnostic.help.push(DiagnosticMessage {
            span: None,
            message: message.into(),
        });
        self
    }

    pub(crate) fn build(self) -> Diagnostic {
        self.diagnostic
    }

    fn new(span: Span, message: String, level: DiagnosticLevel) -> Self {
        Self {
            diagnostic: Diagnostic {
                span,
                message,
                level,
                notes: Vec::new(),
                help: Vec::new(),
            },
        }
    }
}

impl From<Diagnostic> for DiagnosticBuilder {
    fn from(diagnostic: Diagnostic) -> Self {
        DiagnosticBuilder { diagnostic }
    }
}
