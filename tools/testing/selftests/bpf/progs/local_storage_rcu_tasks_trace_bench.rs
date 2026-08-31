// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type bool_ = bool;

const BPF_MAP_TYPE_TASK_STORAGE: u32 = 24;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_storage_map {
    // __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    pub type_: u32,
    // __uint(map_flags, BPF_F_NO_PREALLOC);
    pub map_flags: u32,
    // __type(key, int);
    // __type(value, int);
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut task_storage: task_storage_map = task_storage_map {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
};

#[no_mangle]
pub static mut hits: i64 = 0;
#[no_mangle]
pub static mut gp_hits: i64 = 0;
#[no_mangle]
pub static mut gp_times: i64 = 0;
#[no_mangle]
pub static mut current_gp_start: i64 = 0;
#[no_mangle]
pub static mut unexpected: i64 = 0;
#[no_mangle]
pub static mut postgp_seen: bool_ = false;

extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_task_storage_get(
        map: *mut task_storage_map,
        task: *mut task_struct,
        value: *mut i32,
        flags: u64,
    ) -> *mut i32;
    fn bpf_task_storage_delete(map: *mut task_storage_map, task: *mut task_struct) -> i64;
    fn bpf_ktime_get_ns() -> i64;
}

#[inline]
unsafe fn __sync_add_and_fetch(ptr: *mut i64, val: i64) -> i64 {
    let old = core::intrinsics::atomic_xadd_seqcst(ptr, val);
    old.wrapping_add(val)
}

// SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[link_section = "fentry/sys_getpgid"]
#[no_mangle]
pub unsafe extern "C" fn get_local(ctx: *mut core::ffi::c_void) -> i32 {
    let task: *mut task_struct;
    let mut idx: i32;
    let s: *mut i32;

    let _ = ctx;
    idx = 0;
    task = bpf_get_current_task_btf();
    s = bpf_task_storage_get(
        &mut task_storage,
        task,
        &mut idx,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if s.is_null() {
        return 0;
    }

    *s = 3;
    bpf_task_storage_delete(&mut task_storage, task);
    __sync_add_and_fetch(&mut hits, 1);
    0
}

// SEC("fentry/rcu_tasks_trace_pregp_step")
#[link_section = "fentry/rcu_tasks_trace_pregp_step"]
#[no_mangle]
pub unsafe extern "C" fn pregp_step(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    current_gp_start = bpf_ktime_get_ns();
    0
}

// SEC("fentry/rcu_tasks_trace_postgp")
#[link_section = "fentry/rcu_tasks_trace_postgp"]
#[no_mangle]
pub unsafe extern "C" fn postgp(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    if current_gp_start == 0 && postgp_seen {
        /* Will only happen if prog tracing rcu_tasks_trace_pregp_step doesn't
         * execute before this prog
         */
        __sync_add_and_fetch(&mut unexpected, 1);
        return 0;
    }

    __sync_add_and_fetch(&mut gp_times, bpf_ktime_get_ns() - current_gp_start);
    __sync_add_and_fetch(&mut gp_hits, 1);
    current_gp_start = 0;
    postgp_seen = true;
    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
