//! Grid wrapping around edges.

use crate::{
    grid3::*,
    range::BoundRange,
};
use mint::Vector3;
use std::ops::RangeFull;


pub struct Grid3Wrapping<G> 
where
    G: Grid3,
    <G as Grid3>::XBound: BoundRange,
    <G as Grid3>::YBound: BoundRange,
    <G as Grid3>::ZBound: BoundRange,
{
    inner: G,
}

impl<G> Grid3Wrapping<G>
where
    G: Grid3,
    <G as Grid3>::XBound: BoundRange,
    <G as Grid3>::YBound: BoundRange,
    <G as Grid3>::ZBound: BoundRange,
{
    pub fn new(inner: G) -> Self {
        Grid3Wrapping {
            inner
        }
    }
    
    pub fn wrap_coord<I>(&self, coord: I) -> I 
    where
        I: From<Vector3<i32>> + Into<Vector3<i32>>
    {
        let Vector3 { mut x, mut y, mut z } = coord.into();
        
        let x_start = self.inner.x_bound().lower_inclusive();
        let x_end = self.inner.x_bound().upper_exclusive();
            
        let y_start = self.inner.y_bound().lower_inclusive();
        let y_end = self.inner.y_bound().upper_exclusive();

        let z_start = self.inner.z_bound().lower_inclusive();
        let z_end = self.inner.z_bound().upper_exclusive();
                
        let x_len = x_end - x_start;
        let y_len = y_end - y_start;
        let z_len = z_end - z_start;
            
        x = ((((x - x_start) % x_len) + x_len) % x_len) + x_start;
        y = ((((y - y_start) % y_len) + y_len) % y_len) + y_start;
        z = ((((z - z_start) % z_len) + z_len) % z_len) + z_start;
        
        I::from(Vector3 { x, y, z })
    }
}

impl<G> Grid3 for Grid3Wrapping<G>
where
    G: Grid3,
    <G as Grid3>::XBound: BoundRange,
    <G as Grid3>::YBound: BoundRange,
    <G as Grid3>::ZBound: BoundRange,
{
    type Item = <G as Grid3>::Item;
    type XBound = RangeFull;
    type YBound = RangeFull;
    type ZBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
    fn z_bound(&self) -> RangeFull { RangeFull }
}

impl<G> Grid3Get for Grid3Wrapping<G> 
where
    G: Grid3 + Grid3Get,
    <G as Grid3>::XBound: BoundRange,
    <G as Grid3>::YBound: BoundRange,
    <G as Grid3>::ZBound: BoundRange,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.get(coord)
    }
}

impl<G> Grid3Set for Grid3Wrapping<G> 
where
    G: Grid3 + Grid3Set,
    <G as Grid3>::XBound: BoundRange,
    <G as Grid3>::YBound: BoundRange,
    <G as Grid3>::ZBound: BoundRange,
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.set(coord, elem);
    }
}

impl<G> Grid3Ref for Grid3Wrapping<G> 
where
    G: Grid3 + Grid3Ref,
    <G as Grid3>::XBound: BoundRange,
    <G as Grid3>::YBound: BoundRange,
    <G as Grid3>::ZBound: BoundRange,
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.idx(coord)
    }
}

impl<G> Grid3Mut for Grid3Wrapping<G> 
where
    G: Grid3 + Grid3Mut,
    <G as Grid3>::XBound: BoundRange,
    <G as Grid3>::YBound: BoundRange,
    <G as Grid3>::ZBound: BoundRange,
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = self.wrap_coord(coord.into());
        self.inner.midx(coord)
    }
}
