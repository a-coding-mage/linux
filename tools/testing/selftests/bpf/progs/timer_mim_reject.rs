// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/*
 * Translated from C. Original dependencies:
 * <linux/bpf.h>, <time.h>, <errno.h>, <bpf/bpf_helpers.h>,
 * <bpf/bpf_tracing.h>
 */

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
pub const CLOCK_MONOTONIC: i32 = 1;

pub const ARRAY_KEY: i32 = 1;
pub const ARRAY_KEY2: i32 = 2;
pub const HASH_KEY: i32 = 1234;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
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
    /*
     * C BPF map metadata:
     * __uint(type, BPF_MAP_TYPE_HASH);
     * __uint(max_entries, 1024);
     * __type(key, int);
     * __type(value, struct hmap_elem);
     */
    _private: [u8; 0],
}

#[repr(C)]
pub struct outer_arr {
    /*
     * C BPF map metadata:
     * __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
     * __uint(max_entries, 2);
     * __uint(key_size, sizeof(int));
     * __uint(value_size, sizeof(int));
     * __array(values, struct inner_map);
     */
    pub values: [*mut inner_map; 2],
}

unsafe extern "C" {
    pub fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    pub fn bpf_map_update_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    pub fn bpf_timer_init(timer: *mut bpf_timer, map: *mut bpf_map, clockid: i32) -> i64;
    pub fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        callback: unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut hmap_elem) -> i32,
    ) -> i64;
    pub fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut inner_htab: inner_map = inner_map { _private: [] };

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut outer_arr: outer_arr = outer_arr {
    values: [
        core::ptr::null_mut(),
        unsafe { core::ptr::addr_of_mut!(inner_htab) },
    ],
};

#[unsafe(no_mangle)]
pub static mut err: u64 = 0;
#[unsafe(no_mangle)]
pub static mut ok: u64 = 0;
#[unsafe(no_mangle)]
pub static mut cnt: u64 = 0;

/* callback for inner hash map */
unsafe extern "C" fn timer_cb(
    _map: *mut core::ffi::c_void,
    _key: *mut i32,
    _val: *mut hmap_elem,
) -> i32 {
    return 0;
}

#[unsafe(link_section = "fentry/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(a: i32) -> i32 {
    let _ = a;
    let init: hmap_elem = core::mem::zeroed();
    let mut inner_map: *mut bpf_map;
    let mut inner_map2: *mut bpf_map;
    let mut val: *mut hmap_elem;
    let mut array_key: i32 = ARRAY_KEY;
    let mut array_key2: i32 = ARRAY_KEY2;
    let mut hash_key: i32 = HASH_KEY;

    inner_map = bpf_map_lookup_elem(
        core::ptr::addr_of!(outer_arr) as *const core::ffi::c_void,
        core::ptr::addr_of!(array_key) as *const core::ffi::c_void,
    ) as *mut bpf_map;
    if inner_map.is_null() {
        return 0;
    }

    inner_map2 = bpf_map_lookup_elem(
        core::ptr::addr_of!(outer_arr) as *const core::ffi::c_void,
        core::ptr::addr_of!(array_key2) as *const core::ffi::c_void,
    ) as *mut bpf_map;
    if inner_map2.is_null() {
        return 0;
    }
    bpf_map_update_elem(
        inner_map,
        core::ptr::addr_of!(hash_key) as *const core::ffi::c_void,
        core::ptr::addr_of!(init) as *const core::ffi::c_void,
        0,
    );
    val = bpf_map_lookup_elem(
        inner_map as *const core::ffi::c_void,
        core::ptr::addr_of!(hash_key) as *const core::ffi::c_void,
    ) as *mut hmap_elem;
    if val.is_null() {
        return 0;
    }

    bpf_timer_init(core::ptr::addr_of_mut!((*val).timer), inner_map2, CLOCK_MONOTONIC);
    if bpf_timer_set_callback(core::ptr::addr_of_mut!((*val).timer), timer_cb) != 0 {
        err |= 4;
    }
    if bpf_timer_start(core::ptr::addr_of_mut!((*val).timer), 0, 0) != 0 {
        err |= 8;
    }
    return 0;
}
