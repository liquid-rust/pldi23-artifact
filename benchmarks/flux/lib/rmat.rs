#![feature(register_tool)]
#![register_tool(flux)]

#[path = "./rvec.rs"]
pub mod rvec;

use rvec::RVec;

#[flux::refined_by(rows: int, cols: int)]
pub struct RMat<T> {
    #[flux::field(usize[@cols])]
    _cols: usize,
    #[flux::field(RVec<RVec<T>[cols]>[@rows])]
    inner: RVec<RVec<T>>,
}

impl<T> RMat<T> {
    #[flux::sig(fn(rows: usize, cols: usize, T) -> RMat<T>[rows, cols])]
    pub fn new(rows: usize, cols: usize, elem: T) -> RMat<T>
    where
        T: Copy,
    {
        let mut inner = RVec::new();
        let mut i = 0;
        while i < rows {
            let r = RVec::from_elem_n(elem, cols);
            inner.push(r);
            i += 1;
        }
        Self { _cols: cols, inner }
    }

    #[flux::sig(fn(&RMat<T>[@m, @n], usize{v: v < m}, usize{v: v < n}) -> &T)]
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.inner.get(i).get(j)
    }

    #[flux::sig(fn(&mut RMat<T>[@m, @n], usize{v: v < m}, usize{v: v < n}) -> &mut T)]
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        self.inner.get_mut(i).get_mut(j)
    }
}
