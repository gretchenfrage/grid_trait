//! Change to origin.

use crate::{
    grid3::*,
    range::RangeBoundsPlus,
};
use mint::Vector3;

/// <0, 0> in the inner grid will be new_origin in this grid.
pub struct Grid3NewOrigin<G> 
where
    G: Grid3,
    <G as Grid3>::XBound: RangeBoundsPlus,
    <G as Grid3>::YBound: RangeBoundsPlus,
    <G as Grid3>::ZBound: RangeBoundsPlus,
{
    inner: G,
    new_origin: Vector3<i32>,
    
    // cache these (arbitrary decision)
    new_xbound: <<G as Grid3>::XBound as RangeBoundsPlus>::Output,
    new_ybound: <<G as Grid3>::YBound as RangeBoundsPlus>::Output,
    new_zbound: <<G as Grid3>::ZBound as RangeBoundsPlus>::Output,
}

impl<G> Grid3NewOrigin<G> 
where
    G: Grid3,
    <G as Grid3>::XBound: RangeBoundsPlus,
    <G as Grid3>::YBound: RangeBoundsPlus,
    <G as Grid3>::ZBound: RangeBoundsPlus,
{
    pub fn new<I>(inner: G, new_origin: I) -> Self 
    where
        I: Into<Vector3<i32>>,
    {
        let new_origin = new_origin.into();
        let new_xbound = inner.x_bound().plus(new_origin.x);
        let new_ybound = inner.y_bound().plus(new_origin.y);
        let new_zbound = inner.z_bound().plus(new_origin.z);
        
        Grid3NewOrigin {
            inner,
            new_origin,
            new_xbound,
            new_ybound,
            new_zbound,
        }
    }
    
    pub fn new_origin<I>(&self) -> I 
    where
        I: From<Vector3<i32>>
    {
        I::from(self.new_origin)
    }
    
    pub fn adjust_coord<I>(&self, coord: I) -> I 
    where
        I: From<Vector3<i32>> + Into<Vector3<i32>>
    {
        let mut coord = coord.into();
        coord.x -= self.new_origin.x;
        coord.y -= self.new_origin.y;
        coord.z -= self.new_origin.z;
        I::from(coord)
    }
}

impl<G> Grid3 for Grid3NewOrigin<G> 
where
    G: Grid3,
    <G as Grid3>::XBound: RangeBoundsPlus,
    <G as Grid3>::YBound: RangeBoundsPlus,
    <G as Grid3>::ZBound: RangeBoundsPlus,
{
    type Item = <G as Grid3>::Item;
    type XBound = <<G as Grid3>::XBound as RangeBoundsPlus>::Output;
    type YBound = <<G as Grid3>::YBound as RangeBoundsPlus>::Output;
    type ZBound = <<G as Grid3>::ZBound as RangeBoundsPlus>::Output;
    
    fn x_bound(&self) -> Self::XBound { self.new_xbound.clone() }
    fn y_bound(&self) -> Self::YBound { self.new_ybound.clone() }
    fn z_bound(&self) -> Self::ZBound { self.new_zbound.clone() }
}

impl<G> Grid3Get for Grid3NewOrigin<G> 
where
    G: Grid3 + Grid3Get,
    <G as Grid3>::XBound: RangeBoundsPlus,
    <G as Grid3>::YBound: RangeBoundsPlus,
    <G as Grid3>::ZBound: RangeBoundsPlus,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.get(coord)
    }
}

impl<G> Grid3Set for Grid3NewOrigin<G> 
where
    G: Grid3 + Grid3Set,
    <G as Grid3>::XBound: RangeBoundsPlus,
    <G as Grid3>::YBound: RangeBoundsPlus,
    <G as Grid3>::ZBound: RangeBoundsPlus,
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.set(coord, elem);
    }
}

impl<G> Grid3Ref for Grid3NewOrigin<G> 
where
    G: Grid3 + Grid3Ref,
    <G as Grid3>::XBound: RangeBoundsPlus,
    <G as Grid3>::YBound: RangeBoundsPlus,
    <G as Grid3>::ZBound: RangeBoundsPlus,
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.idx(coord)
    }
}

impl<G> Grid3Mut for Grid3NewOrigin<G> 
where
    G: Grid3 + Grid3Mut,
    <G as Grid3>::XBound: RangeBoundsPlus,
    <G as Grid3>::YBound: RangeBoundsPlus,
    <G as Grid3>::ZBound: RangeBoundsPlus,
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.adjust_coord(coord.into());
        self.inner.midx(coord)
    }
}
