//! An adaptation of the example from
//! https://rosettacode.org/wiki/Knuth_shuffle#Rust
//!
//! Changes:
//!
//! +   Monomorphised types.
//! +   Wrapped built-in types and functions.
//! +   Rewrote loops into supported shape (while bool with no break, continue, or return).
//! +   Replaced ``println!`` with calling trusted functions.
//! +   Moved constants into variables.
//!
//! Verified properties:
//!
//! +   Absence of panics.

extern crate prusti_contracts;
use prusti_contracts::*;

#[path = "lib/vecwrapper.rs"]
mod vecwrapper;
use vecwrapper::VecWrapper;

//extern crate rand;

//use rand::Rng;

// struct ThreadRngWrapper {}

// impl ThreadRngWrapper {
//     #[trusted]
//     #[requires(low < high)]
//     #[ensures(low <= result && result < high)]
//     fn gen_range(&mut self, low: usize, high: usize) -> usize {
//         unimplemented!();
//     }
// }

// #[trusted]
// fn thread_rng() -> ThreadRngWrapper {
//     unimplemented!();
// }

#[trusted]
#[requires(low < high)]
#[ensures(low <= result && result < high)]
fn gen_range(low: usize, high: usize) -> usize {
    unimplemented!();
}

#[ensures(v.len() == old(v.len()))]
fn knuth_shuffle(v: &mut VecWrapper<i32>) {
    let l = v.len();

    let mut n = 0;
    let bgn = 0;
    while n < l {
        body_invariant!(0 <= n && n < l);
        body_invariant!(bgn == 0);
        body_invariant!(l == v.len());
        let i = gen_range(bgn, l - n);
        v.swap(i, l - n - 1);
        n += 1;
    }
}

// #[trusted]
// fn print_vector_before(v: &mut VecWrapperI32) {
//     println!("before: {:?}", v.v);
// }

// #[trusted]
// fn print_vector_after(v: &mut VecWrapperI32) {
//     println!("after:  {:?}", v.v);
// }

// fn test() {
//     let mut v = VecWrapperI32::new();
//     let mut i = 0;
//     while i < 10 {
//         v.push(i);
//     }

//     print_vector_before(&mut v);
//     knuth_shuffle(&mut v);
//     print_vector_after(&mut v);
// }