//! Out-of-bounds index handler.

use crate::grid3::*;
use mint::Vector3;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// All values outside of a grid are supplied by a function.
pub struct Grid3OobHandler<G, I, F>
where
    G: Grid3,
    I: From<Vector3<i32>>,
    F: Fn(I) -> <G as Grid3>::Item,
{
    inner: G,
    func: F,
    p: PhantomData<fn(I)>,
}

impl<G, I, F> Grid3OobHandler<G, I, F>
where
    G: Grid3,
    I: From<Vector3<i32>>,
    F: Fn(I) -> <G as Grid3>::Item,
{
    pub fn new(inner: G, func: F) -> Self {
        Grid3OobHandler {
            inner,
            func,
            p: PhantomData,
        }
    }
}

impl<G, I, F> Grid3 for Grid3OobHandler<G, I, F>
where
    G: Grid3,
    I: From<Vector3<i32>>,
    F: Fn(I) -> <G as Grid3>::Item,
{
    type Item = <G as Grid3>::Item;
    type XBound = RangeFull;
    type YBound = RangeFull;
    type ZBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
    fn z_bound(&self) -> RangeFull { RangeFull }
}

impl<G, I, F> Grid3Get for Grid3OobHandler<G, I, F>
where
    G: Grid3 + Grid3Get,
    I: From<Vector3<i32>>,
    F: Fn(I) -> <G as Grid3>::Item,
{
    fn get<C>(&self, coord: C) -> Self::Item
    where
        C: Into<Vector3<i32>>
    {
        let coord = coord.into();
        if self.inner.in_bounds(coord) {
            self.inner.get(coord)
        } else {
            (self.func)(I::from(coord))
        }
    }
}
