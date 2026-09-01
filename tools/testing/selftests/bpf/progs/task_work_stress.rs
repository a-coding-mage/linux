// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* Dependencies in the original C source:
 * #include <vmlinux.h>
 * #include <string.h>
 * #include <stdbool.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include "bpf_misc.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const ENTRIES: i32 = 128;
pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_NOEXIST: u64 = 1;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_task_work {
    _private: [u8; 0],
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut callback_scheduled: __u64 = 0;
#[no_mangle]
pub static mut callback_success: __u64 = 0;
#[no_mangle]
pub static mut schedule_error: __u64 = 0;
#[no_mangle]
pub static mut delete_success: __u64 = 0;

#[repr(C)]
pub struct elem {
    pub count: __u32,
    pub tw: bpf_task_work,
}

/* Original BPF map declaration:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_HASH);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __uint(max_entries, ENTRIES);
 *     __type(key, int);
 *     __type(value, struct elem);
 * } hmap SEC(".maps");
 */
#[link_section = ".maps"]
#[no_mangle]
pub static mut hmap: bpf_map = bpf_map { _private: [] };

extern "C" {
    fn bpf_ktime_get_ns() -> __u64;
    fn bpf_map_lookup_elem(map: *mut bpf_map, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_task_work_schedule_signal(
        task: *mut task_struct,
        tw: *mut bpf_task_work,
        map: *mut bpf_map,
        callback: unsafe extern "C" fn(*mut bpf_map, *mut core::ffi::c_void, *mut core::ffi::c_void) -> i32,
    ) -> i32;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_get_prandom_u32() -> __u32;
    fn bpf_map_delete_elem(map: *mut bpf_map, key: *const core::ffi::c_void) -> i64;
}

unsafe extern "C" fn process_work(
    _map: *mut bpf_map,
    _key: *mut core::ffi::c_void,
    _value: *mut core::ffi::c_void,
) -> i32 {
    callback_success = callback_success.wrapping_add(1);
    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn schedule_task_work(_ctx: *mut core::ffi::c_void) -> i32 {
    let empty_work: elem = elem {
        count: 0,
        tw: bpf_task_work { _private: [] },
    };
    let mut work: *mut elem;
    let mut key: i32 = 0;
    let err: i32;

    key = (bpf_ktime_get_ns() % ENTRIES as __u64) as i32;
    work = bpf_map_lookup_elem(
        &raw mut hmap,
        &key as *const i32 as *const core::ffi::c_void,
    ) as *mut elem;
    if work.is_null() {
        bpf_map_update_elem(
            &raw mut hmap,
            &key as *const i32 as *const core::ffi::c_void,
            &empty_work as *const elem as *const core::ffi::c_void,
            BPF_NOEXIST,
        );
        work = bpf_map_lookup_elem(
            &raw mut hmap,
            &key as *const i32 as *const core::ffi::c_void,
        ) as *mut elem;
        if work.is_null() {
            return 0;
        }
    }
    err = bpf_task_work_schedule_signal(
        bpf_get_current_task_btf(),
        &mut (*work).tw as *mut bpf_task_work,
        &raw mut hmap,
        process_work,
    );
    if err != 0 {
        schedule_error = schedule_error.wrapping_add(1);
    } else {
        callback_scheduled = callback_scheduled.wrapping_add(1);
    }
    0
}

#[link_section = "syscall"]
#[no_mangle]
pub unsafe extern "C" fn delete_task_work(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 0;
    let err: i64;

    key = (bpf_get_prandom_u32() % ENTRIES as __u32) as i32;
    err = bpf_map_delete_elem(
        &raw mut hmap,
        &key as *const i32 as *const core::ffi::c_void,
    );
    if err == 0 {
        delete_success = delete_success.wrapping_add(1);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
