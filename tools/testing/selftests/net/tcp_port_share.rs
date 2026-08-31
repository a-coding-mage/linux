// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2025 Cloudflare, Inc.

/* Tests for TCP port sharing (bind bucket reuse). */

use std::ffi::CString;
use std::mem;
use std::ptr;

use libc::{
    bind, close, connect, dprintf, getsockname, htons, inet_pton, listen, ntohs, open, setsockopt,
    snprintf, socket, system, unshare, AF_INET, AF_INET6, AF_UNSPEC, EADDRNOTAVAIL, EAFNOSUPPORT,
    INET6_ADDRSTRLEN, IP_BIND_ADDRESS_NO_PORT, O_WRONLY, SOCK_STREAM, SOL_IP, SOL_SOCKET,
    SO_REUSEADDR,
};

const DST_PORT: u16 = 30000;
const SRC_PORT: u16 = 40000;

#[repr(C)]
union SockaddrInetUnion {
    ss: libc::sockaddr_storage,
    v6: libc::sockaddr_in6,
    v4: libc::sockaddr_in,
    sa: libc::sockaddr,
}

#[repr(C)]
struct sockaddr_inet {
    u: SockaddrInetUnion,
    len: libc::socklen_t,
    str_: [libc::c_char; INET6_ADDRSTRLEN as usize + "[]:65535".len() + 1],
}

static one: libc::c_int = 1;

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!($left > $right)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

unsafe fn disconnect(fd: libc::c_int) -> libc::c_int {
    let mut sa: libc::sockaddr = mem::zeroed();
    sa.sa_family = AF_UNSPEC as libc::sa_family_t;
    connect(
        fd,
        &sa as *const libc::sockaddr,
        mem::size_of::<libc::sockaddr>() as libc::socklen_t,
    )
}

unsafe fn getsockname_port(fd: libc::c_int) -> libc::c_int {
    let mut addr: sockaddr_inet = mem::zeroed();
    let err: libc::c_int;

    addr.len = mem::size_of_val(&addr) as libc::socklen_t;
    err = getsockname(fd, &mut addr.u.sa as *mut libc::sockaddr, &mut addr.len);
    if err != 0 {
        return -1;
    }

    match addr.u.sa.sa_family as libc::c_int {
        AF_INET => ntohs(addr.u.v4.sin_port) as libc::c_int,
        AF_INET6 => ntohs(addr.u.v6.sin6_port) as libc::c_int,
        _ => {
            *libc::__errno_location() = EAFNOSUPPORT;
            -1
        }
    }
}

unsafe fn make_inet_addr(
    af: libc::c_int,
    ip: *const libc::c_char,
    port: u16,
    addr: *mut sockaddr_inet,
) {
    let mut fmt = c"".as_ptr();

    ptr::write_bytes(addr as *mut u8, 0, mem::size_of::<sockaddr_inet>());

    match af {
        AF_INET => {
            (*addr).len = mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;
            (*addr).u.v4.sin_family = af as libc::sa_family_t;
            (*addr).u.v4.sin_port = htons(port);
            inet_pton(
                af,
                ip,
                &mut (*addr).u.v4.sin_addr as *mut libc::in_addr as *mut libc::c_void,
            );
            fmt = c"%s:%hu".as_ptr();
        }
        AF_INET6 => {
            (*addr).len = mem::size_of::<libc::sockaddr_in6>() as libc::socklen_t;
            (*addr).u.v6.sin6_family = af as libc::sa_family_t;
            (*addr).u.v6.sin6_port = htons(port);
            inet_pton(
                af,
                ip,
                &mut (*addr).u.v6.sin6_addr as *mut libc::in6_addr as *mut libc::c_void,
            );
            fmt = c"[%s]:%hu".as_ptr();
        }
        _ => {}
    }

    snprintf(
        (*addr).str_.as_mut_ptr(),
        (*addr).str_.len(),
        fmt,
        ip,
        port as libc::c_int,
    );
}

struct tcp_port_share {}

struct tcp_port_share_variant {
    domain: libc::c_int,
    /* IP to listen on and connect to */
    dst_ip: *const libc::c_char,
    /* Primary IP to connect from */
    src1_ip: *const libc::c_char,
    /* Secondary IP to connect from */
    src2_ip: *const libc::c_char,
    /* IP to bind to in order to block the source port */
    bind_ip: *const libc::c_char,
}

