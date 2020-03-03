//! Kolmogorov by-refernce read/write encoding of Grid3.

use crate::grid3::*;
use mint::Vector3;
use std::{
    ops::RangeFull,
    marker::PhantomData,
};

/// Kolmogorov by-reference read/write encoding of Grid3.
/// 
/// This is the most powerful kolmogorov borrowing type,
/// because it contains separate functions for the 
/// exclusive and shared access pathways, and holds a
/// single datum which they both reference.
///
/// This is a Grid3 implementation which only stores a
/// function from coordinate to value. It is subsequently
/// unbounded.
pub struct KolmoRwGrid3<I, R, T, Fr, Fw> 
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector3<i32>>
{
    referent: R,
    reader: Fr,
    writer: Fw,
    p: PhantomData<fn(T, I)>,
}

impl<I, R, T, Fr, Fw> KolmoRwGrid3<I, R, T, Fr, Fw> 
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector3<i32>>
{
    pub fn new(referent: R, reader: Fr, writer: Fw) -> Self {
        KolmoRwGrid3 {
            referent,
            reader,
            writer,
            p: PhantomData,
        }
    }
}

impl<I, R, T, Fr, Fw> Grid3 for KolmoRwGrid3<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
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

impl<I, R, T, Fr, Fw> Grid3Ref for KolmoRwGrid3<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector3<i32>>
{
    fn idx<C>(&self, coord: C) -> &Self::Item
    where
        C: Into<Vector3<i32>>
    {
        (self.reader)(I::from(coord.into()), &self.referent)
    }
}

impl<I, R, T, Fr, Fw> Grid3Mut for KolmoRwGrid3<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector3<i32>>
{
    fn midx<C>(&mut self, coord: C) -> &mut Self::Item
    where
        C: Into<Vector3<i32>>
    {
        (self.writer)(I::from(coord.into()), &mut self.referent)
    }
}


impl<I, R, T, Fr, Fw> Grid3Get for KolmoRwGrid3<I, R, T, Fr, Fw>
where
    T: Clone,
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector3<i32>>
{
    fn get<C: Into<Vector3<i32>>>(&self, coord: C) -> Self::Item 
    { self.idx(coord).clone() }
}

impl<I, R, T, Fr, Fw> Grid3Set for KolmoRwGrid3<I, R, T, Fr, Fw>
where
    Fr: Fn(I, &R) -> &T,
    Fw: FnMut(I, &mut R) -> &mut T,
    I: From<Vector3<i32>>
{
    fn set<C: Into<Vector3<i32>>>(&mut self, coord: C, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
