// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>

#[no_mangle]
pub static mut t_b: i64 = 0;
#[no_mangle]
pub static mut t_c: i64 = 0;
#[no_mangle]
pub static mut t_ret: i64 = 0;

#[no_mangle]
#[link_section = "fexit/bpf_testmod_test_int128_arg"]
pub unsafe extern "C" fn test_int128_arg_fexit(ctx: *mut u64) -> i32 {
    t_b = *ctx.add(2) as i32 as i64;
    t_c = *ctx.add(3) as i64;
    t_ret = *ctx.add(4) as i64;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
