// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2025 Microsoft Corporation
 *
 * Author: Blaise Boscaccy <bboscaccy@linux.microsoft.com>
 */

// Dependencies from the original C file:
// #include "vmlinux.h"
// #include <errno.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

pub const EINVAL: i32 = 22;

#[repr(C)]
pub union bpf_attr {
    _bindgen_union_align: u64,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut monitored_tid: u32 = 0;

#[no_mangle]
#[link_section = "lsm.s/bpf"]
pub unsafe extern "C" fn bpf(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    let tid: u32;

    tid = (bpf_get_current_pid_tgid() & 0xFFFFFFFF) as u32;
    if !kernel || tid != monitored_tid {
        return 0;
    } else {
        return -EINVAL;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
