#![allow(unused)]
#![feature(register_tool)]
#![register_tool(flux)]

// Fig. 1

#[flux::sig(fn(i32[@n]) -> bool[n > 0])]
fn is_pos(n: i32) -> bool {
    if n > 0 {
        true
    } else {
        false
    }
}

#[flux::sig(fn(i32[@x]) -> i32{v: v >= x && v >= 0})]
fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

// Fig. 2

#[flux::alias(type Nat = i32{v: v >= 0})]
type Nat = i32;

#[flux::sig(fn(&mut Nat))]
fn decr(x: &mut Nat) {
    let y = *x;
    *x = y - 1;
}

#[flux::sig(fn(bool) -> Nat)]
fn ref_join(z: bool) -> Nat {
    let mut x = 1;
    let mut y = 2;
    let r = if z { &mut x } else { &mut y };
    decr(r);
    x
}

#[flux::sig(fn() -> Nat)]
fn use_swap() -> Nat {
    let mut x = 0;
    let mut y = 1;
    std::mem::swap(&mut x, &mut y);
    x
}

#[flux::sig(
   fn(x: &strg i32[@n])
   ensures x: i32[n + 1]
)]
fn incr(x: &mut i32) {
    *x += 1;
}

// Fig. 5

#[flux::refined_by(len: int)]
enum List<T> {
    #[flux::variant(List<T>[0])]
    Nil,
    #[flux::variant((T, Box<List<T>[@n]>) -> List<T>[n + 1])]
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    #[flux::sig(
        fn(self: &strg List<T>[@n], List<T>[@m])
        ensures self: List<T>[n+m]
    )]
    fn append(&mut self, other: List<T>) {
        match self {
            List::Cons(_, tl) => tl.append(other),
            List::Nil => *self = other,
        }
    }
}

fn main() {}
