
pub mod map;
pub mod slice;
pub mod enumap;
pub mod wrapping;
pub mod neworigin;
pub mod flatten;
pub mod oobhandler;

use super::*;
use mint::Vector3;
use std::{
    ops::{Deref, DerefMut},
};

// ==== elevate pointer types ====

impl<T> Grid3 for T
where
    T: Deref,
    <T as Deref>::Target: Grid3
{
    type Item = <<T as Deref>::Target as Grid3>::Item;
    type XBound = <<T as Deref>::Target as Grid3>::XBound;
    type YBound = <<T as Deref>::Target as Grid3>::YBound;
    type ZBound = <<T as Deref>::Target as Grid3>::ZBound;
    
    fn x_bound(&self) -> Self::XBound {
        T::deref(self).x_bound()
    }
    
    fn y_bound(&self) -> Self::YBound {
        T::deref(self).y_bound()
    }
    
    fn z_bound(&self) -> Self::ZBound {
        T::deref(self).z_bound()
    }
}

impl<T> Grid3Len for T
where
    T: Deref,
    <T as Deref>::Target: Grid3Len
{}

impl<T> Grid3Get for T
where
    T: Deref,
    <T as Deref>::Target: Grid3Get
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector3<i32>>
    {
        T::deref(self).get(coord)
    }
}

impl<T> Grid3Ref for T
where
    T: Deref,
    <T as Deref>::Target: Grid3Ref
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector3<i32>>
    {
        T::deref(self).idx(coord)
    }
}

impl<T> Grid3Set for T
where
    T: Deref + DerefMut,
    <T as Deref>::Target: Grid3Set
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector3<i32>>
    {
        T::deref_mut(self).set(coord, elem)
    }
}

impl<T> Grid3Mut for T
where
    T: Deref + DerefMut,
    <T as Deref>::Target: Grid3Mut
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector3<i32>>
    {
        T::deref_mut(self).midx(coord)
    }
}
