// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

use core::ffi::c_void;
use core::ptr::{read_volatile, write_volatile};

/* 4-byte aligned .data */
static mut static_var1: i32 = 5;
static mut static_var2: i32 = 6;
#[no_mangle]
pub static mut var2: i32 = -1;
/* 8-byte aligned .rodata */
#[no_mangle]
pub static rovar2: i64 = 0;

/* same "subprog" name in both files */
#[inline(never)]
unsafe fn subprog(x: i32) -> i32 {
    /* but different formula */
    x.wrapping_mul(3)
}

// SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler2(ctx: *const c_void) -> i32 {
    let _ = ctx;
    write_volatile(
        core::ptr::addr_of_mut!(var2),
        subprog(read_volatile(core::ptr::addr_of!(rovar2)) as i32)
            .wrapping_add(read_volatile(core::ptr::addr_of!(static_var1)))
            .wrapping_add(read_volatile(core::ptr::addr_of!(static_var2))),
    );

    0
}

/* different name and/or type of the variable doesn't matter */
// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
// SEC("version")
#[no_mangle]
pub static mut _version: i32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
