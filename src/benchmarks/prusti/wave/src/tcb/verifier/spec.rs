use super::external_specs::vec::*;
use crate::tcb::sbox_mem::{raw_ptr, valid_linmem};
use crate::types::{VmCtx, LINEAR_MEM_SIZE};
use prusti_contracts::*;

#[cfg(feature = "verify")]
predicate! {
    pub fn ctx_safe(ctx: &VmCtx) -> bool {
        //let mem_ptr = raw_ptr(ctx.mem.as_slice());
        ctx.memlen == LINEAR_MEM_SIZE &&
        ctx.argc < 1024 &&
        ctx.envc < 1024 &&
        ctx.arg_buffer.len() < 1024 * 1024 &&
        ctx.env_buffer.len() < 1024 * 1024 &&
        // netlist_unmodified(&ctx.netlist) &&
        valid_linmem(raw_ptr(ctx.mem.as_slice())) //&&
        //mem_ptr <= mem_ptr + count
    }
}
