use crate::internal::fmt::DisplayTimes;
use std::fmt::{self, Display};
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

/// Defines how often a method call is expected
/// to be called.
/// See [`MethodCallBuilder::times`] on how to use this.
///
/// [`MethodCallBuilder::times`]: crate::MethodCallBuilder::times
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ExpectedCalls(ExpectedCallsKind);

#[derive(Eq, PartialEq, Debug, Clone)]
enum ExpectedCallsKind {
    Exact(u64),
    AtLeast(u64),
    AtMost(u64),
    Between { start: u64, end: u64 },
    BetweenInclusive { start: u64, end: u64 },
    Any,
}

impl ExpectedCalls {
    pub(crate) fn contains(&self, value: u64) -> bool {
        match self.0 {
            ExpectedCallsKind::Exact(expected) => expected == value,
            ExpectedCallsKind::AtLeast(min) => value >= min,
            ExpectedCallsKind::AtMost(max) => value <= max,
            ExpectedCallsKind::Between { start, end } => value >= start && value < end,
            ExpectedCallsKind::BetweenInclusive { start, end } => value >= start && value <= end,
            ExpectedCallsKind::Any => true,
        }
    }

    pub(crate) fn max_value(&self) -> Option<u64> {
        match self.0 {
            ExpectedCallsKind::Exact(expected) => Some(expected),
            ExpectedCallsKind::AtMost(max) => Some(max),
            ExpectedCallsKind::Between { end, .. } => Some(end.saturating_sub(1)),
            ExpectedCallsKind::BetweenInclusive { end, .. } => Some(end),
            _ => None,
        }
    }
}

