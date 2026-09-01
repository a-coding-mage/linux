// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * Original C dependencies:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 */

/*
 * Programs below exist only to exercise veristat's -f name filters,
 * their bodies are irrelevant, only the names matter.
 * This file is also included by veristat_bar.c, so that the same set of
 * program names is available in two differently named object files.
 */

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn foo(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bar(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn buz(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
