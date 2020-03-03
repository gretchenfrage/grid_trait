//! Kolmogorov by-mutable-reference encoding of Grid3.

use crate::grid3::*;
use mint::Vector3;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov by-mutable-reference encoding of Grid3.
/// 
/// This is a Grid3 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoMutGrid3<'a, F, I, T> 
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector3<i32>>
{
    func: F,
    p: PhantomData<fn(T, I)>,
}

impl<'a, F, I, T> KolmoMutGrid3<'a, F, I, T>
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector3<i32>>
{
    pub fn new(func: F) -> Self {
        KolmoMutGrid3 {
            func,
            p: PhantomData,
        }
    }
}

impl<'a, F, I, T> Grid3 for KolmoMutGrid3<'a, F, I, T>
where
    F: Fn(I) -> &'a mut T,
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



impl<'a, F, I, T> Grid3Mut for KolmoMutGrid3<'a, F, I, T>
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector3<i32>>
{
    fn midx<C>(&mut self, coord: C) -> &mut Self::Item
    where
        C: Into<Vector3<i32>>
    {
        (self.func)(I::from(coord.into()))
    }
}

impl<'a, F, I, T> Grid3Set for KolmoMutGrid3<'a, F, I, T> 
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector3<i32>>
{
    fn set<C: Into<Vector3<i32>>>(&mut self, coord: C, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
