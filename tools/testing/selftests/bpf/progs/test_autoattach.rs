// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Google */

// Dependencies in the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>

#[no_mangle]
pub static mut prog1_called: bool = false;
#[no_mangle]
pub static mut prog2_called: bool = false;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn prog1(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    prog1_called = true;
    0
}

#[no_mangle]
#[link_section = "raw_tp/sys_exit"]
pub unsafe extern "C" fn prog2(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    prog2_called = true;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
