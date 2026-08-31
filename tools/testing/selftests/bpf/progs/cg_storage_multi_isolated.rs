// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2020 Google LLC.
 */

// C source dependencies:
// #include <errno.h>
// #include <linux/bpf.h>
// #include <linux/ip.h>
// #include <linux/udp.h>
// #include <bpf/bpf_helpers.h>
// #include "progs/cg_storage_multi.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

type __u32 = u32;
type __u64 = u64;

// External kernel/BPF types and constants are supplied by included headers in C.
#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

// Provided by "progs/cg_storage_multi.h".
#[repr(C)]
pub struct cgroup_value {
    pub egress_pkts: __u64,
    pub ingress_pkts: __u64,
}

const BPF_MAP_TYPE_CGROUP_STORAGE: __u32 = 19;

#[repr(C)]
pub struct cgroup_storage_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut cgroup_storage: cgroup_storage_map_def = cgroup_storage_map_def {
    type_: BPF_MAP_TYPE_CGROUP_STORAGE,
    key_size: core::mem::size_of::<bpf_cgroup_storage_key>() as __u32,
    value_size: core::mem::size_of::<cgroup_value>() as __u32,
    max_entries: 0,
};

#[no_mangle]
pub static mut invocations: __u32 = 0;

extern "C" {
    fn bpf_get_local_storage(map: *mut c_void, flags: __u64) -> *mut cgroup_value;
}

#[inline(always)]
unsafe fn sync_fetch_and_add_u64(ptr: *mut __u64, value: __u64) -> __u64 {
    AtomicU64::from_ptr(ptr).fetch_add(value, Ordering::SeqCst)
}

#[inline(always)]
unsafe fn sync_fetch_and_add_u32(ptr: *mut __u32, value: __u32) -> __u32 {
    AtomicU32::from_ptr(ptr).fetch_add(value, Ordering::SeqCst)
}

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn egress1(skb: *mut __sk_buff) -> i32 {
    let ptr_cg_storage: *mut cgroup_value =
        bpf_get_local_storage(&mut cgroup_storage as *mut _ as *mut c_void, 0);

    sync_fetch_and_add_u64(&mut (*ptr_cg_storage).egress_pkts, 1);
    sync_fetch_and_add_u32(&mut invocations, 1);

    1
}

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn egress2(skb: *mut __sk_buff) -> i32 {
    let ptr_cg_storage: *mut cgroup_value =
        bpf_get_local_storage(&mut cgroup_storage as *mut _ as *mut c_void, 0);

    sync_fetch_and_add_u64(&mut (*ptr_cg_storage).egress_pkts, 1);
    sync_fetch_and_add_u32(&mut invocations, 1);

    1
}

#[link_section = "cgroup_skb/ingress"]
#[no_mangle]
pub unsafe extern "C" fn ingress(skb: *mut __sk_buff) -> i32 {
    let ptr_cg_storage: *mut cgroup_value =
        bpf_get_local_storage(&mut cgroup_storage as *mut _ as *mut c_void, 0);

    sync_fetch_and_add_u64(&mut (*ptr_cg_storage).ingress_pkts, 1);
    sync_fetch_and_add_u32(&mut invocations, 1);

    1
}
