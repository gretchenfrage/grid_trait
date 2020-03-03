//! Map-with index, aka. enumap, because it's like enumerate + map.

use crate::grid3::*;
use mint::Vector3;
use std::marker::PhantomData;

pub struct Grid3EnuMap<G, F, T, I> 
where
    G: Grid3,
    I: From<Vector3<i32>>,
    F: Fn(I, <G as Grid3>::Item) -> T,
{
    inner: G,
    func: F,
    p: PhantomData<fn(T, I)>
}

impl<G, F, T, I> Grid3EnuMap<G, F, T, I>
where
    G: Grid3,
    I: From<Vector3<i32>>,
    F: Fn(I, <G as Grid3>::Item) -> T,
{
    pub fn new(inner: G, func: F) -> Self {
        Grid3EnuMap {
            inner,
            func,
            p: PhantomData,
        }
    }
}

impl<G, F, T, I> Grid3 for Grid3EnuMap<G, F, T, I>
where
    G: Grid3,
    I: From<Vector3<i32>>,
    F: Fn(I, <G as Grid3>::Item) -> T,
{
    type Item = T;
    type XBound = <G as Grid3>::XBound;
    type YBound = <G as Grid3>::YBound;
    type ZBound = <G as Grid3>::ZBound;
    
    fn x_bound(&self) -> Self::XBound { self.inner.x_bound() }
    fn y_bound(&self) -> Self::YBound { self.inner.y_bound() }
    fn z_bound(&self) -> Self::ZBound { self.inner.z_bound() }
}

impl<G, F, T, I> Grid3Len for Grid3EnuMap<G, F, T, I>
where
    G: Grid3 + Grid3Len,
    I: From<Vector3<i32>>,
    F: Fn(I, <G as Grid3>::Item) -> T,
{}

impl<G, F, T, I> Grid3Get for Grid3EnuMap<G, F, T, I>
where
    G: Grid3 + Grid3Get,
    I: From<Vector3<i32>>,
    F: Fn(I, <G as Grid3>::Item) -> T,
{
    fn get<C>(&self, coord: C) -> Self::Item
    where
        C: Into<Vector3<i32>>
    {
        let coord = coord.into();
        (self.func)(I::from(coord), self.inner.get(coord))
    }
}
