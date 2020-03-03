
use std::ops::{RangeBounds, Bound};

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