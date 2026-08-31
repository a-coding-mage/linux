// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

// C dependencies: <sys/socket.h>, <netinet/in.h>, "kselftest_harness.h"

use core::ffi::{c_int, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type socklen_t = u32;
type sa_family_t = u16;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SOL_IPV6: c_int = 41;
const SO_REUSEADDR: c_int = 2;
const SO_REUSEPORT: c_int = 15;
const IPV6_V6ONLY: c_int = 26;
const INADDR_ANY: __u32 = 0x00000000;
const INADDR_LOOPBACK: __u32 = 0x7f000001;
const EADDRINUSE: c_int = 98;

const NR_SOCKETS: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in {
    sin_family: sa_family_t,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in6 {
    sin6_family: sa_family_t,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
union SockAddrUnion {
    addr: sockaddr,
    addr4: sockaddr_in,
    addr6: sockaddr_in6,
}

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
}

fn htons(hostshort: u16) -> u16 {
    hostshort.to_be()
}

fn htonl(hostlong: u32) -> u32 {
    hostlong.to_be()
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!(($left) > ($right))
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

static in4addr_any: __u32 = INADDR_ANY;
static in4addr_loopback: __u32 = INADDR_LOOPBACK;
static in6addr_any: in6_addr = in6_addr { s6_addr: [0; 16] };
static in6addr_loopback: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};
static in6addr_v4mapped_any: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0],
};
static in6addr_v4mapped_loopback: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1],
};

#[repr(C)]
struct bind_wildcard {
    fd: [c_int; NR_SOCKETS],
    addrlen: [socklen_t; NR_SOCKETS],
    addr: [SockAddrUnion; NR_SOCKETS],
}

#[repr(C)]
struct bind_wildcard_variant {
    family: [sa_family_t; 2],
    addr: [*const c_void; 2],
    ipv6_only: [bool; 2],

    /* 6 bind() calls below follow two bind() for the defined 2 addresses:
     *
     *   0.0.0.0
     *   127.0.0.1
     *   ::
     *   ::1
     *   ::ffff:0.0.0.0
     *   ::ffff:127.0.0.1
     */
    expected_errno: [c_int; NR_SOCKETS],
    expected_reuse_errno: [c_int; NR_SOCKETS],
}

macro_rules! variant {
    ($name:ident, [$f0:expr, $f1:expr], [$a0:expr, $a1:expr], [$v60:expr, $v61:expr],
     [$($err:expr),* $(,)?], [$($reuse_err:expr),* $(,)?]) => {
        static $name: bind_wildcard_variant = bind_wildcard_variant {
            family: [$f0 as sa_family_t, $f1 as sa_family_t],
            addr: [$a0 as *const _ as *const c_void, $a1 as *const _ as *const c_void],
            ipv6_only: [$v60, $v61],
            expected_errno: [$($err),*],
            expected_reuse_errno: [$($reuse_err),*],
        };
    };
}

