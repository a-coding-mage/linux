// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_experimental.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

type u64 = u64;

#[repr(C)]
pub struct bpf_spin_lock {
    _data: [u8; 0],
}

#[repr(C)]
pub struct bin_data {
    pub data: [::core::ffi::c_char; 256],
    pub lock: bpf_spin_lock,
}

#[repr(C)]
pub struct map_value {
    pub data: *mut bin_data,
}

#[repr(C)]
pub struct array_map {
    _data: [u8; 0],
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __type(key, int);
//     __type(value, struct map_value);
//     __uint(max_entries, 2048);
// } array SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut array: array_map = array_map { _data: [] };

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

#[no_mangle]
pub static mut nomem_err: bool = false;

extern "C" {
    fn bpf_map_lookup_elem(
        map: *mut array_map,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_kptr_xchg(
        kptr: *mut *mut bin_data,
        ptr: *mut bin_data,
    ) -> *mut bin_data;
    fn bpf_obj_drop(ptr: *mut bin_data);
    fn bpf_obj_new_bin_data() -> *mut bin_data;
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(u32, *mut i32) -> i32,
        callback_ctx: *mut i32,
        flags: u64,
    ) -> i64;
}

unsafe extern "C" fn del_array(_i: u32, from: *mut i32) -> i32 {
    let value: *mut map_value;
    let old: *mut bin_data;

    value = bpf_map_lookup_elem(
        ::core::ptr::addr_of_mut!(array),
        from as *const ::core::ffi::c_void,
    ) as *mut map_value;
    if value.is_null() {
        return 1;
    }

    old = bpf_kptr_xchg(::core::ptr::addr_of_mut!((*value).data), ::core::ptr::null_mut());
    if !old.is_null() {
        bpf_obj_drop(old);
    }

    *from += 1;
    0
}

unsafe extern "C" fn add_array(_i: u32, from: *mut i32) -> i32 {
    let old: *mut bin_data;
    let new: *mut bin_data;
    let value: *mut map_value;

    value = bpf_map_lookup_elem(
        ::core::ptr::addr_of_mut!(array),
        from as *const ::core::ffi::c_void,
    ) as *mut map_value;
    if value.is_null() {
        return 1;
    }

    new = bpf_obj_new_bin_data();
    if new.is_null() {
        nomem_err = true;
        return 1;
    }

    old = bpf_kptr_xchg(::core::ptr::addr_of_mut!((*value).data), new);
    if !old.is_null() {
        bpf_obj_drop(old);
    }

    *from += 1;
    0
}

unsafe fn del_then_add_array(from: i32) {
    let mut i: i32;

    i = from;
    bpf_loop(512, del_array, &mut i, 0);

    i = from;
    bpf_loop(512, add_array, &mut i, 0);
}

#[link_section = "fentry/bpf_fentry_test1"]
#[no_mangle]
pub unsafe extern "C" fn test0(a: i32) -> i32 {
    let _ = a;
    del_then_add_array(0);
    0
}

#[link_section = "fentry/bpf_fentry_test2"]
#[no_mangle]
pub unsafe extern "C" fn test1(a: i32, b: u64) -> i32 {
    let _ = a;
    let _ = b;
    del_then_add_array(512);
    0
}

#[link_section = "fentry/bpf_fentry_test3"]
#[no_mangle]
pub unsafe extern "C" fn test2(a: ::core::ffi::c_char, b: i32, c: u64) -> i32 {
    let _ = a;
    let _ = b;
    let _ = c;
    del_then_add_array(1024);
    0
}

#[link_section = "fentry/bpf_fentry_test4"]
#[no_mangle]
pub unsafe extern "C" fn test3(
    a: *mut ::core::ffi::c_void,
    b: ::core::ffi::c_char,
    c: i32,
    d: u64,
) -> i32 {
    let _ = a;
    let _ = b;
    let _ = c;
    let _ = d;
    del_then_add_array(1536);
    0
}
