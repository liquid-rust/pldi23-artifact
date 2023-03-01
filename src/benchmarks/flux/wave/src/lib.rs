#![feature(register_tool)]
#![register_tool(flux)]
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]
#![allow(unused_comparisons)]
#![feature(custom_inner_attributes)]
#![flux::defs {
    qualifier MyQ1(x: int, y: int, a: int) { x + y <= a + LINEAR_MEM_SIZE }
}]

pub mod iov;
pub mod path_resolution;
pub mod runtime;
pub mod rvec;
pub mod tcb;
pub mod types;
