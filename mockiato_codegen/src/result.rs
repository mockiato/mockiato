use proc_macro::Diagnostic;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Empty,
    Diagnostic(Diagnostic),
    MultipleDiagnostics(Vec<Diagnostic>),
}

impl Error {
    pub(crate) fn emit(self) -> Self {
        self.emit_with(|d| d)
    }

    pub(crate) fn emit_with<F>(self, map_fn: F) -> Self
    where
        F: Fn(Diagnostic) -> Diagnostic,
    {
        match self {
            Error::Empty => {}
            Error::Diagnostic(diagnostic) => map_fn(diagnostic).emit(),
            Error::MultipleDiagnostics(diagnostics) => {
                diagnostics.into_iter().for_each(|d| map_fn(d).emit());
            }
        };

        Error::Empty
    }

    pub(crate) fn merge<I>(errors: I) -> Self
    where
        I: Iterator<Item = Error>,
    {
        let mut collected = Vec::new();

        errors.for_each(|err| match err {
            Error::Empty => {}
            Error::Diagnostic(diagnostic) => collected.push(diagnostic),
            Error::MultipleDiagnostics(mut diagnostics) => collected.append(&mut diagnostics),
        });

        Error::MultipleDiagnostics(collected)
    }
}

pub(crate) macro merge_results($iter: expr) {{
    let results: Vec<_> = $iter.collect();

    if results.iter().any(Result::is_err) {
        return Err(crate::Error::merge(
            results.into_iter().filter_map(Result::err),
        ));
    }

    results.into_iter().map(Result::unwrap).collect()
}}
