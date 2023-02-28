extern crate prusti_contracts;
use prusti_contracts::*;
#[path = "lib/vecwrapper.rs"]
pub mod vecwrapper;
use vecwrapper::VecWrapper;

// Note: reduce helper function inlined because support for closures not yet in Prusti
#[requires(vec.len() > 0)]
#[ensures(result < vec.len())]
fn min_index(vec: VecWrapper<i32>) -> usize {
    let mut res = 0;
    let sz = vec.len();
    let mut i = 0;

    while i < sz {
        body_invariant!(res < sz && i < sz);

        res = if vec.lookup(i) < vec.lookup(res) {
            i
        } else {
            res
        };

        i = i + 1;
    }
    res
}
