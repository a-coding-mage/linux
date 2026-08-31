// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025. Huawei Technologies Co., Ltd */

#![no_std]

use core::ffi::c_int;

// Translated from:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
//
// The original C file relies on libbpf's SEC, __uint, __type, and __array
// macros plus BPF_MAP_TYPE_* constants supplied by those headers.
const BPF_MAP_TYPE_ARRAY: usize = 2;
const BPF_MAP_TYPE_HASH_OF_MAPS: usize = 13;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct inner_map_type {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    pub type_: *mut [c_int; BPF_MAP_TYPE_ARRAY],
    // __uint(key_size, 4);
    pub key_size: *mut [c_int; 4],
    // __uint(value_size, 4);
    pub value_size: *mut [c_int; 4],
    // __uint(max_entries, 1);
    pub max_entries: *mut [c_int; 1],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut inner_map: inner_map_type = inner_map_type {
    type_: core::ptr::null_mut(),
    key_size: core::ptr::null_mut(),
    value_size: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
};

#[repr(C)]
pub struct outer_map_type {
    // __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
    pub type_: *mut [c_int; BPF_MAP_TYPE_HASH_OF_MAPS],
    // __uint(max_entries, 64);
    pub max_entries: *mut [c_int; 64],
    // __type(key, int);
    pub key: *mut c_int,
    // __type(value, int);
    pub value: *mut c_int,
    // __array(values, struct inner_map_type);
    pub values: [*mut inner_map_type; 1],
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut outer_map: outer_map_type = outer_map_type {
    type_: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
    values: [unsafe { core::ptr::addr_of_mut!(inner_map) }],
};
