// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Translated from C includes:
// <vmlinux.h>
// <bpf/bpf_tracing.h>
// <bpf/bpf_helpers.h>
// <bpf/bpf_core_read.h>
// "bpf_misc.h"

extern "C" {
    fn bpf_stream_vprintk(stream: u64, fmt: *const core::ffi::c_char, args: *mut core::ffi::c_void, data_len: u64) -> i64;
}

extern "C" {
    static BPF_STDOUT: u64;
}

#[link_section = "syscall"]
// __failure __msg("Possibly NULL pointer passed")
pub unsafe extern "C" fn stream_vprintk_null_arg(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    bpf_stream_vprintk(BPF_STDOUT, b"\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut(), 0);
    return 0;
}

#[link_section = "syscall"]
// __failure __msg("R3 type=scalar expected=")
pub unsafe extern "C" fn stream_vprintk_scalar_arg(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    bpf_stream_vprintk(BPF_STDOUT, b"\0".as_ptr() as *const core::ffi::c_char, 46 as *mut core::ffi::c_void, 0);
    return 0;
}

#[link_section = "syscall"]
// __failure __msg("R2 doesn't point to a const string")
pub unsafe extern "C" fn stream_vprintk_string_arg(ctx: *mut core::ffi::c_void) -> i32 {
    bpf_stream_vprintk(BPF_STDOUT, ctx as *const core::ffi::c_char, core::ptr::null_mut(), 0);
    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];
