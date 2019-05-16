use crate::diagnostic::Diagnostic;
use std::iter::FromIterator;

pub(crate) type Result<T> = std::result::Result<T, Error>;

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
