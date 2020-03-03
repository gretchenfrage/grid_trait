//! Kolmogorov encoding of Grid2.

use crate::grid2::*;
use mint::Vector2;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov encoding of Grid2.
/// 
/// This is a Grid2 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoGrid2<F, I, T> 
where
    F: Fn(I) -> T,
    I: From<Vector2<i32>>
{
    func: F,
    p: PhantomData<fn(T, I)>,
}

impl<F, I, T> KolmoGrid2<F, I, T>
where
    F: Fn(I) -> T,
    I: From<Vector2<i32>>
{
    pub fn new(func: F) -> Self {
        KolmoGrid2 {
            func,
            p: PhantomData,
        }
    }
}

impl<F, I, T> Grid2 for KolmoGrid2<F, I, T>
where
    F: Fn(I) -> T,
    I: From<Vector2<i32>>
{
    type Item = T;
    type XBound = RangeFull;
    type YBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
}

impl<F, I, T> Grid2Get for KolmoGrid2<F, I, T>
where
    F: Fn(I) -> T,
    I: From<Vector2<i32>>
{
    fn get<C>(&self, coord: C) -> Self::Item
    where
        C: Into<Vector2<i32>>
    {
        (self.func)(I::from(coord.into()))
    }
}