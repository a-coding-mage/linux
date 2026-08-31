// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
// C dependencies: <vmlinux.h>, <bpf/bpf_tracing.h>, "../test_kmods/bpf_testmod.h"

type pid_t = i32;

#[repr(C)]
pub struct task_struct {
    pub tgid: pid_t,
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_maybe_null: *mut core::ffi::c_void,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut tgid: pid_t = 0;

/* This is a test BPF program that uses struct_ops to access an argument
 * that may be NULL. This is a test for the verifier to ensure that it can
 * rip PTR_MAYBE_NULL correctly.
 */
#[unsafe(link_section = "struct_ops/test_maybe_null")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_maybe_null(
    dummy: ::core::ffi::c_int,
    task: *mut task_struct,
) -> ::core::ffi::c_int {
    if !task.is_null() {
        unsafe {
            tgid = (*task).tgid;
        }
    }

    0
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_1: bpf_testmod_ops = bpf_testmod_ops {
    test_maybe_null: test_maybe_null as *mut core::ffi::c_void,
};
