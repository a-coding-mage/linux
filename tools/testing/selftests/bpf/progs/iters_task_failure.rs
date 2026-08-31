// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Chuyi Zhou <zhouchuyi@bytedance.com> */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"
// #include "bpf_experimental.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;
use core::ptr;

pub type u64 = u64;
pub type u32 = u32;

#[repr(C)]
pub struct cgroup_subsys_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    pub self_: cgroup_subsys_state,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_task {
    _private: [u8; 0],
}

pub const BPF_TASK_ITER_ALL_PROCS: u32 = 0;
pub const BPF_CGROUP_ITER_DESCENDANTS_POST: u32 = 0;
pub const CSS_TASK_ITER_PROCS: u32 = 0;
pub const BPF_F_TEST_STATE_FREQ: u32 = 0;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    #[link_name = "bpf_cgroup_from_id"]
    pub fn bpf_cgroup_from_id(cgid: u64) -> *mut cgroup;
    #[link_name = "bpf_cgroup_release"]
    pub fn bpf_cgroup_release(p: *mut cgroup);
    #[link_name = "bpf_rcu_read_lock"]
    pub fn bpf_rcu_read_lock();
    #[link_name = "bpf_rcu_read_unlock"]
    pub fn bpf_rcu_read_unlock();

    pub fn bpf_get_current_cgroup_id() -> u64;
    pub fn bpf_get_prandom_u32() -> u32;
    pub fn bpf_iter_task_new(
        it: *mut bpf_iter_task,
        task: *mut task_struct,
        flags: u32,
    ) -> c_void;
    pub fn bpf_iter_task_next(it: *mut bpf_iter_task) -> *mut task_struct;
    pub fn bpf_iter_task_destroy(it: *mut bpf_iter_task);
    pub fn bpf_iter_css_new(
        it: *mut c_void,
        start: *mut cgroup_subsys_state,
        flags: u32,
    ) -> c_void;
    pub fn bpf_iter_css_next(it: *mut c_void) -> *mut cgroup_subsys_state;
    pub fn bpf_iter_css_destroy(it: *mut c_void);
    pub fn bpf_iter_css_task_new(
        it: *mut c_void,
        css: *mut cgroup_subsys_state,
        flags: u32,
    ) -> c_void;
    pub fn bpf_iter_css_task_next(it: *mut c_void) -> *mut task_struct;
    pub fn bpf_iter_css_task_destroy(it: *mut c_void);
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure
// __msg("kernel func bpf_iter_task_new requires RCU critical section protection")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_tasks_without_lock() -> i32 {
    let mut pos: *mut task_struct;
    let mut it: bpf_iter_task = core::mem::zeroed();

    bpf_iter_task_new(&mut it, ptr::null_mut(), BPF_TASK_ITER_ALL_PROCS);
    loop {
        pos = bpf_iter_task_next(&mut it);
        if pos.is_null() {
            break;
        }
    }
    bpf_iter_task_destroy(&mut it);
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure
// __msg("kernel func bpf_iter_css_new requires RCU critical section protection")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_css_without_lock() -> i32 {
    let cg_id: u64 = bpf_get_current_cgroup_id();
    let cgrp: *mut cgroup = bpf_cgroup_from_id(cg_id);
    let root_css: *mut cgroup_subsys_state;
    let mut pos: *mut cgroup_subsys_state;
    let mut it: c_void = core::mem::zeroed();

    if cgrp.is_null() {
        return 0;
    }
    root_css = &mut (*cgrp).self_;

    bpf_iter_css_new(&mut it, root_css, BPF_CGROUP_ITER_DESCENDANTS_POST);
    loop {
        pos = bpf_iter_css_next(&mut it);
        if pos.is_null() {
            break;
        }
    }
    bpf_iter_css_destroy(&mut it);
    bpf_cgroup_release(cgrp);
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure
// __msg("expected an RCU CS when using bpf_iter_task_next")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_tasks_lock_and_unlock() -> i32 {
    let mut pos: *mut task_struct;
    let mut it: bpf_iter_task = core::mem::zeroed();

    bpf_rcu_read_lock();
    bpf_iter_task_new(&mut it, ptr::null_mut(), BPF_TASK_ITER_ALL_PROCS);
    loop {
        pos = bpf_iter_task_next(&mut it);
        if pos.is_null() {
            break;
        }
        bpf_rcu_read_unlock();

        bpf_rcu_read_lock();
    }
    bpf_iter_task_destroy(&mut it);
    bpf_rcu_read_unlock();
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure
// __msg("expected an RCU CS when using bpf_iter_task_next")
// __flag(BPF_F_TEST_STATE_FREQ)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_tasks_rcu_state_pruning() -> i32 {
    let mut it: bpf_iter_task = core::mem::zeroed();

    asm!(
        "call {bpf_rcu_read_lock}",
        "r1 = {it}",
        "r2 = 0",
        "r3 = 0",
        "call {bpf_iter_task_new}",
        "call {bpf_get_prandom_u32}",
        "if w0 == 0 goto 2f",
        // Keep the outer RCU lock active on the straight-line path.
        "call {bpf_rcu_read_lock}",
        "call {bpf_rcu_read_unlock}",
        "goto 3f",
        "2:",
        // Create an unprotected gap on the taken path.
        "call {bpf_rcu_read_unlock}",
        "call {bpf_rcu_read_lock}",
        "3:",
        "r1 = {it}",
        "call {bpf_iter_task_next}",
        "r1 = {it}",
        "call {bpf_iter_task_destroy}",
        "call {bpf_rcu_read_unlock}",
        it = in(reg) &mut it,
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_iter_task_new = sym bpf_iter_task_new,
        bpf_iter_task_next = sym bpf_iter_task_next,
        bpf_iter_task_destroy = sym bpf_iter_task_destroy,
        bpf_rcu_read_lock = sym bpf_rcu_read_lock,
        bpf_rcu_read_unlock = sym bpf_rcu_read_unlock,
    );

    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure
