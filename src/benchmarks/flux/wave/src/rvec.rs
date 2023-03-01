#[flux::opaque]
#[flux::refined_by(len: int)]
pub struct RVec<T> {
    inner: Vec<T>,
}

impl<T> RVec<T> {
    #[flux::trusted]
    #[flux::sig(fn() -> RVec<T>[0])]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[flux::trusted]
    #[flux::sig(fn(self: &strg RVec<T>[@n], T) ensures self: RVec<T>[n+1])]
    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    #[flux::trusted]
    #[flux::sig(fn(&RVec<T>[@n]) -> usize[n])]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[flux::trusted]
    #[flux::sig(fn(T, n: usize) -> RVec<T>[n])]
    pub fn from_elem_n(elem: T, n: usize) -> Self
    where
        T: Clone,
    {
        RVec { inner: vec![elem; n] }
    }

    #[flux::trusted]
    #[flux::sig(fn(&RVec<T>[@n]) -> RVec<T>[n])]
    pub fn clone(&self) -> Self
    where
        T: Clone,
    {
        Self {
            inner: self.inner.clone(),
        }
    }

    #[flux::trusted]
    #[flux::sig(fn(inner: Vec<T>) -> RVec<T>)]
    pub fn from_vec(inner: Vec<T>) -> RVec<T> {
        RVec { inner }
    }

    #[flux::trusted]
    #[flux::sig(fn(RVec<T>) -> Vec<T>)]
    pub fn to_vec(self) -> Vec<T> {
        self.inner
    }
}

impl<T> std::ops::Index<usize> for RVec<T> {
    type Output = T;

    #[flux::trusted]
    #[flux::sig(fn(&RVec<T>[@n], usize{v : 0 <= v && v < n}) -> &T)]
    fn index(&self, index: usize) -> &T {
         &self.inner[index]
    }
}

impl<T> std::ops::IndexMut<usize> for RVec<T> {
    #[flux::trusted]
    #[flux::sig(fn(&mut RVec<T>[@n], usize{v : 0 <= v && v < n}) -> &mut T)]
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.inner[index]
    }
}
