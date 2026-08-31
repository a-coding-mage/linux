// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "../test_kmods/bpf_testmod.h"

unsafe extern "C" {
    pub type bpf_testmod_ops3;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* No arena in the program: attaching to test_arena must be rejected. */
#[unsafe(link_section = "struct_ops/test_arena")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_arena_no_arena(ctx: *mut u64) -> i32 {
    let _ = ctx;
    0
}

#[repr(C)]
pub struct bpf_testmod_ops3_init {
    pub test_arena: *mut core::ffi::c_void,
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_arena_fail: bpf_testmod_ops3_init = bpf_testmod_ops3_init {
    test_arena: test_arena_no_arena as *mut core::ffi::c_void,
};
