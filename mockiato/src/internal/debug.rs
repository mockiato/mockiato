use std::fmt::{self, Debug, Display};

pub struct MaybeDebugWrapper<'a>(pub &'a dyn MaybeDebug);

impl<'a> Debug for MaybeDebugWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(self.0, f)
    }
}

pub trait MaybeDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T> MaybeDebug for T {
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "?")
    }
}

impl<T> MaybeDebug for T
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub(crate) struct DisplayOption<'a, D>(pub(crate) Option<&'a D>)
where
    D: Display;

impl<'a, D> Display for DisplayOption<'a, D>
where
    D: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}
