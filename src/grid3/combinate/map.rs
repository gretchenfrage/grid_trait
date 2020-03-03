//! Dependent by-value Grid3 mapping.

use crate::grid3::*;
use mint::Vector3;
use std::marker::PhantomData;

pub struct Grid3Map<G, F, T> 
where
    G: Grid3,
    F: Fn(<G as Grid3>::Item) -> T,
{
    inner: G,
    func: F,
    p: PhantomData<fn(T)>
}

impl<G, F, T> Grid3Map<G, F, T>
where
    G: Grid3,
    F: Fn(<G as Grid3>::Item) -> T,
{
    pub fn new(inner: G, func: F) -> Self {
        Grid3Map {
            inner,
            func,
            p: PhantomData,
        }
    }
}

impl<G, F, T> Grid3 for Grid3Map<G, F, T>
where
    G: Grid3,
    F: Fn(<G as Grid3>::Item) -> T,
{
    type Item = T;
    type XBound = <G as Grid3>::XBound;
    type YBound = <G as Grid3>::YBound;
    type ZBound = <G as Grid3>::ZBound;
    
    fn x_bound(&self) -> Self::XBound { self.inner.x_bound() }
    fn y_bound(&self) -> Self::YBound { self.inner.y_bound() }
    fn z_bound(&self) -> Self::ZBound { self.inner.z_bound() }
}

impl<G, F, T> Grid3Len for Grid3Map<G, F, T>
where
    G: Grid3 + Grid3Len,
    F: Fn(<G as Grid3>::Item) -> T,
{}

impl<G, F, T> Grid3Get for Grid3Map<G, F, T>
where
    G: Grid3 + Grid3Get,
    F: Fn(<G as Grid3>::Item) -> T,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector3<i32>>
    {
        (self.func)(self.inner.get(coord))
    }
}
