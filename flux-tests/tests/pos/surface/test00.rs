#![feature(register_tool)]
#![register_tool(flux)]

#[flux::sig(fn(x:i32) -> i32{v: v > x})]
pub fn inc(x: i32) -> i32 {
    x + 1
}

#[flux::sig(fn(x:i32) -> i32{v: v < x})]
pub fn dec(x: i32) -> i32 {
    x - 1
}
