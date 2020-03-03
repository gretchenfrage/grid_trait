//! Sub-view of a Grid2.

use crate::{
    range::Range0To,
    grid2::*,
};
use mint::Vector2;
use std::{
    ops::{RangeBounds, Bound},
    fmt::Debug,
};


/// Sub-view of a Grid2.
///
/// The valid coordinates in this grid are a subset
/// of the valid coordinates in the inner grid.
pub struct Grid2Slice<G, X, Y> 
where
    G: Grid2,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
{
    inner: G,
    x_bound: X,
    y_bound: Y,
}

/// Verify that a is more strict than b.
fn more_strict(a: impl RangeBounds<i32>, b: impl RangeBounds<i32>) -> bool {
    
    fn lower_inclusive(bound: Bound<&i32>) -> Option<i32> {
        match bound {
            Bound::Included(&i) => Some(i),
            Bound::Excluded(&i) => Some(i + 1),
            Bound::Unbounded   => None,
        }
    }
    
    fn upper_inclusive(bound: Bound<&i32>) -> Option<i32> {
        match bound {
            Bound::Included(&i) => Some(i),
            Bound::Excluded(&i) => Some(i - 1),
            Bound::Unbounded   => None,
        }
    }
    
    let lower_ok = match (
        lower_inclusive(a.start_bound()), 
        lower_inclusive(b.start_bound()),
    ) {
        (Some(i1), Some(i2)) => i1 >= i2,
        (Some(_), None)      => true,
        (None, Some(_))      => false,
        (None, None)         => true,
    };
    if !lower_ok { return false; }
    
    let upper_ok = match (
        upper_inclusive(a.end_bound()), 
        upper_inclusive(b.end_bound()),
    ) {
        (Some(i1), Some(i2)) => i1 <= i2,
        (Some(_), None)      => true,
        (None, Some(_))      => false,
        (None, None)         => true,
    };
    if !upper_ok { return false; }
    
    true
}

impl<G, X, Y> Grid2Slice<G, X, Y>
where
    G: Grid2,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
{
    /// Fails if the new bounds are not a subset of the old ones.
    pub fn try_new(inner: G, new_x: X, new_y: Y) -> Result<Self, G>
    {
        if more_strict(new_x.clone(), inner.x_bound()) 
            && more_strict(new_y.clone(), inner.y_bound()) {
            Ok(Grid2Slice {
                inner,
                x_bound: new_x,
                y_bound: new_y,
            })
        } else {
            Err(inner)
        }
    }
    
    /// Panics if the new bounds are not a subset of the old ones.
    pub fn new(inner: G, new_x: X, new_y: Y) -> Self
    where
        X: Debug,
        Y: Debug,
        <G as Grid2>::XBound: Debug,
        <G as Grid2>::YBound: Debug,
    {
        if more_strict(new_x.clone(), inner.x_bound()) 
            && more_strict(new_y.clone(), inner.y_bound()) {
            Grid2Slice {
                inner,
                x_bound: new_x,
                y_bound: new_y,
            }
        } else {
            panic!("new bounds are not a subset of old bounds, new={:?}, old={:?}",
                (new_x, new_y),
                (inner.x_bound(), inner.y_bound()));
        }
    }
}

impl<G, X, Y> Grid2 for Grid2Slice<G, X, Y> 
where
    G: Grid2,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
{
    type Item = <G as Grid2>::Item;
    type XBound = X;
    type YBound = Y;
    
    fn x_bound(&self) -> Self::XBound { self.x_bound.clone() }
    fn y_bound(&self) -> Self::YBound { self.y_bound.clone() }
}

impl<G> Grid2Len for Grid2Slice<G, Range0To, Range0To> 
where
    G: Grid2
{}

impl<G, X, Y> Grid2Get for Grid2Slice<G, X, Y>
where
    G: Grid2 + Grid2Get,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        match self.try_get(coord) {
            Some(item) => item,
            None => panic!("invalid index {:?}", coord),
        }
    }
        
    fn try_get<I>(&self, coord: I) -> Option<Self::Item>
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.inner.get(coord))
        } else {
            None
        }
    }
}

impl<G, X, Y> Grid2Set for Grid2Slice<G, X, Y>
where
    G: Grid2 + Grid2Set,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        match self.try_set(coord, elem) {
            Ok(_) => {},
            Err(_) => panic!("invalid index {:?}", coord),
        }
    }
        
    fn try_set<I>(&mut self, coord: I, elem: Self::Item) -> Result<(), Self::Item> 
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            self.inner.set(coord, elem);
            Ok({})
        } else {
            Err(elem)
        }
    }
}

impl<G, X, Y> Grid2Ref for Grid2Slice<G, X, Y>
where
    G: Grid2 + Grid2Ref,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        match self.try_idx(coord) {
            Some(item) => item,
            None => panic!("invalid index {:?}", coord),
        }
    }
    
    fn try_idx<I>(&self, coord: I) -> Option<&Self::Item>
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.inner.idx(coord))
        } else {
            None
        }
    }
}

impl<G, X, Y> Grid2Mut for Grid2Slice<G, X, Y>
where
    G: Grid2 + Grid2Mut,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        match self.try_midx(coord) {
            Some(item) => item,
            None => panic!("invalid index {:?}", coord),
        }
    }
    
    fn try_midx<I>(&mut self, coord: I) -> Option<&mut Self::Item>
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.inner.midx(coord))
        } else {
            None
        }
    }
}