static ipv4: tcp_port_share_variant = tcp_port_share_variant {
    domain: AF_INET,
    dst_ip: c"127.0.0.1".as_ptr(),
    src1_ip: c"127.1.1.1".as_ptr(),
    src2_ip: c"127.2.2.2".as_ptr(),
    bind_ip: c"127.3.3.3".as_ptr(),
};

static ipv6: tcp_port_share_variant = tcp_port_share_variant {
    domain: AF_INET6,
    dst_ip: c"::1".as_ptr(),
    src1_ip: c"2001:db8::1".as_ptr(),
    src2_ip: c"2001:db8::2".as_ptr(),
    bind_ip: c"2001:db8::3".as_ptr(),
};

unsafe fn tcp_port_share_setup() {
    let sc: libc::c_int;

    ASSERT_EQ!(unshare(libc::CLONE_NEWNET), 0);
    ASSERT_EQ!(system(c"ip link set dev lo up".as_ptr()), 0);
    ASSERT_EQ!(system(c"ip addr add dev lo 2001:db8::1/32 nodad".as_ptr()), 0);
    ASSERT_EQ!(system(c"ip addr add dev lo 2001:db8::2/32 nodad".as_ptr()), 0);
    ASSERT_EQ!(system(c"ip addr add dev lo 2001:db8::3/32 nodad".as_ptr()), 0);

    sc = open(c"/proc/sys/net/ipv4/ip_local_port_range".as_ptr(), O_WRONLY);
    ASSERT_GE!(sc, 0);
    ASSERT_GT!(dprintf(sc, c"%hu %hu\n".as_ptr(), SRC_PORT as libc::c_int, SRC_PORT as libc::c_int), 0);
    ASSERT_EQ!(close(sc), 0);
}

unsafe fn tcp_port_share_teardown() {}

/* Verify that an ephemeral port becomes available again after the socket
 * bound to it and blocking it from reuse is closed.
 */
unsafe fn can_reuse_port_after_bind_and_close(variant: *const tcp_port_share_variant) {
    let v = variant;
    let mut addr: sockaddr_inet = mem::zeroed();
    let c1: libc::c_int;
    let c2: libc::c_int;
    let ln: libc::c_int;
    let pb: libc::c_int;

    /* Listen on <dst_ip>:<DST_PORT> */
    ln = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(ln, 0);
    ASSERT_EQ!(
        setsockopt(
            ln,
            SOL_SOCKET,
            SO_REUSEADDR,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).dst_ip, DST_PORT, &mut addr);
    ASSERT_EQ!(bind(ln, &addr.u.sa as *const libc::sockaddr, addr.len), 0);
    ASSERT_EQ!(listen(ln, 2), 0);

    /* Connect from <src1_ip>:<SRC_PORT> */
    c1 = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(c1, 0);
    ASSERT_EQ!(
        setsockopt(
            c1,
            SOL_IP,
            IP_BIND_ADDRESS_NO_PORT,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).src1_ip, 0, &mut addr);
    ASSERT_EQ!(bind(c1, &addr.u.sa as *const libc::sockaddr, addr.len), 0);

    make_inet_addr((*v).domain, (*v).dst_ip, DST_PORT, &mut addr);
    ASSERT_EQ!(connect(c1, &addr.u.sa as *const libc::sockaddr, addr.len), 0);
    ASSERT_EQ!(getsockname_port(c1), SRC_PORT as libc::c_int);

    /* Bind to <bind_ip>:<SRC_PORT>. Block the port from reuse. */
    pb = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(pb, 0);
    ASSERT_EQ!(
        setsockopt(
            pb,
            SOL_SOCKET,
            SO_REUSEADDR,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).bind_ip, SRC_PORT, &mut addr);
    ASSERT_EQ!(bind(pb, &addr.u.sa as *const libc::sockaddr, addr.len), 0);

    /* Try to connect from <src2_ip>:<SRC_PORT>. Expect failure. */
    c2 = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(c2, 0);
    ASSERT_EQ!(
        setsockopt(
            c2,
            SOL_IP,
            IP_BIND_ADDRESS_NO_PORT,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).src2_ip, 0, &mut addr);
    ASSERT_EQ!(bind(c2, &addr.u.sa as *const libc::sockaddr, addr.len), 0);

    make_inet_addr((*v).domain, (*v).dst_ip, DST_PORT, &mut addr);
    ASSERT_EQ!(connect(c2, &addr.u.sa as *const libc::sockaddr, addr.len), -1);
    ASSERT_EQ!(*libc::__errno_location(), EADDRNOTAVAIL);

    /* Unbind from <bind_ip>:<SRC_PORT>. Unblock the port for reuse. */
    ASSERT_EQ!(close(pb), 0);

    /* Connect again from <src2_ip>:<SRC_PORT> */
    EXPECT_EQ!(connect(c2, &addr.u.sa as *const libc::sockaddr, addr.len), 0);
    EXPECT_EQ!(getsockname_port(c2), SRC_PORT as libc::c_int);

    ASSERT_EQ!(close(c2), 0);
    ASSERT_EQ!(close(c1), 0);
    ASSERT_EQ!(close(ln), 0);
}

