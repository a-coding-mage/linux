// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type __u32 = u32;
type __u64 = u64;
type u32 = u32;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
struct arraymap_def {
    type_: u32,
    max_entries: u32,
    key_size: u32,
    value_size: u32,
}

#[repr(C)]
struct callback_ctx {
    output: i32,
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
static mut arraymap: arraymap_def = arraymap_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 3,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
static mut percpu_map: arraymap_def = arraymap_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> __u64;
    fn bpf_for_each_map_elem(
        map: *mut bpf_map,
        callback: unsafe extern "C" fn(
            map: *mut bpf_map,
            key: *mut __u32,
            val: *mut __u64,
            data: *mut callback_ctx,
        ) -> __u64,
        callback_ctx: *mut core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

static bypass_unused: i32 = 1;

unsafe extern "C" fn unused_subprog(
    map: *mut bpf_map,
    key: *mut __u32,
    val: *mut __u64,
    data: *mut callback_ctx,
) -> __u64 {
    unsafe {
        (*data).output = 0;
    }
    1
}

unsafe extern "C" fn check_array_elem(
    map: *mut bpf_map,
    key: *mut __u32,
    val: *mut __u64,
    data: *mut callback_ctx,
) -> __u64 {
    unsafe {
        (*data).output += *val as i32;
        if *key == 1 {
            return 1; /* stop the iteration */
        }
    }
    0
}

static mut cpu: __u32 = 0;
static mut percpu_val: __u64 = 0;

unsafe extern "C" fn check_percpu_elem(
    map: *mut bpf_map,
    key: *mut __u32,
    val: *mut __u64,
    data: *mut callback_ctx,
) -> __u64 {
    unsafe {
        cpu = bpf_get_smp_processor_id() as __u32;
        percpu_val = *val;
    }
    0
}

static mut arraymap_output: u32 = 0;

#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_pkt_access(skb: *mut __sk_buff) -> i32 {
    let mut data: callback_ctx = callback_ctx { output: 0 };

    unsafe {
        data.output = 0;
        bpf_for_each_map_elem(
            core::ptr::addr_of_mut!(arraymap).cast::<bpf_map>(),
            check_array_elem,
            core::ptr::addr_of_mut!(data).cast::<core::ffi::c_void>(),
            0,
        );
        if bypass_unused == 0 {
            bpf_for_each_map_elem(
                core::ptr::addr_of_mut!(arraymap).cast::<bpf_map>(),
                unused_subprog,
                core::ptr::addr_of_mut!(data).cast::<core::ffi::c_void>(),
                0,
            );
        }
        arraymap_output = data.output as u32;

        bpf_for_each_map_elem(
            core::ptr::addr_of_mut!(percpu_map).cast::<bpf_map>(),
            check_percpu_elem,
            core::ptr::null_mut(),
            0,
        );
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
