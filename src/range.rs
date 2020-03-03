/// Extensions to integer ranges.

use std::ops::{
    RangeBounds,
    Range,
    RangeFrom,
    RangeFull,
    RangeInclusive,
    RangeTo,
    RangeToInclusive,
    Bound,
};

/// Range from 0 to N.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Range0To {
    pub end: i32,
}

impl RangeBounds<i32> for Range0To {
    fn start_bound(&self) -> Bound<&i32> {
        Bound::Included(&0)
    }
    
    fn end_bound(&self) -> Bound<&i32> {
        Bound::Excluded(&self.end)
    }
}


/// Performing multiplication on RangeBounds types.
pub trait RangeBoundsTimes: Sized + Clone {
    fn times(&self, n: i32) -> Self;
}

impl RangeBoundsTimes for Range0To {
    fn times(&self, n: i32) -> Range0To {
        Range0To {
            end: self.end * n,
        }
    }
}

impl RangeBoundsTimes for Range<i32> {
    fn times(&self, n: i32) -> Self {
        Range {
            start: self.start * n,
            end: self.end * n,
        }
    }
}

impl RangeBoundsTimes for RangeFrom<i32> {
    fn times(&self, n: i32) -> Self {
        RangeFrom {
            start: self.start * n,
        }
    }
}

impl RangeBoundsTimes for RangeFull {
    fn times(&self, _n: i32) -> Self {
        RangeFull
    }
}

impl RangeBoundsTimes for RangeInclusive<i32> {
    fn times(&self, n: i32) -> Self {
        RangeInclusive::new(
            *self.start() * n,
            *self.end() * n,
        )
    }
}

impl RangeBoundsTimes for RangeTo<i32> {
    fn times(&self, n: i32) -> Self {
        RangeTo {
            end: self.end * n,
        }
    }
}

impl RangeBoundsTimes for RangeToInclusive<i32> {
    fn times(&self, n: i32) -> Self {
        RangeToInclusive {
            end: self.end * n,
        }
    }
}


/// Performing addition on RangeBounds types.
pub trait RangeBoundsPlus {
    type Output: RangeBounds<i32> + Clone;
    
    fn plus(&self, n: i32) -> Self::Output;
}

impl RangeBoundsPlus for Range0To {
    type Output = Range<i32>;
    
    fn plus(&self, n: i32) -> Range<i32> {
        Range {
            start: 1,
            end: self.end + n,
        }
    }
}

impl RangeBoundsPlus for Range<i32> {
    type Output = Self;
    
    fn plus(&self, n: i32) -> Self {
        Range {
            start: self.start + n,
            end: self.end + n,
        }
    }
}

impl RangeBoundsPlus for RangeFrom<i32> {
    type Output = Self;
    
    fn plus(&self, n: i32) -> Self {
        RangeFrom {
            start: self.start + n,
        }
    }
}

impl RangeBoundsPlus for RangeFull {
    type Output = Self;
    
    fn plus(&self, _n: i32) -> Self {
        RangeFull
    }
}

impl RangeBoundsPlus for RangeInclusive<i32> {
    type Output = Self;
    
    fn plus(&self, n: i32) -> Self {
        RangeInclusive::new(
            *self.start() + n,
            *self.end() + n,
        )
    }
}

impl RangeBoundsPlus for RangeTo<i32> {
    type Output = Self;
    
    fn plus(&self, n: i32) -> Self {
        RangeTo {
            end: self.end + n,
        }
    }
}

impl RangeBoundsPlus for RangeToInclusive<i32> {
    type Output = Self;
    
    fn plus(&self, n: i32) -> Self {
        RangeToInclusive {
            end: self.end + n,
        }
    }
}


/// A range which is not unbounded on either end.
pub trait BoundRange {
    fn lower_inclusive(&self) -> i32;
    fn upper_exclusive(&self) -> i32;
}

impl BoundRange for Range<i32> {
    fn lower_inclusive(&self) -> i32 { self.start }
    fn upper_exclusive(&self) -> i32 { self.end }
}

impl BoundRange for RangeInclusive<i32> {
    fn lower_inclusive(&self) -> i32 { *self.start() }
    fn upper_exclusive(&self) -> i32 { *self.end() - 1 }
}

impl BoundRange for Range0To {
    fn lower_inclusive(&self) -> i32 { 0 }
    fn upper_exclusive(&self) -> i32 { self.end }
}