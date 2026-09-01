// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C includes translated as external dependency intent:
// <stdbool.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>

#[no_mangle]
pub static mut in: i32 = 0;

#[no_mangle]
pub static mut out: i32 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn raw_tp_prog(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    out = in;
    return 0;
}

#[no_mangle]
#[link_section = "tp_btf/sys_enter"]
pub unsafe extern "C" fn tp_btf_prog(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;
    out = in;
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
