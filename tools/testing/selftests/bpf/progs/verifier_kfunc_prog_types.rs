// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * <vmlinux.h>
 * <bpf/bpf_tracing.h>
 * <bpf/bpf_helpers.h>
 * "bpf_misc.h"
 * "cgrp_kfunc_common.h"
 * "cpumask_common.h"
 * "task_kfunc_common.h"
 */

#[repr(C)]
pub struct task_struct {
    pub pid: i32,
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_from_pid(pid: i32) -> *mut task_struct;
    fn bpf_task_acquire(task: *mut task_struct) -> *mut task_struct;
    fn bpf_task_release(task: *mut task_struct);

    fn bpf_cgroup_from_id(id: u64) -> *mut cgroup;
    fn bpf_cgroup_acquire(cgrp: *mut cgroup) -> *mut cgroup;
    fn bpf_cgroup_release(cgrp: *mut cgroup);

    fn bpf_cpumask_create() -> *mut bpf_cpumask;
    fn bpf_cpumask_acquire(cpumask: *mut bpf_cpumask) -> *mut bpf_cpumask;
    fn bpf_cpumask_set_cpu(cpu: u32, cpumask: *mut bpf_cpumask);
    fn bpf_cpumask_test_cpu(cpu: u32, cpumask: *const cpumask) -> bool;
    fn bpf_cpumask_release(cpumask: *mut bpf_cpumask);
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

/***************
 * Task kfuncs *
 ***************/

unsafe fn task_kfunc_load_test() {
    let current: *mut task_struct;
    let ref_1: *mut task_struct;
    let ref_2: *mut task_struct;

    current = bpf_get_current_task_btf();
    ref_1 = bpf_task_from_pid((*current).pid);
    if ref_1.is_null() {
        return;
    }

    ref_2 = bpf_task_acquire(ref_1);
    if !ref_2.is_null() {
        bpf_task_release(ref_2);
    }
    bpf_task_release(ref_1);
}

#[no_mangle]
#[link_section = "raw_tp"]
/* __success */
pub unsafe extern "C" fn task_kfunc_raw_tp() -> i32 {
    task_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "syscall"]
/* __success */
pub unsafe extern "C" fn task_kfunc_syscall() -> i32 {
    task_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "tracepoint"]
/* __success */
pub unsafe extern "C" fn task_kfunc_tracepoint() -> i32 {
    task_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "perf_event"]
/* __success */
pub unsafe extern "C" fn task_kfunc_perf_event() -> i32 {
    task_kfunc_load_test();
    0
}

/*****************
 * cgroup kfuncs *
 *****************/

unsafe fn cgrp_kfunc_load_test() {
    let cgrp: *mut cgroup;
    let ref_: *mut cgroup;

    cgrp = bpf_cgroup_from_id(0);
    if cgrp.is_null() {
        return;
    }

    ref_ = bpf_cgroup_acquire(cgrp);
    if ref_.is_null() {
        bpf_cgroup_release(cgrp);
        return;
    }

    bpf_cgroup_release(ref_);
    bpf_cgroup_release(cgrp);
}

#[no_mangle]
#[link_section = "raw_tp"]
/* __success */
pub unsafe extern "C" fn cgrp_kfunc_raw_tp() -> i32 {
    cgrp_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "syscall"]
/* __success */
pub unsafe extern "C" fn cgrp_kfunc_syscall() -> i32 {
    cgrp_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "tracepoint"]
/* __success */
pub unsafe extern "C" fn cgrp_kfunc_tracepoint() -> i32 {
    cgrp_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "perf_event"]
/* __success */
pub unsafe extern "C" fn cgrp_kfunc_perf_event() -> i32 {
    cgrp_kfunc_load_test();
    0
}

/******************
 * cpumask kfuncs *
 ******************/

unsafe fn cpumask_kfunc_load_test() {
    let alloc: *mut bpf_cpumask;
    let ref_: *mut bpf_cpumask;

    alloc = bpf_cpumask_create();
    if alloc.is_null() {
        return;
    }

    ref_ = bpf_cpumask_acquire(alloc);
    bpf_cpumask_set_cpu(0, alloc);
    bpf_cpumask_test_cpu(0, ref_ as *const cpumask);

    bpf_cpumask_release(ref_);
    bpf_cpumask_release(alloc);
}

#[no_mangle]
#[link_section = "raw_tp"]
/* __success */
pub unsafe extern "C" fn cpumask_kfunc_raw_tp() -> i32 {
    cpumask_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "syscall"]
/* __success */
pub unsafe extern "C" fn cpumask_kfunc_syscall() -> i32 {
    cpumask_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "tracepoint"]
/* __success */
pub unsafe extern "C" fn cpumask_kfunc_tracepoint() -> i32 {
    cpumask_kfunc_load_test();
    0
}

#[no_mangle]
#[link_section = "perf_event"]
/* __success */
pub unsafe extern "C" fn cpumask_kfunc_perf_event() -> i32 {
    cpumask_kfunc_load_test();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
