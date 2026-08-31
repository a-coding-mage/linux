// SPDX-License-Identifier: GPL-2.0

// #define KBUILD_MODNAME "xdp_dummy"
// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int};

extern "C" {
    type xdp_md;
}

extern "C" {
    static XDP_PASS: c_int;
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdp_dummy_prog(ctx: *mut xdp_md) -> c_int {
    let _ = ctx;
    XDP_PASS
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn __x64_sys_nop(ctx: *mut xdp_md) -> c_int {
    let _ = ctx;
    XDP_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];
