//! Flattening a Grid2 of Grid2.

use crate::{
    grid2::*,
    range::RangeBoundsTimes,
};
use mint::Vector2;

/// Flattened Grid2 of Grid2.
///
/// This unfortunately cannot detect ahead-of-time bugs
/// if the inner grid has length less than the stride, 
/// so panics will probably occur if that happens.
pub struct Grid2Flat<G>
where
    G: Grid2,
    <G as Grid2>::Item: Grid2,
    <G as Grid2>::XBound: Clone,
    <G as Grid2>::YBound: Clone,
{
    doublegrid: G,
    stride: Vector2<i32>,
    new_xbound: <G as Grid2>::XBound,
    new_ybound: <G as Grid2>::YBound,
}

impl<G> Grid2Flat<G>
where
    G: Grid2,
    <G as Grid2>::Item: Grid2,
    <G as Grid2>::XBound: Clone,
    <G as Grid2>::YBound: Clone,
{
    pub fn new<I>(doublegrid: G, stride: I) -> Self 
    where
        I: Into<Vector2<i32>>,
        <G as Grid2>::XBound: RangeBoundsTimes,
        <G as Grid2>::YBound: RangeBoundsTimes,
    {
        let stride = stride.into();
        let new_xbound = doublegrid.x_bound().times(stride.x);
        let new_ybound = doublegrid.y_bound().times(stride.y);
        Grid2Flat {
            doublegrid,
            stride,
            new_xbound,
            new_ybound,
        }
    }
    
    pub fn stride<I>(&self) -> I
    where
        I: From<Vector2<i32>> 
    {
        I::from(self.stride)
    }
    
    pub fn outer_inner_coord<I>(&self, coord: I) -> (I, I)
    where
        I: Into<Vector2<i32>> + From<Vector2<i32>>
    {
        let Vector2 { x, y } = coord.into();

        let stride_x = self.stride.x;
        let stride_y = self.stride.y;
        
        let rem_x = ((x % stride_x) + stride_x) % stride_x;
        let rem_y = ((y % stride_y) + stride_y) % stride_y;
        
        let div_x = (x - rem_x) / stride_x;
        let div_y = (y - rem_y) / stride_y;
        
        let rem = Vector2 { x: rem_x, y: rem_y };
        let div = Vector2 { x: div_x, y: div_y };
        
        (I::from(div), I::from(rem))
    }
}

impl<G> Grid2 for Grid2Flat<G>
where
    G: Grid2,
    <G as Grid2>::Item: Grid2,
    <G as Grid2>::XBound: Clone,
    <G as Grid2>::YBound: Clone,
{
    type Item = <<G as Grid2>::Item as Grid2>::Item;
    type XBound = <G as Grid2>::XBound;
    type YBound = <G as Grid2>::YBound;
    
    fn x_bound(&self) -> <G as Grid2>::XBound { self.new_xbound.clone() }
    fn y_bound(&self) -> <G as Grid2>::YBound { self.new_ybound.clone() }
}

impl<G> Grid2Get for Grid2Flat<G>
where
    G: Grid2 + Grid2Ref,
    <G as Grid2>::Item: Grid2 + Grid2Get,
    <G as Grid2>::XBound: Clone,
    <G as Grid2>::YBound: Clone,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.idx(outer).get(inner)
    }
}

impl<G> Grid2Set for Grid2Flat<G>
where
    G: Grid2 + Grid2Mut,
    <G as Grid2>::Item: Grid2 + Grid2Set,
    <G as Grid2>::XBound: Clone,
    <G as Grid2>::YBound: Clone,
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector2<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.midx(outer).set(inner, elem);
    }
}

impl<G> Grid2Ref for Grid2Flat<G>
where
    G: Grid2 + Grid2Ref,
    <G as Grid2>::Item: Grid2 + Grid2Ref,
    <G as Grid2>::XBound: Clone,
    <G as Grid2>::YBound: Clone,
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.idx(outer).idx(inner)
    }
}

impl<G> Grid2Mut for Grid2Flat<G>
where
    G: Grid2 + Grid2Mut,
    <G as Grid2>::Item: Grid2 + Grid2Mut,
    <G as Grid2>::XBound: Clone,
    <G as Grid2>::YBound: Clone,
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.midx(outer).midx(inner)
    }
}
