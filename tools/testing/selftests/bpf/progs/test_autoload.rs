// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Translated from C includes:
 * "vmlinux.h"
 * <bpf/bpf_helpers.h>
 * <bpf/bpf_tracing.h>
 * <bpf/bpf_core_read.h>
 */

pub static mut prog1_called: bool = false;
pub static mut prog2_called: bool = false;
pub static mut prog3_called: bool = false;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn prog1(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        prog1_called = true;
    }
    0
}

#[no_mangle]
#[link_section = "raw_tp/sys_exit"]
pub unsafe extern "C" fn prog2(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    unsafe {
        prog2_called = true;
    }
    0
}

#[repr(C)]
pub struct fake_kernel_struct {
    pub whatever: i32,
}
/* C attribute preserved_access_index applies to fake_kernel_struct. */

#[no_mangle]
#[link_section = "fentry/unexisting-kprobe-will-fail-if-loaded"]
pub unsafe extern "C" fn prog3(ctx: *const core::ffi::c_void) -> i32 {
    let fake: *mut fake_kernel_struct = ctx as *mut core::ffi::c_void as *mut fake_kernel_struct;
    unsafe {
        (*fake).whatever = 123;
        prog3_called = true;
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
