//! Kolmogorov by-refernce read/write encoding of Grid2.

use crate::grid2::*;
use mint::Vector2;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov by-reference read/write encoding of Grid2.
/// 
/// This is the most powerful kolmogorov borrowing type,
/// because it contains separate functions for the 
/// exclusive and shared access pathways, and holds a
/// single datum which they both reference.
///
/// This is a Grid2 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoRwGrid2<I, R, T, Fr, Fw> 
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector2<i32>>
{
    referent: R,
    reader: Fr,
    writer: Fw,
    p: PhantomData<fn(T, I)>,
}

impl<I, R, T, Fr, Fw> KolmoRwGrid2<I, R, T, Fr, Fw> 
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector2<i32>>
{
    pub fn new(referent: R, reader: Fr, writer: Fw) -> Self {
        KolmoRwGrid2 {
            referent,
            reader,
            writer,
            p: PhantomData,
        }
    }
}

impl<I, R, T, Fr, Fw> Grid2 for KolmoRwGrid2<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector2<i32>>
{
    type Item = T;
    type XBound = RangeFull;
    type YBound = RangeFull;
    
    fn x_bound(&self) -> RangeFull { RangeFull }
    fn y_bound(&self) -> RangeFull { RangeFull }
}

impl<I, R, T, Fr, Fw> Grid2Ref for KolmoRwGrid2<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector2<i32>>
{
    fn idx<C>(&self, coord: C) -> &Self::Item
    where
        C: Into<Vector2<i32>>
    {
        (self.reader)(I::from(coord.into()), &self.referent)
    }
}

impl<I, R, T, Fr, Fw> Grid2Mut for KolmoRwGrid2<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector2<i32>>
{
    fn midx<C>(&mut self, coord: C) -> &mut Self::Item
    where
        C: Into<Vector2<i32>>
    {
        (self.writer)(I::from(coord.into()), &mut self.referent)
    }
}


impl<I, R, T, Fr, Fw> Grid2Get for KolmoRwGrid2<I, R, T, Fr, Fw>
where
    T: Clone,
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector2<i32>>
{
    fn get<C: Into<Vector2<i32>>>(&self, coord: C) -> Self::Item 
    { self.idx(coord).clone() }
}

impl<I, R, T, Fr, Fw> Grid2Set for KolmoRwGrid2<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector2<i32>>
{
    fn set<C: Into<Vector2<i32>>>(&mut self, coord: C, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
