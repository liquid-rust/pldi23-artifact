use prusti_contracts::*;

use crate::types::*;

// Once again, Prusti does not accept that this is pure
#[trusted]
#[pure]
#[requires(index < vec.len() )]
pub fn iovs_checked_lookup(vec: &NativeIoVecs, index: usize) -> NativeIoVec {
    vec.iovs[index]
}

#[trusted]
#[ensures(result.len() == old(vec.len()))]
pub fn clone_vec_u8(vec: &Vec<u8>) -> Vec<u8> {
    vec.clone()
}

// Once again, Prusti does not accept that this is pure
#[trusted]
#[pure]
#[requires(index < vec.len() )]
pub fn wasm_iovs_checked_lookup(vec: &WasmIoVecs, index: usize) -> WasmIoVec {
    vec.iovs[index]
}
