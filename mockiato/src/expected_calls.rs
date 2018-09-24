use std::ops::{Range, RangeFrom, RangeInclusive, RangeToInclusive};

pub enum ExpectedCalls {
    Any,
    Exact(u64),
    AtLeast(u64),
    AtMost(u64),
    Between(Range<u64>),
    BetweenInclusive(RangeInclusive<u64>),
}

impl From<u64> for ExpectedCalls {
    fn from(value: u64) -> ExpectedCalls {
        ExpectedCalls::Exact(value)
    }
}

impl From<RangeFrom<u64>> for ExpectedCalls {
    fn from(range: RangeFrom<u64>) -> ExpectedCalls {
        ExpectedCalls::AtLeast(range.start)
    }
}

impl From<Range<u64>> for ExpectedCalls {
    fn from(range: Range<u64>) -> ExpectedCalls {
        ExpectedCalls::Between(range)
    }
}

impl From<RangeInclusive<u64>> for ExpectedCalls {
    fn from(range: RangeInclusive<u64>) -> ExpectedCalls {
        ExpectedCalls::BetweenInclusive(range)
    }
}

impl From<RangeToInclusive<u64>> for ExpectedCalls {
    fn from(range: RangeToInclusive<u64>) -> ExpectedCalls {
        ExpectedCalls::AtMost(range.end)
    }
}
