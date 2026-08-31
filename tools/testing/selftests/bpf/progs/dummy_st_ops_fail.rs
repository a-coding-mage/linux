// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;

#[repr(C)]
pub struct bpf_dummy_ops_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dummy_ops {
    pub test_1: *mut c_void,
    pub test_2: *mut c_void,
    pub test_sleepable: *mut c_void,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "struct_ops.s/test_2"]
// __failure
// __msg("attach to unsupported member test_2 of struct bpf_dummy_ops")
pub unsafe extern "C" fn test_unsupported_field_sleepable(
    state: *mut bpf_dummy_ops_state,
    a1: i32,
    a2: u16,
    a3: i8,
    a4: u64,
) -> i32 {
    let _ = state;
    let _ = a1;
    let _ = a2;
    let _ = a3;
    let _ = a4;

    /* Tries to mark an unsleepable field in struct bpf_dummy_ops as sleepable. */
    0
}

#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut dummy_1: bpf_dummy_ops = bpf_dummy_ops {
    test_1: core::ptr::null_mut(),
    test_2: test_unsupported_field_sleepable as *mut c_void,
    test_sleepable: core::ptr::null_mut(),
};
