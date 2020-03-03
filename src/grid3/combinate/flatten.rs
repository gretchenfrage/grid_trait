//! Flattening a Grid3 of Grid3.

use crate::{
    grid3::*,
    range::RangeBoundsTimes,
};
use mint::Vector3;

/// Flattened Grid3 of Grid3.
///
/// This unfortunately cannot detect ahead-of-time bugs
/// if the inner grid has length less than the stride, 
/// so panics will probably occur if that happens.
pub struct Grid3Flat<G>
where
    G: Grid3,
    <G as Grid3>::Item: Grid3,
    <G as Grid3>::XBound: Clone,
    <G as Grid3>::YBound: Clone,
    <G as Grid3>::ZBound: Clone,
{
    doublegrid: G,
    stride: Vector3<i32>,
    new_xbound: <G as Grid3>::XBound,
    new_ybound: <G as Grid3>::YBound,
    new_zbound: <G as Grid3>::ZBound,
}

impl<G> Grid3Flat<G>
where
    G: Grid3,
    <G as Grid3>::Item: Grid3,
    <G as Grid3>::XBound: Clone,
    <G as Grid3>::YBound: Clone,
    <G as Grid3>::ZBound: Clone,
{
    pub fn new<I>(doublegrid: G, stride: I) -> Self 
    where
        I: Into<Vector3<i32>>,
        <G as Grid3>::XBound: RangeBoundsTimes,
        <G as Grid3>::YBound: RangeBoundsTimes,
        <G as Grid3>::ZBound: RangeBoundsTimes,
    {
        let stride = stride.into();
        let new_xbound = doublegrid.x_bound().times(stride.x);
        let new_ybound = doublegrid.y_bound().times(stride.y);
        let new_zbound = doublegrid.z_bound().times(stride.z);
        Grid3Flat {
            doublegrid,
            stride,
            new_xbound,
            new_ybound,
            new_zbound,
        }
    }
    
    pub fn stride<I>(&self) -> I
    where
        I: From<Vector3<i32>> 
    {
        I::from(self.stride)
    }
    
    pub fn outer_inner_coord<I>(&self, coord: I) -> (I, I)
    where
        I: Into<Vector3<i32>> + From<Vector3<i32>>
    {
        let Vector3 { x, y, z } = coord.into();

        let stride_x = self.stride.x;
        let stride_y = self.stride.y;
        let stride_z = self.stride.z;
        
        let rem_x = ((x % stride_x) + stride_x) % stride_x;
        let rem_y = ((y % stride_y) + stride_y) % stride_y;
        let rem_z = ((z % stride_z) + stride_z) % stride_z;
        
        let div_x = (x - rem_x) / stride_x;
        let div_y = (y - rem_y) / stride_y;
        let div_z = (z - rem_z) / stride_z;
        
        let rem = Vector3 { x: rem_x, y: rem_y, z: rem_z, };
        let div = Vector3 { x: div_x, y: div_y, z: div_z, };
        
        (I::from(div), I::from(rem))
    }
}

impl<G> Grid3 for Grid3Flat<G>
where
    G: Grid3,
    <G as Grid3>::Item: Grid3,
    <G as Grid3>::XBound: Clone,
    <G as Grid3>::YBound: Clone,
    <G as Grid3>::ZBound: Clone,
{
    type Item = <<G as Grid3>::Item as Grid3>::Item;
    type XBound = <G as Grid3>::XBound;
    type YBound = <G as Grid3>::YBound;
    type ZBound = <G as Grid3>::ZBound;
    
    fn x_bound(&self) -> <G as Grid3>::XBound { self.new_xbound.clone() }
    fn y_bound(&self) -> <G as Grid3>::YBound { self.new_ybound.clone() }
    fn z_bound(&self) -> <G as Grid3>::ZBound { self.new_zbound.clone() }
}

impl<G> Grid3Get for Grid3Flat<G>
where
    G: Grid3 + Grid3Ref,
    <G as Grid3>::Item: Grid3 + Grid3Get,
    <G as Grid3>::XBound: Clone,
    <G as Grid3>::YBound: Clone,
    <G as Grid3>::ZBound: Clone,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.idx(outer).get(inner)
    }
}

impl<G> Grid3Set for Grid3Flat<G>
where
    G: Grid3 + Grid3Mut,
    <G as Grid3>::Item: Grid3 + Grid3Set,
    <G as Grid3>::XBound: Clone,
    <G as Grid3>::YBound: Clone,
    <G as Grid3>::ZBound: Clone
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector3<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.midx(outer).set(inner, elem);
    }
}

impl<G> Grid3Ref for Grid3Flat<G>
where
    G: Grid3 + Grid3Ref,
    <G as Grid3>::Item: Grid3 + Grid3Ref,
    <G as Grid3>::XBound: Clone,
    <G as Grid3>::YBound: Clone,
    <G as Grid3>::ZBound: Clone
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.idx(outer).idx(inner)
    }
}

impl<G> Grid3Mut for Grid3Flat<G>
where
    G: Grid3 + Grid3Mut,
    <G as Grid3>::Item: Grid3 + Grid3Mut,
    <G as Grid3>::XBound: Clone,
    <G as Grid3>::YBound: Clone,
    <G as Grid3>::ZBound: Clone
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let (outer, inner) = self.outer_inner_coord(coord.into());
        self.doublegrid.midx(outer).midx(inner)
    }
}
