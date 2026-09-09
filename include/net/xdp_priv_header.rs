/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux networking and RHashtable interfaces:
// linux/rhashtable.h and net/xdp.h.

/* Private to net/core/xdp.c, but used by trace/events/xdp.h */
#[repr(C)]
pub struct xdp_mem_allocator {
    pub mem: xdp_mem_info,
    pub allocator: xdp_mem_allocator_allocator,
    pub node: rhash_head,
    pub rcu: rcu_head,
}

#[repr(C)]
pub union xdp_mem_allocator_allocator {
    pub allocator: *mut core::ffi::c_void,
    pub page_pool: *mut page_pool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
