#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(flux)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;

#[flux::sig(fn(&RVec<i32>[@n], RVec<i32>[n]) -> i32)]
pub fn dotprod(v1: &RVec<i32>, v2: RVec<i32>) -> i32 {
    let n = v1.len();
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        let x1 = *v1.get(i);
        let x2 = *v2.get(i);
        sum += x1 * x2;
        i += 1;
    }
    sum
}
