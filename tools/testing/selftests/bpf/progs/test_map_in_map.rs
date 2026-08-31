// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018 Facebook */

/* C dependencies:
 * #include <stddef.h>
 * #include <linux/bpf.h>
 * #include <linux/types.h>
 * #include <bpf/bpf_helpers.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::c_void;

type __u32 = u32;

extern "C" {
    static XDP_DROP: i32;
    static XDP_PASS: i32;
    static BPF_MAP_TYPE_ARRAY_OF_MAPS: u32;
    static BPF_MAP_TYPE_HASH_OF_MAPS: u32;
    static BPF_MAP_TYPE_PERF_EVENT_ARRAY: u32;

    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> i64;
}

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mim_array_t {
    /* __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS); */
    pub type_: u32,
    /* __uint(max_entries, 1); */
    pub max_entries: u32,
    /* __uint(map_flags, 0); */
    pub map_flags: u32,
    /* __type(key, __u32); */
    pub key: __u32,
    /* __type(value, __u32); */
    pub value: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut mim_array: mim_array_t = mim_array_t {
    type_: 0, /* BPF_MAP_TYPE_ARRAY_OF_MAPS */
    max_entries: 1,
    map_flags: 0,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct mim_hash_t {
    /* __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS); */
    pub type_: u32,
    /* __uint(max_entries, 1); */
    pub max_entries: u32,
    /* __uint(map_flags, 0); */
    pub map_flags: u32,
    /* __type(key, int); */
    pub key: i32,
    /* __type(value, __u32); */
    pub value: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut mim_hash: mim_hash_t = mim_hash_t {
    type_: 0, /* BPF_MAP_TYPE_HASH_OF_MAPS */
    max_entries: 1,
    map_flags: 0,
    key: 0,
    value: 0,
};

/* The following three maps are used to test
 * perf_event_array map can be an inner
 * map of hash/array_of_maps.
 */
#[repr(C)]
pub struct perf_event_array {
    /* __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY); */
    pub type_: u32,
    /* __type(key, __u32); */
    pub key: __u32,
    /* __type(value, __u32); */
    pub value: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut inner_map0: perf_event_array = perf_event_array {
    type_: 0, /* BPF_MAP_TYPE_PERF_EVENT_ARRAY */
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct mim_array_pe_t {
    /* __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS); */
    pub type_: u32,
    /* __uint(max_entries, 1); */
    pub max_entries: u32,
    /* __type(key, __u32); */
    pub key: __u32,
    /* __array(values, struct perf_event_array); */
    pub values: [*mut perf_event_array; 1],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut mim_array_pe: mim_array_pe_t = mim_array_pe_t {
    type_: 0, /* BPF_MAP_TYPE_ARRAY_OF_MAPS */
    max_entries: 1,
    key: 0,
    values: [unsafe { &mut inner_map0 as *mut perf_event_array }],
};

#[repr(C)]
pub struct mim_hash_pe_t {
    /* __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS); */
    pub type_: u32,
    /* __uint(max_entries, 1); */
    pub max_entries: u32,
    /* __type(key, __u32); */
    pub key: __u32,
    /* __array(values, struct perf_event_array); */
    pub values: [*mut perf_event_array; 1],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut mim_hash_pe: mim_hash_pe_t = mim_hash_pe_t {
    type_: 0, /* BPF_MAP_TYPE_HASH_OF_MAPS */
    max_entries: 1,
    key: 0,
    values: [unsafe { &mut inner_map0 as *mut perf_event_array }],
};

#[link_section = "xdp"]
#[no_mangle]
pub unsafe extern "C" fn xdp_mimtest0(ctx: *mut xdp_md) -> i32 {
    let mut value: i32 = 123;
    let mut value_p: *mut i32;
    let mut key: i32 = 0;
    let mut map: *mut c_void;

    let _ = ctx;

    map = bpf_map_lookup_elem(
        &mim_array as *const mim_array_t as *const c_void,
        &key as *const i32 as *const c_void,
    );
    if map.is_null() {
        return XDP_DROP;
    }

    bpf_map_update_elem(
        map,
        &key as *const i32 as *const c_void,
        &value as *const i32 as *const c_void,
        0,
    );
    value_p = bpf_map_lookup_elem(map, &key as *const i32 as *const c_void) as *mut i32;
    if value_p.is_null() || *value_p != 123 {
        return XDP_DROP;
    }

    map = bpf_map_lookup_elem(
        &mim_hash as *const mim_hash_t as *const c_void,
        &key as *const i32 as *const c_void,
    );
    if map.is_null() {
        return XDP_DROP;
    }

    bpf_map_update_elem(
        map,
        &key as *const i32 as *const c_void,
        &value as *const i32 as *const c_void,
        0,
    );

    return XDP_PASS;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
