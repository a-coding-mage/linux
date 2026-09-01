// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies:
// #include "test_progs.h"
// #include "core_kern.lskel.h"

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct core_kern_lskel {
    pub bss: *mut core_kern_lskel_bss,
}

#[repr(C)]
pub struct core_kern_lskel_bss {
    pub proto_out: [c_int; 3],
}

unsafe extern "C" {
    fn core_kern_lskel__open_and_load() -> *mut core_kern_lskel;
    fn core_kern_lskel__core_relo_proto__attach(skel: *mut core_kern_lskel) -> c_int;
    fn core_kern_lskel__destroy(skel: *mut core_kern_lskel);

    fn ASSERT_OK_PTR(ptr: *mut core_kern_lskel, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_TRUE(value: bool, name: *const c_char) -> bool;
    fn ASSERT_FALSE(value: bool, name: *const c_char) -> bool;

    fn usleep(usec: c_uint) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_core_kern_lskel() {
    let mut skel: *mut core_kern_lskel;
    let link_fd: c_int;

    skel = unsafe { core_kern_lskel__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) } {
        return;
    }

    link_fd = unsafe { core_kern_lskel__core_relo_proto__attach(skel) };
    if !unsafe { ASSERT_GT(link_fd, 0, c"attach(core_relo_proto)".as_ptr()) } {
        unsafe { core_kern_lskel__destroy(skel) };
        return;
    }

    /* trigger tracepoints */
    unsafe { usleep(1) };
    unsafe {
        ASSERT_TRUE(
            (*(*skel).bss).proto_out[0] != 0,
            c"bpf_core_type_exists".as_ptr(),
        );
        ASSERT_FALSE(
            (*(*skel).bss).proto_out[1] != 0,
            c"!bpf_core_type_exists".as_ptr(),
        );
        ASSERT_TRUE(
            (*(*skel).bss).proto_out[2] != 0,
            c"bpf_core_type_exists. nested".as_ptr(),
        );
    }

    unsafe { core_kern_lskel__destroy(skel) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
