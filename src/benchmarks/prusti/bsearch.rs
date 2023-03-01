extern crate prusti_contracts;
use prusti_contracts::*;

#[path = "lib/vecwrapper.rs"]
pub mod vecwrapper;
use vecwrapper::VecWrapper;

pub fn binary_search(k: i32, items: &VecWrapper<i32>) -> usize {
    let size = items.len();
    if size <= 0 {
        return size;
    }

    let mut low: usize = 0;
    let mut high: usize = size - 1;

    while low <= high {
        body_invariant!(low < size && high < size);
        // SAFE   let middle = (high + low) / 2;
        // let middle = high + ((high - low) / 2);
        let middle = low + ((high - low) / 2);
        let current = items.lookup(middle);
        if current == k {
            return middle;
        }
        if current > k {
            if middle == 0 {
                return size;
            }
            high = middle - 1
        }
        if current < k {
            low = middle + 1
        }
    }
    size
}
