// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/*
 * Translated from C BPF source. Original includes:
 * "vmlinux.h", "bpf_tracing_net.h", <bpf/bpf_tracing.h>,
 * <bpf/bpf_helpers.h>.
 */

pub type __u8 = u8;
pub type __u32 = u32;

pub const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
pub const BPF_MAP_TYPE_TASK_STORAGE: u32 = 29;
pub const BPF_F_NO_PREALLOC: u32 = 1;
pub const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;

#[repr(C)]
pub struct task_struct {
    pub tgid: __u32,
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[no_mangle]
pub static mut create_errs: i64 = 0;

#[no_mangle]
pub static mut create_cnts: i64 = 0;

#[no_mangle]
pub static mut bench_pid: __u32 = 0;

#[repr(C)]
pub struct storage {
    pub data: [__u8; 64],
}

#[repr(C)]
pub struct bpf_map_def_storage {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
}

/* SEC(".maps") */
#[no_mangle]
#[link_section = ".maps"]
pub static sk_storage_map: bpf_map_def_storage = bpf_map_def_storage {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<storage>() as u32,
};

/* SEC(".maps") */
#[no_mangle]
#[link_section = ".maps"]
pub static task_storage_map: bpf_map_def_storage = bpf_map_def_storage {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<storage>() as u32,
};

extern "C" {
    pub fn bpf_task_storage_get(
        map: *const bpf_map_def_storage,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut storage;

    pub fn bpf_sk_storage_get(
        map: *const bpf_map_def_storage,
        sk: *mut sock,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut storage;

    pub fn bpf_get_current_pid_tgid() -> u64;
}

/* SEC("tp_btf/sched_process_fork") */
#[no_mangle]
#[link_section = "tp_btf/sched_process_fork"]
pub unsafe extern "C" fn sched_process_fork(
    parent: *mut task_struct,
    child: *mut task_struct,
) -> i32 {
    let stg: *mut storage;

    if (*parent).tgid != bench_pid {
        return 0;
    }

    stg = bpf_task_storage_get(
        &task_storage_map,
        child,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if !stg.is_null() {
        core::intrinsics::atomic_xadd_seqcst(&mut create_cnts, 1);
    } else {
        core::intrinsics::atomic_xadd_seqcst(&mut create_errs, 1);
    }

    0
}

/* SEC("lsm.s/socket_post_create") */
#[no_mangle]
#[link_section = "lsm.s/socket_post_create"]
pub unsafe extern "C" fn socket_post_create(
    sock: *mut socket,
    family: i32,
    type_: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let sk: *mut sock = (*sock).sk;
    let stg: *mut storage;
    let pid: __u32;

    let _ = family;
    let _ = type_;
    let _ = protocol;
    let _ = kern;

    pid = (bpf_get_current_pid_tgid() >> 32) as __u32;
    if pid != bench_pid || sk.is_null() {
        return 0;
    }

    stg = bpf_sk_storage_get(
        &sk_storage_map,
        sk,
        core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );

    if !stg.is_null() {
        core::intrinsics::atomic_xadd_seqcst(&mut create_cnts, 1);
    } else {
        core::intrinsics::atomic_xadd_seqcst(&mut create_errs, 1);
    }

    0
}

/* SEC("license") */
#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
