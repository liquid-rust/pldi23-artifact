use crate::{
    path_resolution::resolve_path,
    rvec::RVec,
    tcb::path::{HostPath, HostPathSafe},
    types::*,
};
use RuntimeError::*;

#[flux::alias(type FitsBool(buf: int, cnt: int) = bool[0 <= buf && 0 <= cnt && buf <= buf + cnt && buf + cnt < LINEAR_MEM_SIZE])]
pub type FitsBool = bool;

#[flux::alias(type FitsUsize(buf: int) = usize{cnt : 0 <= buf && 0 <= cnt && buf <= buf + cnt && buf + cnt < LINEAR_MEM_SIZE})]
pub type FitsUsize = usize;

impl VmCtx {
    /// Check whether sandbox pointer is actually inside the sandbox
    // TODO: can I eliminate this in favor os in_lin_mem_usize?
    #[flux::sig(fn(&VmCtx, ptr:SboxPtr) -> bool[0 <= ptr && ptr < LINEAR_MEM_SIZE])]
    pub fn in_lin_mem(&self, ptr: SboxPtr) -> bool {
        (ptr as usize >= 0) && (ptr as usize) < self.memlen
    }

    #[flux::sig(fn(&VmCtx, ptr:usize) -> bool[0 <= ptr && ptr < LINEAR_MEM_SIZE])]
    pub fn in_lin_mem_usize(&self, ptr: usize) -> bool {
        ptr >= 0 && ptr < self.memlen
    }

    /// Check whether buffer is entirely within sandbox
    // Can I eliminate this in favor of fits_in_lin_mem_usize
    #[flux::sig(fn(&VmCtx, buf: SboxPtr, cnt: u32) -> FitsBool(buf, cnt))]
    pub fn fits_in_lin_mem(&self, buf: SboxPtr, cnt: u32) -> FitsBool {
        let total_size = (buf as usize) + (cnt as usize);
        if total_size >= self.memlen {
            return false;
        }
        self.in_lin_mem(buf) && self.in_lin_mem(cnt) && buf <= buf + cnt
    }

    #[flux::sig(fn(&VmCtx, buf:usize, cnt:usize) -> FitsBool(buf, cnt))]
    pub fn fits_in_lin_mem_usize(&self, buf: usize, cnt: usize) -> FitsBool {
        let total_size = buf + cnt;
        if total_size >= self.memlen {
            return false;
        }
        self.in_lin_mem_usize(buf) && self.in_lin_mem_usize(cnt) && buf <= buf + cnt
    }

    /// Copy buffer from sandbox to host
    #[flux::sig(fn(&VmCtx, src:SboxPtr, n:u32{0 <= n && src + n < LINEAR_MEM_SIZE}) -> RVec<u8>[n])]
    pub fn copy_buf_from_sandbox(&self, src: SboxPtr, n: u32) -> RVec<u8> {
        let mut host_buffer: RVec<u8> = RVec::from_elem_n(0, n as usize);
        // FLUX-TODO-capacity: host_buffer.reserve_exact(n as usize);
        // assert!(src >= 0);
        // assert!(((n as usize) < self.memlen) && ((n as usize) >= 0));
        self.memcpy_from_sandbox(&mut host_buffer, src, n);
        host_buffer
    }

    /// Copy buffer from from host to sandbox
    #[flux::sig(fn(&mut VmCtx[@cx], SboxPtr, &RVec<u8>, u32) -> Result<(), RuntimeError>)]
    pub fn copy_buf_to_sandbox(
        &mut self,
        dst: SboxPtr,
        src: &RVec<u8>,
        n: u32,
    ) -> Result<(), RuntimeError> {
        if src.len() < n as usize || !self.fits_in_lin_mem(dst, n) {
            return Err(Efault);
        }
        self.memcpy_to_sandbox(dst, src, n);
        Ok(())
    }

