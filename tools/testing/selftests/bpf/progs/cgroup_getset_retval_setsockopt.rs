// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2021 Google LLC.
 */

// C dependencies: <errno.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::sync::atomic::{AtomicU32, Ordering};

type __u32 = u32;
type __s32 = i32;

const EUNATCH: i32 = 49;
const EISCONN: i32 = 106;

#[repr(C)]
pub struct bpf_sockopt {
    pub optlen: __s32,
}

extern "C" {
    fn bpf_get_retval() -> __s32;
    fn bpf_set_retval(retval: __s32) -> __s32;
}

#[no_mangle]
pub static invocations: AtomicU32 = AtomicU32::new(0);
#[no_mangle]
pub static mut assertion_error: __u32 = 0;
#[no_mangle]
pub static mut retval_value: __u32 = 0;
#[no_mangle]
pub static mut page_size: __s32 = 0;

// SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn get_retval(ctx: *mut bpf_sockopt) -> i32 {
    retval_value = bpf_get_retval() as __u32;
    invocations.fetch_add(1, Ordering::SeqCst);

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size {
        (*ctx).optlen = 0;
    }

    return 1;
}

// SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn set_eunatch(ctx: *mut bpf_sockopt) -> i32 {
    invocations.fetch_add(1, Ordering::SeqCst);

    if bpf_set_retval(-EUNATCH) != 0 {
        assertion_error = 1;
    }

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size {
        (*ctx).optlen = 0;
    }

    return 0;
}

// SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn set_eisconn(ctx: *mut bpf_sockopt) -> i32 {
    invocations.fetch_add(1, Ordering::SeqCst);

    if bpf_set_retval(-EISCONN) != 0 {
        assertion_error = 1;
    }

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size {
        (*ctx).optlen = 0;
    }

    return 0;
}

// SEC("cgroup/setsockopt")
#[no_mangle]
pub unsafe extern "C" fn legacy_eperm(ctx: *mut bpf_sockopt) -> i32 {
    invocations.fetch_add(1, Ordering::SeqCst);

    /* optval larger than PAGE_SIZE use kernel's buffer. */
    if (*ctx).optlen > page_size {
        (*ctx).optlen = 0;
    }

    return 0;
}
