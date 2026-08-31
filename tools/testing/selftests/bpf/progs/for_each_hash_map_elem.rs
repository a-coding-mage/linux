// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Dependency intent from C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_MAP_TYPE_PERCPU_HASH: u32 = 5;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

unsafe extern "C" {
    pub fn bpf_map_delete_elem(map: *mut bpf_map, key: *const core::ffi::c_void) -> i64;
    pub fn bpf_get_smp_processor_id() -> __u64;
    pub fn bpf_for_each_map_elem(
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut hashmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 3,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut percpu_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
};

#[repr(C)]
pub struct callback_ctx {
    pub ctx: *mut __sk_buff,
    pub input: core::ffi::c_int,
    pub output: core::ffi::c_int,
}

unsafe extern "C" fn check_hash_elem(
    map: *mut bpf_map,
    key: *mut __u32,
    val: *mut __u64,
    data: *mut callback_ctx,
) -> __u64 {
    let skb: *mut __sk_buff = unsafe { (*data).ctx };
    let k: __u32;
    let v: __u64;

    if !skb.is_null() {
        k = unsafe { *key };
        v = unsafe { *val };
        if unsafe { (*skb).len } == 10000 && k == 10 && v == 10 {
            unsafe {
                (*data).output = 3;
            } /* impossible path */
        } else {
            unsafe {
                (*data).output = 4;
            }
        }
    } else {
        unsafe {
            (*data).output = (*data).input;
            bpf_map_delete_elem(map, key as *const core::ffi::c_void);
        }
    }

    0
}

#[unsafe(no_mangle)]
pub static mut cpu: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut percpu_called: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut percpu_key: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut percpu_val: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut percpu_output: core::ffi::c_int = 0;

unsafe extern "C" fn check_percpu_elem(
    _map: *mut bpf_map,
    key: *mut __u32,
    val: *mut __u64,
    _unused: *mut callback_ctx,
) -> __u64 {
    let mut data: callback_ctx = callback_ctx {
        ctx: core::ptr::null_mut(),
        input: 0,
        output: 0,
    };

    unsafe {
        percpu_called = percpu_called.wrapping_add(1);
        cpu = bpf_get_smp_processor_id() as __u32;
        percpu_key = *key;
        percpu_val = *val;

        data.ctx = core::ptr::null_mut();
        data.input = 100;
        data.output = 0;
        bpf_for_each_map_elem(
            core::ptr::addr_of_mut!(hashmap).cast::<bpf_map>(),
            check_hash_elem,
            (&mut data as *mut callback_ctx).cast::<core::ffi::c_void>(),
            0,
        );
        percpu_output = data.output;
    }

    0
}

#[unsafe(no_mangle)]
pub static mut hashmap_output: core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut hashmap_elems: core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut percpu_map_elems: core::ffi::c_int = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn test_pkt_access(skb: *mut __sk_buff) -> core::ffi::c_int {
    let mut data: callback_ctx = callback_ctx {
        ctx: core::ptr::null_mut(),
        input: 0,
        output: 0,
    };

    unsafe {
        data.ctx = skb;
        data.input = 10;
        data.output = 0;
        hashmap_elems = bpf_for_each_map_elem(
            core::ptr::addr_of_mut!(hashmap).cast::<bpf_map>(),
            check_hash_elem,
            (&mut data as *mut callback_ctx).cast::<core::ffi::c_void>(),
            0,
        ) as core::ffi::c_int;
        hashmap_output = data.output;

        percpu_map_elems = bpf_for_each_map_elem(
            core::ptr::addr_of_mut!(percpu_map).cast::<bpf_map>(),
            check_percpu_elem,
            core::ptr::null_mut(),
            0,
        ) as core::ffi::c_int;
    }
    0
}
