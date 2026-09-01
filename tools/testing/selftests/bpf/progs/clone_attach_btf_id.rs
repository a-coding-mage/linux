// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta */
// C includes translated as external dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_upper_case_globals)]

use core::ffi::c_int;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "fentry/bpf_fentry_test1"]
#[no_mangle]
pub extern "C" fn fentry_handler(a: c_int) -> c_int {
    let _ = a;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
