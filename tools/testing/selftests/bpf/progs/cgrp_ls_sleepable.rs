// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Translated from C. Dependencies originally supplied by:
 * <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h", "err.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __s32 = i32;
type __u64 = u64;
type u64 = u64;

#[repr(C)]
pub struct kernfs_node {
    pub id: u64,
}

#[repr(C)]
pub struct cgroup {
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct css_set {
    pub dfl_cgrp: *mut cgroup,
}

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
    pub cgroups: *mut css_set,
}

#[repr(C)]
pub struct bpf_iter__cgroup {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct bpf_local_storage_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_local_storage_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

const BPF_MAP_TYPE_CGRP_STORAGE: u32 = 0; /* supplied by BPF headers in C */
const BPF_F_NO_PREALLOC: u32 = 0; /* supplied by BPF headers in C */
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 0; /* supplied by BPF headers in C */

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = ".maps"]
pub static mut map_a: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_CGRP_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

#[no_mangle]
pub static mut target_pid: __s32 = 0;
#[no_mangle]
pub static mut cgroup_id: __u64 = 0;
#[no_mangle]
pub static mut update_err: i64 = 0;
#[no_mangle]
pub static mut target_hid: i32 = 0;
#[no_mangle]
pub static mut is_cgroup1: bool = false;

extern "C" {
    fn bpf_task_get_cgroup1(task: *mut task_struct, hierarchy_id: i32) -> *mut cgroup;
    fn bpf_cgroup_release(cgrp: *mut cgroup);
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();

    fn bpf_cgrp_storage_get(
        map: *mut bpf_map_def,
        cgrp: *mut cgroup,
        value: u64,
        flags: u64,
    ) -> *mut i64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn IS_ERR_VALUE(ptr: *mut bpf_local_storage_data) -> bool;
    fn PTR_ERR(ptr: *mut bpf_local_storage_data) -> i64;
}

#[no_mangle]
#[link_section = "?iter.s/cgroup"]
pub unsafe extern "C" fn cgroup_iter(ctx: *mut bpf_iter__cgroup) -> i32 {
    let cgrp: *mut cgroup = (*ctx).cgroup;
    let ptr: *mut i64;

    if cgrp.is_null() {
        return 0;
    }

    ptr = bpf_cgrp_storage_get(
        &raw mut map_a,
        cgrp,
        0,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        cgroup_id = (*(*cgrp).kn).id;
    }
    return 0;
}

unsafe fn __no_rcu_lock(cgrp: *mut cgroup) {
    let ptr: *mut i64;

    /* Note that trace rcu is held in sleepable prog, so we can use
     * bpf_cgrp_storage_get() in sleepable prog.
     */
    ptr = bpf_cgrp_storage_get(
        &raw mut map_a,
        cgrp,
        0,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !ptr.is_null() {
        cgroup_id = (*(*cgrp).kn).id;
    }
}

/* C section was: SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
#[link_section = "?fentry.s/sys_getpgid"]
pub unsafe extern "C" fn cgrp1_no_rcu_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let cgrp: *mut cgroup;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    /* bpf_task_get_cgroup1 can work in sleepable prog */
    cgrp = bpf_task_get_cgroup1(task, target_hid);
    if cgrp.is_null() {
        return 0;
    }

    __no_rcu_lock(cgrp);
    bpf_cgroup_release(cgrp);
    return 0;
}

/* C section was: SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
#[link_section = "?fentry.s/sys_getpgid"]
pub unsafe extern "C" fn no_rcu_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    /* task->cgroups is untrusted in sleepable prog outside of RCU CS */
    __no_rcu_lock((*(*task).cgroups).dfl_cgrp);
    return 0;
}

/* C section was: SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[no_mangle]
#[link_section = "?fentry.s/sys_getpgid"]
pub unsafe extern "C" fn yes_rcu_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let mut cgrp: *mut cgroup;
    let ptr: *mut i64;

    task = bpf_get_current_task_btf();
    if (*task).pid != target_pid {
        return 0;
    }

    if is_cgroup1 {
        bpf_rcu_read_lock();
        cgrp = bpf_task_get_cgroup1(task, target_hid);
        if cgrp.is_null() {
            bpf_rcu_read_unlock();
            return 0;
        }

        ptr = bpf_cgrp_storage_get(&raw mut map_a, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
        if !ptr.is_null() {
            cgroup_id = (*(*cgrp).kn).id;
        }
        bpf_cgroup_release(cgrp);
        bpf_rcu_read_unlock();
        return 0;
    }

    bpf_rcu_read_lock();
    cgrp = (*(*task).cgroups).dfl_cgrp;
    /* cgrp is trusted under RCU CS */
    ptr = bpf_cgrp_storage_get(&raw mut map_a, cgrp, 0, BPF_LOCAL_STORAGE_GET_F_CREATE);
    if !ptr.is_null() {
        cgroup_id = (*(*cgrp).kn).id;
    }
    bpf_rcu_read_unlock();
    return 0;
}

#[no_mangle]
#[link_section = "fexit/bpf_local_storage_update"]
pub unsafe extern "C" fn fexit_update(
    owner: *mut core::ffi::c_void,
    smap: *mut bpf_local_storage_map,
    value: *mut core::ffi::c_void,
    map_flags: u64,
    swap_uptrs: bool,
    ret: *mut bpf_local_storage_data,
) -> i32 {
    let task: *mut task_struct = bpf_get_current_task_btf();

    if (*task).pid != target_pid {
        return 0;
    }

    if IS_ERR_VALUE(ret) {
        update_err = PTR_ERR(ret);
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
