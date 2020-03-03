//! Dependent by-value Grid2 mapping.

use crate::grid2::*;
use mint::Vector2;
use std::marker::PhantomData;

pub struct Grid2Map<G, F, T> 
where
    G: Grid2,
    F: Fn(<G as Grid2>::Item) -> T,
{
    inner: G,
    func: F,
    p: PhantomData<fn(T)>
}

impl<G, F, T> Grid2Map<G, F, T>
where
    G: Grid2,
    F: Fn(<G as Grid2>::Item) -> T,
{
    pub fn new(inner: G, func: F) -> Self {
        Grid2Map {
            inner,
            func,
            p: PhantomData,
        }
    }
}

impl<G, F, T> Grid2 for Grid2Map<G, F, T>
where
    G: Grid2,
    F: Fn(<G as Grid2>::Item) -> T,
{
    type Item = T;
    type XBound = <G as Grid2>::XBound;
    type YBound = <G as Grid2>::YBound;
    
    fn x_bound(&self) -> Self::XBound { self.inner.x_bound() }
    fn y_bound(&self) -> Self::YBound { self.inner.y_bound() }
}

impl<G, F, T> Grid2Len for Grid2Map<G, F, T>
where
    G: Grid2 + Grid2Len,
    F: Fn(<G as Grid2>::Item) -> T,
{}

impl<G, F, T> Grid2Get for Grid2Map<G, F, T>
where
    G: Grid2 + Grid2Get,
    F: Fn(<G as Grid2>::Item) -> T,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector2<i32>>
    {
        (self.func)(self.inner.get(coord))
    }
}