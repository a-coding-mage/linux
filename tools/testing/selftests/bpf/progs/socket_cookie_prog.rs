// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// C dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_endian.h,
// bpf/bpf_tracing.h.

const AF_INET6: u32 = 10;

#[repr(C)]
pub struct socket_cookie {
    pub cookie_key: __u64,
    pub cookie_value: __u32,
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, struct socket_cookie);
// } socket_cookies SEC(".maps");
#[no_mangle]
#[link_section = ".maps"]
pub static mut socket_cookies: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<socket_cookie>() as __u32,
    max_entries: 0,
    map_flags: BPF_F_NO_PREALLOC,
};

/*
 * These three programs get executed in a row on connect() syscalls. The
 * userspace side of the test creates a client socket, issues a connect() on it
 * and then checks that the local storage associated with this socket has:
 * cookie_value == local_port << 8 | 0xFF
 * The different parts of this cookie_value are appended by those hooks if they
 * all agree on the output of bpf_get_socket_cookie().
 */
#[no_mangle]
#[link_section = "cgroup/connect6"]
pub unsafe extern "C" fn set_cookie(ctx: *mut bpf_sock_addr) -> i32 {
    let mut p: *mut socket_cookie;

    if (*ctx).family != AF_INET6 || (*ctx).user_family != AF_INET6 {
        return 1;
    }

    p = bpf_sk_storage_get(
        &raw mut socket_cookies as *mut _ as *mut core::ffi::c_void,
        (*ctx).sk as *mut core::ffi::c_void,
        0 as *mut core::ffi::c_void,
        BPF_SK_STORAGE_GET_F_CREATE,
    ) as *mut socket_cookie;
    if p.is_null() {
        return 1;
    }

    (*p).cookie_value = 0xF;
    (*p).cookie_key = bpf_get_socket_cookie(ctx as *mut core::ffi::c_void);

    1
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn update_cookie_sockops(ctx: *mut bpf_sock_ops) -> i32 {
    let sk: *mut bpf_sock = (*ctx).sk;
    let mut p: *mut socket_cookie;

    if (*ctx).family != AF_INET6 {
        return 1;
    }

    if (*ctx).op != BPF_SOCK_OPS_TCP_CONNECT_CB {
        return 1;
    }

    if sk.is_null() {
        return 1;
    }

    p = bpf_sk_storage_get(
        &raw mut socket_cookies as *mut _ as *mut core::ffi::c_void,
        sk as *mut core::ffi::c_void,
        0 as *mut core::ffi::c_void,
        0,
    ) as *mut socket_cookie;
    if p.is_null() {
        return 1;
    }

    if (*p).cookie_key != bpf_get_socket_cookie(ctx as *mut core::ffi::c_void) {
        return 1;
    }

    (*p).cookie_value |= (*ctx).local_port << 8;

    1
}

#[no_mangle]
#[link_section = "fexit/inet_stream_connect"]
pub unsafe extern "C" fn update_cookie_tracing(
    sock: *mut socket,
    uaddr: *mut sockaddr,
    addr_len: i32,
    flags: i32,
) -> i32 {
    let mut p: *mut socket_cookie;

    let _ = addr_len;
    let _ = flags;

    if (*uaddr).sa_family as u32 != AF_INET6 {
        return 0;
    }

    p = bpf_sk_storage_get(
        &raw mut socket_cookies as *mut _ as *mut core::ffi::c_void,
        (*sock).sk as *mut core::ffi::c_void,
        0 as *mut core::ffi::c_void,
        0,
    ) as *mut socket_cookie;
    if p.is_null() {
        return 0;
    }

    if (*p).cookie_key != bpf_get_socket_cookie((*sock).sk as *mut core::ffi::c_void) {
        return 0;
    }

    (*p).cookie_value |= 0xF0;

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_sk_storage_get(
        map: *mut core::ffi::c_void,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: __u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_get_socket_cookie(ctx: *mut core::ffi::c_void) -> __u64;
}

pub type __u32 = u32;
pub type __u64 = u64;

extern "C" {
    pub type bpf_sock_addr;
    pub type bpf_sock;
    pub type bpf_sock_ops;
    pub type socket;
    pub type sockaddr;
    pub type bpf_map_def;
}

extern "C" {
    static BPF_MAP_TYPE_SK_STORAGE: __u32;
    static BPF_F_NO_PREALLOC: __u32;
    static BPF_SK_STORAGE_GET_F_CREATE: __u64;
    static BPF_SOCK_OPS_TCP_CONNECT_CB: __u32;
}
