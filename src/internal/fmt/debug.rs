use std::fmt::{self, Debug};

/// A wrapper around a [`MaybeDebug`] type that implements [`Debug`].
pub struct MaybeDebugWrapper<'a>(pub &'a dyn MaybeDebug);

impl<'a> Debug for MaybeDebugWrapper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(self.0, f)
    }
}

/// This trait makes every type [`Debug`] by falling
/// back to "?" when [`Debug`] is not implemented.
pub trait MaybeDebug {
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
