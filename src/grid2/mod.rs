//! Two-dimensional data grid.

/// Combinators.
pub mod combinate;

/// Implementations.
pub mod backends;

use crate::{
    range::Range0To,
    grid2::combinate::{
        flatten::RangeBoundsTimes,
        neworigin::RangeBoundsPlus,
        wrapping::BoundRange,
    },
};
use mint::Vector2;
use std::{
    ops::RangeBounds,
    fmt::Debug,
};

/// Allocate a grid on the heap.
pub fn alloc<I, T>(x_len: i32, y_len: i32, startval: T) -> backends::heap::ArrayGrid2<T>
where
    T: Clone
{
    backends::heap::ArrayGrid2::broadcast(x_len, y_len, startval)
}

/// Allocate a grid on the heap, populate with a function.
pub fn alloc_gen<I, T, F>(x_len: i32, y_len: i32, generator: F) -> backends::heap::ArrayGrid2<T>
where
    I: From<Vector2<i32>>,
    F: FnMut(I) -> T,
{
    backends::heap::ArrayGrid2::new(x_len, y_len, generator)
}

/// Inline 3x3 array grid.
pub fn array3x3<I, T>(startval: T) -> backends::inline3x3::Inline3x3Grid<T>
where
    T: Clone
{
    backends::inline3x3::Inline3x3Grid::broadcast(startval)
}

/// Inline 3x3 array grid, populate with a function.
pub fn array3x3_gen<I, T, F>(generator: F) -> backends::inline3x3::Inline3x3Grid<T>
where
    I: From<Vector2<i32>>,
    F: FnMut(I) -> T,
{
    backends::inline3x3::Inline3x3Grid::new(generator)
}

/// Represent a coord → Item function as a grid.
pub fn value_fn<I, T, F>(f: F) -> backends::kolmo::KolmoGrid2<F, I, T>
where
    I: From<Vector2<i32>>,
    F: Fn(I) -> T,
{
    backends::kolmo::KolmoGrid2::new(f)
}

/// Represent a coord → &Item function as a grid.
pub fn ref_fn<'a, I, T, F>(f: F) -> backends::kolmoref::KolmoRefGrid2<'a, F, I, T>
where
    I: From<Vector2<i32>>,
    T: 'a,
    F: Fn(I) -> &'a T,
{
    backends::kolmoref::KolmoRefGrid2::new(f)
}

/// Represent a coord → &mut Item function as a grid.
pub fn mut_fn<'a, I, T, F>(f: F) -> backends::kolmomut::KolmoMutGrid2<'a, F, I, T>
where
    I: From<Vector2<i32>>,
    T: 'a,
    F: Fn(I) -> &'a mut T,
{
    backends::kolmomut::KolmoMutGrid2::new(f)
}

/// Read/write through closures.
///
/// This is a powerful type, which acts like a combination
/// of `ref_fn` and `mut_fn`. This grid owns a *referent* 
/// value, and contains a *reader* and *writer* function
/// which immutable and mutable (respectively) borrow the
/// elements from the referent.
pub fn reader_writer<I, R, T, Fr, Fw>(referent: R, reader: Fr, writer: Fw) -> backends::kolmorw::KolmoRwGrid2<I, R, T, Fr, Fw>
where
    I: From<Vector2<i32>>,
    Fr: Fn(I, &R) -> &T,
    Fw: Fn(I, &mut R) -> &mut T,
{
    backends::kolmorw::KolmoRwGrid2::new(referent, reader, writer)
}

/// Top-level trait for 2D grids.
pub trait Grid2 {
    type Item;
    type XBound: RangeBounds<i32>;
    type YBound: RangeBounds<i32>;
    
    fn x_bound(&self) -> Self::XBound;
    fn y_bound(&self) -> Self::YBound;
    
    fn in_bounds<I>(&self, coord: I) -> bool 
    where
        I: Into<Vector2<i32>>
    {
        let Vector2 { x, y } = coord.into();
        
        self.x_bound().contains(&x)
        && self.y_bound().contains(&y)
    }
    
    /// Element by-value mapping.
    fn map<F, T>(self, func: F) -> combinate::map::Grid2Map<Self, F, T>
    where
        Self: Sized,
        F: Fn(Self::Item) -> T,
    {
        combinate::map::Grid2Map::new(self, func)
    }
    
    /// Element by-value+coord mapping.
    fn enumap<I, F, T>(self, func: F) -> combinate::enumap::Grid2EnuMap<Self, F, T, I>
    where
        Self: Sized,
        I: From<Vector2<i32>>,
        F: Fn(I, Self::Item) -> T,
    {
        combinate::enumap::Grid2EnuMap::new(self, func)
    }
    
    /// Flattening a grid of grids with a regular stride.
    fn flatten<I>(self, stride: I) -> combinate::flatten::Grid2Flat<Self>
    where
        Self: Sized,
        Self::Item: Grid2,
        Self::XBound: Clone + RangeBoundsTimes,
        Self::YBound: Clone + RangeBoundsTimes,
        I: Into<Vector2<i32>>,
    {
        combinate::flatten::Grid2Flat::new(self, stride)
    }
    
    /// <0, 0> in this grid becomes new_origin in resultant grid.
    fn new_origin<I>(self, new_origin: I) -> combinate::neworigin::Grid2NewOrigin<Self>
    where
        Self: Sized,
        Self::XBound: RangeBoundsPlus,
        Self::YBound: RangeBoundsPlus,
        I: Into<Vector2<i32>>,
    {
        combinate::neworigin::Grid2NewOrigin::new(self, new_origin)
    }
    
