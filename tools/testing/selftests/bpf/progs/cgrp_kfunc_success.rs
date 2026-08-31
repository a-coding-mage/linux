// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external Rust dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include "cgrp_kfunc_common.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_void};

pub type u64 = u64;

#[repr(C)]
pub struct cgroup_subsys_state {
    pub id: u64,
}

#[repr(C)]
pub struct kernfs_node {
    pub id: u64,
}

#[repr(C)]
pub struct cgroup {
    // Layout and complete definition are supplied by vmlinux.h bindings.
    pub self_: cgroup_subsys_state,
    pub level: i32,
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct __cgrps_kfunc_map_value {
    pub cgrp: *mut cgroup,
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_cgroup_acquire(cgrp: *mut cgroup) -> *mut cgroup;
    fn bpf_cgroup_release(cgrp: *mut cgroup);
    fn bpf_cgroup_ancestor(cgrp: *mut cgroup, level: i32) -> *mut cgroup;
    fn bpf_cgroup_from_id(cgid: u64) -> *mut cgroup;
    fn bpf_kptr_xchg(kptr: *mut *mut cgroup, val: *mut cgroup) -> *mut cgroup;
    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();

    fn cgrps_kfunc_map_insert(cgrp: *mut cgroup) -> isize;
    fn cgrps_kfunc_map_value_lookup(cgrp: *mut cgroup) -> *mut __cgrps_kfunc_map_value;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
pub static mut err: i32 = 0;
#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut invocations: i32 = 0;

/* Prototype for all of the program trace events below:
 *
 * TRACE_EVENT(cgroup_mkdir,
 *         TP_PROTO(struct cgroup *cgrp, const char *path),
 *         TP_ARGS(cgrp, path)
 */

unsafe fn is_test_kfunc_task() -> bool {
    let cur_pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    let same: bool = unsafe { pid == cur_pid };

    if same {
        unsafe {
            let current = core::ptr::read_volatile(core::ptr::addr_of!(invocations));
            core::ptr::write_volatile(core::ptr::addr_of_mut!(invocations), current.wrapping_add(1));
        }
    }

    same
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgrp_acquire_release_argument(
    cgrp: *mut cgroup,
    path: *const c_char,
) -> i32 {
    let acquired: *mut cgroup;

    let _ = path;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    acquired = unsafe { bpf_cgroup_acquire(cgrp) };
    if acquired.is_null() {
        unsafe { err = 1 };
    } else {
        unsafe { bpf_cgroup_release(acquired) };
    }

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgrp_acquire_leave_in_map(
    cgrp: *mut cgroup,
    path: *const c_char,
) -> i32 {
    let status: isize;

    let _ = path;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    status = unsafe { cgrps_kfunc_map_insert(cgrp) };
    if status != 0 {
        unsafe { err = 1 };
    }

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgrp_xchg_release(cgrp: *mut cgroup, path: *const c_char) -> i32 {
    let mut kptr: *mut cgroup;
    let cg: *mut cgroup;
    let v: *mut __cgrps_kfunc_map_value;
    let status: isize;

    let _ = path;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    status = unsafe { cgrps_kfunc_map_insert(cgrp) };
    if status != 0 {
        unsafe { err = 1 };
        return 0;
    }

    v = unsafe { cgrps_kfunc_map_value_lookup(cgrp) };
    if v.is_null() {
        unsafe { err = 2 };
        return 0;
    }

    kptr = unsafe { (*v).cgrp };
    if kptr.is_null() {
        unsafe { err = 4 };
        return 0;
    }

    cg = unsafe { bpf_cgroup_ancestor(kptr, 1) };
    if !cg.is_null() {
        /* verifier only check */
        unsafe { bpf_cgroup_release(cg) };
    }

    kptr = unsafe { bpf_kptr_xchg(core::ptr::addr_of_mut!((*v).cgrp), core::ptr::null_mut()) };
    if kptr.is_null() {
        unsafe { err = 3 };
        return 0;
    }

    unsafe { bpf_cgroup_release(kptr) };

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgrp_get_release(cgrp: *mut cgroup, path: *const c_char) -> i32 {
    let kptr: *mut cgroup;
    let v: *mut __cgrps_kfunc_map_value;
    let status: isize;

    let _ = path;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    status = unsafe { cgrps_kfunc_map_insert(cgrp) };
    if status != 0 {
        unsafe { err = 1 };
        return 0;
    }

    v = unsafe { cgrps_kfunc_map_value_lookup(cgrp) };
    if v.is_null() {
        unsafe { err = 2 };
        return 0;
    }

    unsafe { bpf_rcu_read_lock() };
    kptr = unsafe { (*v).cgrp };
    if kptr.is_null() {
        unsafe { err = 3 };
    }
    unsafe { bpf_rcu_read_unlock() };

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgrp_get_ancestors(cgrp: *mut cgroup, path: *const c_char) -> i32 {
    let self_: *mut cgroup;
    let ancestor1: *mut cgroup;
    let mut invalid: *mut cgroup;

    let _ = path;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    self_ = unsafe { bpf_cgroup_ancestor(cgrp, (*cgrp).level) };
    if self_.is_null() {
        unsafe { err = 1 };
        return 0;
    }

    if unsafe { (*self_).self_.id != (*cgrp).self_.id } {
        unsafe { bpf_cgroup_release(self_) };
        unsafe { err = 2 };
        return 0;
    }
    unsafe { bpf_cgroup_release(self_) };

    ancestor1 = unsafe { bpf_cgroup_ancestor(cgrp, (*cgrp).level - 1) };
    if ancestor1.is_null() {
        unsafe { err = 3 };
        return 0;
    }
    unsafe { bpf_cgroup_release(ancestor1) };

    invalid = unsafe { bpf_cgroup_ancestor(cgrp, 10000) };
    if !invalid.is_null() {
        unsafe { bpf_cgroup_release(invalid) };
        unsafe { err = 4 };
        return 0;
    }

    invalid = unsafe { bpf_cgroup_ancestor(cgrp, -1) };
    if !invalid.is_null() {
        unsafe { bpf_cgroup_release(invalid) };
        unsafe { err = 5 };
        return 0;
    }

    0
}

#[unsafe(link_section = "tp_btf/cgroup_mkdir")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgrp_from_id(cgrp: *mut cgroup, path: *const c_char) -> i32 {
    let parent: *mut cgroup;
    let mut res: *mut cgroup;
    let parent_cgid: u64;

    let _ = path;

    if !unsafe { is_test_kfunc_task() } {
        return 0;
    }

    /* @cgrp's ID is not visible yet, let's test with the parent */
    parent = unsafe { bpf_cgroup_ancestor(cgrp, (*cgrp).level - 1) };
    if parent.is_null() {
        unsafe { err = 1 };
        return 0;
    }

    parent_cgid = unsafe { (*(*parent).kn).id };
    unsafe { bpf_cgroup_release(parent) };

    res = unsafe { bpf_cgroup_from_id(parent_cgid) };
    if res.is_null() {
        unsafe { err = 2 };
        return 0;
    }

    unsafe { bpf_cgroup_release(res) };

    if res != parent {
        unsafe { err = 3 };
        return 0;
    }

    res = unsafe { bpf_cgroup_from_id(-1i64 as u64) };
    if !res.is_null() {
        unsafe { bpf_cgroup_release(res) };
        unsafe { err = 4 };
        return 0;
    }

    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgrp_from_id_ns(ctx: *mut c_void) -> i32 {
    let cg: *mut cgroup;

    let _ = ctx;

    cg = unsafe { bpf_cgroup_from_id(1) };
    if cg.is_null() {
        return 42;
    }
    unsafe { bpf_cgroup_release(cg) };
    0
}
