// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

const BPF_ANY: u64 = 0;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct bigelement {
    pub a: i32,
    pub b: [i8; 4096],
    pub c: i64,
}

/*
 * SEC(".maps")
 *
 * struct {
 *      __uint(type, BPF_MAP_TYPE_HASH);
 *      __uint(max_entries, 2);
 *      __type(key, struct bigelement);
 *      __type(value, __u32);
 * } hash_map;
 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut hash_map: [u8; 0] = [];

/*
 * SEC(".maps")
 *
 * struct {
 *      __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
 *      __uint(max_entries, 1);
 *      __type(key, __u32);
 *      __type(value, struct bigelement);
 * } key_map;
 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut key_map: [u8; 0] = [];

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn bpf_hash_large_key_test(ctx: *mut core::ffi::c_void) -> i32 {
    let mut zero: i32 = 0;
    let mut value: i32 = 42;
    let mut key: *mut bigelement;

    key = bpf_map_lookup_elem(
        &mut hash_map as *mut _ as *mut core::ffi::c_void,
        &mut zero as *mut _ as *const core::ffi::c_void,
    ) as *mut bigelement;
    if key.is_null() {
        return 0;
    }

    (*key).c = 1;
    if bpf_map_update_elem(
        &mut hash_map as *mut _ as *mut core::ffi::c_void,
        key as *const core::ffi::c_void,
        &mut value as *mut _ as *const core::ffi::c_void,
        BPF_ANY,
    ) != 0
    {
        return 0;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
