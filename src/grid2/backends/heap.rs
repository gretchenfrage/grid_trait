//! Heap-allocated array implementation of Grid2.

use crate::{
    range::Range0To,
    grid2::*,
};
use mint::Vector2;

/// Heap-allocated array implementation of Grid2.
pub struct ArrayGrid2<T> {
    alloc: Box<[T]>,
    x_len: i32,
    y_len: i32,
}

impl<T> ArrayGrid2<T> {
    pub fn new<I, F>(x_len: i32, y_len: i32, mut startval: F) -> Self
    where
        I: From<Vector2<i32>>,
        F: FnMut(I) -> T
    {
        assert!(x_len >= 0);
        assert!(y_len >= 0);
        
        let len = x_len * y_len;
        let mut v: Vec<T> = Vec::with_capacity(len as usize);
        for y in 0..y_len {
            for x in 0..x_len {
                let item = startval(I::from(Vector2 { x, y }));
                v.push(item);
            }
        }
        ArrayGrid2 {
            alloc: v.into_boxed_slice(),
            x_len,
            y_len,
        }
    }
    
    pub fn broadcast(x_len: i32, y_len: i32, startval: T) -> Self
    where
        T: Clone 
    {
        Self::new(x_len, y_len, |_: Vector2<i32>| startval.clone())
    }
    
    fn inner_index(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || x > self.x_len || y < 0 || y > self.y_len {
            None
        } else {
            Some(y * self.x_len + x)
        }
    }
}

impl<T> Grid2 for ArrayGrid2<T> {
    type Item = T;
    type XBound = Range0To;
    type YBound = Range0To;
    
    fn x_bound(&self) -> Range0To {
        Range0To { end: self.x_len }
    }
    
    fn y_bound(&self) -> Range0To {
        Range0To { end: self.y_len }
    }
}

impl<T> Grid2Len for ArrayGrid2<T> {}

impl<T> Grid2Ref for ArrayGrid2<T> {
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        let Vector2 { x, y } = coord;
        let option = self
            .inner_index(x, y)
            .map(|i| &self.alloc[i as usize]);
        match option {
            Some(o) => o,
            None => panic!("invalid index {:?}", coord),
        }
    }
}

impl<T> Grid2Mut for ArrayGrid2<T> {
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        let Vector2 { x, y } = coord;
        let option = self
            .inner_index(x, y)
            .map(move |i| &mut self.alloc[i as usize]);
        match option {
            Some(o) => o,
            None => panic!("invalid index {:?}", coord),
        }
    }
}

impl<T: Clone> Grid2Get for ArrayGrid2<T> {
    fn get<I: Into<Vector2<i32>>>(&self, coord: I) -> Self::Item 
    { self.idx(coord).clone() }
}

impl<T> Grid2Set for ArrayGrid2<T> {
    fn set<I: Into<Vector2<i32>>>(&mut self, coord: I, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
