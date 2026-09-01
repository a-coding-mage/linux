// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// Translated from C. Original dependencies:
// <vmlinux.h>, <string.h>, <stdbool.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct bpf_task_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_copy_from_user_str(
        dst: *mut c_void,
        size: u32,
        unsafe_ptr: *const c_void,
        flags: u64,
    ) -> i64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_task_work_schedule_resume(
        task: *mut task_struct,
        work: *mut bpf_task_work,
        map: *mut c_void,
        callback: unsafe extern "C" fn(*mut bpf_map, *mut c_void, *mut c_void) -> c_int,
    ) -> i64;
}

// char _license[] SEC("license") = "GPL";
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
pub static mut user_ptr: *const c_void = ptr::null();

#[repr(C)]
pub struct elem {
    pub data: [c_char; 128],
    pub tw: bpf_task_work,
}

// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct elem);
// } hmap SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut hmap: bpf_map = bpf_map { _private: [] };

// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, struct elem);
// } arrmap SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut arrmap: bpf_map = bpf_map { _private: [] };

unsafe extern "C" fn process_work(
    _map: *mut bpf_map,
    _key: *mut c_void,
    value: *mut c_void,
) -> c_int {
    let work: *mut elem = value as *mut elem;

    unsafe {
        bpf_copy_from_user_str(
            (*work).data.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&(*work).data) as u32,
            user_ptr as *const c_void,
            0,
        );
    }
    0
}

#[unsafe(no_mangle)]
pub static mut key: c_int = 0;

// SEC("perf_event")
// __failure __msg("doesn't match map pointer in R3")
#[unsafe(link_section = "perf_event")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mismatch_map(_args: *mut pt_regs) -> c_int {
    let mut work: *mut elem;
    let task: *mut task_struct;

    unsafe {
        task = bpf_get_current_task_btf();
        work = bpf_map_lookup_elem(&raw mut arrmap as *mut c_void, &raw const key as *const c_void)
            as *mut elem;
        if work.is_null() {
            return 0;
        }
        bpf_task_work_schedule_resume(
            task,
            &mut (*work).tw,
            &raw mut hmap as *mut c_void,
            process_work,
        );
    }
    0
}

// SEC("perf_event")
// __failure __msg("R2 doesn't point to a map value")
#[unsafe(link_section = "perf_event")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn no_map_task_work(_args: *mut pt_regs) -> c_int {
    let task: *mut task_struct;
    let mut tw: bpf_task_work = bpf_task_work { _private: [] };

    unsafe {
        task = bpf_get_current_task_btf();
        bpf_task_work_schedule_resume(
            task,
            &mut tw,
            &raw mut hmap as *mut c_void,
            process_work,
        );
    }
    0
}

// SEC("perf_event")
// __failure __msg("Possibly NULL pointer passed to trusted R2")
#[unsafe(link_section = "perf_event")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_work_null(_args: *mut pt_regs) -> c_int {
    let task: *mut task_struct;

    unsafe {
        task = bpf_get_current_task_btf();
        bpf_task_work_schedule_resume(
            task,
            ptr::null_mut(),
            &raw mut hmap as *mut c_void,
            process_work,
        );
    }
    0
}

// SEC("perf_event")
// __failure __msg("Possibly NULL pointer passed to trusted R3")
#[unsafe(link_section = "perf_event")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_null(_args: *mut pt_regs) -> c_int {
    let mut work: *mut elem;
    let task: *mut task_struct;

    unsafe {
        task = bpf_get_current_task_btf();
        work = bpf_map_lookup_elem(&raw mut arrmap as *mut c_void, &raw const key as *const c_void)
            as *mut elem;
        if work.is_null() {
            return 0;
        }
        bpf_task_work_schedule_resume(task, &mut (*work).tw, ptr::null_mut(), process_work);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
