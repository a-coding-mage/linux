// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Translated from:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
const BPF_MAP_TYPE_BLOOM_FILTER: u32 = 30;

/* SYS_PREFIX is supplied by bpf_misc.h in the original C source. */
const SYS_PREFIX: &str = "";

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct map_random_data_type {
    /* __uint(type, BPF_MAP_TYPE_ARRAY); */
    pub type_: u32,
    /* __type(key, __u32); */
    pub key: __u32,
    /* __type(value, __u32); */
    pub value: __u32,
    /* __uint(max_entries, 1000); */
    pub max_entries: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_random_data: map_random_data_type = map_random_data_type {
    type_: BPF_MAP_TYPE_ARRAY,
    key: 0,
    value: 0,
    max_entries: 1000,
};

#[repr(C)]
pub struct map_bloom_type {
    /* __uint(type, BPF_MAP_TYPE_BLOOM_FILTER); */
    pub type_: u32,
    /* __type(value, __u32); */
    pub value: __u32,
    /* __uint(max_entries, 10000); */
    pub max_entries: u32,
    /* __uint(map_extra, 5); */
    pub map_extra: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_bloom: map_bloom_type = map_bloom_type {
    type_: BPF_MAP_TYPE_BLOOM_FILTER,
    value: 0,
    max_entries: 10000,
    map_extra: 5,
};

#[repr(C)]
pub struct outer_map_type {
    /* __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS); */
    pub type_: u32,
    /* __type(key, int); */
    pub key: i32,
    /* __type(value, int); */
    pub value: i32,
    /* __uint(max_entries, 1); */
    pub max_entries: u32,
    /* __array(values, struct map_bloom_type); */
    pub values: [map_bloom_type; 0],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut outer_map: outer_map_type = outer_map_type {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    key: 0,
    value: 0,
    max_entries: 1,
    values: [],
};

#[repr(C)]
pub struct callback_ctx {
    pub map: *mut bpf_map,
}

#[unsafe(no_mangle)]
pub static mut error: i32 = 0;

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut bpf_map;
    fn bpf_map_peek_elem(map: *mut bpf_map, value: *mut core::ffi::c_void) -> i32;
    fn bpf_for_each_map_elem(
        map: *const core::ffi::c_void,
        callback: unsafe extern "C" fn(
            *mut bpf_map,
            *mut __u32,
            *mut __u32,
            *mut callback_ctx,
        ) -> __u64,
        callback_ctx: *mut callback_ctx,
        flags: __u64,
    ) -> i32;
}

unsafe extern "C" fn check_elem(
    _map: *mut bpf_map,
    _key: *mut __u32,
    val: *mut __u32,
    data: *mut callback_ctx,
) -> __u64 {
    let err: i32;

    err = unsafe { bpf_map_peek_elem((*data).map, val as *mut core::ffi::c_void) };
    if err != 0 {
        unsafe {
            error |= 1;
        }
        return 1; /* stop the iteration */
    }

    0
}

/* SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[unsafe(link_section = "fentry/sys_getpgid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn inner_map(ctx: *mut core::ffi::c_void) -> i32 {
    let inner_map: *mut bpf_map;
    let mut data: callback_ctx = callback_ctx {
        map: core::ptr::null_mut(),
    };
    let mut key: i32 = 0;

    let _ = ctx;

    inner_map = unsafe {
        bpf_map_lookup_elem(
            &raw const outer_map as *const core::ffi::c_void,
            &mut key as *mut i32 as *const core::ffi::c_void,
        )
    };
    if inner_map.is_null() {
        unsafe {
            error |= 2;
        }
        return 0;
    }

    data.map = inner_map;
    unsafe {
        bpf_for_each_map_elem(
            &raw const map_random_data as *const core::ffi::c_void,
            check_elem,
            &mut data,
            0,
        );
    }

    0
}

/* SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[unsafe(link_section = "fentry/sys_getpgid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_bloom(ctx: *mut core::ffi::c_void) -> i32 {
    let mut data: callback_ctx = callback_ctx {
        map: core::ptr::null_mut(),
    };

    let _ = ctx;

    data.map = &raw mut map_bloom as *mut bpf_map;
    unsafe {
        bpf_for_each_map_elem(
            &raw const map_random_data as *const core::ffi::c_void,
            check_elem,
            &mut data,
            0,
        );
    }

    0
}
