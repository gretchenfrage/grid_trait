//! Out-of-bounds index handler.

use crate::grid2::*;
use mint::Vector2;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// All values outside of a grid are supplied by a function.
pub struct Grid2OobHandler<G, I, F> 
where
    G: Grid2,
    I: From<Vector2<i32>>,
    F: Fn(I) -> <G as Grid2>::Item,
{
    inner: G,
    func: F,
    p: PhantomData<fn(I)>,
}

impl<G, I, F> Grid2OobHandler<G, I, F>
where
    G: Grid2,
    I: From<Vector2<i32>>,
    F: Fn(I) -> <G as Grid2>::Item,
{
    pub fn new(inner: G, func: F) -> Self {
        Grid2OobHandler {
            inner,
            func,
            p: PhantomData,
        }
    }
}

impl<G, I, F> Grid2 for Grid2OobHandler<G, I, F>
where
    G: Grid2,
    I: From<Vector2<i32>>,
    F: Fn(I) -> <G as Grid2>::Item,
{
    type Item = <G as Grid2>::Item;
    type XBound = RangeFull;
    type YBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
}

impl<G, I, F> Grid2Get for Grid2OobHandler<G, I, F>
where
    G: Grid2 + Grid2Get,
    I: From<Vector2<i32>>,
    F: Fn(I) -> <G as Grid2>::Item,
{
    fn get<C>(&self, coord: C) -> Self::Item
    where
        C: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.inner.in_bounds(coord) {
            self.inner.get(coord)
        } else {
            (self.func)(I::from(coord))
        }
    }
}