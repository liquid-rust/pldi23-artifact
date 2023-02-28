//! We have a trusted version of `RMat` to avoid including it in the verification time of
//! individual benchmarks as we don't have a way to exclude a module from the total time.
#![feature(register_tool)]
#![register_tool(flux)]

#[flux::opaque]
#[flux::refined_by(rows: int, cols: int)]
pub struct RMat<T> {
    _cols: usize,
    inner: Vec<Vec<T>>,
}

impl<T> RMat<T> {
    #[flux::trusted]
    #[flux::sig(fn(rows: usize, cols: usize, T) -> RMat<T>[rows, cols])]
    pub fn new(rows: usize, cols: usize, elem: T) -> RMat<T>
    where
        T: Copy,
    {
        let mut inner = Vec::new();
        let mut i = 0;
        while i < rows {
            let r = vec![elem; cols];
            inner.push(r);
            i += 1;
        }
        Self { _cols: cols, inner }
    }

    #[flux::trusted]
    #[flux::sig(fn(&RMat<T>[@m, @n], usize{v: v < m}, usize{v: v < n}) -> &T)]
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.inner[i][j]
    }

    #[flux::trusted]
    #[flux::sig(fn(&mut RMat<T>[@m, @n], usize{v: v < m}, usize{v: v < n}) -> &mut T)]
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.inner[i][j]
    }
}
