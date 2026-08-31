// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// Dependencies from vmlinux.h, bpf/bpf_tracing.h, and
// ../test_kmods/bpf_testmod.h are expected to be supplied by the build.

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut rand: i32 = 0;

#[unsafe(no_mangle)]
pub static mut arr: [i32; 1] = [0; 1];

#[unsafe(link_section = "struct_ops/test_1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_1_turn_off() -> i32 {
    unsafe {
        return arr[rand as usize]; /* potentially way out of range access */
    }
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_1: *mut core::ffi::c_void,
}

unsafe impl Sync for bpf_testmod_ops {}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static ops: bpf_testmod_ops = bpf_testmod_ops {
    test_1: test_1_turn_off as *mut core::ffi::c_void,
};