/* (IPv4, IPv4) */
variant!(v4_any_v4_local, [AF_INET, AF_INET], [&in4addr_any, &in4addr_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v4_local_v4_any, [AF_INET, AF_INET], [&in4addr_loopback, &in4addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);

/* (IPv4, IPv6) */
variant!(v4_any_v6_any, [AF_INET, AF_INET6], [&in4addr_any, &in6addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v4_any_v6_any_only, [AF_INET, AF_INET6], [&in4addr_any, &in6addr_any], [false, true],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v4_any_v6_local, [AF_INET, AF_INET6], [&in4addr_any, &in6addr_loopback], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v4_any_v6_v4mapped_any, [AF_INET, AF_INET6], [&in4addr_any, &in6addr_v4mapped_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v4_any_v6_v4mapped_local, [AF_INET, AF_INET6], [&in4addr_any, &in6addr_v4mapped_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v4_local_v6_any, [AF_INET, AF_INET6], [&in4addr_loopback, &in6addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v4_local_v6_any_only, [AF_INET, AF_INET6], [&in4addr_loopback, &in6addr_any], [false, true],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v4_local_v6_local, [AF_INET, AF_INET6], [&in4addr_loopback, &in6addr_loopback], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v4_local_v6_v4mapped_any, [AF_INET, AF_INET6], [&in4addr_loopback, &in6addr_v4mapped_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v4_local_v6_v4mapped_local, [AF_INET, AF_INET6], [&in4addr_loopback, &in6addr_v4mapped_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);

/* (IPv6, IPv4) */
variant!(v6_any_v4_any, [AF_INET6, AF_INET], [&in6addr_any, &in4addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_only_v4_any, [AF_INET6, AF_INET], [&in6addr_any, &in4addr_any], [true, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_v4_local, [AF_INET6, AF_INET], [&in6addr_any, &in4addr_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_only_v4_local, [AF_INET6, AF_INET], [&in6addr_any, &in4addr_loopback], [true, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_local_v4_any, [AF_INET6, AF_INET], [&in6addr_loopback, &in4addr_any], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_local_v4_local, [AF_INET6, AF_INET], [&in6addr_loopback, &in4addr_loopback], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_any_v4_any, [AF_INET6, AF_INET], [&in6addr_v4mapped_any, &in4addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_any_v4_local, [AF_INET6, AF_INET], [&in6addr_v4mapped_any, &in4addr_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_local_v4_any, [AF_INET6, AF_INET], [&in6addr_v4mapped_loopback, &in4addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_local_v4_local, [AF_INET6, AF_INET], [&in6addr_v4mapped_loopback, &in4addr_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);

/* (IPv6, IPv6) */
variant!(v6_any_v6_any, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_only_v6_any, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_any], [true, false],
    [0, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_v6_any_only, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_any], [false, true],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_only_v6_any_only, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_any], [true, true],
    [0, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_v6_local, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_only_v6_local, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_loopback], [true, false],
    [0, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_v6_v4mapped_any, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_v4mapped_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_only_v6_v4mapped_any, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_v4mapped_any], [true, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_v6_v4mapped_local, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_v4mapped_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_any_only_v6_v4mapped_local, [AF_INET6, AF_INET6], [&in6addr_any, &in6addr_v4mapped_loopback], [true, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_local_v6_any, [AF_INET6, AF_INET6], [&in6addr_loopback, &in6addr_any], [false, false],
    [0, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_local_v6_any_only, [AF_INET6, AF_INET6], [&in6addr_loopback, &in6addr_any], [false, true],
    [0, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_local_v6_v4mapped_any, [AF_INET6, AF_INET6], [&in6addr_loopback, &in6addr_v4mapped_any], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_local_v6_v4mapped_local, [AF_INET6, AF_INET6], [&in6addr_loopback, &in6addr_v4mapped_loopback], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_any_v6_any, [AF_INET6, AF_INET6], [&in6addr_v4mapped_any, &in6addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_any_v6_any_only, [AF_INET6, AF_INET6], [&in6addr_v4mapped_any, &in6addr_any], [false, true],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_any_v6_local, [AF_INET6, AF_INET6], [&in6addr_v4mapped_any, &in6addr_loopback], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_any_v6_v4mapped_local, [AF_INET6, AF_INET6], [&in6addr_v4mapped_any, &in6addr_v4mapped_loopback], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_loopback_v6_any, [AF_INET6, AF_INET6], [&in6addr_v4mapped_loopback, &in6addr_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_loopback_v6_any_only, [AF_INET6, AF_INET6], [&in6addr_v4mapped_loopback, &in6addr_any], [false, true],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_loopback_v6_local, [AF_INET6, AF_INET6], [&in6addr_v4mapped_loopback, &in6addr_loopback], [false, false],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE]);
variant!(v6_v4mapped_loopback_v6_v4mapped_any, [AF_INET6, AF_INET6], [&in6addr_v4mapped_loopback, &in6addr_v4mapped_any], [false, false],
    [0, EADDRINUSE, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE],
    [0, 0, EADDRINUSE, EADDRINUSE, EADDRINUSE, 0, EADDRINUSE, EADDRINUSE]);

unsafe fn setup_addr(self_: *mut bind_wildcard, i: c_int, family: c_int, addr_const: *const c_void) {
    if family == AF_INET {
        let addr4 = &mut (*self_).addr[i as usize].addr4 as *mut sockaddr_in;
        let addr4_const = addr_const as *const __u32;

        (*addr4).sin_family = AF_INET as sa_family_t;
        (*addr4).sin_port = htons(0);
        (*addr4).sin_addr.s_addr = htonl(*addr4_const);

        (*self_).addrlen[i as usize] = size_of::<sockaddr_in>() as socklen_t;
    } else {
        let addr6 = &mut (*self_).addr[i as usize].addr6 as *mut sockaddr_in6;
        let addr6_const = addr_const as *const in6_addr;

        (*addr6).sin6_family = AF_INET6 as sa_family_t;
        (*addr6).sin6_port = htons(0);
        (*addr6).sin6_addr = *addr6_const;

        (*self_).addrlen[i as usize] = size_of::<sockaddr_in6>() as socklen_t;
    }
}

unsafe fn bind_wildcard_setup(self_: *mut bind_wildcard, variant: *const bind_wildcard_variant) {
    setup_addr(self_, 0, (*variant).family[0] as c_int, (*variant).addr[0]);
    setup_addr(self_, 1, (*variant).family[1] as c_int, (*variant).addr[1]);

    setup_addr(self_, 2, AF_INET, &in4addr_any as *const _ as *const c_void);
    setup_addr(self_, 3, AF_INET, &in4addr_loopback as *const _ as *const c_void);

    setup_addr(self_, 4, AF_INET6, &in6addr_any as *const _ as *const c_void);
    setup_addr(self_, 5, AF_INET6, &in6addr_loopback as *const _ as *const c_void);
    setup_addr(self_, 6, AF_INET6, &in6addr_v4mapped_any as *const _ as *const c_void);
    setup_addr(
        self_,
        7,
        AF_INET6,
        &in6addr_v4mapped_loopback as *const _ as *const c_void,
    );
}

unsafe fn bind_wildcard_teardown(self_: *mut bind_wildcard) {
    let mut i: c_int = 0;

    while i < NR_SOCKETS as c_int {
        close((*self_).fd[i as usize]);
        i += 1;
    }
}

unsafe fn bind_socket(
    _metadata: *mut __test_metadata,
    self_: *mut bind_wildcard,
    variant: *const bind_wildcard_variant,
    i: c_int,
    reuse: c_int,
) {
    let mut ret: c_int;

    (*self_).fd[i as usize] = socket((*self_).addr[i as usize].addr.sa_family as c_int, SOCK_STREAM, 0);
    ASSERT_GT!((*self_).fd[i as usize], 0);

    if i < 2 && (*variant).ipv6_only[i as usize] {
        let one: c_int = 1;
        ret = setsockopt(
            (*self_).fd[i as usize],
            SOL_IPV6,
            IPV6_V6ONLY,
            &one as *const _ as *const c_void,
            size_of::<c_int>() as socklen_t,
        );
        ASSERT_EQ!(ret, 0);
    }

    if i < 2 && reuse != 0 {
        let one: c_int = 1;
        ret = setsockopt(
            (*self_).fd[i as usize],
            SOL_SOCKET,
            reuse,
            &one as *const _ as *const c_void,
            size_of::<c_int>() as socklen_t,
        );
        ASSERT_EQ!(ret, 0);
    }

    (*self_).addr[i as usize].addr4.sin_port = (*self_).addr[0].addr4.sin_port;

    ret = bind(
        (*self_).fd[i as usize],
        &(*self_).addr[i as usize].addr as *const sockaddr,
        (*self_).addrlen[i as usize],
    );

    if reuse != 0 {
        if (*variant).expected_reuse_errno[i as usize] != 0 {
            ASSERT_EQ!(ret, -1);
            ASSERT_EQ!(*__errno_location(), (*variant).expected_reuse_errno[i as usize]);
        } else {
            ASSERT_EQ!(ret, 0);
        }
    } else if (*variant).expected_errno[i as usize] != 0 {
        ASSERT_EQ!(ret, -1);
        ASSERT_EQ!(*__errno_location(), (*variant).expected_errno[i as usize]);
    } else {
        ASSERT_EQ!(ret, 0);
    }

    if i == 0 {
        ret = getsockname(
            (*self_).fd[0],
            &mut (*self_).addr[0].addr as *mut sockaddr,
            &mut (*self_).addrlen[0] as *mut socklen_t,
        );
        ASSERT_EQ!(ret, 0);
    }
}

unsafe fn bind_wildcard_plain(
    _metadata: *mut __test_metadata,
    self_: *mut bind_wildcard,
    variant: *const bind_wildcard_variant,
) {
    let mut i: c_int = 0;

    while i < NR_SOCKETS as c_int {
        bind_socket(_metadata, self_, variant, i, 0);
        i += 1;
    }
}

unsafe fn bind_wildcard_reuseaddr(
    _metadata: *mut __test_metadata,
    self_: *mut bind_wildcard,
    variant: *const bind_wildcard_variant,
) {
    let mut i: c_int = 0;

    while i < NR_SOCKETS as c_int {
        bind_socket(_metadata, self_, variant, i, SO_REUSEADDR);
        i += 1;
    }
}

unsafe fn bind_wildcard_reuseport(
    _metadata: *mut __test_metadata,
    self_: *mut bind_wildcard,
    variant: *const bind_wildcard_variant,
) {
    let mut i: c_int = 0;

    while i < NR_SOCKETS as c_int {
        bind_socket(_metadata, self_, variant, i, SO_REUSEPORT);
        i += 1;
    }
}

fn main() {
    let _ = ptr::null::<__test_metadata>();
}
