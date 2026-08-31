// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, "../test_kmods/bpf_testmod.h"

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = "struct_ops/test_1")]
#[unsafe(no_mangle)]
pub extern "C" fn test_1_forgotten() -> i32 {
    return 0;
}

#[repr(C)]
pub struct bpf_testmod_ops {
    _unused: [u8; 0],
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static ops: bpf_testmod_ops = bpf_testmod_ops {
    /* we forgot to reference test_1_forgotten above, oops */
};
