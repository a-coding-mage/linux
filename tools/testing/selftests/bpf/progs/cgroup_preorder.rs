// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct bpf_sockopt {
    _private: [u8; 0],
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut idx: u32 = 0;

#[no_mangle]
pub static mut result: [u8; 4] = [0; 4];

// SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn child(ctx: *mut bpf_sockopt) -> i32 {
    let _ = ctx;

    if idx < 4 {
        let old_idx = idx;
        idx = idx.wrapping_add(1);
        result[old_idx as usize] = 1;
    }
    return 1;
}

// SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn child_2(ctx: *mut bpf_sockopt) -> i32 {
    let _ = ctx;

    if idx < 4 {
        let old_idx = idx;
        idx = idx.wrapping_add(1);
        result[old_idx as usize] = 2;
    }
    return 1;
}

// SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn parent(ctx: *mut bpf_sockopt) -> i32 {
    let _ = ctx;

    if idx < 4 {
        let old_idx = idx;
        idx = idx.wrapping_add(1);
        result[old_idx as usize] = 3;
    }
    return 1;
}

// SEC("cgroup/getsockopt")
#[no_mangle]
pub unsafe extern "C" fn parent_2(ctx: *mut bpf_sockopt) -> i32 {
    let _ = ctx;

    if idx < 4 {
        let old_idx = idx;
        idx = idx.wrapping_add(1);
        result[old_idx as usize] = 4;
    }
    return 1;
}
