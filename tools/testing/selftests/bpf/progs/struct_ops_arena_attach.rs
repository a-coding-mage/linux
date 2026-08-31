// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[no_mangle]
#[link_section = "fentry"]
pub extern "C" fn fentry_test_arena(st_ops_ctx: *mut u64) -> i32 {
    let _ = st_ops_ctx;
    0
}

#[no_mangle]
#[link_section = "fexit"]
pub extern "C" fn fexit_test_arena(st_ops_ctx: *mut u64, ret: i32) -> i32 {
    let _ = st_ops_ctx;
    let _ = ret;
    0
}

#[no_mangle]
#[link_section = "freplace"]
pub extern "C" fn freplace_test_arena(st_ops_ctx: *mut u64) -> i32 {
    let _ = st_ops_ctx;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
