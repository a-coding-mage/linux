// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_core_read.h>

use core::ffi::c_void;

// External BPF helper/CO-RE/map symbols supplied by the BPF build environment.
extern "C" {
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_nonexistent_kfunc() -> i32;
}

const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct task_struct___bad {
    pub pid: i32,
    pub fake_field: i32,
    pub fake_field_subprog: *mut c_void,
}

// Original C used __attribute__((preserve_access_index)) on task_struct___bad.

// Original C section: SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn bad_relo(ctx: *const c_void) -> i32 {
    static mut t: *mut task_struct___bad = core::ptr::null_mut();

    let _ = ctx;
    // Original C: bpf_core_field_size(t->fake_field)
    core::mem::size_of_val(&(*t).fake_field) as i32
}

#[inline(never)]
unsafe fn bad_subprog() -> i32 {
    static mut t: *mut task_struct___bad = core::ptr::null_mut();

    /* ugliness below is a field offset relocation */
    (&mut (*t).fake_field_subprog as *mut _ as *mut c_void as isize
        - t as *mut c_void as isize) as i32
}

// Original C section: SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn bad_relo_subprog(ctx: *const c_void) -> i32 {
    static mut t: *mut task_struct___bad = core::ptr::null_mut();

    let _ = ctx;
    bad_subprog() + core::mem::size_of_val(&(*t).pid) as i32
}

// Original C map definition used:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, int);
// } existing_map SEC(".maps");
#[repr(C)]
pub struct existing_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

#[no_mangle]
pub static existing_map: existing_map_def = existing_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

// Original C map definition used:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 1);
//     __type(key, int);
//     __type(value, int);
// } missing_map SEC(".maps");
#[repr(C)]
pub struct missing_map_def {
    pub type_: u32,
    pub max_entries: u32,
}

#[no_mangle]
pub static missing_map: missing_map_def = missing_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
};

// Original C section: SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn use_missing_map(ctx: *const c_void) -> i32 {
    let mut zero: i32 = 0;
    let mut value: *mut i32;

    let _ = ctx;
    value = bpf_map_lookup_elem(
        &existing_map as *const _ as *const c_void,
        &mut zero as *mut _ as *const c_void,
    ) as *mut i32;

    value = bpf_map_lookup_elem(
        &missing_map as *const _ as *const c_void,
        &mut zero as *mut _ as *const c_void,
    ) as *mut i32;

    (value != core::ptr::null_mut()) as i32
}

// Original C declaration: extern int bpf_nonexistent_kfunc(void) __ksym __weak;
// The __ksym and __weak attributes are supplied by the BPF build environment.

// Original C section: SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn use_missing_kfunc(ctx: *const c_void) -> i32 {
    let _ = ctx;
    bpf_nonexistent_kfunc();

    0
}

// Original C section: SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
