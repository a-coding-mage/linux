// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, "../test_kmods/bpf_testmod.h"

extern "C" {
    type task_struct;
}

type pid_t = i32;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut tgid: pid_t = 0;

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_maybe_null: *mut core::ffi::c_void,
}

#[allow(non_snake_case)]
#[no_mangle]
#[link_section = "struct_ops/test_maybe_null_struct_ptr"]
pub unsafe extern "C" fn test_maybe_null_struct_ptr(
    _dummy: i32,
    task: *mut task_struct,
) -> i32 {
    tgid = (*(task as *mut task_struct_with_tgid)).tgid;

    0
}

#[repr(C)]
struct task_struct_with_tgid {
    _opaque: [u8; 0],
    tgid: pid_t,
}

#[no_mangle]
#[link_section = ".struct_ops.link"]
pub static mut testmod_struct_ptr: bpf_testmod_ops = bpf_testmod_ops {
    test_maybe_null: test_maybe_null_struct_ptr as *mut core::ffi::c_void,
};
