// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// Translated from C includes:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// C source used __weak.
#[unsafe(no_mangle)]
pub extern "C" fn token_ksym_subprog() -> ::core::ffi::c_int {
    0
}

#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xdp_main(xdp: *mut xdp_md) -> ::core::ffi::c_int {
    let _ = xdp;
    token_ksym_subprog()
}
