/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by <uapi/asm/papr-sysparm.h>.
// Build-time declarations from that header remain external to this translation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_sysparm_t {
    pub token: u32,
}

#[inline]
pub const fn mk_papr_sysparm(x_: u32) -> papr_sysparm_t {
    papr_sysparm_t { token: x_ }
}

/*
 * Derived from the "Defined Parameters" table in PAPR 7.3.16 System
 * Parameters Option. Where the spec says "characteristics", we use
 * "attrs" in the symbolic names to keep them from getting too
 * unwieldy.
 */
pub const PAPR_SYSPARM_SHARED_PROC_LPAR_ATTRS: papr_sysparm_t = mk_papr_sysparm(20);
pub const PAPR_SYSPARM_PROC_MODULE_INFO: papr_sysparm_t = mk_papr_sysparm(43);
pub const PAPR_SYSPARM_COOP_MEM_OVERCOMMIT_ATTRS: papr_sysparm_t = mk_papr_sysparm(44);
pub const PAPR_SYSPARM_TLB_BLOCK_INVALIDATE_ATTRS: papr_sysparm_t = mk_papr_sysparm(50);
pub const PAPR_SYSPARM_LPAR_NAME: papr_sysparm_t = mk_papr_sysparm(55);
pub const PAPR_SYSPARM_HVPIPE_ENABLE: papr_sysparm_t = mk_papr_sysparm(64);

/**
 * struct papr_sysparm_buf - RTAS work area layout for system parameter functions.
 *
 * This is the memory layout of the buffers passed to/from
 * ibm,get-system-parameter and ibm,set-system-parameter. It is
 * distinct from the papr_sysparm_io_block structure that is passed
 * between user space and the kernel.
 */
#[repr(C)]
pub struct papr_sysparm_buf {
    pub len: u16,
    pub val: [u8; PAPR_SYSPARM_MAX_OUTPUT],
}

unsafe extern "C" {
    pub fn papr_sysparm_buf_alloc() -> *mut papr_sysparm_buf;
    pub fn papr_sysparm_buf_free(buf: *mut papr_sysparm_buf);
    pub fn papr_sysparm_set(param: papr_sysparm_t, buf: *const papr_sysparm_buf) -> i32;
    pub fn papr_sysparm_get(param: papr_sysparm_t, buf: *mut papr_sysparm_buf) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
