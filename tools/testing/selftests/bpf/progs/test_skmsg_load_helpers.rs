// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Isovalent, Inc.
//
// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;
pub type __u64 = u64;

// External BPF/kernel definitions supplied by included headers in the original C source.
pub const BPF_MAP_TYPE_SOCKMAP: __u32 = 15;
pub const BPF_MAP_TYPE_SOCKHASH: __u32 = 18;
pub const BPF_MAP_TYPE_SK_STORAGE: __u32 = 24;
pub const BPF_F_NO_PREALLOC: __u32 = 1;
pub const BPF_SK_STORAGE_GET_F_CREATE: __u64 = 1;
pub const SK_PASS: i32 = 1;
pub const SK_DROP: i32 = 0;

#[repr(C)]
pub struct sk_msg_md {
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub tgid: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

unsafe extern "C" {
    pub fn bpf_get_current_task() -> *mut core::ffi::c_void;
    pub fn bpf_get_current_pid_tgid() -> __u64;
    pub fn bpf_sk_storage_get(
        map: *mut core::ffi::c_void,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: __u64,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_probe_read_kernel(
        dst: *mut core::ffi::c_void,
        size: __u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    pub fn bpf_sk_storage_delete(
        map: *mut core::ffi::c_void,
        sk: *mut core::ffi::c_void,
    ) -> i64;
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sock_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 2,
    map_flags: 0,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sock_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKHASH,
    max_entries: 2,
    map_flags: 0,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut socket_storage: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    max_entries: 0,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
};

unsafe fn prog_msg_verdict_common(msg: *mut sk_msg_md) -> i32 {
    let task: *mut task_struct = bpf_get_current_task() as *mut task_struct;
    let mut verdict: i32 = SK_PASS;
    let mut tpid: __u32 = 0;

    let pid: __u32 = (bpf_get_current_pid_tgid() >> 32) as __u32;
    let sk_stg: *mut __u64 = bpf_sk_storage_get(
        core::ptr::addr_of_mut!(socket_storage) as *mut core::ffi::c_void,
        (*msg).sk,
        core::ptr::null_mut(),
        BPF_SK_STORAGE_GET_F_CREATE,
    ) as *mut __u64;
    if sk_stg.is_null() {
        return SK_DROP;
    }
    *sk_stg = pid as __u64;
    bpf_probe_read_kernel(
        core::ptr::addr_of_mut!(tpid) as *mut core::ffi::c_void,
        core::mem::size_of_val(&tpid) as __u32,
        core::ptr::addr_of!((*task).tgid) as *const core::ffi::c_void,
    );
    if pid != tpid {
        verdict = SK_DROP;
    }
    bpf_sk_storage_delete(
        core::ptr::addr_of_mut!(socket_storage) as *mut core::ffi::c_void,
        (*msg).sk as *mut core::ffi::c_void,
    );
    verdict
}

#[unsafe(link_section = "sk_msg")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_msg_verdict(msg: *mut sk_msg_md) -> i32 {
    prog_msg_verdict_common(msg)
}

#[unsafe(link_section = "sk_msg")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_msg_verdict_clone(msg: *mut sk_msg_md) -> i32 {
    prog_msg_verdict_common(msg)
}

#[unsafe(link_section = "sk_msg")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_msg_verdict_clone2(msg: *mut sk_msg_md) -> i32 {
    prog_msg_verdict_common(msg)
}

#[unsafe(link_section = "sk_skb/stream_verdict")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_skb_verdict(_skb: *mut __sk_buff) -> i32 {
    SK_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
