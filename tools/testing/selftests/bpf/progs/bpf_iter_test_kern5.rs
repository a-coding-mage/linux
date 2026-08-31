// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_HASH: u32 = 1;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct key_t {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub key: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct hashmap1_def {
    // Original C BPF map declaration:
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __uint(max_entries, 3);
    // __type(key, struct key_t);
    // __type(value, __u64);
    pub type_: *mut [u32; BPF_MAP_TYPE_HASH as usize],
    pub max_entries: *mut [u32; 3],
    pub key: *mut key_t,
    pub value: *mut __u64,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut hashmap1: hashmap1_def = hashmap1_def {
    type_: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

#[no_mangle]
pub static mut key_sum: __u32 = 0;

#[no_mangle]
#[link_section = "iter/bpf_map_elem"]
pub unsafe extern "C" fn dump_bpf_hash_map(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let key: *mut core::ffi::c_void = unsafe { (*ctx).key };

    if key == core::ptr::null_mut() {
        return 0;
    }

    /* out of bound access w.r.t. hashmap1 */
    unsafe {
        key_sum = key_sum.wrapping_add(
            *(key.cast::<u8>().add(core::mem::size_of::<key_t>()).cast::<__u32>()),
        );
    }
    return 0;
}
