// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

/* 8-byte aligned .data */
static mut static_var1: core::ffi::c_long = 2;
static mut static_var2: core::ffi::c_int = 3;
#[no_mangle]
pub static mut var1: core::ffi::c_int = -1;
/* 4-byte aligned .rodata */
static rovar1: core::ffi::c_int = 0;

/* same "subprog" name in both files */
#[inline(never)]
fn subprog(x: core::ffi::c_int) -> core::ffi::c_int {
    /* but different formula */
    x * 2
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handler1(ctx: *const core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    var1 = subprog(core::ptr::read_volatile(&rovar1))
        + core::ptr::read_volatile(&static_var1) as core::ffi::c_int
        + core::ptr::read_volatile(&static_var2);

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut LICENSE: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];
#[no_mangle]
#[link_section = "version"]
pub static mut VERSION: core::ffi::c_int = 1;
