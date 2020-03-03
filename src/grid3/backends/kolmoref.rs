//! Kolmogorov by-refernce encoding of Grid3.

use crate::grid3::*;
use mint::Vector3;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov by-reference encoding of Grid3.
/// 
/// This is a Grid3 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoRefGrid3<'a, F, I, T> 
where
    F: Fn(I) -> &'a T,
    T: 'a,
    I: From<Vector3<i32>>
{
    func: F,
    p: PhantomData<fn(T, I)>,
}

impl<'a, F, I, T> KolmoRefGrid3<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: 'a,
    I: From<Vector3<i32>>
{
    pub fn new(func: F) -> Self {
        KolmoRefGrid3 {
            func,
            p: PhantomData,
        }
    }
}

impl<'a, F, I, T> Grid3 for KolmoRefGrid3<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: 'a,
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

impl<'a, F, I, T> Grid3Ref for KolmoRefGrid3<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: 'a,
    I: From<Vector3<i32>>
{
    fn idx<C>(&self, coord: C) -> &Self::Item
    where
        C: Into<Vector3<i32>>
    {
        (self.func)(I::from(coord.into()))
    }
}

impl<'a, F, I, T> Grid3Get for KolmoRefGrid3<'a, F, I, T>
where
    F: Fn(I) -> &'a T,
    T: Clone + 'a,
    I: From<Vector3<i32>>
{
    fn get<C: Into<Vector3<i32>>>(&self, coord: C) -> Self::Item 
    { self.idx(coord).clone() }
}

