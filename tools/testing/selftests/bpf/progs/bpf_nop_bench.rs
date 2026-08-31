// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bench_bpf_timing.bpf.h"

use core::ffi::c_void;

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn bench_nop(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    BENCH_BPF_LOOP!(0, {})
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
