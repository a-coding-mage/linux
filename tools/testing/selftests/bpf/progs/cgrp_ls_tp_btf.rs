// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type pid_t = i32;

pub const BPF_MAP_TYPE_CGRP_STORAGE: u32 = 19;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;
pub const MAGIC_VALUE: i64 = 0xabcd1234;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
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
    pub pid: pid_t,
    pub cgroups: *mut css_set,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[used]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[used]
#[unsafe(link_section = ".maps")]
pub static mut map_a: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_CGRP_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

#[used]
#[unsafe(link_section = ".maps")]
pub static mut map_b: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_CGRP_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

pub static mut target_pid: pid_t = 0;
pub static mut mismatch_cnt: i32 = 0;
pub static mut enter_cnt: i32 = 0;
pub static mut exit_cnt: i32 = 0;
pub static mut target_hid: i32 = 0;
pub static mut is_cgroup1: bool = false;

unsafe extern "C" {
    pub fn bpf_task_get_cgroup1(task: *mut task_struct, hierarchy_id: i32) -> *mut cgroup;
    pub fn bpf_cgroup_release(cgrp: *mut cgroup);

    pub fn bpf_get_current_task_btf() -> *mut task_struct;
    pub fn bpf_cgrp_storage_get(
        map: *mut bpf_map_def,
        cgrp: *mut cgroup,
        value: u64,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_cgrp_storage_delete(map: *mut bpf_map_def, cgrp: *mut cgroup) -> i32;
}

unsafe fn __sync_fetch_and_add_i32(ptr: *mut i32, val: i32) -> i32 {
    unsafe {
        let old = core::ptr::read_volatile(ptr);
        core::ptr::write_volatile(ptr, old.wrapping_add(val));
        old
    }
}

unsafe fn __on_enter(regs: *mut pt_regs, id: i64, cgrp: *mut cgroup) {
    let mut ptr: *mut i64;
    let err: i32;

    let _ = regs;
    let _ = id;

    /* populate value 0 */
    ptr = unsafe {
        bpf_cgrp_storage_get(
            &raw mut map_a,
            cgrp,
            0,
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        ) as *mut i64
    };
    if ptr.is_null() {
        return;
    }

    /* delete value 0 */
    err = unsafe { bpf_cgrp_storage_delete(&raw mut map_a, cgrp) };
    if err != 0 {
        return;
    }

    /* value is not available */
    ptr = unsafe { bpf_cgrp_storage_get(&raw mut map_a, cgrp, 0, 0) as *mut i64 };
    if !ptr.is_null() {
        return;
    }

    /* re-populate the value */
    ptr = unsafe {
        bpf_cgrp_storage_get(
            &raw mut map_a,
            cgrp,
            0,
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        ) as *mut i64
    };
    if ptr.is_null() {
        return;
    }
    unsafe {
        __sync_fetch_and_add_i32(&raw mut enter_cnt, 1);
        *ptr = MAGIC_VALUE + enter_cnt as i64;
    }
}

#[unsafe(link_section = "tp_btf/sys_enter")]
#[unsafe(export_name = "on_enter")]
pub unsafe extern "C" fn on_enter(regs: *mut pt_regs, id: i64) -> i32 {
    let task: *mut task_struct;
    let cgrp: *mut cgroup;

    task = unsafe { bpf_get_current_task_btf() };
    if unsafe { (*task).pid != target_pid } {
        return 0;
    }

    if unsafe { is_cgroup1 } {
        cgrp = unsafe { bpf_task_get_cgroup1(task, target_hid) };
        if cgrp.is_null() {
            return 0;
        }

        unsafe { __on_enter(regs, id, cgrp) };
        unsafe { bpf_cgroup_release(cgrp) };
        return 0;
    }

    unsafe { __on_enter(regs, id, (*(*task).cgroups).dfl_cgrp) };
    0
}

unsafe fn __on_exit(regs: *mut pt_regs, id: i64, cgrp: *mut cgroup) {
    let ptr: *mut i64;

    let _ = regs;
    let _ = id;

    ptr = unsafe {
        bpf_cgrp_storage_get(
            &raw mut map_a,
            cgrp,
            0,
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        ) as *mut i64
    };
    if ptr.is_null() {
        return;
    }

    unsafe {
        __sync_fetch_and_add_i32(&raw mut exit_cnt, 1);
        if *ptr != MAGIC_VALUE + exit_cnt as i64 {
            __sync_fetch_and_add_i32(&raw mut mismatch_cnt, 1);
        }
    }
}

#[unsafe(link_section = "tp_btf/sys_exit")]
#[unsafe(export_name = "on_exit")]
pub unsafe extern "C" fn on_exit(regs: *mut pt_regs, id: i64) -> i32 {
    let task: *mut task_struct;
    let cgrp: *mut cgroup;

    task = unsafe { bpf_get_current_task_btf() };
    if unsafe { (*task).pid != target_pid } {
        return 0;
    }

    if unsafe { is_cgroup1 } {
        cgrp = unsafe { bpf_task_get_cgroup1(task, target_hid) };
        if cgrp.is_null() {
            return 0;
        }

        unsafe { __on_exit(regs, id, cgrp) };
        unsafe { bpf_cgroup_release(cgrp) };
        return 0;
    }

    unsafe { __on_exit(regs, id, (*(*task).cgroups).dfl_cgrp) };
    0
}
