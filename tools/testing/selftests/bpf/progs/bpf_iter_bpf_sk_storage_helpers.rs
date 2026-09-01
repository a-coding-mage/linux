// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Google LLC. */
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;

pub const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
pub const BPF_F_NO_PREALLOC: u32 = 1;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct task_struct {
    pub tgid: i32,
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock_common {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct bpf_iter__bpf_sk_storage_map {
    pub sk: *mut sock_common,
}

#[repr(C)]
pub struct bpf_iter__task_file {
    pub task: *mut task_struct,
    pub file: *mut file,
}

#[repr(C)]
pub struct bpf_iter__tcp {
    pub sk_common: *mut sock_common,
}

#[repr(C)]
pub struct sk_stg_map_def {
    // __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    pub type_: u32,
    // __uint(map_flags, BPF_F_NO_PREALLOC);
    pub map_flags: u32,
    // __type(key, int);
    // __type(value, int);
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut sk_stg_map: sk_stg_map_def = sk_stg_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
};

extern "C" {
    pub fn bpf_sk_storage_delete(map: *mut c_void, sk: *mut c_void) -> i64;
    pub fn bpf_sock_from_file(file: *mut file) -> *mut socket;
    pub fn bpf_sk_storage_get(
        map: *mut c_void,
        sk: *mut c_void,
        value: *mut c_void,
        flags: u64,
    ) -> *mut c_void;
}

#[link_section = "iter/bpf_sk_storage_map"]
#[no_mangle]
pub unsafe extern "C" fn delete_bpf_sk_storage_map(
    ctx: *mut bpf_iter__bpf_sk_storage_map,
) -> i32 {
    if !(*ctx).sk.is_null() {
        bpf_sk_storage_delete(
            &mut sk_stg_map as *mut sk_stg_map_def as *mut c_void,
            (*ctx).sk as *mut c_void,
        );
    }

    0
}

#[link_section = "iter/task_file"]
#[no_mangle]
pub unsafe extern "C" fn fill_socket_owner(ctx: *mut bpf_iter__task_file) -> i32 {
    let task: *mut task_struct = (*ctx).task;
    let file: *mut file = (*ctx).file;
    let sock: *mut socket;
    let sock_tgid: *mut i32;

    if task.is_null() || file.is_null() {
        return 0;
    }

    sock = bpf_sock_from_file(file);
    if sock.is_null() {
        return 0;
    }

    sock_tgid = bpf_sk_storage_get(
        &mut sk_stg_map as *mut sk_stg_map_def as *mut c_void,
        (*sock).sk as *mut c_void,
        core::ptr::null_mut(),
        0,
    ) as *mut i32;
    if sock_tgid.is_null() {
        return 0;
    }

    *sock_tgid = (*task).tgid;

    0
}

#[link_section = "iter/tcp"]
#[no_mangle]
pub unsafe extern "C" fn negate_socket_local_storage(ctx: *mut bpf_iter__tcp) -> i32 {
    let sk_common: *mut sock_common = (*ctx).sk_common;
    let sock_tgid: *mut i32;

    if sk_common.is_null() {
        return 0;
    }

    sock_tgid = bpf_sk_storage_get(
        &mut sk_stg_map as *mut sk_stg_map_def as *mut c_void,
        sk_common as *mut c_void,
        core::ptr::null_mut(),
        0,
    ) as *mut i32;
    if sock_tgid.is_null() {
        return 0;
    }

    *sock_tgid = -*sock_tgid;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
