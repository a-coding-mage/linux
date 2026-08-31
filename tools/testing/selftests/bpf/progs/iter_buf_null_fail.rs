// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Qi Tang */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::ffi::c_void;

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub key: *mut c_void,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* Verify that the verifier rejects direct access to nullable PTR_TO_BUF. */
#[no_mangle]
#[link_section = "iter/bpf_map_elem"]
// __failure
// __msg("invalid mem access")
pub unsafe extern "C" fn iter_buf_null_deref(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    /*
     * ctx->key is PTR_TO_BUF | PTR_MAYBE_NULL | MEM_RDONLY.
     * Direct access without null check must be rejected.
     */
    let mut v: u32 = core::ptr::read_volatile((*ctx).key as *const u32);

    core::ptr::read_volatile(&v);
    return 0;
}

/* Verify that access after a null check is still accepted. */
#[no_mangle]
#[link_section = "iter/bpf_map_elem"]
// __success
pub unsafe extern "C" fn iter_buf_null_check_ok(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let key: *mut u32 = (*ctx).key as *mut u32;

    if key.is_null() {
        return 0;
    }

    let mut v: u32 = core::ptr::read_volatile(key as *const u32);

    core::ptr::read_volatile(&v);
    return 0;
}
