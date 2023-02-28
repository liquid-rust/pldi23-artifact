extern crate prusti_contracts;

use prusti_contracts::*;
#[path = "lib/vecwrapper.rs"]
pub mod vecwrapper;
use vecwrapper::VecWrapper;

//#[lr::sig(fn(src: & n@RVec<i32>, dst: &mut RVec<i32>[n]) -> i32; dst: RVec<i32>[n])]
#[requires(src.len() == dst.len())]
#[ensures(dst.len() == old(dst.len()))]
fn bcopy_aux(src: &VecWrapper<i32>, dst: &mut VecWrapper<i32>) {
    let mut i = 0;
    let n = src.len();
    while i < n {
        body_invariant!(dst.len() == src.len() && i < src.len());
        dst.store(i, src.lookup(i));
        i += 1;
    }
}

//#[lr::sig(fn(src: & n@RVec<i32>) -> RVec<i32>[n])]
#[ensures(result.len() == src.len())]
pub fn bcopy(src: &VecWrapper<i32>) -> VecWrapper<i32> {
    let sz = src.len();
    let mut dst = VecWrapper::<i32>::from_elem_n(0, sz);
    bcopy_aux(src, &mut dst);
    dst
}