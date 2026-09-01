// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook

// C dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
// bpf/bpf_core_read.h, and bpf_misc.h.

#[repr(C)]
pub struct whatever {}

#[unsafe(link_section = "kprobe")]
// __success __log_level(2)
// context type is wrong, making it impossible to freplace this program
pub unsafe extern "C" fn btf_unreliable_kprobe(ctx: *mut whatever) -> ::core::ffi::c_int {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "license")]
#[used]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
