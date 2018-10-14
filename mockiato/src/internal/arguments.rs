use crate::internal::debug::MaybeDebugWrapper;
use crate::internal::matcher::{ArgumentMatcher, ArgumentsMatcher};
use std::fmt::{self, Debug};
use std::marker::PhantomData;

///
/// A function's arguments.
/// This trait is implemented for tuples with up to 12 members.
///
pub trait Arguments<'mock> {
    type Matcher: ArgumentsMatcher<'mock, Self>;

    fn debug_arguments(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! arguments_impl {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<'mock, $($T),+> Arguments<'mock> for ($($T,)+) {
                type Matcher = (
                    $(Box<ArgumentMatcher<$T> + 'mock>,)+
                );

                fn debug_arguments(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let mut builder = f.debug_tuple("");

                    $(
                        builder.field(&format!("{:?}", MaybeDebugWrapper(&self.$idx)));
                    )*

                    builder.finish()
                }
            }
        )+
    }
}

arguments_impl! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}

pub(crate) struct DebugArguments<'a, 'mock, A>(&'a A, PhantomData<&'mock ()>)
where
    A: Arguments<'mock>;

impl<'a, 'mock, A> DebugArguments<'a, 'mock, A>
where
    A: Arguments<'mock>,
{
    pub(crate) fn new(arguments: &'a A) -> Self {
        DebugArguments(arguments, PhantomData)
    }
}

impl<'a, 'mock, A> Debug for DebugArguments<'a, 'mock, A>
where
    A: Arguments<'mock>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.debug_arguments(f)
    }
}
