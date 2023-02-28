extern crate prusti_contracts;

use prusti_contracts::*;

pub struct VecWrapperFull {
    v: Vec<usize>,
}

impl VecWrapperFull {
    #[trusted]
    #[pure]
    pub fn len(&self) -> usize {
        self.v.len()
    }

    #[trusted]
    #[pure]
    #[requires(i < self.len())]
    pub fn lookup(&self, i: usize) -> usize {
        self.v[i]
    }

    #[trusted]
    #[requires(index < self.len())]
    #[ensures(self.len() == old(self.len()))]
    #[ensures(forall(|i:usize| (i < self.len() && i != index) ==> self.lookup(i) == old(self.lookup(i))))]
    #[ensures(self.lookup(index) == value)]
    pub fn store(&mut self, index: usize, value: usize) {
        self.v[index] = value;
    }

    #[trusted]
    #[ensures(result.len() == n)]
    #[ensures(forall(|i: usize| i < n ==> result.lookup(i) == elem))]
    pub fn from_elem_n(elem: usize, n: usize) -> VecWrapperFull
    {
        let mut vec = Vec::new();
        let mut i = 0;
        while i < n {
            vec.push(elem);
            i += 1;
        }
        VecWrapperFull { v: vec }
    }
}