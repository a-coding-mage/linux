// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

#![no_std]

use core::ffi::c_void;

/* Dependencies originally supplied by:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 * #include "uptr_test_common.h"
 */

extern "C" {
    static BPF_MAP_TYPE_TASK_STORAGE: u32;
    static BPF_F_NO_PREALLOC: u32;
    static MAGIC_VALUE: i32;
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct user_data {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}

#[repr(C)]
pub struct value_lock_type {
    pub lock: bpf_spin_lock,
    pub udata: *mut user_data,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut bpf_map_def,
        task: *mut task_struct,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut datamap: bpf_map_def = bpf_map_def {
    type_: unsafe { BPF_MAP_TYPE_TASK_STORAGE },
    map_flags: unsafe { BPF_F_NO_PREALLOC },
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<value_lock_type>() as u32,
};

/* load test only. not used */
#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn not_used(ctx: *mut c_void) -> i32 {
    let mut ptr: *mut value_lock_type;
    let mut task: *mut task_struct;
    let mut udata: *mut user_data;

    let _ = ctx;

    task = bpf_get_current_task_btf();
    ptr = bpf_task_storage_get(&mut datamap, task, core::ptr::null_mut(), 0)
        as *mut value_lock_type;
    if ptr.is_null() {
        return 0;
    }

    bpf_spin_lock(&mut (*ptr).lock);

    udata = (*ptr).udata;
    if udata.is_null() {
        bpf_spin_unlock(&mut (*ptr).lock);
        return 0;
    }
    (*udata).result = MAGIC_VALUE + (*udata).a + (*udata).b;

    bpf_spin_unlock(&mut (*ptr).lock);

    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