    /// Copy arg buffer from from host to sandbox
    #[flux::sig(fn(&mut {VmCtx[@cx] : cx.arg_buf == n}, dst: SboxPtr, n:u32) -> Result<(), RuntimeError>)]
    pub fn copy_arg_buffer_to_sandbox(&mut self, dst: SboxPtr, n: u32) -> Result<(), RuntimeError> {
        if !self.fits_in_lin_mem(dst, n) {
            return Err(Efault);
        }
        let arg_buffer = &self.arg_buffer.clone();
        self.memcpy_to_sandbox(dst, &arg_buffer, n);
        Ok(())
    }

    /// Copy arg buffer from from host to sandbox
    #[flux::sig(fn(&mut {VmCtx[@cx] : cx.env_buf == n}, dst: SboxPtr, n:u32) -> Result<(), RuntimeError>)]
    pub fn copy_environ_buffer_to_sandbox(
        &mut self,
        dst: SboxPtr,
        n: u32,
    ) -> Result<(), RuntimeError> {
        if !self.fits_in_lin_mem(dst, n) {
            return Err(Efault);
        }
        let env_buffer = &self.env_buffer.clone();
        self.memcpy_to_sandbox(dst, &env_buffer, n);
        Ok(())
    }

    #[flux::sig(fn(&VmCtx[@cx], SboxPtr, u32, should_follow:bool, HostFd) -> Result<HostPathSafe(should_follow), RuntimeError>)]
    pub fn translate_path(
        &self,
        path: SboxPtr,
        path_len: u32,
        should_follow: bool,
        dirfd: HostFd,
    ) -> Result<HostPathSafe, RuntimeError> {
        if !self.fits_in_lin_mem(path, path_len) {
            return Err(Eoverflow);
        }
        let host_buffer = self.copy_buf_from_sandbox(path, path_len);
        resolve_path(host_buffer, should_follow, dirfd)
        // self.resolve_path(host_buffer)
    }

    #[flux::sig(fn(&VmCtx, FitsUsize(2)) -> u16)]
    pub fn read_u16(&self, start: FitsUsize) -> u16 {
        let bytes: [u8; 2] = [self.mem[start], self.mem[start + 1]];
        u16::from_le_bytes(bytes)
    }

    /// read u32 from wasm linear memory
    // Not thrilled about this implementation, but it works
    #[flux::sig(fn(&VmCtx, FitsUsize(4)) -> u32)]
    pub fn read_u32(&self, start: FitsUsize) -> u32 {
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
    #[flux::sig(fn(&VmCtx, FitsUsize(8)) -> u64)]
    pub fn read_u64(&self, start: FitsUsize) -> u64 {
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
    pub fn read_u32_pair(&self, start: usize) -> RuntimeResult<(u32, u32)> {
        if !self.fits_in_lin_mem_usize(start, 8) {
            return Err(Eoverflow);
        }
        let x1 = self.read_u32(start);
        let x2 = self.read_u32(start + 4);
        Ok((x1, x2))
    }

    #[flux::sig(fn(&VmCtx[@cx], &RVec<WasmIoVec>) -> RVec<NativeIoVecOk(cx.base)>)]
    pub fn translate_iovs(&self, iovs: &RVec<WasmIoVec>) -> RVec<NativeIoVecOk> {
        let mut idx = 0;
        let mut native_iovs = NativeIoVecs::new();
        let iovcnt = iovs.len();
        while idx < iovcnt {
            let iov = iovs[idx];
            let native_iov = self.translate_iov(iov);
            native_iovs.push(native_iov);
            idx += 1;
        }

        native_iovs
    }

    // TODO @cx is redundant here but due to https://github.com/liquid-rust/flux/issues/158
    #[flux::sig(fn (&mut VmCtx[@cx], FitsUsize(1), v: u8))]
    pub fn write_u8(&mut self, offset: FitsUsize, v: u8) {
        self.mem[offset] = v;
    }
}
