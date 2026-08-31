// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook

// Dependencies from the original C source:
// - vmlinux.h
// - <bpf/bpf_helpers.h>
// - <bpf/bpf_tracing.h>
// - <bpf/bpf_core_read.h>
// - bpf_misc.h
//
// SYS_PREFIX is supplied externally by the BPF test infrastructure.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut kprobe_res: ::core::ffi::c_int = 0;

/**
 * This program will be manually made sleepable on the userspace side
 * and should thus be unattachable.
 */
// Original section: SEC("kprobe/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn handle_kprobe_sleepable(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let _ = ctx;
    kprobe_res = 1;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];
