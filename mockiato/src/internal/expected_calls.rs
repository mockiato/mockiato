use std::fmt::{self, Display};
use std::ops::{Range, RangeFrom, RangeInclusive, RangeToInclusive};

#[derive(Eq, PartialEq, Debug)]
pub struct ExpectedCalls(ExpectedCallsKind);

#[derive(Eq, PartialEq, Debug)]
enum ExpectedCallsKind {
    Any,
    Exact(u64),
    AtLeast(u64),
    AtMost(u64),
    Between { start: u64, end: u64 },
    BetweenInclusive { start: u64, end: u64 },
}

impl ExpectedCalls {
    pub(crate) fn matches_value(&self, value: u64) -> bool {
        match self.0 {
            ExpectedCallsKind::Any => true,
            ExpectedCallsKind::Exact(expected) => expected == value,
            ExpectedCallsKind::AtLeast(min) => value >= min,
            ExpectedCallsKind::AtMost(max) => value <= max,
            ExpectedCallsKind::Between { start, end } => value >= start && value < end,
            ExpectedCallsKind::BetweenInclusive { start, end } => value >= start && value <= end,
        }
    }
}

impl Default for ExpectedCalls {
    fn default() -> Self {
        ExpectedCalls(ExpectedCallsKind::Any)
    }
}

impl From<u64> for ExpectedCalls {
    fn from(value: u64) -> ExpectedCalls {
        ExpectedCalls(ExpectedCallsKind::Exact(value))
    }
}

impl From<RangeFrom<u64>> for ExpectedCalls {
    fn from(range: RangeFrom<u64>) -> ExpectedCalls {
        ExpectedCalls(ExpectedCallsKind::AtLeast(range.start))
    }
}

impl From<Range<u64>> for ExpectedCalls {
    fn from(range: Range<u64>) -> ExpectedCalls {
        ExpectedCalls(ExpectedCallsKind::Between {
            start: range.start,
            end: range.end,
        })
    }
}

impl From<RangeInclusive<u64>> for ExpectedCalls {
    fn from(range: RangeInclusive<u64>) -> ExpectedCalls {
        let (start, end) = range.into_inner();

        ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start, end })
    }
}

impl From<RangeToInclusive<u64>> for ExpectedCalls {
    fn from(range: RangeToInclusive<u64>) -> ExpectedCalls {
        ExpectedCalls(ExpectedCallsKind::AtMost(range.end))
    }
}

impl Display for ExpectedCalls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ExpectedCallsKind::Any => write!(f, "any amount of times"),
            ExpectedCallsKind::AtLeast(min) => write!(f, "at least {}", DisplayTimes(min)),
            ExpectedCallsKind::AtMost(max) => write!(f, "at most {}", DisplayTimes(max)),
            ExpectedCallsKind::Between { start, end } => {
                write!(f, "between {} and {} times", start, end)
            }
            ExpectedCallsKind::BetweenInclusive { start, end } => {
                write!(f, "between {} and {} times (inclusive)", start, end)
            }
            ExpectedCallsKind::Exact(value) => write!(f, "exactly {}", DisplayTimes(value)),
        }
    }
}

struct DisplayTimes(u64);

