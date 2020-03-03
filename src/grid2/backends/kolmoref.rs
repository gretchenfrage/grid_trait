//! Kolmogorov by-refernce encoding of Grid2.

use crate::grid2::*;
use mint::Vector2;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov by-reference encoding of Grid2.
/// 
/// This is a Grid2 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoRefGrid2<'a, F, I, T> 
where
    F: Fn(I) -> &'a T,
    T: 'a,
    I: From<Vector2<i32>>
{
    func: F,
    p: PhantomData<fn(T, I)>,
}

impl<'a, F, I, T> KolmoRefGrid2<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: 'a,
    I: From<Vector2<i32>>
{
    pub fn new(func: F) -> Self {
        KolmoRefGrid2 {
            func,
            p: PhantomData,
        }
    }
}

impl<'a, F, I, T> Grid2 for KolmoRefGrid2<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: 'a,
    I: From<Vector2<i32>>
{
    type Item = T;
    type XBound = RangeFull;
    type YBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
}

impl<'a, F, I, T> Grid2Ref for KolmoRefGrid2<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: 'a,
    I: From<Vector2<i32>>
{
    fn idx<C>(&self, coord: C) -> &Self::Item
    where
        C: Into<Vector2<i32>>
    {
        (self.func)(I::from(coord.into()))
    }
}

impl<'a, F, I, T> Grid2Get for KolmoRefGrid2<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: Clone + 'a,
    I: From<Vector2<i32>>
{
    fn get<C: Into<Vector2<i32>>>(&self, coord: C) -> Self::Item 
    { self.idx(coord).clone() }
}

