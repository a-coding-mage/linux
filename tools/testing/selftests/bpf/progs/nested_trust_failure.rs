// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include "nested_trust_common.h"

extern "C" {
    fn bpf_cpumask_test_cpu(cpu: u32, cpumask: *const core::ffi::c_void) -> i32;
    fn bpf_sk_storage_get(
        map: *mut core::ffi::c_void,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct task_struct {
    pub user_cpus_ptr: *const core::ffi::c_void,
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    pub next: *mut core::ffi::c_void,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct sk_storage_map_def {
    // __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    // __uint(map_flags, BPF_F_NO_PREALLOC);
    // __type(key, int);
    // __type(value, u64);
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut sk_storage_map: sk_storage_map_def = sk_storage_map_def {};

/* Prototype for all of the program trace events below:
 *
 * TRACE_EVENT(task_newtask,
 *         TP_PROTO(struct task_struct *p, u64 clone_flags)
 */

// SEC("tp_btf/task_newtask")
// __failure __msg("R2 must be")
#[no_mangle]
#[link_section = "tp_btf/task_newtask"]
pub unsafe extern "C" fn test_invalid_nested_user_cpus(
    task: *mut task_struct,
    clone_flags: u64,
) -> i32 {
    let _ = clone_flags;
    bpf_cpumask_test_cpu(0, (*task).user_cpus_ptr);
    0
}

/* Although R2 is of type sk_buff but sock_common is expected, we will hit untrusted ptr first. */
// SEC("tp_btf/tcp_probe")
// __failure __msg("R2 type=untrusted_ptr_ expected=ptr_, trusted_ptr_, rcu_ptr_")
#[no_mangle]
#[link_section = "tp_btf/tcp_probe"]
pub unsafe extern "C" fn test_invalid_skb_field(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let _ = sk;
    bpf_sk_storage_get(
        &mut sk_storage_map as *mut sk_storage_map_def as *mut core::ffi::c_void,
        (*skb).next,
        core::ptr::null_mut(),
        0,
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
