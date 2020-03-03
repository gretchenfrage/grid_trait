//! Kolmogorov by-mutable-reference encoding of Grid2.

use crate::grid2::*;
use mint::Vector2;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov by-mutable-reference encoding of Grid2.
/// 
/// This is a Grid2 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoMutGrid2<'a, F, I, T> 
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector2<i32>>
{
    func: F,
    p: PhantomData<fn(T, I)>,
}

impl<'a, F, I, T> KolmoMutGrid2<'a, F, I, T>
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector2<i32>>
{
    pub fn new(func: F) -> Self {
        KolmoMutGrid2 {
            func,
            p: PhantomData,
        }
    }
}

impl<'a, F, I, T> Grid2 for KolmoMutGrid2<'a, F, I, T>
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector2<i32>>
{
    type Item = T;
    type XBound = RangeFull;
    type YBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
}



impl<'a, F, I, T> Grid2Mut for KolmoMutGrid2<'a, F, I, T>
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector2<i32>>
{
    fn midx<C>(&mut self, coord: C) -> &mut Self::Item
    where
        C: Into<Vector2<i32>>
    {
        (self.func)(I::from(coord.into()))
    }
}

impl<'a, F, I, T> Grid2Set for KolmoMutGrid2<'a, F, I, T> 
where
    F: Fn(I) -> &'a mut T,
    T: 'a,
    I: From<Vector2<i32>>
{
    fn set<C: Into<Vector2<i32>>>(&mut self, coord: C, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
