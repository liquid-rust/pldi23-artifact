use prusti_contracts::*;
use std::vec::Vec;

#[extern_spec]
impl<T> Vec<T> {
    #[ensures(result.len() == 0)]
    #[ensures(result.capacity() == 0)]
    fn new() -> Vec<T>;

    #[pure]
    fn len<A>(&self) -> usize;

    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(self.capacity() >= old(self.capacity()))]
    fn push<A>(&mut self, value: T);

    #[ensures(self.len() == 0)]
    fn clear<A>(&mut self);

    #[pure]
    fn capacity<A>(&self) -> usize;

    #[ensures(self.capacity() >= old(self.len() + additional))]
    #[ensures(self.len() == old(self.len()))]
    fn reserve_exact<A>(&mut self, additional: usize);

    #[pure]
    fn as_slice<A>(&self) -> &[T];
}
