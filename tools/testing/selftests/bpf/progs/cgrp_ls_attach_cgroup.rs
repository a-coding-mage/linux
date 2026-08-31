// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// Dependencies from the original C source:
// vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h, bpf_tracing_net.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type __u64 = u64;

const AF_INET6: u32 = 10;
const BPF_MAP_TYPE_CGRP_STORAGE: u32 = 19;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;
const BPF_SOCK_OPS_TCP_CONNECT_CB: u32 = 3;

#[repr(C)]
pub struct socket_cookie {
    pub cookie_key: __u64,
    pub cookie_value: __u64,
}

#[repr(C)]
pub struct bpf_sock_addr {
    pub family: u32,
    pub user_family: u32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: u32,
    pub family: u32,
    pub local_port: u32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tcp_sock {
    pub inet_conn: inet_connection_sock,
}

#[repr(C)]
pub struct inet_connection_sock {
    pub icsk_inet: inet_sock,
}

#[repr(C)]
pub struct inet_sock {
    pub sk: sock,
}

#[repr(C)]
pub struct sock {
    pub sk_cgrp_data: sock_cgroup_data,
}

#[repr(C)]
pub struct sock_cgroup_data {
    pub cgroup: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
}

#[repr(C)]
pub struct socket_cookies_map {
    _private: [u8; 0],
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// Original C map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_CGRP_STORAGE);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
//     __type(key, int);
//     __type(value, struct socket_cookie);
// } socket_cookies SEC(".maps");
//
// SEC(".maps")
#[no_mangle]
pub static mut socket_cookies: socket_cookies_map = socket_cookies_map { _private: [] };

extern "C" {
    fn bpf_skc_to_tcp_sock(sk: *mut bpf_sock) -> *mut tcp_sock;
    fn bpf_cgrp_storage_get(
        map: *mut socket_cookies_map,
        cgroup: *mut core::ffi::c_void,
        value: u64,
        flags: u64,
    ) -> *mut socket_cookie;
    fn bpf_get_socket_cookie(ctx: *mut core::ffi::c_void) -> __u64;
}

// SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn set_cookie(ctx: *mut bpf_sock_addr) -> i32 {
    let mut p: *mut socket_cookie;
    let mut tcp_sk: *mut tcp_sock;
    let mut sk: *mut bpf_sock;

    if (*ctx).family != AF_INET6 || (*ctx).user_family != AF_INET6 {
        return 1;
    }

    sk = (*ctx).sk;
    if sk.is_null() {
        return 1;
    }

    tcp_sk = bpf_skc_to_tcp_sock(sk);
    if tcp_sk.is_null() {
        return 1;
    }

    p = bpf_cgrp_storage_get(
        &mut socket_cookies,
        (*tcp_sk).inet_conn.icsk_inet.sk.sk_cgrp_data.cgroup,
        0,
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    );
    if p.is_null() {
        return 1;
    }

    (*p).cookie_value = 0xF;
    (*p).cookie_key = bpf_get_socket_cookie(ctx as *mut core::ffi::c_void);
    return 1;
}

// SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn update_cookie_sockops(ctx: *mut bpf_sock_ops) -> i32 {
    let mut p: *mut socket_cookie;
    let mut tcp_sk: *mut tcp_sock;
    let mut sk: *mut bpf_sock;

    if (*ctx).family != AF_INET6 || (*ctx).op != BPF_SOCK_OPS_TCP_CONNECT_CB {
        return 1;
    }

    sk = (*ctx).sk;
    if sk.is_null() {
        return 1;
    }

    tcp_sk = bpf_skc_to_tcp_sock(sk);
    if tcp_sk.is_null() {
        return 1;
    }

    p = bpf_cgrp_storage_get(
        &mut socket_cookies,
        (*tcp_sk).inet_conn.icsk_inet.sk.sk_cgrp_data.cgroup,
        0,
        0,
    );
    if p.is_null() {
        return 1;
    }

    if (*p).cookie_key != bpf_get_socket_cookie(ctx as *mut core::ffi::c_void) {
        return 1;
    }

    (*p).cookie_value |= ((*ctx).local_port << 8) as __u64;
    return 1;
}

// SEC("fexit/inet_stream_connect")
// Original C used BPF_PROG(update_cookie_tracing, struct socket *sock,
//                         struct sockaddr *uaddr, int addr_len, int flags)
#[no_mangle]
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

    p = bpf_cgrp_storage_get(
        &mut socket_cookies,
        (*(*sock).sk).sk_cgrp_data.cgroup,
        0,
        0,
    );
    if p.is_null() {
        return 0;
    }

    if (*p).cookie_key != bpf_get_socket_cookie((*sock).sk as *mut core::ffi::c_void) {
        return 0;
    }

    (*p).cookie_value |= 0xF0;
    return 0;
}