impl Display for DisplayTimes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            1 => write!(f, "1 time"),
            _ => write!(f, "{} times", self.0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn any_matches_any_value() {
        assert!(ExpectedCalls(ExpectedCallsKind::Any).matches_value(0));
        assert!(ExpectedCalls(ExpectedCallsKind::Any).matches_value(1));
        assert!(ExpectedCalls(ExpectedCallsKind::Any).matches_value(23));
        assert!(ExpectedCalls(ExpectedCallsKind::Any).matches_value(100));
    }

    #[test]
    fn exact_matches_specified_value() {
        assert!(ExpectedCalls(ExpectedCallsKind::Exact(4)).matches_value(4));
    }

    #[test]
    fn exact_does_not_match_other_values() {
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(6)).matches_value(1));
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(6)).matches_value(0));
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(10)).matches_value(1));
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(100)).matches_value(1));
    }

    #[test]
    fn at_least_matches_minimum_value() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).matches_value(4));
    }

    #[test]
    fn at_least_matches_values_above_minimum() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).matches_value(5));
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).matches_value(10));
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).matches_value(100));
    }

    #[test]
    fn at_least_does_not_match_values_below_minimum() {
        assert!(!ExpectedCalls(ExpectedCallsKind::AtLeast(50)).matches_value(49));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtLeast(50)).matches_value(0));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtLeast(50)).matches_value(16));
    }

    #[test]
    fn at_most_matches_maximum() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).matches_value(40));
    }

    #[test]
    fn at_most_matches_values_below_maximum() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).matches_value(0));
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).matches_value(39));
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).matches_value(24));
    }

    #[test]
    fn at_most_does_not_match_values_above_maximum() {
        assert!(!ExpectedCalls(ExpectedCallsKind::AtMost(20)).matches_value(21));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtMost(20)).matches_value(67));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtMost(20)).matches_value(100));
    }

    #[test]
    fn between_does_not_match_values_outside_of_range() {
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).matches_value(0));
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).matches_value(9));
        assert!(
            !ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).matches_value(21)
        );
        assert!(
            !ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).matches_value(40)
        );
    }

    #[test]
    fn between_does_not_match_end_value() {
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 1, end: 3 }).matches_value(3));
    }

    #[test]
    fn between_matches_start_value() {
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 1, end: 3 }).matches_value(1));
    }

    #[test]
    fn between_matches_values_in_range() {
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).matches_value(11));
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).matches_value(15));
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).matches_value(19));
    }

    #[test]
    fn between_inclusive_does_not_match_values_outside_of_range() {
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 })
                .matches_value(0)
        );
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 })
                .matches_value(9)
        );
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 })
                .matches_value(21)
        );
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 })
                .matches_value(40)
        );
    }

    #[test]
    fn between_inclusive_matches_end_value() {
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 1, end: 3 })
                .matches_value(3)
        );
    }

    #[test]
    fn between_inclusive_matches_start_value() {
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 1, end: 3 })
                .matches_value(1)
        );
    }

    #[test]
    fn between_inclusive_matches_values_in_range() {
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 })
                .matches_value(11)
        );
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 })
                .matches_value(15)
        );
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 })
                .matches_value(19)
        );
    }

    #[test]
    fn can_be_converted_from_int() {
        assert_eq!(
            ExpectedCalls(ExpectedCallsKind::Exact(5)),
            ExpectedCalls::from(5)
        );
    }

    #[test]
    fn can_be_converted_from_range() {
        assert_eq!(
            ExpectedCalls(ExpectedCallsKind::Between { start: 3, end: 7 }),
            ExpectedCalls::from(3..7)
        );
    }

    #[test]
    fn can_be_converted_from_range_from() {
        assert_eq!(
            ExpectedCalls(ExpectedCallsKind::AtLeast(2)),
            ExpectedCalls::from(2..)
        );
    }

    #[test]
    fn can_be_converted_from_range_to() {
        assert_eq!(
            ExpectedCalls(ExpectedCallsKind::AtMost(4)),
            ExpectedCalls::from(..=4)
        );
    }

    #[test]
    fn can_be_converted_from_inclusive_range() {
        assert_eq!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 21, end: 56 }),
            ExpectedCalls::from(21..=56)
        )
    }

    #[test]
    fn ranges_with_bigger_start_than_end_do_not_match() {
        // The built-in RangeBounds::contains() method (https://doc.rust-lang.org/std/ops/trait.RangeBounds.html#method.contains)
        // does the same thing with these ranges.
        // See: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2015&gist=c87572feae3b49e7ad150ad1494f6042

        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 100, end: 0 })
                .matches_value(50),
        );

        assert!(
            !ExpectedCalls(ExpectedCallsKind::Between { start: 100, end: 0 }).matches_value(50),
        );
    }
}
