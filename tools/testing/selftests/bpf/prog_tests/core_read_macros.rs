// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Rust translation of includes:
 * #include <test_progs.h>
 * #include "test_core_read_macros.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

#[repr(C)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: *mut c_void,
}

/* ___shuffled flavor is just an illusion for BPF code, it doesn't really
 * exist and user-space needs to provide data in the memory layout that
 * matches callback_head. We just defined ___shuffled flavor to make it easier
 * to work with the skeleton
 */
#[repr(C)]
pub struct callback_head___shuffled {
    pub next: *mut callback_head___shuffled,
    pub func: *mut c_void,
}

#[repr(C)]
pub struct test_core_read_macros {
    pub bss: *mut test_core_read_macros__bss,
}

#[repr(C)]
pub struct test_core_read_macros__bss {
    pub my_pid: c_int,
    pub k_probe_in: callback_head,
    pub k_core_in: callback_head,
    pub u_probe_in: *mut callback_head,
    pub u_core_in: *mut callback_head___shuffled,
    pub k_probe_out: c_uint,
    pub k_core_out: c_uint,
    pub u_probe_out: c_uint,
    pub u_core_out: c_uint,
}

unsafe extern "C" {
    fn test_core_read_macros__open_and_load() -> *mut test_core_read_macros;
    fn test_core_read_macros__attach(skel: *mut test_core_read_macros) -> c_int;
    fn test_core_read_macros__destroy(skel: *mut test_core_read_macros);

    fn getpid() -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn CHECK(condition: c_int, name: *const c_char, format: *const c_char, ...) -> c_int;
    fn ASSERT_EQ(actual: c_uint, expected: c_uint, name: *const c_char);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_core_read_macros() {
    let mut duration: c_int = 0;
    let mut err: c_int;
    let skel: *mut test_core_read_macros;
    let bss: *mut test_core_read_macros__bss;
    let mut u_probe_in: callback_head = callback_head {
        next: core::ptr::null_mut(),
        func: core::ptr::null_mut(),
    };
    let mut u_core_in: callback_head___shuffled = callback_head___shuffled {
        next: core::ptr::null_mut(),
        func: core::ptr::null_mut(),
    };

    skel = test_core_read_macros__open_and_load();
    if CHECK(
        skel.is_null() as c_int,
        b"skel_open\0".as_ptr() as *const c_char,
        b"failed to open skeleton\n\0".as_ptr() as *const c_char,
    ) != 0
    {
        return;
    }
    bss = (*skel).bss;
    (*bss).my_pid = getpid();

    /* next pointers have to be set from the kernel side */
    (*bss).k_probe_in.func = 0x1234 as c_long as *mut c_void;
    (*bss).k_core_in.func = 0xabcd as c_long as *mut c_void;

    u_probe_in.next = &mut u_probe_in;
    u_probe_in.func = 0x5678 as c_long as *mut c_void;
    (*bss).u_probe_in = &mut u_probe_in;

    u_core_in.next = &mut u_core_in;
    u_core_in.func = 0xdbca as c_long as *mut c_void;
    (*bss).u_core_in = &mut u_core_in;

    err = test_core_read_macros__attach(skel);
    if CHECK(
        err,
        b"skel_attach\0".as_ptr() as *const c_char,
        b"skeleton attach failed: %d\n\0".as_ptr() as *const c_char,
        err,
    ) != 0
    {
        test_core_read_macros__destroy(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    ASSERT_EQ((*bss).k_probe_out, 0x1234, b"k_probe_out\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).k_core_out, 0xabcd, b"k_core_out\0".as_ptr() as *const c_char);

    ASSERT_EQ((*bss).u_probe_out, 0x5678, b"u_probe_out\0".as_ptr() as *const c_char);
    ASSERT_EQ((*bss).u_core_out, 0xdbca, b"u_core_out\0".as_ptr() as *const c_char);

    test_core_read_macros__destroy(skel);

    let _ = duration;
}
