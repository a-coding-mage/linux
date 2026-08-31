// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Meta Platforms Inc. */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

extern "C" {
    fn __sink(a: __u64);
}

pub type __u64 = u64;

extern "C" {
    static XDP_DROP: __u64;
}

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[inline(never)]
unsafe fn f0() {
    let a: __u64 = 1;

    __sink(a);
}

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn global_func(xdp: *mut xdp_md) -> __u64 {
    let _ = xdp;
    f0();
    XDP_DROP
}
