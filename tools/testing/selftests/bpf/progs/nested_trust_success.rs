// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies:
 * #include <vmlinux.h>
 * #include <bpf/bpf_tracing.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 * #include "nested_trust_common.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type u64 = ::core::ffi::c_ulonglong;

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub cpus_ptr: *const cpumask,
    pub cpus_mask: cpumask,
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

extern "C" {
    fn bpf_cpumask_test_cpu(cpu: u32, cpumask: *const cpumask) -> i32;
    fn bpf_sk_storage_get(
        map: *mut ::core::ffi::c_void,
        sk: *mut sock,
        value: *mut ::core::ffi::c_void,
        flags: u64,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_cpumask_first_zero(cpumask: *const cpumask) -> u32;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = ".maps"]
pub static mut sk_storage_map: bpf_map_def = bpf_map_def {
    /* BPF_MAP_TYPE_SK_STORAGE */
    type_: 0,
    /* BPF_F_NO_PREALLOC */
    map_flags: 0,
    key_size: ::core::mem::size_of::<::core::ffi::c_int>() as u32,
    value_size: ::core::mem::size_of::<u64>() as u32,
};

/* SEC("tp_btf/task_newtask") */
/* __success */
#[no_mangle]
#[link_section = "tp_btf/task_newtask"]
pub unsafe extern "C" fn test_read_cpumask(task: *mut task_struct, clone_flags: u64) -> i32 {
    let _ = clone_flags;

    bpf_cpumask_test_cpu(0, (*task).cpus_ptr);
    return 0;
}

/* SEC("tp_btf/tcp_probe") */
/* __success */
#[no_mangle]
#[link_section = "tp_btf/tcp_probe"]
pub unsafe extern "C" fn test_skb_field(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let _ = sk;

    bpf_sk_storage_get(
        &mut sk_storage_map as *mut bpf_map_def as *mut ::core::ffi::c_void,
        (*skb).sk,
        0 as *mut ::core::ffi::c_void,
        0,
    );
    return 0;
}

/* SEC("tp_btf/task_newtask") */
/* __success */
#[no_mangle]
#[link_section = "tp_btf/task_newtask"]
pub unsafe extern "C" fn test_nested_offset(task: *mut task_struct, clone_flags: u64) -> i32 {
    let _ = clone_flags;

    bpf_cpumask_first_zero(&(*task).cpus_mask as *const cpumask);
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
