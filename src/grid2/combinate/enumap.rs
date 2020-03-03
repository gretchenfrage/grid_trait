//! Map-with index, aka. enumap, because it's like enumerate + map.

use crate::grid2::*;
use mint::Vector2;
use std::marker::PhantomData;

pub struct Grid2EnuMap<G, F, T, I> 
where
    G: Grid2,
    I: From<Vector2<i32>>,
    F: Fn(I, <G as Grid2>::Item) -> T,
{
    inner: G,
    func: F,
    p: PhantomData<fn(T, I)>
}

impl<G, F, T, I> Grid2EnuMap<G, F, T, I>
where
    G: Grid2,
    I: From<Vector2<i32>>,
    F: Fn(I, <G as Grid2>::Item) -> T,
{
    pub fn new(inner: G, func: F) -> Self {
        Grid2EnuMap {
            inner,
            func,
            p: PhantomData,
        }
    }
}

impl<G, F, T, I> Grid2 for Grid2EnuMap<G, F, T, I>
where
    G: Grid2,
    I: From<Vector2<i32>>,
    F: Fn(I, <G as Grid2>::Item) -> T,
{
    type Item = T;
    type XBound = <G as Grid2>::XBound;
    type YBound = <G as Grid2>::YBound;
    
    fn x_bound(&self) -> Self::XBound { self.inner.x_bound() }
    fn y_bound(&self) -> Self::YBound { self.inner.y_bound() }
}

impl<G, F, T, I> Grid2Len for Grid2EnuMap<G, F, T, I>
where
    G: Grid2 + Grid2Len,
    I: From<Vector2<i32>>,
    F: Fn(I, <G as Grid2>::Item) -> T,
{}

impl<G, F, T, I> Grid2Get for Grid2EnuMap<G, F, T, I>
where
    G: Grid2 + Grid2Get,
    I: From<Vector2<i32>>,
    F: Fn(I, <G as Grid2>::Item) -> T,
{
    fn get<C>(&self, coord: C) -> Self::Item
    where
        C: Into<Vector2<i32>>
    {
        let coord = coord.into();
        (self.func)(I::from(coord), self.inner.get(coord))
    }
}