/* Verify that a socket auto-bound during connect() blocks port reuse after
 * disconnect (connect(AF_UNSPEC)) followed by an explicit port bind().
 */
unsafe fn port_block_after_disconnect(variant: *const tcp_port_share_variant) {
    let v = variant;
    let mut addr: sockaddr_inet = mem::zeroed();
    let c1: libc::c_int;
    let c2: libc::c_int;
    let ln: libc::c_int;
    let pb: libc::c_int;

    /* Listen on <dst_ip>:<DST_PORT> */
    ln = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(ln, 0);
    ASSERT_EQ!(
        setsockopt(
            ln,
            SOL_SOCKET,
            SO_REUSEADDR,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).dst_ip, DST_PORT, &mut addr);
    ASSERT_EQ!(bind(ln, &addr.u.sa as *const libc::sockaddr, addr.len), 0);
    ASSERT_EQ!(listen(ln, 2), 0);

    /* Connect from <src1_ip>:<SRC_PORT> */
    c1 = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(c1, 0);
    ASSERT_EQ!(
        setsockopt(
            c1,
            SOL_IP,
            IP_BIND_ADDRESS_NO_PORT,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).src1_ip, 0, &mut addr);
    ASSERT_EQ!(bind(c1, &addr.u.sa as *const libc::sockaddr, addr.len), 0);

    make_inet_addr((*v).domain, (*v).dst_ip, DST_PORT, &mut addr);
    ASSERT_EQ!(connect(c1, &addr.u.sa as *const libc::sockaddr, addr.len), 0);
    ASSERT_EQ!(getsockname_port(c1), SRC_PORT as libc::c_int);

    /* Disconnect the socket and bind it to <bind_ip>:<SRC_PORT> to block the port */
    ASSERT_EQ!(disconnect(c1), 0);
    ASSERT_EQ!(
        setsockopt(
            c1,
            SOL_SOCKET,
            SO_REUSEADDR,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).bind_ip, SRC_PORT, &mut addr);
    ASSERT_EQ!(bind(c1, &addr.u.sa as *const libc::sockaddr, addr.len), 0);

    /* Trigger port-addr bucket state update with another bind() and close() */
    pb = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(pb, 0);
    ASSERT_EQ!(
        setsockopt(
            pb,
            SOL_SOCKET,
            SO_REUSEADDR,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).bind_ip, SRC_PORT, &mut addr);
    ASSERT_EQ!(bind(pb, &addr.u.sa as *const libc::sockaddr, addr.len), 0);

    ASSERT_EQ!(close(pb), 0);

    /* Connect from <src2_ip>:<SRC_PORT>. Expect failure. */
    c2 = socket((*v).domain, SOCK_STREAM, 0);
    ASSERT_GE!(c2, 0);
    ASSERT_EQ!(
        setsockopt(
            c2,
            SOL_IP,
            IP_BIND_ADDRESS_NO_PORT,
            &one as *const libc::c_int as *const libc::c_void,
            mem::size_of_val(&one) as libc::socklen_t,
        ),
        0
    );

    make_inet_addr((*v).domain, (*v).src2_ip, 0, &mut addr);
    ASSERT_EQ!(bind(c2, &addr.u.sa as *const libc::sockaddr, addr.len), 0);

    make_inet_addr((*v).domain, (*v).dst_ip, DST_PORT, &mut addr);
    EXPECT_EQ!(connect(c2, &addr.u.sa as *const libc::sockaddr, addr.len), -1);
    EXPECT_EQ!(*libc::__errno_location(), EADDRNOTAVAIL);

    ASSERT_EQ!(close(c2), 0);
    ASSERT_EQ!(close(c1), 0);
    ASSERT_EQ!(close(ln), 0);
}

fn main() {
    unsafe {
        for variant in [&ipv4, &ipv6] {
            tcp_port_share_setup();
            can_reuse_port_after_bind_and_close(variant);
            tcp_port_share_teardown();

            tcp_port_share_setup();
            port_block_after_disconnect(variant);
            tcp_port_share_teardown();
        }
    }
}
