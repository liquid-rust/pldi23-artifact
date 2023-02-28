extern crate prusti_contracts;

use prusti_contracts::*;
#[path = "lib/vecwrapper.rs"]
pub mod vecwrapper;
use vecwrapper::VecWrapper;

//#[lr::sig(fn(v1: &n@RVec<i32>, v2:RVec<i32>[n]) -> i32)]
#[requires(v1.len() == v2.len())]
pub fn dotprod(v1: &VecWrapper<i32>, v2:VecWrapper<i32>) -> i32 {
  let n = v1.len();
  let mut sum = 0;
  let mut i = 0;
  while i < n {
    body_invariant!(i < v1.len());
    let x1 = v1.lookup(i);
    let x2 = v2.lookup(i);
    sum += x1 * x2;
    i += 1;
  }
  sum
}