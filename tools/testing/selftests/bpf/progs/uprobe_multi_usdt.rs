// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/usdt.bpf.h>

use crate::pt_regs;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut count: ::core::ffi::c_int = 0;

#[no_mangle]
#[link_section = "usdt"]
pub unsafe extern "C" fn usdt0(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let _ = ctx;
    count += 1;
    0
}
