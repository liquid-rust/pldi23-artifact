#![allow(unused_attributes)]
#![feature(register_tool)]
#![register_tool(flux)]

#[path = "lib/rvec.rs"]
pub mod rvec;
use rvec::RVec;
use std::f32;

#[flux::sig(fn(px: &mut RVec<f32>[@n], py: &mut RVec<f32>[n]) requires 2 <= n)]
pub fn fft(px: &mut RVec<f32>, py: &mut RVec<f32>) {
    loop_a(px, py);
    loop_b(px, py);
    loop_c(px, py);
}

#[flux::sig(fn({&mut RVec<f32>[@n] | n > 0}, py: &mut RVec<f32>[n]))]
fn loop_a(px: &mut RVec<f32>, py: &mut RVec<f32>) {
    let n = px.len() - 1;
    let mut n2 = n;
    let mut n4 = n / 4;

    while 2 < n2 {
        let e = 2.0 * f32::consts::PI / (n2 as f32);
        let e3 = 3.0 * e;
        let mut a = 0.0;
        let mut a3 = 0.0;
        let mut j = 1;
        while j <= n4 {
            let cc1 = f32::cos(a);
            let ss1 = f32::sin(a);
            let cc3 = f32::cos(a3);
            let ss3 = f32::sin(a3);
            a = a + e;
            a3 = a3 + e3;

            let mut is = j;
            let mut id = 2 * n2;
            while is < n {
                let mut i0 = is;
                let mut i1 = i0 + n4;
                let mut i2 = i1 + n4;
                let mut i3 = i2 + n4;

                while i3 <= n {
                    let r1 = *px.get(i0) - *px.get(i2);
                    *px.get_mut(i0) = *px.get(i0) + *px.get(i2);
                    let r2 = *px.get(i1) - *px.get(i3);
                    *px.get_mut(i1) = *px.get(i1) + *px.get(i3);
                    let s1 = *py.get(i0) - *py.get(i2);
                    *py.get_mut(i0) = *py.get(i0) + *py.get(i2);
                    let s2 = *py.get(i1) - *py.get(i3);
                    *py.get_mut(i1) = *py.get(i1) + *py.get(i3);

                    let s3 = r1 - s2;
                    let r1 = r1 + s2;
                    let s2 = r2 - s1;
                    let r2 = r2 + s1;
                    *px.get_mut(i2) = r1 * cc1 - s2 * ss1;
                    *py.get_mut(i2) = (0. - s2) * cc1 - r1 * ss1;
                    *px.get_mut(i3) = s3 * cc3 + r2 * ss3;
                    *py.get_mut(i3) = r2 * cc3 - s3 * ss3;

                    i0 = i0 + id;
                    i1 = i1 + id;
                    i2 = i2 + id;
                    i3 = i3 + id;
                }

                is = 2 * id - n2 + j;
                id = 4 * id;
            }
            j += 1
        }
        n2 = n2 / 2;
        n4 = n4 / 2;
    }
}

#[flux::sig(fn({&mut RVec<f32>[@n] | n > 0}, py: &mut RVec<f32>[n]))]
fn loop_b(px: &mut RVec<f32>, py: &mut RVec<f32>) {
    let n = px.len() - 1;
    let mut is = 1;
    let mut id = 4;
    while is < n {
        let mut i0 = is;
        let mut i1 = is + 1;
        while i1 <= n {
            let r1 = *px.get(i0);
            *px.get_mut(i0) = r1 + *px.get(i1);
            *px.get_mut(i1) = r1 - *px.get(i1);

            let r1 = *py.get(i0);
            *py.get_mut(i0) = r1 + *py.get(i1);
            *py.get_mut(i1) = r1 - *py.get(i1);

            i0 = i0 + id;
            i1 = i1 + id;
        }
        is = 2 * id - 1;
        id = 4 * id;
    }
}

#[flux::sig(fn(px: &mut RVec<f32>[@n], py: &mut RVec<f32>[n]) requires 2 <= n)]
fn loop_c(px: &mut RVec<f32>, py: &mut RVec<f32>) {
    let n = px.len() - 1;
    let mut i = 1;
    let mut j = 1;
    while i < n {
        if i < j {
            let xt = *px.get(j);
            *px.get_mut(j) = *px.get(i);
            *px.get_mut(i) = xt;

            let xt = *py.get(j);
            *py.get_mut(j) = *py.get(i);
            *py.get_mut(i) = xt;
        }
        i += 1;
        j = loop_c1(j, n / 2);
    }
}

#[flux::sig(fn (j:usize{0<=j}, k: usize{0<=k}) -> usize{v:v<=k+k})]
pub fn loop_c1(j: usize, k: usize) -> usize {
    if j <= k {
        j + k
    } else {
        loop_c1(j - k, k / 2)
    }
}

#[flux::sig(fn (np:usize) -> f32 requires 2 <= np)]
pub fn fft_test(np: usize) -> f32 {
    let enp = np as f32;
    let n2 = np / 2;
    let npm = n2 - 1;
    let mut pxr = RVec::from_elem_n(0.0, np + 1);
    let mut pxi = RVec::from_elem_n(0.0, np + 1);
    let t = f32::consts::PI / enp;
    *pxr.get_mut(1) = (enp - 1.0) * 0.5;
    *pxi.get_mut(1) = 0.0;
    *pxr.get_mut(n2 + 1) = 0.0 - 0.5;
    *pxi.get_mut(n2 + 1) = 0.0;
    let mut i = 1;
    while i <= npm {
        let j = np - i;
        *pxr.get_mut(i + 1) = 0.0 - 0.5;
        *pxr.get_mut(j + 1) = 0.0 - 0.5;
        let z = t * (i as f32);
        let y = 0.5 * f32::cos(z) / (z as f32);
        *pxi.get_mut(i + 1) = 0.0 - y;
        *pxi.get_mut(j + 1) = y;
        i += 1;
    }

    fft(&mut pxr, &mut pxi);

    let mut zr = 0.0;
    let mut zi = 0.0;
    let mut _kr = 0;
    let mut _ki = 0;
    let mut i = 0;
    while i < np {
        let a = f32::abs(*pxr.get(i + 1) - (i as f32));
        if zr < a {
            zr = a;
            _kr = i;
        }
        let a = f32::abs(*pxi.get(i + 1));
        if zi < a {
            zi = a;
            _ki = i;
        }
        i += 1;
    }
    if f32::abs(zr) < f32::abs(zi) {
        zi
    } else {
        zr
    }
}

#[flux::sig(fn() -> i32)]
pub fn doit() -> i32 {
    let mut i = 4;
    let mut np = 16;
    while i <= 16 {
        fft_test(np);
        i = i + 1;
        np = np * 2;
    }
    0
}
