use self::partial_eq::PartialEqMatcher;
use crate::internal::arguments::Arguments;
use std::fmt::Debug;

mod partial_eq;

pub trait IntoArgumentMatcher<'a, T> {
    fn into_argument_matcher(self) -> Box<dyn ArgumentMatcher<T> + 'a>;
}

impl<'a, T> IntoArgumentMatcher<'a, T> for T
where
    T: PartialEq + 'a,
{
    default fn into_argument_matcher(self) -> Box<dyn ArgumentMatcher<T> + 'a> {
        Box::new(PartialEqMatcher::from(self))
    }
}

pub trait ArgumentMatcher<T>: Debug {
    fn matches_argument(&self, input: &T) -> bool;
}

pub trait ArgumentsMatcher<'mock>: Debug {
    type Arguments: Arguments;

    fn matches_arguments(&self, input: &Self::Arguments) -> bool;
}

impl<'mock, A> ArgumentsMatcher<'mock> for (Box<dyn ArgumentMatcher<A> + 'mock>,) {
    type Arguments = (A,);

    fn matches_arguments(&self, input: &Self::Arguments) -> bool {
        self.0.matches_argument(&input.0)
    }
}

/*macro_rules! arguments_matcher_impl {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<'mock, $($T),+> ArgumentsMatcher<'mock> for ($(Box<dyn ArgumentMatcher<$T> + 'mock>,)+) {
                type Arguments = ($($T,)+);

                fn matches_arguments(&self, input: &($($T,)+)) -> bool {
                    $(self.$idx.matches_argument(&input.$idx))&&+
                }
            }
        )+
    }
}

arguments_matcher_impl! {
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
}*/