    /// Provide function to provide elments at out-of-bounds coordinates.
    ///
    /// This produces an unbounded grid.
    fn oob_handler<I, F>(self, handler: F) -> combinate::oobhandler::Grid2OobHandler<Self, I, F>
    where
        Self: Sized,
        I: From<Vector2<i32>>,
        F: Fn(I) -> Self::Item,
    {
        combinate::oobhandler::Grid2OobHandler::new(self, handler)
    }
    
    /// View a sub-rectangle of this grid.
    /// 
    /// If the new bounds are not a subset of the current bounds,
    /// this will panic.
    fn subview<X, Y>(self, new_x: X, new_y: Y) -> combinate::slice::Grid2Slice<Self, X, Y>
    where
        Self: Sized,
        Self::XBound: Debug,
        Self::YBound: Debug,
        X: RangeBounds<i32> + Clone + Debug,
        Y: RangeBounds<i32> + Clone + Debug,
    {
        combinate::slice::Grid2Slice::new(self, new_x, new_y)
    }
    
    /// View a sub-rectangle of this grid.
    /// 
    /// If the new bounds are not a subset of the current bounds,
    /// this will fail.
    fn try_subview<X, Y>(self, new_x: X, new_y: Y) -> Result<combinate::slice::Grid2Slice<Self, X, Y>, Self>
    where
        Self: Sized,
        Self::XBound: Debug,
        Self::YBound: Debug,
        X: RangeBounds<i32> + Clone + Debug,
        Y: RangeBounds<i32> + Clone + Debug,
    {
        combinate::slice::Grid2Slice::try_new(self, new_x, new_y)
    }
    
    /// View a sub-rectangle of this grid, beginning at origin.
    /// 
    /// If the new bounds are not a subset of the current bounds,
    /// this will panic.
    fn subview_0to(self, new_x_len: i32, new_y_len: i32) -> combinate::slice::Grid2Slice<Self, Range0To, Range0To>
    where
        Self: Sized,
        Self::XBound: Debug,
        Self::YBound: Debug,
    {
        combinate::slice::Grid2Slice::new(
            self, 
            Range0To { end: new_x_len },
            Range0To { end: new_y_len })
    }
    
    /// View a sub-rectangle of this grid, beginning at origin.
    /// 
    /// If the new bounds are not a subset of the current bounds,
    /// this will fail.
    fn try_subview_0to(self, new_x_len: i32, new_y_len: i32) -> Result<combinate::slice::Grid2Slice<Self, Range0To, Range0To>, Self>
    where
        Self: Sized,
        Self::XBound: Debug,
        Self::YBound: Debug,
    {
        combinate::slice::Grid2Slice::try_new(
            self, 
            Range0To { end: new_x_len },
            Range0To { end: new_y_len })
    }
    
    /// View of this grid which wraps around the edges.
    ///
    /// The input grid must be bounded in all directions, and the
    /// output grid is completely unbounded.
    fn wrapping(self) -> combinate::wrapping::Grid2Wrapping<Self>
    where
        Self: Sized,
        Self::XBound: BoundRange,
        Self::YBound: BoundRange,
    {
        combinate::wrapping::Grid2Wrapping::new(self)
    }
    
    /// Collect a grid's elements into a heap allocation.
    ///
    /// The grid must be bound from 0 to a finite limit.
    fn collect(&self) -> backends::heap::ArrayGrid2<Self::Item>
    where
        Self: Grid2Get,
        Self::XBound: Into<Range0To>,
        Self::YBound: Into<Range0To>,
    {
        let x_len = self.x_bound().into().end;
        let y_len = self.y_bound().into().end;
        backends::heap::ArrayGrid2::new(
            x_len, y_len,
            |coord: Vector2<i32>| self.get(coord))
    }
}

/// 2D grid bounded from 0 to a finite number.
pub trait Grid2Len: Grid2<XBound=Range0To, YBound=Range0To> {
    fn x_len(&self) -> i32 {
        self.x_bound().end
    }
    
    fn y_len(&self) -> i32 {
        self.y_bound().end
    }
}

/// 2D grid read by value.
pub trait Grid2Get: Grid2 {
    fn get<I>(&self, coord: I) -> Self::Item
    where
        I: Into<Vector2<i32>>;
        
    fn try_get<I>(&self, coord: I) -> Option<Self::Item>
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.get(coord))
        } else {
            None
        }
    }
}

/// 2D grid write by value.
pub trait Grid2Set: Grid2 {
    fn set<I>(&mut self, coord: I, elem: Self::Item)
    where
        I: Into<Vector2<i32>>;
        
    fn try_set<I>(&mut self, coord: I, elem: Self::Item) -> Result<(), Self::Item> 
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            self.set(coord, elem);
            Ok({})
        } else {
            Err(elem)
        }
    }
}

/// 2D grid read by reference.
pub trait Grid2Ref: Grid2 {
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>;
    
    fn try_idx<I>(&self, coord: I) -> Option<&Self::Item>
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.idx(coord))
        } else {
            None
        }
    }
}

/// 2D grid write by reference.
pub trait Grid2Mut: Grid2 {
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>;
    
    fn try_midx<I>(&mut self, coord: I) -> Option<&mut Self::Item>
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        if self.in_bounds(coord) {
            Some(self.midx(coord))
        } else {
            None
        }
    }
}