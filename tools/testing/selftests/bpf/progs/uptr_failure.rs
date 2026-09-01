// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_experimental.h",
// "bpf_misc.h", and "uptr_test_common.h".

use core::ffi::c_void;
use core::ptr;

const BPF_MAP_TYPE_TASK_STORAGE: u32 = 27;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct udata_type {
    pub result: i32,
}

#[repr(C)]
pub struct nested_value_type {
    pub udata: *mut udata_type,
}

#[repr(C)]
pub struct value_type {
    pub udata: *mut udata_type,
    pub nested: nested_value_type,
}

#[repr(C)]
pub struct datamap_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: i32,
    pub value: value_type,
}

// Original C: anonymous BPF map definition in SEC(".maps").
#[no_mangle]
#[link_section = ".maps"]
pub static mut datamap: datamap_def = datamap_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: 0,
    value: value_type {
        udata: ptr::null_mut(),
        nested: nested_value_type {
            udata: ptr::null_mut(),
        },
    },
};

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut datamap_def,
        task: *mut task_struct,
        value: *mut c_void,
        flags: u64,
    ) -> *mut value_type;
    fn bpf_kptr_xchg(ptr: *mut *mut udata_type, value: *mut udata_type) -> *mut udata_type;
    fn bpf_obj_new_value_type() -> *mut value_type;
    fn bpf_obj_drop_value_type(ptr: *mut value_type);
}

// SEC("?syscall")
// __failure __msg("store to uptr disallowed")
#[no_mangle]
#[link_section = "?syscall"]
pub unsafe extern "C" fn uptr_write(ctx: *const c_void) -> i32 {
    let task: *mut task_struct;
    let v: *mut value_type;

    let _ = ctx;

    task = unsafe { bpf_get_current_task_btf() };
    v = unsafe {
        bpf_task_storage_get(
            &raw mut datamap,
            task,
            ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        (*v).udata = ptr::null_mut();
    }
    0
}

// SEC("?syscall")
// __failure __msg("store to uptr disallowed")
#[no_mangle]
#[link_section = "?syscall"]
pub unsafe extern "C" fn uptr_write_nested(ctx: *const c_void) -> i32 {
    let task: *mut task_struct;
    let v: *mut value_type;

    let _ = ctx;

    task = unsafe { bpf_get_current_task_btf() };
    v = unsafe {
        bpf_task_storage_get(
            &raw mut datamap,
            task,
            ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        (*v).nested.udata = ptr::null_mut();
    }
    0
}

// SEC("?syscall")
// __failure __msg("R1 invalid mem access 'mem_or_null'")
#[no_mangle]
#[link_section = "?syscall"]
pub unsafe extern "C" fn uptr_no_null_check(ctx: *const c_void) -> i32 {
    let task: *mut task_struct;
    let v: *mut value_type;

    let _ = ctx;

    task = unsafe { bpf_get_current_task_btf() };
    v = unsafe {
        bpf_task_storage_get(
            &raw mut datamap,
            task,
            ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        (*(*v).udata).result = 0;
    }

    0
}

// SEC("?syscall")
// __failure __msg("doesn't point to kptr")
#[no_mangle]
#[link_section = "?syscall"]
pub unsafe extern "C" fn uptr_kptr_xchg(ctx: *const c_void) -> i32 {
    let task: *mut task_struct;
    let v: *mut value_type;

    let _ = ctx;

    task = unsafe { bpf_get_current_task_btf() };
    v = unsafe {
        bpf_task_storage_get(
            &raw mut datamap,
            task,
            ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        )
    };
    if v.is_null() {
        return 0;
    }

    unsafe {
        bpf_kptr_xchg(&raw mut (*v).udata, ptr::null_mut());
    }

    0
}

// SEC("?syscall")
// __failure __msg("invalid mem access 'scalar'")
#[no_mangle]
#[link_section = "?syscall"]
pub unsafe extern "C" fn uptr_obj_new(ctx: *const c_void) -> i32 {
    let v: *mut value_type;

    let _ = ctx;

    // Original C: bpf_obj_new(typeof(*v)).
    v = unsafe { bpf_obj_new_value_type() };
    if v.is_null() {
        return 0;
    }

    unsafe {
        if !(*v).udata.is_null() {
            (*(*v).udata).result = 0;
        }
    }

    unsafe {
        bpf_obj_drop_value_type(v);
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
