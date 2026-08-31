// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

// C dependencies:
// #include "vmlinux.h"
// #include <errno.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

/* From include/linux/mm.h. */
pub const FMODE_WRITE: u32 = 0x2;

#[repr(C)]
pub struct data_input {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, __u32);
    // __type(value, __u32);
}

#[link_section = ".maps"]
pub static mut data_input: data_input = data_input {};

#[link_section = "license"]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

#[link_section = "lsm/bpf_map"]
pub unsafe extern "C" fn check_access(map: *mut bpf_map, fmode: fmode_t) -> core::ffi::c_int {
    if map != (&raw mut data_input as *mut data_input as *mut bpf_map) {
        return 0;
    }

    if (fmode & FMODE_WRITE as fmode_t) != 0 {
        return -EACCES;
    }
    barrier();

    0
}
