use crate::rvec::RVec;
use crate::types::{NativeIoVec, NativeIoVecOk, SboxPtr, VmCtx, WasmIoVec};

impl VmCtx {
    #[flux::sig(fn(&VmCtx[@cx], WasmIoVec) -> NativeIoVecOk(cx.base))]
    pub fn translate_iov(&self, iov: WasmIoVec) -> NativeIoVecOk {
        let swizzled_base = self.raw + iov.iov_base as usize;
        NativeIoVec {
            iov_base: swizzled_base,
            iov_len: iov.iov_len as usize,
        }
    }

    // FLUX-TODO: capacity
    #[flux::trusted]
    #[flux::sig(fn(&VmCtx, &mut RVec<u8>[n], src: SboxPtr{src + n < LINEAR_MEM_SIZE}, n:u32{0 <= n}))]
    #[allow(unused_variables)]
    pub fn memcpy_from_sandbox(&self, dst: &mut RVec<u8>, src: SboxPtr, n: u32) {
        todo!()
    }

    #[allow(unused_variables)]
    #[flux::trusted]
    #[flux::sig(fn(&mut VmCtx[@cx], dst: SboxPtr{dst + n < LINEAR_MEM_SIZE}, &RVec<u8>{sz:n <= sz}, n:u32))]
    pub fn memcpy_to_sandbox(&mut self, dst: SboxPtr, src: &RVec<u8>, n: u32) {
        todo!()
    }
}
