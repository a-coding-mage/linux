// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Intel Corporation */
/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub type __u64 = u64;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

pub const BPF_MAP_TYPE_HASH: u32 = 1;

#[used]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct hashmap_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[used]
#[unsafe(link_section = ".maps")]
pub static mut hashmap: hashmap_def = hashmap_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 128,
    key_size: core::mem::size_of::<__u64>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

unsafe extern "C" {
    pub fn bpf_map_delete_elem(map: *mut bpf_map, key: *const core::ffi::c_void) -> i64;
    pub fn bpf_map_update_elem(
        map: *mut bpf_map,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    pub fn bpf_for_each_map_elem(
        map: *mut core::ffi::c_void,
        callback: Option<
            unsafe extern "C" fn(
                map: *mut bpf_map,
                key: *mut __u64,
                val: *mut __u64,
                arg: *mut core::ffi::c_void,
            ) -> i32,
        >,
        callback_ctx: *mut core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

unsafe extern "C" fn cb(
    map: *mut bpf_map,
    key: *mut __u64,
    val: *mut __u64,
    arg: *mut core::ffi::c_void,
) -> i32 {
    let _ = arg;

    unsafe {
        bpf_map_delete_elem(map, key as *const core::ffi::c_void);
        bpf_map_update_elem(
            map,
            key as *const core::ffi::c_void,
            val as *const core::ffi::c_void,
            0,
        );
    }
    0
}

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_pkt_access(skb: *mut __sk_buff) -> i32 {
    let _ = skb;

    unsafe {
        bpf_for_each_map_elem(
            &raw mut hashmap as *mut core::ffi::c_void,
            Some(cb),
            core::ptr::null_mut(),
            0,
        );
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
