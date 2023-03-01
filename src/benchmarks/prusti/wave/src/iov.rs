#[cfg(feature = "verify")]
use crate::tcb::verifier::*;
use crate::{types::*, unwrap_result};
use prusti_contracts::*;
use RuntimeError::*;

#[requires(ctx_safe(ctx))]
#[ensures(ctx_safe(ctx))]
#[ensures(
    match &result {
        Ok(wasm_iovs) => wasm_iovs.len() >= 0 &&  (forall(|idx: usize|  (idx < wasm_iovs.len() && idx >= 0) ==> {
            let iov = wasm_iovs.lookup(idx);
            let buf = iov.iov_base;
            let cnt = iov.iov_len;
            (buf >= 0) && (cnt >= 0) &&
            (buf as usize) + (cnt as usize) < LINEAR_MEM_SIZE &&
            (buf <= buf + cnt)
        })),
        _ => true,
    }
)]
pub fn parse_iovs(ctx: &VmCtx, iovs: u32, iovcnt: u32) -> RuntimeResult<WasmIoVecs> {
    let mut i = 0;
    let mut wasm_iovs = WasmIoVecs::new();
    while i < iovcnt {
        body_invariant!(ctx_safe(ctx));
        body_invariant!(wasm_iovs.len() >= 0);
        body_invariant!(
            forall(|idx: usize|  (idx < wasm_iovs.len() && idx >= 0) ==> {
                let iov = wasm_iovs.lookup(idx);
                let buf = iov.iov_base;
                let cnt = iov.iov_len;
                (buf >= 0) && (cnt >= 0) &&
                (buf as usize) + (cnt as usize) < LINEAR_MEM_SIZE &&
                (buf <= buf + cnt)
            })
        );

        let start = (iovs + i * 8) as usize;
        let v = ctx.read_u32_pair(start);
        unwrap_result!(v);
        let (ptr, len) = v;

        if !ctx.fits_in_lin_mem(ptr, len) {
            return Err(Efault);
        }

        wasm_iovs.push(WasmIoVec {
            iov_base: ptr,
            iov_len: len,
        });
        i += 1;
    }
    assert!(wasm_iovs.len() >= 0);

    Ok(wasm_iovs)
}
