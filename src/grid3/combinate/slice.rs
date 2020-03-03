//! Sub-view of a Grid3.

use crate::{
    range::Range0To,
    grid3::*,
};
use mint::Vector3;
use std::{
    ops::{RangeBounds, Bound},
    fmt::Debug,
};


/// Sub-view of a Grid3.
///
/// The valid coordinates in this grid are a subset
/// of the valid coordinates in the inner grid.
pub struct Grid3Slice<G, X, Y, Z> 
where
    G: Grid3,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
    Z: RangeBounds<i32> + Clone,
{
    inner: G,
    x_bound: X,
    y_bound: Y,
    z_bound: Z,
}

/// Verify that a is more strict than b.
fn more_strict(a: impl RangeBounds<i32>, b: impl RangeBounds<i32>) -> bool {
    
    fn lower_inclusive(bound: Bound<&i32>) -> Option<i32> {
        match bound {
            Bound::Included(&i) => Some(i),
            Bound::Excluded(&i) => Some(i + 1),
            Bound::Unbounded   => None,
        }
    }
    
    fn upper_inclusive(bound: Bound<&i32>) -> Option<i32> {
        match bound {
            Bound::Included(&i) => Some(i),
            Bound::Excluded(&i) => Some(i - 1),
            Bound::Unbounded   => None,
        }
    }
    
    let lower_ok = match (
        lower_inclusive(a.start_bound()), 
        lower_inclusive(b.start_bound()),
    ) {
        (Some(i1), Some(i2)) => i1 >= i2,
        (Some(_), None)      => true,
        (None, Some(_))      => false,
        (None, None)         => true,
    };
    if !lower_ok { return false; }
    
    let upper_ok = match (
        upper_inclusive(a.end_bound()), 
        upper_inclusive(b.end_bound()),
    ) {
        (Some(i1), Some(i2)) => i1 <= i2,
        (Some(_), None)      => true,
        (None, Some(_))      => false,
        (None, None)         => true,
    };
    if !upper_ok { return false; }
    
    true
}

impl<G, X, Y, Z> Grid3Slice<G, X, Y, Z>
where
    G: Grid3,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
    Z: RangeBounds<i32> + Clone,
{
    /// Fails if the new bounds are not a subset of the old ones.
    pub fn try_new(inner: G, new_x: X, new_y: Y, new_z: Z) -> Result<Self, G>
    {
        if more_strict(new_x.clone(), inner.x_bound()) 
            && more_strict(new_y.clone(), inner.y_bound())
            && more_strict(new_z.clone(), inner.z_bound())
        {
            Ok(Grid3Slice {
                inner,
                x_bound: new_x,
                y_bound: new_y,
                z_bound: new_z,
            })
        } else {
            Err(inner)
        }
    }
    
    /// Panics if the new bounds are not a subset of the old ones.
    pub fn new(inner: G, new_x: X, new_y: Y, new_z: Z) -> Self
    where
        X: Debug,
        Y: Debug,
        Z: Debug,
        <G as Grid3>::XBound: Debug,
        <G as Grid3>::YBound: Debug,
        <G as Grid3>::ZBound: Debug,
    {
        if more_strict(new_x.clone(), inner.x_bound()) 
            && more_strict(new_y.clone(), inner.y_bound())
            && more_strict(new_z.clone(), inner.z_bound())
        {
            Grid3Slice {
                inner,
                x_bound: new_x,
                y_bound: new_y,
                z_bound: new_z,
            }
        } else {
            panic!("new bounds are not a subset of old bounds, new={:?}, old={:?}",
                (new_x, new_y, new_z),
                (inner.x_bound(), inner.y_bound(), inner.z_bound()));
        }
    }
}

impl<G, X, Y, Z> Grid3 for Grid3Slice<G, X, Y, Z>
where
    G: Grid3,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
    Z: RangeBounds<i32> + Clone,
{
    type Item = <G as Grid3>::Item;
    type XBound = X;
    type YBound = Y;
    type ZBound = Z;
    
    fn x_bound(&self) -> Self::XBound { self.x_bound.clone() }
    fn y_bound(&self) -> Self::YBound { self.y_bound.clone() }
    fn z_bound(&self) -> Self::ZBound { self.z_bound.clone() }
}

impl<G> Grid3Len for Grid3Slice<G, Range0To, Range0To, Range0To> 
where
    G: Grid3
{}

impl<G, X, Y, Z> Grid3Get for Grid3Slice<G, X, Y, Z>
where
    G: Grid3 + Grid3Get,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
    Z: RangeBounds<i32> + Clone,
{
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        match self.try_get(coord) {
            Some(item) => item,
            None => panic!("invalid index {:?}", coord),
        }
    }
        
    fn try_get<I>(&self, coord: I) -> Option<Self::Item>
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.inner.get(coord))
        } else {
            None
        }
    }
}

impl<G, X, Y, Z> Grid3Set for Grid3Slice<G, X, Y, Z>
where
    G: Grid3 + Grid3Set,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
    Z: RangeBounds<i32> + Clone,
{
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        match self.try_set(coord, elem) {
            Ok(_) => {},
            Err(_) => panic!("invalid index {:?}", coord),
        }
    }
        
    fn try_set<I>(&mut self, coord: I, elem: Self::Item) -> Result<(), Self::Item> 
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            self.inner.set(coord, elem);
            Ok({})
        } else {
            Err(elem)
        }
    }
}

impl<G, X, Y, Z> Grid3Ref for Grid3Slice<G, X, Y, Z>
where
    G: Grid3 + Grid3Ref,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
    Z: RangeBounds<i32> + Clone,
{
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        match self.try_idx(coord) {
            Some(item) => item,
            None => panic!("invalid index {:?}", coord),
        }
    }
    
    fn try_idx<I>(&self, coord: I) -> Option<&Self::Item>
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.inner.idx(coord))
        } else {
            None
        }
    }
}

impl<G, X, Y, Z> Grid3Mut for Grid3Slice<G, X, Y, Z>
where
    G: Grid3 + Grid3Mut,
    X: RangeBounds<i32> + Clone,
    Y: RangeBounds<i32> + Clone,
    Z: RangeBounds<i32> + Clone,
{
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        match self.try_midx(coord) {
            Some(item) => item,
            None => panic!("invalid index {:?}", coord),
        }
    }
    
    fn try_midx<I>(&mut self, coord: I) -> Option<&mut Self::Item>
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.inner.midx(coord))
        } else {
            None
        }
    }
}
