// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

extern "C" {
    pub static XDP_DROP: i32;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

#[no_mangle]
#[link_section = "freplace/xdp_prog1"]
pub unsafe extern "C" fn new_xdp_prog2(xd: *mut xdp_md) -> i32 {
    unsafe { XDP_DROP }
}
