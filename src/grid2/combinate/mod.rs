
pub mod map;
pub mod slice;
pub mod enumap;
pub mod wrapping;
pub mod neworigin;
pub mod flatten;
pub mod oobhandler;

use super::*;
use mint::Vector2;
use std::{
    ops::{Deref, DerefMut},
};

// ==== elevate pointer types ====

impl<T> Grid2 for T
where
    T: Deref,
    <T as Deref>::Target: Grid2
{
    type Item = <<T as Deref>::Target as Grid2>::Item;
    type XBound = <<T as Deref>::Target as Grid2>::XBound;
    type YBound = <<T as Deref>::Target as Grid2>::YBound;
    
    fn x_bound(&self) -> Self::XBound {
        T::deref(self).x_bound()
    }
    
    fn y_bound(&self) -> Self::YBound {
        T::deref(self).y_bound()
    }
}

impl<T> Grid2Len for T
where
    T: Deref,
    <T as Deref>::Target: Grid2Len
{}

impl<T> Grid2Get for T
where
    T: Deref,
    <T as Deref>::Target: Grid2Get
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector2<i32>>
    {
        T::deref(self).get(coord)
    }
}

impl<T> Grid2Ref for T
where
    T: Deref,
    <T as Deref>::Target: Grid2Ref
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>
    {
        T::deref(self).idx(coord)
    }
}

impl<T> Grid2Set for T
where
    T: Deref + DerefMut,
    <T as Deref>::Target: Grid2Set
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector2<i32>>
    {
        T::deref_mut(self).set(coord, elem)
    }
}

impl<T> Grid2Mut for T
where
    T: Deref + DerefMut,
    <T as Deref>::Target: Grid2Mut
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>
    {
        T::deref_mut(self).midx(coord)
    }
}
