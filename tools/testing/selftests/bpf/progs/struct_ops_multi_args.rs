// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Varun R Mallya */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "../test_kmods/bpf_testmod.h"
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_refcounted_multi: *mut core::ffi::c_void,
}

unsafe extern "C" {
    fn bpf_task_release(task: *mut task_struct);
    fn bpf_tail_call(
        ctx: *mut core::ffi::c_void,
        prog_array_map: *const core::ffi::c_void,
        index: u32,
    );
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct prog_array_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut prog_array: prog_array_map_def = prog_array_map_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u32>() as u32,
};

// SEC("struct_ops/test_refcounted_multi")
// __failure __msg("program with __ref argument cannot tail call")
#[unsafe(link_section = "struct_ops/test_refcounted_multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_refcounted_multi(ctx: *mut u64) -> i32 {
    /* ctx[2] is used because the refcounted variable is the third argument */
    let refcounted_task: *mut task_struct = unsafe { *ctx.add(2) as *mut task_struct };

    unsafe {
        bpf_task_release(refcounted_task);
        bpf_tail_call(
            ctx as *mut core::ffi::c_void,
            &raw const prog_array as *const core::ffi::c_void,
            0,
        );
    }

    0
}

// SEC(".struct_ops.link")
#[unsafe(link_section = ".struct_ops.link")]
#[unsafe(no_mangle)]
pub static mut testmod_ref_acquire: bpf_testmod_ops = bpf_testmod_ops {
    test_refcounted_multi: test_refcounted_multi as *mut core::ffi::c_void,
};
