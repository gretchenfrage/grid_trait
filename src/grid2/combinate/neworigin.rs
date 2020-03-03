//! Change to origin.

use crate::{
    grid2::*,
    range::RangeBoundsPlus,
};
use mint::Vector2;

/// <0, 0> in the inner grid will be new_origin in this grid.
pub struct Grid2NewOrigin<G> 
where
    G: Grid2,
    <G as Grid2>::XBound: RangeBoundsPlus,
    <G as Grid2>::YBound: RangeBoundsPlus,
{
    inner: G,
    new_origin: Vector2<i32>,
    
    // cache these (arbitrary decision)
    new_xbound: <<G as Grid2>::XBound as RangeBoundsPlus>::Output,
    new_ybound: <<G as Grid2>::YBound as RangeBoundsPlus>::Output,
}

impl<G> Grid2NewOrigin<G> 
where
    G: Grid2,
    <G as Grid2>::XBound: RangeBoundsPlus,
    <G as Grid2>::YBound: RangeBoundsPlus,
{
    pub fn new<I>(inner: G, new_origin: I) -> Self 
    where
        I: Into<Vector2<i32>>,
    {
        let new_origin = new_origin.into();
        let new_xbound = inner.x_bound().plus(new_origin.x);
        let new_ybound = inner.y_bound().plus(new_origin.y);
        
        Grid2NewOrigin {
            inner,
            new_origin,
            new_xbound,
            new_ybound,
        }
    }
    
    pub fn new_origin<I>(&self) -> I 
    where
        I: From<Vector2<i32>>
    {
        I::from(self.new_origin)
    }
    
    pub fn adjust_coord<I>(&self, coord: I) -> I 
    where
        I: From<Vector2<i32>> + Into<Vector2<i32>>
    {
        let mut coord = coord.into();
        coord.x -= self.new_origin.x;
        coord.y -= self.new_origin.y;
        I::from(coord)
    }
}

impl<G> Grid2 for Grid2NewOrigin<G> 
where
    G: Grid2,
    <G as Grid2>::XBound: RangeBoundsPlus,
    <G as Grid2>::YBound: RangeBoundsPlus,
{
    type Item = <G as Grid2>::Item;
    type XBound = <<G as Grid2>::XBound as RangeBoundsPlus>::Output;
    type YBound = <<G as Grid2>::YBound as RangeBoundsPlus>::Output;
    
    fn x_bound(&self) -> Self::XBound { self.new_xbound.clone() }
    fn y_bound(&self) -> Self::YBound { self.new_ybound.clone() }
}

impl<G> Grid2Get for Grid2NewOrigin<G> 
where
    G: Grid2 + Grid2Get,
    <G as Grid2>::XBound: RangeBoundsPlus,
    <G as Grid2>::YBound: RangeBoundsPlus,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.get(coord)
    }
}

impl<G> Grid2Set for Grid2NewOrigin<G> 
where
    G: Grid2 + Grid2Set,
    <G as Grid2>::XBound: RangeBoundsPlus,
    <G as Grid2>::YBound: RangeBoundsPlus,
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.set(coord, elem);
    }
}

impl<G> Grid2Ref for Grid2NewOrigin<G> 
where
    G: Grid2 + Grid2Ref,
    <G as Grid2>::XBound: RangeBoundsPlus,
    <G as Grid2>::YBound: RangeBoundsPlus,
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.idx(coord)
    }
}

impl<G> Grid2Mut for Grid2NewOrigin<G> 
where
    G: Grid2 + Grid2Mut,
    <G as Grid2>::XBound: RangeBoundsPlus,
    <G as Grid2>::YBound: RangeBoundsPlus,
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.midx(coord)
    }
}