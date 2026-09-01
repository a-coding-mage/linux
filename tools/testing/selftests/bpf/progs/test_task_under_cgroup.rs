// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Bytedance */

/*
 * C dependencies removed from executable Rust:
 * <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
 * The declarations below preserve the symbols this file uses from them.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_long, c_uint};

type u64 = u64;
type __u64 = u64;

const BPF_LINK_CREATE: c_int = 28;

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub pid: c_int,
    pub tgid: c_int,
}

#[repr(C)]
pub union bpf_attr {
    _bindgen_union_align: u64,
}

extern "C" {
    #[link_name = "bpf_cgroup_from_id"]
    fn bpf_cgroup_from_id(cgid: u64) -> *mut cgroup;
    #[link_name = "bpf_task_under_cgroup"]
    fn bpf_task_under_cgroup(task: *mut task_struct, ancestor: *mut cgroup) -> c_long;
    #[link_name = "bpf_cgroup_release"]
    fn bpf_cgroup_release(p: *mut cgroup);
    #[link_name = "bpf_task_acquire"]
    fn bpf_task_acquire(p: *mut task_struct) -> *mut task_struct;
    #[link_name = "bpf_task_release"]
    fn bpf_task_release(p: *mut task_struct);

    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
}

#[no_mangle]
pub static local_pid: c_int = 0;
#[no_mangle]
pub static cgid: __u64 = 0;
#[no_mangle]
pub static mut remote_pid: c_int = 0;

#[no_mangle]
#[link_section = "tp_btf/task_newtask"]
pub unsafe extern "C" fn tp_btf_run(task: *mut task_struct, clone_flags: u64) -> c_int {
    let mut cgrp: *mut cgroup = core::ptr::null_mut();
    let acquired: *mut task_struct;

    let _ = clone_flags;

    if core::ptr::read_volatile(&local_pid) != (bpf_get_current_pid_tgid() >> 32) as c_int {
        return 0;
    }

    acquired = bpf_task_acquire(task);
    if acquired.is_null() {
        return 0;
    }

    if core::ptr::read_volatile(&local_pid) == (*acquired).tgid {
        goto_out(cgrp, acquired);
        return 0;
    }

    cgrp = bpf_cgroup_from_id(core::ptr::read_volatile(&cgid));
    if cgrp.is_null() {
        goto_out(cgrp, acquired);
        return 0;
    }

    if bpf_task_under_cgroup(acquired, cgrp) != 0 {
        remote_pid = (*acquired).tgid;
    }

    goto_out(cgrp, acquired);

    return 0;
}

#[inline(always)]
unsafe fn goto_out(cgrp: *mut cgroup, acquired: *mut task_struct) {
    if !cgrp.is_null() {
        bpf_cgroup_release(cgrp);
    }
    bpf_task_release(acquired);
}

#[no_mangle]
#[link_section = "lsm.s/bpf"]
pub unsafe extern "C" fn lsm_run(
    cmd: c_int,
    attr: *mut bpf_attr,
    size: c_uint,
    kernel: bool,
) -> c_int {
    let mut cgrp: *mut cgroup = core::ptr::null_mut();
    let task: *mut task_struct;
    let mut ret: c_int = 0;

    let _ = attr;
    let _ = size;
    let _ = kernel;

    task = bpf_get_current_task_btf();
    if core::ptr::read_volatile(&local_pid) != (*task).pid {
        return 0;
    }

    if cmd != BPF_LINK_CREATE {
        return 0;
    }

    /* 1 is the root cgroup */
    cgrp = bpf_cgroup_from_id(1);
    if cgrp.is_null() {
        return ret;
    }
    if bpf_task_under_cgroup(task, cgrp) == 0 {
        ret = -1;
    }
    bpf_cgroup_release(cgrp);

    return ret;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
