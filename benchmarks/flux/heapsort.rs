#![feature(register_tool)]
#![register_tool(flux)]

#[path = "lib/rvec.rs"]
mod rvec;
use rvec::RVec;

#[flux::sig(fn(vec: &mut RVec<i32>[@n]))]
pub fn heap_sort(vec: &mut RVec<i32>) {
    let len = vec.len();

    if len <= 0 {
        return;
    }

    let mut start = len / 2;
    while start > 0 {
        start -= 1;
        shift_down(vec, start, len - 1);
    }

    let mut end = len;
    while end > 1 {
        end -= 1;
        let start = 0;
        vec.swap(start, end);
        shift_down(vec, start, end - 1);
    }
}

#[flux::sig(fn(vec: &mut RVec<i32>[@len], s:usize{s < len}, e:usize{e < len}))]
pub fn shift_down(vec: &mut RVec<i32>, start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        } else {
            if child + 1 <= end && *vec.get(child) < *vec.get(child + 1) {
                child += 1;
            }
            if *vec.get(root) < *vec.get(child) {
                vec.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }
}
