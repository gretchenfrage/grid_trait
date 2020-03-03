//! 3x3 allocation-free array implementation of Grid2.

use crate::{
    range::Range0To,
    grid2::*,
};
use mint::Vector2;

pub struct Inline3x3Grid<T> {
    array: [[T; 3]; 3],
}

impl<T> Inline3x3Grid<T> {
    pub fn new<I, F>(mut startval: F) -> Self 
    where
        I: From<Vector2<i32>>,
        F: FnMut(I) -> T
    {
        fn xy<I: From<Vector2<i32>>>(x: i32, y: i32) -> I {
            I::from(Vector2 { x, y })
        }
        
        let array = [
            [startval(xy(0, 0)), startval(xy(0, 1)), startval(xy(0, 2))],
            [startval(xy(1, 0)), startval(xy(1, 1)), startval(xy(1, 2))],
            [startval(xy(2, 0)), startval(xy(2, 1)), startval(xy(2, 2))],
        ];
        Inline3x3Grid { array }
    }
    
    
    pub fn broadcast(startval: T) -> Self 
    where
        T: Clone
    {
        Self::new(|_: Vector2<i32>| startval.clone())
    }
}

impl<T> From<[[T; 3]; 3]> for Inline3x3Grid<T> {
    fn from(array: [[T; 3]; 3]) -> Self {
        Inline3x3Grid { array }
    }
}

impl<T> Grid2 for Inline3x3Grid<T> {
    type Item = T;
    type XBound = Range0To;
    type YBound = Range0To;
    
    fn x_bound(&self) -> Range0To {
        Range0To { end: 3 }
    }
    
    fn y_bound(&self) -> Range0To {
        Range0To { end: 3 }
    }
}

impl<T> Grid2Len for Inline3x3Grid<T> {}

impl<T> Grid2Ref for Inline3x3Grid<T> {
    fn idx<I>(&self, coord: I) -> &Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        &self.array[coord.x as usize][coord.y as usize]
    }
}

impl<T> Grid2Mut for Inline3x3Grid<T> {
    fn midx<I>(&mut self, coord: I) -> &mut Self::Item
    where
        I: Into<Vector2<i32>>
    {
        let coord = coord.into();
        &mut self.array[coord.x as usize][coord.y as usize]
    }
}

impl<T: Clone> Grid2Get for Inline3x3Grid<T> {
    fn get<I: Into<Vector2<i32>>>(&self, coord: I) -> Self::Item 
    { self.idx(coord).clone() }
}

impl<T> Grid2Set for Inline3x3Grid<T> {
    fn set<I: Into<Vector2<i32>>>(&mut self, coord: I, elem: Self::Item) 
    { *self.midx(coord) = elem; }
}
