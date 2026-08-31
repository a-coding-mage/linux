// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Dependencies from the original C includes:
// "vmlinux.h", <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

type __u64 = u64;

const BPF_MAP_TYPE_TASK_STORAGE: u32 = 27;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_storage_map {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut task_storage: task_storage_map = task_storage_map {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[unsafe(no_mangle)]
pub static mut run_count: i32 = 0;
#[unsafe(no_mangle)]
pub static mut valid_ptr_count: i32 = 0;
#[unsafe(no_mangle)]
pub static mut null_ptr_count: i32 = 0;

unsafe extern "C" {
    fn bpf_task_storage_get(
        map: *mut task_storage_map,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut __u64;
}

#[unsafe(link_section = "fentry/exit_creds")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trace_exit_creds(task: *mut task_struct) -> i32 {
    let ptr: *mut __u64;

    ptr = unsafe {
        bpf_task_storage_get(
            &raw mut task_storage,
            task,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if !ptr.is_null() {
        unsafe {
            core::intrinsics::atomic_xadd_relaxed(&raw mut valid_ptr_count, 1);
        }
    } else {
        unsafe {
            core::intrinsics::atomic_xadd_relaxed(&raw mut null_ptr_count, 1);
        }
    }

    unsafe {
        core::intrinsics::atomic_xadd_relaxed(&raw mut run_count, 1);
    }
    0
}
