//! Heap-allocated array implementation of Grid3.

use crate::{
    range::Range0To,
    grid3::*,
};
use mint::Vector3;

/// Heap-allocated array implementation of Grid3.
pub struct ArrayGrid3<T> {
    alloc: Box<[T]>,
    x_len: i32,
    y_len: i32,
    z_len: i32,
}

impl<T> ArrayGrid3<T> {
    pub fn new<I, F>(x_len: i32, y_len: i32, z_len: i32, mut startval: F) -> Self
    where
        I: From<Vector3<i32>>,
        F: FnMut(I) -> T
    {
        assert!(x_len >= 0);
        assert!(y_len >= 0);
        assert!(z_len >= 0);
        
        let len = x_len * y_len * z_len;
        let mut v: Vec<T> = Vec::with_capacity(len as usize);
        for z in 0..z_len {
            for y in 0..y_len {
                for x in 0..x_len {
                    let item = startval(I::from(Vector3 { x, y, z }));
                    v.push(item);
                }
            }
        }
        ArrayGrid3 {
            alloc: v.into_boxed_slice(),
            x_len,
            y_len,
            z_len,
        }
    }
    
    pub fn broadcast(x_len: i32, y_len: i32, z_len: i32, startval: T) -> Self
    where
        T: Clone 
    {
        Self::new(x_len, y_len, z_len, |_: Vector3<i32>| startval.clone())
    }
    
    fn inner_index(&self, x: i32, y: i32, z: i32) -> Option<i32> {
        if x < 0 || x > self.x_len || y < 0 || y > self.y_len || z < 0 || z > self.z_len {
            None
        } else {
            Some(z * self.x_len * self.y_len + y * self.x_len + x)
        }
    }
}

impl<T> Grid3 for ArrayGrid3<T> {
    type Item = T;
    type XBound = Range0To;
    type YBound = Range0To;
    type ZBound = Range0To;
    
    fn x_bound(&self) -> Range0To {
        Range0To { end: self.x_len }
    }
    
    fn y_bound(&self) -> Range0To {
        Range0To { end: self.y_len }
    }
    
    fn z_bound(&self) -> Range0To {
        Range0To { end: self.z_len }
    }
}

impl<T> Grid3Len for ArrayGrid3<T> {}

impl<T> Grid3Ref for ArrayGrid3<T> {
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        let Vector3 { x, y, z } = coord;
        let option = self
            .inner_index(x, y, z)
            .map(|i| &self.alloc[i as usize]);
        match option {
            Some(o) => o,
            None => panic!("invalid index {:?}", coord),
        }
    }
}

impl<T> Grid3Mut for ArrayGrid3<T> {
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        let Vector3 { x, y, z } = coord;
        let option = self
            .inner_index(x, y, z)
            .map(move |i| &mut self.alloc[i as usize]);
        match option {
            Some(o) => o,
            None => panic!("invalid index {:?}", coord),
        }
    }
}

impl<T: Clone> Grid3Get for ArrayGrid3<T> {
    fn get<I: Into<Vector3<i32>>>(&self, coord: I) -> Self::Item 
    { self.idx(coord).clone() }
}

impl<T> Grid3Set for ArrayGrid3<T> {
    fn set<I: Into<Vector3<i32>>>(&mut self, coord: I, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
