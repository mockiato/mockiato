use std::fmt::{self, Display};

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
