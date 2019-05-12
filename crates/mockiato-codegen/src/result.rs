use proc_macro2::Span;
use std::iter::FromIterator;

pub(crate) type Result<T> = std::result::Result<T, Error>;

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

#[derive(Debug)]
pub(crate) struct DiagnosticBuilder {
    span: Span,
    message: String,
    level: DiagnosticLevel,
    notes: Vec<DiagnosticMessage>,
    help: Vec<DiagnosticMessage>,
}

impl DiagnosticBuilder {
    pub(crate) fn error(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, message.into(), DiagnosticLevel::Error)
    }

    pub(crate) fn note_with_span(mut self, span: Span, message: impl Into<String>) -> Self {
        self.notes.push(DiagnosticMessage {
            span: Some(span),
            message: message.into(),
        });
        self
    }

    pub(crate) fn note(mut self, message: impl Into<String>) -> Self {
        self.notes.push(DiagnosticMessage {
            span: None,
            message: message.into(),
        });
        self
    }

    pub(crate) fn help(mut self, message: impl Into<String>) -> Self {
        self.help.push(DiagnosticMessage {
            span: None,
            message: message.into(),
        });
        self
    }

    pub(crate) fn build(self) -> Diagnostic {
        Diagnostic {
            span: self.span,
            message: self.message,
            level: self.level,
            notes: self.notes,
            help: self.help,
        }
    }

    fn new(span: Span, message: String, level: DiagnosticLevel) -> Self {
        Self {
            span,
            message,
            level,
            notes: Vec::new(),
            help: Vec::new(),
        }
    }
}

impl From<Diagnostic> for DiagnosticBuilder {
    fn from(diagnostic: Diagnostic) -> Self {
        DiagnosticBuilder {
            span: diagnostic.span,
            message: diagnostic.message,
            level: diagnostic.level,
            notes: diagnostic.notes,
            help: diagnostic.help,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) diagnostics: Vec<Diagnostic>,
}

impl FromIterator<Error> for Error {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Error>,
    {
        let diagnostics = iter
            .into_iter()
            .map(|error| error.diagnostics.into_iter())
            .flatten()
            .collect();
        Self { diagnostics }
    }
}

impl FromIterator<Diagnostic> for Error {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Diagnostic>,
    {
        let diagnostics = iter.into_iter().collect();
        Self { diagnostics }
    }
}

impl From<Diagnostic> for Error {
    fn from(diagnostic: Diagnostic) -> Error {
        Error {
            diagnostics: vec![diagnostic],
        }
    }
}

pub(crate) fn merge_results<T, I>(results: I) -> Result<impl Iterator<Item = T>>
where
    I: Iterator<Item = Result<T>>,
{
    let results: Vec<_> = results.collect();
    if results.iter().any(Result::is_err) {
        Err(results.into_iter().filter_map(Result::err).collect())
    } else {
        Ok(results.into_iter().map(Result::unwrap))
    }
}
