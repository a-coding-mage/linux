// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* C includes removed:
 * <linux/bpf.h>, <time.h>, <errno.h>, <bpf/bpf_helpers.h>,
 * <bpf/bpf_tracing.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

type __u64 = u64;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
const CLOCK_MONOTONIC: i32 = 1;

#[repr(C)]
pub struct bpf_timer {
    _opaque: [u64; 2],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hmap_elem {
    pub pad: i32, /* unused */
    pub timer: bpf_timer,
}

#[repr(C)]
pub struct inner_map {
    /* __uint(type, BPF_MAP_TYPE_HASH); */
    /* __uint(max_entries, 1024); */
    /* __type(key, int); */
    /* __type(value, struct hmap_elem); */
    pub _phantom: [u8; 0],
}

#[repr(C)]
pub struct outer_arr {
    /* __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS); */
    /* __uint(max_entries, 2); */
    /* __uint(key_size, sizeof(int)); */
    /* __uint(value_size, sizeof(int)); */
    /* __array(values, struct inner_map); */
    pub values: [*mut inner_map; 2],
}

unsafe extern "C" {
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut hmap_elem) -> i32,
    ) -> i32;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> i32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i32;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut bpf_map, clockid: i32) -> i32;
}

/* SEC("license") */
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* SEC(".maps") */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut inner_htab: inner_map = inner_map { _phantom: [] };

const ARRAY_KEY: usize = 1;
const HASH_KEY: i32 = 1234;

/* SEC(".maps") */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut outer_arr: outer_arr = outer_arr {
    values: [core::ptr::null_mut(), unsafe { &raw mut inner_htab }],
};

#[unsafe(no_mangle)]
pub static mut err: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut ok: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut cnt: __u64 = 0;

unsafe extern "C" fn timer_cb1(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    val: *mut hmap_elem,
) -> i32;

unsafe extern "C" fn timer_cb2(
    _map: *mut core::ffi::c_void,
    _key: *mut i32,
    val: *mut hmap_elem,
) -> i32 {
    unsafe {
        cnt = cnt.wrapping_add(1);
        bpf_timer_set_callback(&raw mut (*val).timer, timer_cb1);
        if bpf_timer_start(&raw mut (*val).timer, 1000, 0) != 0 {
            err |= 1;
        }
        ok |= 1;
    }
    0
}

/* callback for inner hash map */
unsafe extern "C" fn timer_cb1(
    map: *mut core::ffi::c_void,
    key: *mut i32,
    val: *mut hmap_elem,
) -> i32 {
    unsafe {
        cnt = cnt.wrapping_add(1);
        bpf_timer_set_callback(&raw mut (*val).timer, timer_cb2);
        if bpf_timer_start(&raw mut (*val).timer, 1000, 0) != 0 {
            err |= 2;
        }
        /* Do a lookup to make sure 'map' and 'key' pointers are correct */
        bpf_map_lookup_elem(map, key.cast());
        ok |= 2;
    }
    0
}

/* SEC("fentry/bpf_fentry_test1") */
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/bpf_fentry_test1")]
pub unsafe extern "C" fn test1(a: i32) -> i32 {
    let init: hmap_elem = unsafe { core::mem::zeroed() };
    let mut inner_map: *mut bpf_map;
    let mut val: *mut hmap_elem;
    let array_key: i32 = ARRAY_KEY as i32;
    let hash_key: i32 = HASH_KEY;

    let _ = a;

    unsafe {
        inner_map = bpf_map_lookup_elem(
            (&raw mut outer_arr).cast(),
            (&raw const array_key).cast(),
        )
        .cast();
        if inner_map.is_null() {
            return 0;
        }

        bpf_map_update_elem(
            inner_map.cast(),
            (&raw const hash_key).cast(),
            (&raw const init).cast(),
            0,
        );
        val = bpf_map_lookup_elem(inner_map.cast(), (&raw const hash_key).cast()).cast();
        if val.is_null() {
            return 0;
        }

        bpf_timer_init(&raw mut (*val).timer, inner_map, CLOCK_MONOTONIC);
        if bpf_timer_set_callback(&raw mut (*val).timer, timer_cb1) != 0 {
            err |= 4;
        }
        if bpf_timer_start(&raw mut (*val).timer, 0, 0) != 0 {
            err |= 8;
        }
    }
    0
}
