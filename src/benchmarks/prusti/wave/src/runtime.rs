use crate::tcb::misc::*;
#[cfg(feature = "verify")]
use crate::tcb::path::path_safe;
#[cfg(feature = "verify")]
use crate::tcb::sbox_mem::{raw_ptr, valid_linmem};
#[cfg(feature = "verify")]
use crate::tcb::verifier::*;
use crate::{path_resolution::resolve_path, tcb::path::HostPath, types::*};
use prusti_contracts::*;
use RuntimeError::*;

impl VmCtx {
    /// Check whether sandbox pointer is actually inside the sandbox    // TODO: can I eliminate this in favor os in_lin_mem_usize?
    #[pure]
    #[ensures((result == true) ==> (ptr as usize >= 0) && (ptr as usize) < self.memlen)]
    pub fn in_lin_mem(&self, ptr: SboxPtr) -> bool {
        (ptr as usize >= 0) && (ptr as usize) < self.memlen
    }

    #[pure]
    #[ensures((result == true) ==> ptr >= 0 && ptr < self.memlen)]
    pub fn in_lin_mem_usize(&self, ptr: usize) -> bool {
        ptr >= 0 && ptr < self.memlen
    }

    /// Check whether buffer is entirely within sandbox
    // Can I eliminate this in favor of fits_in_lin_mem_usize
    #[pure]
    #[ensures(result == true ==>
        (buf >= 0) && (cnt >= 0) &&
        (buf as usize) + (cnt as usize) < self.memlen &&
        (buf <= buf + cnt)
    )]
    pub fn fits_in_lin_mem(&self, buf: SboxPtr, cnt: u32) -> bool {
        let total_size = (buf as usize) + (cnt as usize);
        if total_size >= self.memlen {
            return false;
        }
        self.in_lin_mem(buf) && self.in_lin_mem(cnt) && buf <= buf + cnt
    }

    #[pure]
    #[ensures(result == true ==>
        buf >= 0 && cnt >= 0 &&
        buf + cnt < self.memlen &&
        (buf <= buf + cnt)
    )]
    pub fn fits_in_lin_mem_usize(&self, buf: usize, cnt: usize) -> bool {
        let total_size = buf + cnt;
        if total_size >= self.memlen {
            return false;
        }
        self.in_lin_mem_usize(buf) && self.in_lin_mem_usize(cnt) && buf <= buf + cnt
    }

    /// Copy buffer from sandbox to host
    #[requires(self.fits_in_lin_mem(src, n))]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    #[ensures(result.len() == (n as usize) )]
    pub fn copy_buf_from_sandbox(&self, src: SboxPtr, n: u32) -> Vec<u8> {
        let mut host_buffer: Vec<u8> = Vec::new();
        host_buffer.reserve_exact(n as usize);
        // assert!(src >= 0);
        // assert!(((n as usize) < self.memlen) && ((n as usize) >= 0));
        self.memcpy_from_sandbox(&mut host_buffer, src, n);
        host_buffer
    }

    /// Copy buffer from from host to sandbox
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    #[ensures(self.memlen == old(self.memlen))]
    pub fn copy_buf_to_sandbox(
        &mut self,
        dst: SboxPtr,
        src: &Vec<u8>,
        n: u32,
    ) -> RuntimeResult<()> {
        if src.len() < n as usize || !self.fits_in_lin_mem(dst, n) {
            return Err(Efault);
        }
        self.memcpy_to_sandbox(dst, src, n);
        Ok(())
    }

    /// Copy arg buffer from from host to sandbox
    #[requires(self.arg_buffer.len() == (n as usize))]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    pub fn copy_arg_buffer_to_sandbox(&mut self, dst: SboxPtr, n: u32) -> RuntimeResult<()> {
        if !self.fits_in_lin_mem(dst, n) {
            return Err(Efault);
        }
        let arg_buffer = clone_vec_u8(&self.arg_buffer);
        self.memcpy_to_sandbox(dst, &arg_buffer, n);
        Ok(())
    }

    /// Copy arg buffer from from host to sandbox
    #[requires(self.env_buffer.len() == (n as usize) )]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    pub fn copy_environ_buffer_to_sandbox(&mut self, dst: SboxPtr, n: u32) -> RuntimeResult<()> {
        if !self.fits_in_lin_mem(dst, n) {
            return Err(Efault);
        }
        let env_buffer = clone_vec_u8(&self.env_buffer);
        self.memcpy_to_sandbox(dst, &env_buffer, n);
        Ok(())
    }

    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    #[ensures(
        match &result {
            Ok(v) => path_safe(&v, should_follow),
            _ => true,
        }
    )]
    pub fn translate_path(
        &self,
        path: SboxPtr,
        path_len: u32,
        should_follow: bool,
        dirfd: HostFd,
    ) -> RuntimeResult<HostPath> {
        if !self.fits_in_lin_mem(path, path_len) {
            return Err(Eoverflow);
        }
        let host_buffer = self.copy_buf_from_sandbox(path, path_len);
        resolve_path(host_buffer, should_follow, dirfd)
        // self.resolve_path(host_buffer)
    }

    #[requires(self.fits_in_lin_mem_usize(start, 2))]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    pub fn read_u16(&self, start: usize) -> u16 {
        let bytes: [u8; 2] = [self.mem[start], self.mem[start + 1]];
        u16::from_le_bytes(bytes)
    }

    /// read u32 from wasm linear memory
    // Not thrilled about this implementation, but it works
    #[requires(self.fits_in_lin_mem_usize(start, 4 ))]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    pub fn read_u32(&self, start: usize) -> u32 {
        let bytes: [u8; 4] = [
            self.mem[start],
            self.mem[start + 1],
            self.mem[start + 2],
            self.mem[start + 3],
        ];
        u32::from_le_bytes(bytes)
    }

    /// read u64 from wasm linear memory
    // Not thrilled about this implementation, but it works
    // TODO: need to test different implementatiosn for this function
    #[requires(self.fits_in_lin_mem_usize(start, 8))]
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    pub fn read_u64(&self, start: usize) -> u64 {
        let bytes: [u8; 8] = [
            self.mem[start],
            self.mem[start + 1],
            self.mem[start + 2],
            self.mem[start + 3],
            self.mem[start + 4],
            self.mem[start + 5],
            self.mem[start + 6],
            self.mem[start + 7],
        ];
        u64::from_le_bytes(bytes)
    }

    /// read (u32,u32) from wasm linear memory
    #[requires(ctx_safe(self))]
    #[ensures(ctx_safe(self))]
    pub fn read_u32_pair(&self, start: usize) -> RuntimeResult<(u32, u32)> {
        if !self.fits_in_lin_mem_usize(start, 8) {
            return Err(Eoverflow);
        }
        let x1 = self.read_u32(start);
        let x2 = self.read_u32(start + 4);
        Ok((x1, x2))
    }

    #[requires(ctx_safe(self))]
    #[requires(iovs.len() >= 0)]
    #[ensures(ctx_safe(self))]
    #[ensures({
        let mem_ptr = raw_ptr(self.mem.as_slice());
        let mem_len = self.memlen;
        iovs.len() == result.len() &&
        forall(|idx: usize|  (idx >= 0 && idx < result.len()) ==> {
            let wasm_iov = old(iovs.lookup(idx));
            let iov = result.lookup(idx);
            iov.iov_base == raw_ptr(self.mem.as_slice()) + (wasm_iov.iov_base as usize) &&
            iov.iov_len == (wasm_iov.iov_len as usize)
        })
    })]
    pub fn translate_iovs(&self, iovs: &WasmIoVecs) -> NativeIoVecs {
        let mut idx = 0;
        let mut native_iovs = NativeIoVecs::new();
        let iovcnt = iovs.len();
        while idx < iovcnt {
            body_invariant!(idx < iovcnt);
            body_invariant!(native_iovs.len() == idx);
            body_invariant!(ctx_safe(self));
            body_invariant!(
                forall(|idx: usize|  (idx >= 0 && idx < native_iovs.len()) ==> {
                    let wasm_iov = iovs.lookup(idx);
                    let iov = native_iovs.lookup(idx);
                    iov.iov_base == raw_ptr(self.mem.as_slice()) + (wasm_iov.iov_base as usize) &&
                    iov.iov_len == (wasm_iov.iov_len as usize)
                })
            );

            let iov = iovs.lookup(idx);
            let native_iov = self.translate_iov(iov);
            native_iovs.push(native_iov);
            idx += 1;
        }

        native_iovs
    }
}
