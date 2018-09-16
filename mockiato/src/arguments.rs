pub trait ArgumentMatcher<T> {
    fn matches_arg(&self, value: &T) -> bool;
}

pub trait ArgumentsMatcher<T> {
    fn matches_args(&self, args: &T) -> bool;
}

impl<T> ArgumentMatcher<T> for T
where
    T: PartialEq,
{
    fn matches_arg(&self, value: &T) -> bool {
        self == value
    }
}

macro_rules! arguments_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T0:ident,$T1:ident)+
        }
    )+) => {
        $(
            impl<$($T0),+> ArgumentsMatcher<($($T0,)+)> for ($(Box<dyn ArgumentMatcher<$T0>>,)+)
            {
                fn matches_args(&self, args: &($($T0,)+)) -> bool {
                    $(self.$idx.matches_arg(&args.$idx))&&+
                }
            }
        )+
    };
}

arguments_impls!(
    Tuple1 {
        (0) -> A0, A1
    }
    Tuple2 {
        (0) -> A0, A1
        (1) -> B0, B1
    }
    Tuple3 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
    }
    Tuple4 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
    }
    Tuple5 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
    }
    Tuple6 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
        (5) -> F0, F1
    }
    Tuple7 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
        (5) -> F0, F1
        (6) -> G0, G1
    }
    Tuple8 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
        (5) -> F0, F1
        (6) -> G0, G1
        (7) -> H0, H1
    }
    Tuple9 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
        (5) -> F0, F1
        (6) -> G0, G1
        (7) -> H0, H1
        (8) -> I0, I1
    }
    Tuple10 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
        (5) -> F0, F1
        (6) -> G0, G1
        (7) -> H0, H1
        (8) -> I0, I1
        (9) -> J0, J1
    }
    Tuple11 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
        (5) -> F0, F1
        (6) -> G0, G1
        (7) -> H0, H1
        (8) -> I0, I1
        (9) -> J0, J1
        (10) -> K0, K1
    }
    Tuple12 {
        (0) -> A0, A1
        (1) -> B0, B1
        (2) -> C0, C1
        (3) -> D0, D1
        (4) -> E0, E1
        (5) -> F0, F1
        (6) -> G0, G1
        (7) -> H0, H1
        (8) -> I0, I1
        (9) -> J0, J1
        (10) -> K0, K1
        (11) -> L0, L1
    }
);
