// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_ANY: __u64 = 0;

#[repr(C)]
pub struct bpf_map_def_arraymap1 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_map_def_hashmap1 {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub meta: *mut bpf_iter_meta,
    pub map: *mut core::ffi::c_void,
    pub key: *mut __u32,
    pub value: *mut __u64,
}

extern "C" {
    fn bpf_seq_write(seq: *mut core::ffi::c_void, data: *const core::ffi::c_void, len: __u32) -> i64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = ".maps"]
pub static mut arraymap1: bpf_map_def_arraymap1 = bpf_map_def_arraymap1 {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 3,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut hashmap1: bpf_map_def_hashmap1 = bpf_map_def_hashmap1 {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 10,
    key_size: core::mem::size_of::<__u64>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
pub static mut key_sum: __u32 = 0;
#[no_mangle]
pub static mut val_sum: __u64 = 0;

#[no_mangle]
#[link_section = "iter/bpf_map_elem"]
pub unsafe extern "C" fn dump_bpf_array_map(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let mut hmap_val: *mut __u32;
    let key: *mut __u32 = (*ctx).key;
    let val: *mut __u64 = (*ctx).value;

    if key == core::ptr::null_mut() || val == core::ptr::null_mut() {
        return 0;
    }

    bpf_seq_write(
        (*(*ctx).meta).seq,
        key as *const core::ffi::c_void,
        core::mem::size_of::<__u32>() as __u32,
    );
    bpf_seq_write(
        (*(*ctx).meta).seq,
        val as *const core::ffi::c_void,
        core::mem::size_of::<__u64>() as __u32,
    );
    key_sum = key_sum.wrapping_add(*key);
    val_sum = val_sum.wrapping_add(*val);

    /* workaround - It's necessary to do this convoluted (val, key)
     * write into hashmap1, instead of simply doing
     *   bpf_map_update_elem(&hashmap1, val, key, BPF_ANY);
     * because key has MEM_RDONLY flag and bpf_map_update elem expects
     * types without this flag
     */
    bpf_map_update_elem(
        core::ptr::addr_of_mut!(hashmap1) as *mut core::ffi::c_void,
        val as *const core::ffi::c_void,
        val as *const core::ffi::c_void,
        BPF_ANY,
    );
    hmap_val = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(hashmap1) as *mut core::ffi::c_void,
        val as *const core::ffi::c_void,
    ) as *mut __u32;
    if !hmap_val.is_null() {
        *hmap_val = *key;
    }

    *val = (*key) as __u64;
    return 0;
}
