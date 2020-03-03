//! 3x3 allocation-free array implementation of Grid3.

use crate::{
    range::Range0To,
    grid3::*,
};
use mint::Vector3;

pub struct Inline3x3x3Grid<T> {
    array: [[[T; 3]; 3]; 3],
}

impl<T> Inline3x3x3Grid<T> {
    pub fn new<I, F>(mut startval: F) -> Self 
    where
        I: From<Vector3<i32>>,
        F: FnMut(I) -> T
    {
        fn xyz<I: From<Vector3<i32>>>(x: i32, y: i32, z: i32) -> I {
            I::from(Vector3 { x, y, z })
        }
        
        let array =
        [
            [
                [startval(xyz(0, 0, 0)), startval(xyz(0, 0, 1)), startval(xyz(0, 0, 2))],
                [startval(xyz(0, 1, 0)), startval(xyz(0, 1, 1)), startval(xyz(0, 1, 2))],
                [startval(xyz(0, 2, 0)), startval(xyz(0, 2, 1)), startval(xyz(0, 2, 2))],
            ],
            [
                [startval(xyz(1, 0, 0)), startval(xyz(1, 0, 1)), startval(xyz(1, 0, 2))],
                [startval(xyz(1, 1, 0)), startval(xyz(1, 1, 1)), startval(xyz(1, 1, 2))],
                [startval(xyz(1, 2, 0)), startval(xyz(1, 2, 1)), startval(xyz(1, 2, 2))],
            ],
            [
                [startval(xyz(2, 0, 0)), startval(xyz(2, 0, 1)), startval(xyz(2, 0, 2))],
                [startval(xyz(2, 1, 0)), startval(xyz(2, 1, 1)), startval(xyz(2, 1, 2))],
                [startval(xyz(2, 2, 0)), startval(xyz(2, 2, 1)), startval(xyz(2, 2, 2))],
            ]
        ];
        Inline3x3x3Grid { array }
    }
    
    
    pub fn broadcast(startval: T) -> Self 
    where
        T: Clone
    {
        Self::new(|_: Vector3<i32>| startval.clone())
    }
}

impl<T> From<[[[T; 3]; 3]; 3]> for Inline3x3x3Grid<T> {
    fn from(array: [[[T; 3]; 3]; 3]) -> Self {
        Inline3x3x3Grid { array }
    }
}

impl<T> Grid3 for Inline3x3x3Grid<T> {
    type Item = T;
    type XBound = Range0To;
    type YBound = Range0To;
    type ZBound = Range0To;
    
    fn x_bound(&self) -> Range0To {
        Range0To { end: 3 }
    }
    
    fn y_bound(&self) -> Range0To {
        Range0To { end: 3 }
    }
    
    fn z_bound(&self) -> Range0To {
        Range0To { end: 3 }
    }
}

impl<T> Grid3Len for Inline3x3x3Grid<T> {}

impl<T> Grid3Ref for Inline3x3x3Grid<T> {
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        &self.array[coord.x as usize][coord.y as usize][coord.z as usize]
    }
}

impl<T> Grid3Mut for Inline3x3x3Grid<T> {
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector3<i32>>
    {
        let coord = coord.into();
        &mut self.array[coord.x as usize][coord.y as usize][coord.z as usize]
    }
}

impl<T: Clone> Grid3Get for Inline3x3x3Grid<T> {
    fn get<I: Into<Vector3<i32>>>(&self, coord: I) -> Self::Item 
    { self.idx(coord).clone() }
}

impl<T> Grid3Set for Inline3x3x3Grid<T> {
    fn set<I: Into<Vector3<i32>>>(&mut self, coord: I, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
