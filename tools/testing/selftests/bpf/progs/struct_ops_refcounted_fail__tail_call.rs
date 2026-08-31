// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

// Dependencies from vmlinux.h, bpf/bpf_tracing.h, ../test_kmods/bpf_testmod.h,
// and bpf_misc.h are expected to be supplied by the surrounding BPF build.
type __u32 = u32;

const BPF_MAP_TYPE_PROG_ARRAY: i32 = 3;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_refcounted: *mut c_void,
}

unsafe extern "C" {
    fn bpf_task_release(task: *mut task_struct);
    fn bpf_tail_call(ctx: *mut c_void, prog_array: *const c_void, index: __u32);
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct prog_array_map {
    pub type_: *mut [i32; BPF_MAP_TYPE_PROG_ARRAY as usize],
    pub max_entries: *mut [i32; 1],
    pub key_size: *mut [i32; core::mem::size_of::<__u32>()],
    pub value_size: *mut [i32; core::mem::size_of::<__u32>()],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut prog_array: prog_array_map = prog_array_map {
    type_: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
    key_size: core::ptr::null_mut(),
    value_size: core::ptr::null_mut(),
};

/* Test that the verifier rejects a program with referenced kptr arguments
 * that tail call
 */
#[unsafe(link_section = "struct_ops/test_refcounted")]
#[unsafe(no_mangle)]
// __failure
// __msg("program with __ref argument cannot tail call")
pub unsafe extern "C" fn refcounted_fail__tail_call(ctx: *mut u64) -> i32 {
    let task: *mut task_struct = *(ctx.add(1)) as *mut task_struct;

    bpf_task_release(task);
    bpf_tail_call(
        ctx as *mut c_void,
        core::ptr::addr_of!(prog_array) as *const c_void,
        0,
    );

    0
}

#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_ref_acquire: bpf_testmod_ops = bpf_testmod_ops {
    test_refcounted: refcounted_fail__tail_call as *mut c_void,
};
