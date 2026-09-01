// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) Meta Platforms, Inc. and affiliates. */

// Dependencies in the original C source:
// #include <stdbool.h>
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

/* volatile to force a read, compiler may assume 0 otherwise */
#[no_mangle]
pub static rovar1: core::ffi::c_int = 0;
#[no_mangle]
pub static mut out1: core::ffi::c_int = 0;

/* Override weak symbol in test_subskeleton_lib */
#[no_mangle]
pub static mut var5: core::ffi::c_int = 5;

unsafe extern "C" {
    #[link_name = "CONFIG_BPF_SYSCALL"]
    pub static CONFIG_BPF_SYSCALL: bool;

    pub fn lib_routine() -> core::ffi::c_int;
}

#[unsafe(link_section = "raw_tp/sys_enter")]
#[no_mangle]
pub unsafe extern "C" fn handler1(ctx: *const core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    let _ = core::ptr::read_volatile(&CONFIG_BPF_SYSCALL as *const bool);

    out1 = lib_routine() * core::ptr::read_volatile(&rovar1 as *const core::ffi::c_int);
    0
}

#[unsafe(link_section = "license")]
#[no_mangle]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
