extern crate prusti_contracts;

use prusti_contracts::*;

pub struct VecWrapper<T> {
    pub v: Vec<T>,
}

impl<T: Copy> VecWrapper<T> {
    #[trusted]
    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    #[trusted]
    #[pure]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    #[trusted]
    #[pure]
    #[requires(i < self.len())]
    pub fn lookup(&self, i: usize) -> T {
        self.v[i]
    }

    #[trusted]
    #[pure]
    #[requires(i < self.len())]
    pub fn get(&self, i: usize) -> &T {
        &self.v[i]
    }

    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()))]
    pub fn store(&mut self, index: usize, value: T) {
        self.v[index] = value;
    }

    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()))]
    #[after_expiry(self.len() == old(self.len()))]
    pub fn index_mut(&mut self, index: usize) -> &mut T {
        self.v.get_mut(index).unwrap()
    }

    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    pub fn push(&mut self, value: T) {
        self.v.push(value);
    }

    #[trusted]
    #[ensures(result.len() == n)]
    pub fn from_elem_n(elem: T, n: usize) -> VecWrapper<T>
    {
        let mut vec = Vec::new();
        let mut i = 0;
        while i < n {
            vec.push(elem);
            i += 1;
        }
        VecWrapper { v: vec }
    }

    #[trusted]
    #[ensures(result == (self.len() == 0))]
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    #[trusted]
    #[requires(self.len() > 0)]
    #[ensures(self.len() == old(self.len()) - 1)]
    pub fn pop(&mut self) -> T {
        self.v.pop().unwrap()
    }

    #[trusted]
    #[requires(index_a < self.len())]
    #[requires(index_b < self.len())]
    #[ensures(self.len() == old(self.len()))]
    pub fn swap(&mut self, index_a: usize, index_b: usize) {
        self.v.swap(index_a, index_b);
    }
}
