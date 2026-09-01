// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Facebook */

/*
 * C dependencies removed from executable Rust:
 * - "vmlinux.h"
 * - "bpf_tracing_net.h"
 * - <bpf/bpf_helpers.h>
 * - <bpf/bpf_tracing.h>
 * - <bpf/bpf_core_read.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};
use core::ptr;

pub type __u32 = u32;
pub type __u64 = u64;

extern "C" {
    static BPF_MAP_TYPE_SK_STORAGE: c_int;
    static BPF_F_NO_PREALLOC: c_int;

    fn bpf_sk_storage_get(
        map: *mut c_void,
        sk: *mut sock,
        value: *mut c_void,
        flags: __u64,
    ) -> *mut c_int;
}

#[repr(C)]
pub struct atomic64_t {
    pub counter: __u64,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: __u32,
}

#[repr(C)]
pub struct sock {
    pub sk_cookie: atomic64_t,
    pub sk_omem_alloc: atomic_t,
}

#[no_mangle]
pub static mut sk_ptr: *mut c_void = ptr::null_mut();

#[no_mangle]
pub static mut cookie_found: c_int = 0;

#[no_mangle]
pub static mut cookie: __u64 = 0;

#[no_mangle]
pub static mut omem: __u32 = 0;

/*
 * Original BPF map declaration:
 *
 * struct {
 *     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __type(key, int);
 *     __type(value, int);
 * } sk_storage SEC(".maps");
 *
 * The libbpf declaration macros encode map metadata in C type information.
 */
#[repr(C)]
pub struct sk_storage_map {
    pub r#type: *mut c_void,
    pub map_flags: *mut c_void,
    pub key: *mut c_int,
    pub value: *mut c_int,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut sk_storage: sk_storage_map = sk_storage_map {
    r#type: ptr::null_mut(),
    map_flags: ptr::null_mut(),
    key: ptr::null_mut(),
    value: ptr::null_mut(),
};

#[no_mangle]
#[link_section = "fexit/bpf_sk_storage_free"]
pub unsafe extern "C" fn bpf_sk_storage_free(sk: *mut sock) -> c_int {
    if sk_ptr != sk as *mut c_void {
        return 0;
    }

    if (*sk).sk_cookie.counter != cookie {
        return 0;
    }

    cookie_found += 1;
    omem = (*sk).sk_omem_alloc.counter;

    return 0;
}

#[no_mangle]
#[link_section = "fentry/inet6_sock_destruct"]
pub unsafe extern "C" fn inet6_sock_destruct(sk: *mut sock) -> c_int {
    let value: *mut c_int;

    if cookie == 0 || (*sk).sk_cookie.counter != cookie {
        return 0;
    }

    value = bpf_sk_storage_get(
        &mut sk_storage as *mut sk_storage_map as *mut c_void,
        sk,
        ptr::null_mut(),
        0,
    );
    if !value.is_null() && *value == 0xdeadbeef_u32 as c_int {
        cookie_found += 1;
        sk_ptr = sk as *mut c_void;
    }

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
