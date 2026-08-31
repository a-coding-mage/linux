// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct MapDef {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

// __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
// __uint(map_flags, BPF_F_NO_PREALLOC);
// __type(key, int);
// __type(value, long);
#[no_mangle]
#[link_section = ".maps"]
pub static mut map_a: MapDef = MapDef {
    type_: BPF_MAP_TYPE_CGRP_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

// __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
// __uint(map_flags, BPF_F_NO_PREALLOC);
// __type(key, int);
// __type(value, long);
#[no_mangle]
#[link_section = ".maps"]
pub static mut map_b: MapDef = MapDef {
    type_: BPF_MAP_TYPE_CGRP_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

#[no_mangle]
pub static mut target_hid: i32 = 0;

#[no_mangle]
pub static mut is_cgroup1: bool = false;

extern "C" {
    pub static BPF_MAP_TYPE_CGRP_STORAGE: u32;
    pub static BPF_F_NO_PREALLOC: u32;
    pub static BPF_LOCAL_STORAGE_GET_F_CREATE: u64;

    pub fn bpf_task_get_cgroup1(task: *mut task_struct, hierarchy_id: i32) -> *mut cgroup;
    pub fn bpf_cgroup_release(cgrp: *mut cgroup);
    pub fn bpf_get_current_task_btf() -> *mut task_struct;
    pub fn bpf_cgrp_storage_get(
        map: *mut MapDef,
        cgrp: *mut cgroup,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut i64;
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct css_set {
    pub dfl_cgrp: *mut cgroup,
}

#[repr(C)]
pub struct task_struct {
    pub cgroups: *mut css_set,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe fn __on_update(cgrp: *mut cgroup) {
    let mut ptr: *mut i64;

    ptr = bpf_cgrp_storage_get(
        core::ptr::addr_of_mut!(map_a),
        cgrp,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        *ptr += 1;
    }

    ptr = bpf_cgrp_storage_get(
        core::ptr::addr_of_mut!(map_b),
        cgrp,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        *ptr += 1;
    }
}

#[no_mangle]
#[link_section = "fentry/bpf_local_storage_update"]
pub unsafe extern "C" fn on_update() -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();
    let cgrp: *mut cgroup;

    if is_cgroup1 {
        cgrp = bpf_task_get_cgroup1(task, target_hid);
        if cgrp.is_null() {
            return 0;
        }

        __on_update(cgrp);
        bpf_cgroup_release(cgrp);
        return 0;
    }

    __on_update((*(*task).cgroups).dfl_cgrp);
    0
}

unsafe fn __on_enter(_regs: *mut pt_regs, _id: i64, cgrp: *mut cgroup) {
    let mut ptr: *mut i64;

    ptr = bpf_cgrp_storage_get(
        core::ptr::addr_of_mut!(map_a),
        cgrp,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        *ptr = 200;
    }

    ptr = bpf_cgrp_storage_get(
        core::ptr::addr_of_mut!(map_b),
        cgrp,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        *ptr = 100;
    }
}

#[no_mangle]
#[link_section = "tp_btf/sys_enter"]
pub unsafe extern "C" fn on_enter(regs: *mut pt_regs, id: i64) -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();
    let cgrp: *mut cgroup;

    if is_cgroup1 {
        cgrp = bpf_task_get_cgroup1(task, target_hid);
        if cgrp.is_null() {
            return 0;
        }

        __on_enter(regs, id, cgrp);
        bpf_cgroup_release(cgrp);
        return 0;
    }

    __on_enter(regs, id, (*(*task).cgroups).dfl_cgrp);
    0
}
