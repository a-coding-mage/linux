// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2021 Google LLC.
 */

// Dependencies from the original C file:
// #include <errno.h>
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

pub const EISCONN: i32 = 106;

extern "C" {
    fn bpf_get_retval() -> u32;
    fn bpf_set_retval(retval: i32) -> i32;
}

#[repr(C)]
pub struct bpf_sockopt {
    pub sk: *mut core::ffi::c_void,
    pub optval: *mut core::ffi::c_void,
    pub optval_end: *mut core::ffi::c_void,
    pub level: i32,
    pub optname: i32,
    pub optlen: i32,
    pub retval: i32,
}

extern "C" {
    fn __sync_fetch_and_add(ptr: *mut u32, value: u32) -> u32;
}

#[no_mangle]
pub static mut invocations: u32 = 0;
#[no_mangle]
pub static mut assertion_error: u32 = 0;
#[no_mangle]
pub static mut retval_value: u32 = 0;
#[no_mangle]
pub static mut ctx_retval_value: u32 = 0;
#[no_mangle]
pub static mut page_size: u32 = 0;

#[link_section = "cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn get_retval(ctx: *mut bpf_sockopt) -> i32 {
    retval_value = bpf_get_retval();
    ctx_retval_value = (*ctx).retval as u32;
    __sync_fetch_and_add(&mut invocations, 1);

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size as i32 {
        (*ctx).optlen = 0;
    }

    1
}

#[link_section = "cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn set_eisconn(ctx: *mut bpf_sockopt) -> i32 {
    __sync_fetch_and_add(&mut invocations, 1);

    if bpf_set_retval(-EISCONN) != 0 {
        assertion_error = 1;
    }

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size as i32 {
        (*ctx).optlen = 0;
    }

    1
}

#[link_section = "cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn clear_retval(ctx: *mut bpf_sockopt) -> i32 {
    __sync_fetch_and_add(&mut invocations, 1);

    (*ctx).retval = 0;

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size as i32 {
        (*ctx).optlen = 0;
    }

    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
