//! Kolmogorov encoding of Grid3.

use crate::grid3::*;
use mint::Vector3;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov encoding of Grid3.
/// 
/// This is a Grid3 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoGrid3<F, I, T> 
where
    F: Fn(I) -> T,
    I: From<Vector3<i32>>
{
    func: F,
    p: PhantomData<fn(T, I)>,
}

impl<F, I, T> KolmoGrid3<F, I, T>
where
    F: Fn(I) -> T,
    I: From<Vector3<i32>>
{
    pub fn new(func: F) -> Self {
        KolmoGrid3 {
            func,
            p: PhantomData,
        }
    }
}

impl<F, I, T> Grid3 for KolmoGrid3<F, I, T>
where
    F: Fn(I) -> T,
    I: From<Vector3<i32>>
{
    type Item = T;
    type XBound = RangeFull;
    type YBound = RangeFull;
    type ZBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
    fn z_bound(&self) -> RangeFull { RangeFull }
}

impl<F, I, T> Grid3Get for KolmoGrid3<F, I, T>
where
    F: Fn(I) -> T,
    I: From<Vector3<i32>>
{
    fn get<C>(&self, coord: C) -> Self::Item
    where
        C: Into<Vector3<i32>>
    {
        (self.func)(I::from(coord.into()))
    }
}
