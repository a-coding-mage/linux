// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "uptr_test_common.h"

extern "C" {
    #[link_name = "bpf_task_from_pid"]
    fn bpf_task_from_pid(pid: s32) -> *mut task_struct;
    #[link_name = "bpf_task_release"]
    fn bpf_task_release(p: *mut task_struct);
    #[link_name = "bpf_cgroup_release"]
    fn bpf_cgroup_release(cgrp: *mut cgroup);

    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut core::ffi::c_void,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut value_type;
    fn bpf_kptr_xchg(kptr: *mut *mut cgroup, ptr: *mut cgroup) -> *mut cgroup;
}

// BPF map definition translated from:
// struct {
//     __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, struct value_type);
// } datamap SEC(".maps");
#[repr(C)]
pub struct datamap_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key: i32,
    pub value: value_type,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut datamap: datamap_def = datamap_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: 0,
    value: value_type {
        cgrp: core::ptr::null_mut(),
        udata: core::ptr::null_mut(),
        nested: nested_value_type {
            udata: core::ptr::null_mut(),
        },
    },
};

#[no_mangle]
pub static mut target_pid: pid_t = 0;
#[no_mangle]
pub static mut parent_pid: pid_t = 0;

#[link_section = "tp_btf/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn on_enter(ctx: *mut u64) -> i32 {
    let mut task: *mut task_struct;
    let mut data_task: *mut task_struct;
    let mut ptr: *mut value_type;
    let mut udata: *mut user_data;
    let mut cgrp: *mut cgroup;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    data_task = bpf_task_from_pid(parent_pid);
    if data_task.is_null() {
        return 0;
    }

    ptr = bpf_task_storage_get(
        &mut datamap as *mut datamap_def as *mut core::ffi::c_void,
        data_task,
        core::ptr::null_mut(),
        0,
    );
    bpf_task_release(data_task);
    if ptr.is_null() {
        return 0;
    }

    cgrp = bpf_kptr_xchg(&mut (*ptr).cgrp, core::ptr::null_mut());
    if !cgrp.is_null() {
        let lvl: i32 = (*cgrp).level;

        bpf_cgroup_release(cgrp);
        return lvl;
    }

    udata = (*ptr).udata;
    if udata.is_null() || (*udata).result != 0 {
        return 0;
    }
    (*udata).result = MAGIC_VALUE + (*udata).a + (*udata).b;

    udata = (*ptr).nested.udata;
    if !udata.is_null() && (*udata).nested_result == 0 {
        (*udata).nested_result = (*udata).result;
    }

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
