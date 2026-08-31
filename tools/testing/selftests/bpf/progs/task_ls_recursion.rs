// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

const EBUSY: i32 = 16;
const BPF_MAP_TYPE_TASK_STORAGE: u32 = 0;
const BPF_F_NO_PREALLOC: u32 = 0;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 0;

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut nr_del_errs: i32 = 0;
#[no_mangle]
pub static mut test_pid: i32 = 0;

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_a: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut map_b: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut bpf_map_def,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut i64;
    fn bpf_task_storage_delete(map: *mut bpf_map_def, task: *mut task_struct) -> i32;
}

#[link_section = "fentry/bpf_local_storage_update"]
#[no_mangle]
pub unsafe extern "C" fn on_update() -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();
    let mut ptr: *mut i64;

    if test_pid == 0 || (*task).pid != test_pid {
        return 0;
    }

    /* This will succeed as there is no real deadlock */
    ptr = bpf_task_storage_get(
        &mut map_a,
        task,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        let err: i32;

        *ptr += 1;
        err = bpf_task_storage_delete(&mut map_a, task);
        if err == -EBUSY {
            nr_del_errs += 1;
        }
    }

    /* This will succeed as there is no real deadlock */
    ptr = bpf_task_storage_get(
        &mut map_b,
        task,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        *ptr += 1;
    }

    return 0;
}

#[link_section = "tp_btf/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn on_enter(regs: *mut pt_regs, id: i64) -> i32 {
    let mut task: *mut task_struct;
    let mut ptr: *mut i64;

    task = bpf_get_current_task_btf();
    if test_pid == 0 || (*task).pid != test_pid {
        return 0;
    }

    ptr = bpf_task_storage_get(
        &mut map_a,
        task,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() && *ptr == 0 {
        *ptr = 200;
    }

    ptr = bpf_task_storage_get(
        &mut map_b,
        task,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() && *ptr == 0 {
        *ptr = 100;
    }
    return 0;
}
