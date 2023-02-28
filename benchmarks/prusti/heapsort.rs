//! An adaptation of the example from
//! https://rosettacode.org/wiki/Sorting_algorithms/Heapsort#Rust
//!
//! Changes:
//!
//! +   Monomorphised types.
//! +   Wrapped built-in types and functions.
//! +   Replaced closure with a function.
//! +   Rewrote loops into supported shape (while bool with no break, continue, or return).
//!
//! Verified properties:
//!
//! +   Absence of panics.
extern crate prusti_contracts;
use prusti_contracts::*;

#[path = "lib/vecwrapper.rs"]
pub mod vecwrapper;
use vecwrapper::VecWrapper;

#[ensures(array.len() == old(array.len()))]
fn heap_sort(array: &mut VecWrapper<i32>) {
    let len = array.len();

    let mut start = len/2;
    // Create heap
    while start > 0 {
        body_invariant!(len == array.len());
        body_invariant!(start <= len/2);
        body_invariant!(start > 0);
        start -= 1;
        shift_down(array, start, len - 1);
    }

    let mut end = len;
    while end > 1 {
        body_invariant!(len == array.len());
        body_invariant!(end <= len);
        body_invariant!(end > 1);

        end -= 1;
        let start = 0;
        array.swap(start, end);
        shift_down(array, start, end - 1);
    }
}

#[requires(end < array.len())]
#[requires(start < array.len())]
#[requires(end < array.len())]
#[ensures(array.len() == old(array.len()))]
fn shift_down(array: &mut VecWrapper<i32>, start: usize, end: usize) {
    let mut root = start;
    let mut continue_loop = true;
    while continue_loop {
        body_invariant!(root < array.len());
        body_invariant!(end < array.len());
        body_invariant!(array.len() == old(array.len()));
        let mut child = root * 2 + 1;
        if child > end {
            continue_loop = false;
        } else {
            if child + 1 <= end {
                if array.lookup(child) < array.lookup(child + 1) {
                    child += 1;
                }
            }
            if array.lookup(root) < array.lookup(child) {
                array.swap(root, child);
                root = child
            } else {
                continue_loop = false;
            }
        }
    }
}

pub fn main() {}
