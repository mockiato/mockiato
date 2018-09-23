use crate::matcher::{ArgumentMatcher, ArgumentsMatcher};

///
/// A function's arguments.
/// This trait is implemented for tuples with up to 12 members.
///
pub trait Arguments<'mock> {
    type Matcher: ArgumentsMatcher<'mock, Self>;
}

impl<'mock, A> Arguments<'mock> for (A,) {
    type Matcher = (Box<ArgumentMatcher<A> + 'mock>,);
}

impl<'mock, A, B> Arguments<'mock> for (A, B) {
    type Matcher = (
        Box<ArgumentMatcher<A> + 'mock>,
        Box<ArgumentMatcher<B> + 'mock>,
    );
}