// __msg("expected an RCU CS when using bpf_iter_css_next")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_css_lock_and_unlock() -> i32 {
    let cg_id: u64 = bpf_get_current_cgroup_id();
    let cgrp: *mut cgroup = bpf_cgroup_from_id(cg_id);
    let root_css: *mut cgroup_subsys_state;
    let mut pos: *mut cgroup_subsys_state;
    let mut it: c_void = core::mem::zeroed();

    if cgrp.is_null() {
        return 0;
    }
    root_css = &mut (*cgrp).self_;

    bpf_rcu_read_lock();
    bpf_iter_css_new(&mut it, root_css, BPF_CGROUP_ITER_DESCENDANTS_POST);
    loop {
        pos = bpf_iter_css_next(&mut it);
        if pos.is_null() {
            break;
        }
        bpf_rcu_read_unlock();

        bpf_rcu_read_lock();
    }
    bpf_iter_css_destroy(&mut it);
    bpf_rcu_read_unlock();
    bpf_cgroup_release(cgrp);
    0
}

// SEC("?fentry/" SYS_PREFIX "sys_getpgid")
// __failure
// __msg("css_task_iter is only allowed in bpf_lsm, bpf_iter and sleepable progs")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_css_task_for_each() -> i32 {
    let cg_id: u64 = bpf_get_current_cgroup_id();
    let cgrp: *mut cgroup = bpf_cgroup_from_id(cg_id);
    let css: *mut cgroup_subsys_state;
    let mut task: *mut task_struct;
    let mut it: c_void = core::mem::zeroed();

    if cgrp.is_null() {
        return 0;
    }
    css = &mut (*cgrp).self_;

    bpf_iter_css_task_new(&mut it, css, CSS_TASK_ITER_PROCS);
    loop {
        task = bpf_iter_css_task_next(&mut it);
        if task.is_null() {
            break;
        }
    }
    bpf_iter_css_task_destroy(&mut it);
    bpf_cgroup_release(cgrp);
    0
}
