use crate::rvec::RVec;

#[flux::constant]
pub const LINEAR_MEM_SIZE: usize = 4294965096; //4GB

#[flux::constant]
pub const TWO_POWER_20: usize = 1024 * 1024;

#[flux::constant]
pub const PATH_MAX: usize = 4096;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[flux::alias(type SboxPtr[n: int] = u32[@n])]
pub type SboxPtr = u32;
pub type HostPtr = usize;

pub enum RuntimeError {
    Success = 0,
    Efault,
    Eoverflow,
    Eloop,
    Enotcapable,
    Enametoolong,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[flux::refined_by(iov_base: int)]
pub struct WasmIoVec {
    #[flux::field({ u32[@iov_base] : 0 <= iov_base})]
    pub iov_base: u32,
    #[flux::field(u32{ len : 0 <= len && iov_base <= iov_base + len && iov_base + len < LINEAR_MEM_SIZE })]
    pub iov_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[flux::refined_by(iov_base: int, iov_len: int)]
pub struct NativeIoVec {
    #[flux::field(usize[@iov_base])]
    pub iov_base: usize,
    #[flux::field(usize[@iov_len])]
    pub iov_len: usize,
}

#[flux::alias(type NativeIoVecOk(base: int) = NativeIoVec{v: v.iov_base + v.iov_len <= base + LINEAR_MEM_SIZE})]
pub type NativeIoVecOk = NativeIoVec;

pub type NativeIoVecs = RVec<NativeIoVec>;
// An `assert` function, whose precondition expects only `true`
#[flux::sig(fn(bool[true]))]
pub fn assert(_b: bool) {}

#[macro_export]
macro_rules! unwrap_result {
    ($p:ident) => {
        let $p = match $p {
            Ok(oc) => oc,
            Err(e) => {
                return Err(e);
            }
        };
    };
}

#[flux::refined_by(arg_buf: int, env_buf: int, base: int)]
pub struct VmCtx {
    #[flux::field(usize[@base])]
    pub raw: usize,
    #[flux::field(RVec<u8>[LINEAR_MEM_SIZE])]
    pub mem: RVec<u8>,
    #[flux::field(usize[LINEAR_MEM_SIZE])]
    pub memlen: usize,
    #[flux::field(RVec<u8>[@arg_buf])]
    pub arg_buffer: RVec<u8>,
    #[flux::field(RVec<u8>[@env_buf])]
    pub env_buffer: RVec<u8>,
    #[flux::field(usize{v: v < 1024})]
    pub envc: usize,
    #[flux::field(usize{v: v < 1024})]
    pub argc: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HostFd(usize);

impl HostFd {
    pub(crate) fn to_raw(&self) -> usize {
        self.0
    }

    #[allow(dead_code)]
    pub(crate) fn from_raw(w: usize) -> HostFd {
        HostFd(w)
    }
}

//////////////////////////////////////////////////////////////////////////////
// Various rust features that are not supported by flux
//////////////////////////////////////////////////////////////////////////////

#[flux::trusted]
#[flux::sig(fn (&RVec<T>) -> usize{v:0<=v})]
pub fn raw_ptr<T>(_v: &RVec<T>) -> usize {
    unimplemented!()
}
