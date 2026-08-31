// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// Original C dependencies:
// #include <linux/bpf.h>
// #include <stdint.h>
// #include <linux/types.h>
// #include <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut count: u64 = 0;

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_enable_stats(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    core::intrinsics::atomic_xadd_seqcst(&mut count, 1);
    0
}
