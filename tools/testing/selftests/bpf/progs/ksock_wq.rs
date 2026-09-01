// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Isovalent */

// C dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf_experimental.h,
// bpf_tracing_net.h, errno.h, ksock_common.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type u32 = ::core::ffi::c_uint;

extern "C" {
    static mut AF_INET: ::core::ffi::c_int;
    static mut SOCK_DGRAM: ::core::ffi::c_int;
    static mut IPPROTO_UDP: ::core::ffi::c_int;
    static mut ENOENT: ::core::ffi::c_int;

    fn bpf_ksock_create(
        opts: *mut bpf_ksock_create_opts,
        opts__sz: ::core::ffi::c_ulong,
        err: *mut ::core::ffi::c_int,
    ) -> *mut bpf_ksock;
    fn bpf_ksock_release(ks: *mut bpf_ksock);
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_wq_init(wq: *mut bpf_wq, map: *mut c_void, flags: ::core::ffi::c_ulonglong)
        -> ::core::ffi::c_int;
    fn bpf_wq_set_callback(
        wq: *mut bpf_wq,
        callback: unsafe extern "C" fn(
            map: *mut c_void,
            key: *mut ::core::ffi::c_int,
            value: *mut c_void,
        ) -> ::core::ffi::c_int,
        flags: ::core::ffi::c_ulonglong,
    ) -> ::core::ffi::c_int;
    fn bpf_wq_start(wq: *mut bpf_wq, flags: ::core::ffi::c_ulonglong) -> ::core::ffi::c_int;
}

#[repr(C)]
pub struct bpf_wq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_ksock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_ksock_create_opts {
    pub family: ::core::ffi::c_int,
    pub type_: ::core::ffi::c_int,
    pub protocol: ::core::ffi::c_int,
}

#[repr(C)]
pub struct ksock_wq_value {
    pub work: bpf_wq,
}

// Original C declaration used BPF map-definition macros:
// __uint(type, BPF_MAP_TYPE_ARRAY);
// __uint(max_entries, 1);
// __type(key, u32);
// __type(value, struct ksock_wq_value);
#[no_mangle]
#[link_section = ".maps"]
pub static mut work_map: ksock_wq_value = ksock_wq_value {
    work: bpf_wq { _private: [] },
};

#[no_mangle]
pub static mut create_err: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut callback_done: u32 = 0;

unsafe extern "C" fn ksock_wq_callback(
    _map: *mut c_void,
    _key: *mut ::core::ffi::c_int,
    _value: *mut c_void,
) -> ::core::ffi::c_int {
    let mut opts: bpf_ksock_create_opts = bpf_ksock_create_opts {
        family: AF_INET,
        type_: SOCK_DGRAM,
        protocol: IPPROTO_UDP,
    };
    let mut err: ::core::ffi::c_int = 0;

    let ks: *mut bpf_ksock = bpf_ksock_create(
        &mut opts,
        ::core::mem::size_of_val(&opts) as ::core::ffi::c_ulong,
        &mut err,
    );
    if !ks.is_null() {
        bpf_ksock_release(ks);
    }
    create_err = err;
    core::intrinsics::atomic_xadd_seqcst(&mut callback_done, 1);
    0
}

#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn ksock_wq_start(_ctx: *mut c_void) -> ::core::ffi::c_int {
    let mut key: u32 = 0;
    let mut err: ::core::ffi::c_int;

    let value: *mut ksock_wq_value = bpf_map_lookup_elem(
        &mut work_map as *mut ksock_wq_value as *mut c_void,
        &mut key as *mut u32 as *const c_void,
    ) as *mut ksock_wq_value;
    if value.is_null() {
        return -ENOENT;
    }
    err = bpf_wq_init(
        &mut (*value).work,
        &mut work_map as *mut ksock_wq_value as *mut c_void,
        0,
    );
    if err != 0 {
        return err;
    }
    err = bpf_wq_set_callback(&mut (*value).work, ksock_wq_callback, 0);
    if err != 0 {
        return err;
    }
    bpf_wq_start(&mut (*value).work, 0)
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
