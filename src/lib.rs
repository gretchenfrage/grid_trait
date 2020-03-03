
extern crate mint;

pub mod range;
pub mod grid2;

/*
/// Generic two-dimensional grid.
pub trait Grid2: Sized {
    type Item;
    
    fn x_len(&self) -> i32;
    fn y_len(&self) -> i32;
    
    fn get<C>(&self, coord: C) -> Option<&Self::Item>
    where
        C: Into<Vector2<i32>>;
            
    fn get_mut<C>(&mut self, coord: C) -> Option<&mut Self::Item>
    where
        C: Into<Vector2<i32>>;
        
    fn ops(&self) -> Grid2Ops<&Self> 
    { Grid2Ops(self) }
    
    fn ops_mut(&mut self) -> Grid2Ops<&mut Self>
    { Grid2Ops(self) }
}

/// Wrapping view of a Grid2 to enable convenience operations.
pub struct Grid2Ops<G>(pub G);

impl<G> Grid2Ops<G> 
where
    G: Deref,
    G::Target: Grid2,
{
    pub fn x_len(&self) -> i32 { self.0.x_len() }
    pub fn y_len(&self) -> i32 { self.0.y_len() }
    pub fn len_xy<V: From<[i32; 2]>>(&self) -> V {
        [self.x_len(), self.y_len()].into()
    }
    
    pub fn get<C>(&self, coord: C) -> Option<&<<G as Deref>::Target as Grid2>::Item>
    where
        C: Into<Vector2<i32>>
    { self.0.get(coord) }
}

impl<G> Grid2Ops<G> 
where
    G: DerefMut,
    G::Target: Grid2,
{
    pub fn get_mut<C>(&mut self, coord: C) -> Option<&mut <<G as Deref>::Target as Grid2>::Item>
    where
        C: Into<Vector2<i32>>
    { self.0.get_mut(coord) }
}

impl<G, C> Index<C> for Grid2Ops<G> 
where
    G: Deref,
    G::Target: Grid2,
    C: Into<Vector2<i32>> + Debug + Clone,
{
    type Output = <<G as Deref>::Target as Grid2>::Item;
    
    fn index(&self, i: C) -> &Self::Output {
        match self.0.get(i.clone()) {
            Some(o) => o,
            None => panic!("invalid index {:?}", i),
        }
    }
}

impl<G, C> IndexMut<C> for Grid2Ops<G> 
where
    G: DerefMut,
    G::Target: Grid2,
    C: Into<Vector2<i32>> + Debug + Clone,
{    
    fn index_mut(&mut self, i: C) -> &mut Self::Output {
        match self.0.get_mut(i.clone()) {
            Some(o) => o,
            None => panic!("invalid index {:?}", i),
        }
    }
}

/// Simple heap-allocated Grid2 implementation.
pub struct HeapGrid2<T> {
    alloc: Box<[T]>,
    x_len: i32,
    y_len: i32,
}

impl<T> HeapGrid2<T> {
    pub fn new<F>(x_len: i32, y_len: i32, mut startval: F) -> Self
    where
        F: FnMut() -> T
    {
        let len = x_len * y_len;
        let mut v: Vec<T> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            v.push(startval());
        }
        HeapGrid2 {
            alloc: v.into_boxed_slice(),
            x_len,
            y_len,
        }
    }
    
    fn inner_index(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || x > self.x_len || y < 0 || y > self.y_len {
            None
        } else {
            Some(y * self.x_len + x)
        }
    }
}

impl<T> Grid2 for HeapGrid2<T> {
    type Item = T;
    
    fn x_len(&self) -> i32 { self.x_len }
    fn y_len(&self) -> i32 { self.y_len }
    
    fn get<C>(&self, coord: C) -> Option<&Self::Item>
    where
        C: Into<Vector2<i32>>
    {
        let Vector2 { x, y } = coord.into();
        self.inner_index(x, y).map(|i| &self.alloc[i as usize])
    }
            
    fn get_mut<C>(&mut self, coord: C) -> Option<&mut Self::Item>
    where
        C: Into<Vector2<i32>>
    {
        let Vector2 { x, y } = coord.into();
        self.inner_index(x, y).map(move |i| &mut self.alloc[i as usize])
    }
}

*/