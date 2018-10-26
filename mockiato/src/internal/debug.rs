use std::fmt::{self, Debug, Display};

pub struct MaybeDebugWrapper<'a>(pub &'a dyn MaybeDebug);

impl<'a> Debug for MaybeDebugWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(self.0, f)
    }
}

pub struct MaybeDebugExtWrapper<'a>(pub &'a dyn MaybeDebugExt);

impl<'a> Debug for MaybeDebugExtWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebugExt::fmt(self.0, f)
    }
}

/// This trait makes every type [`Debug`] by falling
/// back to "?" when [`Debug`] is not implemented.
pub trait MaybeDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

/// A wrapper around [`MaybeDebug`] for container types such as [`Box`],
/// because specialization does not allow impls for `Box<T>`.
pub trait MaybeDebugExt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T> MaybeDebug for T
where
    T: ?Sized,
{
    default fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "?")
    }
}

impl<T> MaybeDebug for T
where
    T: fmt::Debug + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T> MaybeDebugExt for Box<T>
where
    T: MaybeDebug + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&**self, f)
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
