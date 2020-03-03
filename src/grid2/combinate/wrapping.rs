//! Grid wrapping around edges.

use crate::{
    grid2::*,
    range::Range0To,
};
use mint::Vector2;
use std::ops::{
    Range, 
    RangeInclusive, 
    RangeFull
};

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


pub struct Grid2Wrapping<G> 
where
    G: Grid2,
    <G as Grid2>::XBound: BoundRange,
    <G as Grid2>::YBound: BoundRange,
{
    inner: G,
}

impl<G> Grid2Wrapping<G>
where
    G: Grid2,
    <G as Grid2>::XBound: BoundRange,
    <G as Grid2>::YBound: BoundRange,
{
    pub fn new(inner: G) -> Self {
        Grid2Wrapping {
            inner
        }
    }
    
    pub fn wrap_coord<I>(&self, coord: I) -> I 
    where
        I: From<Vector2<i32>> + Into<Vector2<i32>>
    {
        let Vector2 { mut x, mut y } = coord.into();
        
        let x_start = self.inner.x_bound().lower_inclusive();
        let x_end = self.inner.x_bound().upper_exclusive();
            
        let y_start = self.inner.y_bound().lower_inclusive();
        let y_end = self.inner.y_bound().upper_exclusive();
                
        let x_len = x_end - x_start;
        let y_len = y_end - y_start;
            
        x = ((((x - x_start) % x_len) + x_len) % x_len) + x_start;
        y = ((((y - y_start) % y_len) + y_len) % y_len) + y_start;
        
        I::from(Vector2 { x, y })
    }
}

impl<G> Grid2 for Grid2Wrapping<G>
where
    G: Grid2,
    <G as Grid2>::XBound: BoundRange,
    <G as Grid2>::YBound: BoundRange,
{
    type Item = <G as Grid2>::Item;
    type XBound = RangeFull;
    type YBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
}

impl<G> Grid2Get for Grid2Wrapping<G> 
where
    G: Grid2 + Grid2Get,
    <G as Grid2>::XBound: BoundRange,
    <G as Grid2>::YBound: BoundRange,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.get(coord)
    }
}

impl<G> Grid2Set for Grid2Wrapping<G> 
where
    G: Grid2 + Grid2Set,
    <G as Grid2>::XBound: BoundRange,
    <G as Grid2>::YBound: BoundRange,
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.set(coord, elem);
    }
}

impl<G> Grid2Ref for Grid2Wrapping<G> 
where
    G: Grid2 + Grid2Ref,
    <G as Grid2>::XBound: BoundRange,
    <G as Grid2>::YBound: BoundRange,
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.idx(coord)
    }
}

impl<G> Grid2Mut for Grid2Wrapping<G> 
where
    G: Grid2 + Grid2Mut,
    <G as Grid2>::XBound: BoundRange,
    <G as Grid2>::YBound: BoundRange,
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.midx(coord)
    }
}
