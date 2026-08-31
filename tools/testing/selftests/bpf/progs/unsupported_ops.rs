// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "../test_kmods/bpf_testmod.h"

use core::ffi::c_void;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SEC("struct_ops/unsupported_ops")
// __failure
// __msg("attach to unsupported member unsupported_ops of struct bpf_testmod_ops")
#[unsafe(link_section = "struct_ops/unsupported_ops")]
#[unsafe(no_mangle)]
pub extern "C" fn unsupported_ops() -> i32 {
    0
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub unsupported_ops: *mut c_void,
}

// SEC(".struct_ops.link")
#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod: bpf_testmod_ops = bpf_testmod_ops {
    unsupported_ops: unsupported_ops as *mut c_void,
};