impl Default for ExpectedCalls {
    fn default() -> Self {
        ExpectedCalls(ExpectedCallsKind::Exact(1))
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

impl From<RangeFull> for ExpectedCalls {
    fn from(_: RangeFull) -> ExpectedCalls {
        ExpectedCalls(ExpectedCallsKind::Any)
    }
}

impl Display for ExpectedCalls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ExpectedCallsKind::AtLeast(min) => match min {
                _ if min == 0 => write!(f, "any amount of times"),
                _ => write!(f, "at least {}", DisplayTimes(min)),
            },
            ExpectedCallsKind::AtMost(max) => write!(f, "at most {}", DisplayTimes(max)),
            ExpectedCallsKind::Between { start, end } => {
                write!(f, "between {} and {} times", start, end)
            }
            ExpectedCallsKind::BetweenInclusive { start, end } => {
                write!(f, "between {} and {} times (inclusive)", start, end)
            }
            ExpectedCallsKind::Exact(value) => write!(f, "exactly {}", DisplayTimes(value)),
            ExpectedCallsKind::Any => write!(f, "any amount of times"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exact_matches_specified_value() {
        assert!(ExpectedCalls(ExpectedCallsKind::Exact(4)).contains(4));
    }

    #[test]
    fn exact_does_not_match_other_values() {
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(6)).contains(1));
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(6)).contains(0));
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(10)).contains(1));
        assert!(!ExpectedCalls(ExpectedCallsKind::Exact(100)).contains(1));
    }

    #[test]
    fn at_least_matches_minimum_value() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).contains(4));
    }

    #[test]
    fn at_least_matches_values_above_minimum() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).contains(5));
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).contains(10));
        assert!(ExpectedCalls(ExpectedCallsKind::AtLeast(4)).contains(100));
    }

    #[test]
    fn at_least_does_not_match_values_below_minimum() {
        assert!(!ExpectedCalls(ExpectedCallsKind::AtLeast(50)).contains(49));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtLeast(50)).contains(0));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtLeast(50)).contains(16));
    }

    #[test]
    fn at_most_matches_maximum() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).contains(40));
    }

    #[test]
    fn at_most_matches_values_below_maximum() {
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).contains(0));
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).contains(39));
        assert!(ExpectedCalls(ExpectedCallsKind::AtMost(40)).contains(24));
    }

    #[test]
    fn at_most_does_not_match_values_above_maximum() {
        assert!(!ExpectedCalls(ExpectedCallsKind::AtMost(20)).contains(21));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtMost(20)).contains(67));
        assert!(!ExpectedCalls(ExpectedCallsKind::AtMost(20)).contains(100));
    }

    #[test]
    fn between_does_not_match_values_outside_of_range() {
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).contains(0));
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).contains(9));
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).contains(21));
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).contains(40));
    }

    #[test]
    fn between_does_not_match_end_value() {
        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 1, end: 3 }).contains(3));
    }

    #[test]
    fn between_matches_start_value() {
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 1, end: 3 }).contains(1));
    }

    #[test]
    fn between_matches_values_in_range() {
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).contains(11));
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).contains(15));
        assert!(ExpectedCalls(ExpectedCallsKind::Between { start: 10, end: 20 }).contains(19));
    }

    #[test]
    fn between_inclusive_does_not_match_values_outside_of_range() {
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 }).contains(0)
        );
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 }).contains(9)
        );
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 }).contains(21)
        );
        assert!(
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 }).contains(40)
        );
    }

    #[test]
    fn between_inclusive_matches_end_value() {
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 1, end: 3 }).contains(3)
        );
    }

    #[test]
    fn between_inclusive_matches_start_value() {
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 1, end: 3 }).contains(1)
        );
    }

    #[test]
    fn between_inclusive_matches_values_in_range() {
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 }).contains(11)
        );
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 }).contains(15)
        );
        assert!(
            ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 10, end: 20 }).contains(19)
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
            !ExpectedCalls(ExpectedCallsKind::BetweenInclusive { start: 100, end: 0 }).contains(50),
        );

        assert!(!ExpectedCalls(ExpectedCallsKind::Between { start: 100, end: 0 }).contains(50),);
    }

    #[test]
    fn between_inclusive_matches_when_start_equals_end() {
        assert!(ExpectedCalls(ExpectedCallsKind::BetweenInclusive {
            start: 100,
            end: 100
        })
        .contains(100));
    }

    #[test]
    fn between_does_not_match_when_start_equals_end() {
        assert!(!ExpectedCalls(ExpectedCallsKind::Between {
            start: 100,
            end: 100
        })
        .contains(100));
    }

    #[test]
    fn can_be_converted_from_full_range() {
        assert_eq!(
            ExpectedCalls(ExpectedCallsKind::Any),
            ExpectedCalls::from(..)
        )
    }

    #[test]
    fn at_least_is_displayed_correctly() {
        let formatted = format!("{}", ExpectedCalls::from(5..));

        assert_eq!("at least 5 times", formatted);
    }

    #[test]
    fn at_least_zero_is_displayed_correctly() {
        let formatted = format!("{}", ExpectedCalls::from(0..));

        assert_eq!("any amount of times", formatted);
    }

    #[test]
    fn any_is_displayed_correctly() {
        let formatted = format!("{}", ExpectedCalls::from(..));

        assert_eq!("any amount of times", formatted);
    }

    #[test]
    fn exact_has_max_value() {
        assert_eq!(Some(6), ExpectedCalls::from(6).max_value(),);
    }

    #[test]
    fn at_most_has_max_value() {
        assert_eq!(Some(6), ExpectedCalls::from(..=6).max_value(),);
    }

    #[test]
    fn between_has_max_value() {
        assert_eq!(Some(1), ExpectedCalls::from(1..2).max_value(),);
    }

    #[test]
    fn between_max_value_does_not_underflow() {
        assert_eq!(Some(0), ExpectedCalls::from(0..0).max_value(),);
    }

    #[test]
    fn between_inclusive_has_max_value() {
        assert_eq!(Some(2), ExpectedCalls::from(1..=2).max_value(),);
    }

    #[test]
    fn at_least_has_no_max_value() {
        assert!(ExpectedCalls::from(1..).max_value().is_none());
    }

    #[test]
    fn any_has_no_max_value() {
        assert!(ExpectedCalls::from(..).max_value().is_none());
    }
}
