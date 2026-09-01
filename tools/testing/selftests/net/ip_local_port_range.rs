// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2023 Cloudflare

/* Test IP_LOCAL_PORT_RANGE socket option: IPv4 + IPv6, TCP + UDP.
 *
 * Tests assume that net.ipv4.ip_local_port_range is [40000, 49999].
 * Don't run these directly but with ip_local_port_range.sh script.
 */

/* C dependencies: <fcntl.h>, <netinet/ip.h>, "kselftest_harness.h" */

use libc::{
    bind, close, connect, getsockname, getsockopt, htonl, htons, in6_addr, in_addr, ntohs, socket,
    sockaddr, sockaddr_in, sockaddr_in6, socklen_t, AF_INET, AF_INET6, EADDRINUSE, EAFNOSUPPORT,
    EINVAL, INADDR_ANY, INADDR_LOOPBACK, IPPROTO_SCTP, SOL_IP, SOL_SOCKET, SO_DOMAIN, SOCK_DGRAM,
    SOCK_STREAM,
};
use std::mem;

const IP_LOCAL_PORT_RANGE: i32 = 51;
const IPPROTO_MPTCP: i32 = 262;
const IP_BIND_ADDRESS_NO_PORT: i32 = 24;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

extern "C" {
    static in6addr_loopback: in6_addr;
    static in6addr_any: in6_addr;
}

#[repr(C)]
union SockaddrStorage {
    sa: sockaddr,
    v4: sockaddr_in,
    v6: sockaddr_in6,
}

#[inline]
unsafe fn errno_location() -> *mut i32 {
    libc::__errno_location()
}

unsafe fn pack_port_range(lo: __u16, hi: __u16) -> __u32 {
    (((hi as __u32) << 16) | ((lo as __u32) << 0)) as __u32
}

unsafe fn unpack_port_range(range: __u32, lo: *mut __u16, hi: *mut __u16) {
    *lo = (range & 0xffff) as __u16;
    *hi = (range >> 16) as __u16;
}

unsafe fn get_so_domain(fd: i32) -> i32 {
    let mut domain: i32 = 0;
    let mut len: socklen_t;
    let err: i32;

    len = mem::size_of_val(&domain) as socklen_t;
    err = getsockopt(
        fd,
        SOL_SOCKET,
        SO_DOMAIN,
        &mut domain as *mut _ as *mut _,
        &mut len,
    );
    if err != 0 {
        return -1;
    }

    domain
}

unsafe fn bind_to_loopback_any_port(fd: i32) -> i32 {
    let mut addr: SockaddrStorage = mem::zeroed();
    let addr_len: socklen_t;

    match get_so_domain(fd) {
        AF_INET => {
            addr.v4.sin_family = AF_INET as _;
            addr.v4.sin_port = htons(0);
            addr.v4.sin_addr = in_addr {
                s_addr: htonl(INADDR_LOOPBACK),
            };
            addr_len = mem::size_of::<sockaddr_in>() as socklen_t;
        }
        AF_INET6 => {
            addr.v6.sin6_family = AF_INET6 as _;
            addr.v6.sin6_port = htons(0);
            addr.v6.sin6_addr = in6addr_loopback;
            addr_len = mem::size_of::<sockaddr_in6>() as socklen_t;
        }
        _ => {
            return -1;
        }
    }

    bind(fd, &addr.sa as *const sockaddr, addr_len)
}

unsafe fn get_sock_port(fd: i32) -> i32 {
    let mut addr: SockaddrStorage = mem::zeroed();
    let mut addr_len: socklen_t;
    let err: i32;

    addr_len = mem::size_of::<SockaddrStorage>() as socklen_t;
    err = getsockname(fd, &mut addr.sa as *mut sockaddr, &mut addr_len);
    if err != 0 {
        return -1;
    }

    match addr.sa.sa_family as i32 {
        AF_INET => ntohs(addr.v4.sin_port) as i32,
        AF_INET6 => ntohs(addr.v6.sin6_port) as i32,
        _ => {
            *errno_location() = EAFNOSUPPORT;
            -1
        }
    }
}

unsafe fn get_ip_local_port_range(fd: i32, range: *mut __u32) -> i32 {
    let mut len: socklen_t;
    let mut val: __u32 = 0;
    let err: i32;

    len = mem::size_of_val(&val) as socklen_t;
    err = getsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut val as *mut _ as *mut _,
        &mut len,
    );
    if err != 0 {
        return -1;
    }

    *range = val;
    0
}

#[repr(C)]
struct ip_local_port_range {}

fn ip_local_port_range_setup(_self_: *mut ip_local_port_range) {}

fn ip_local_port_range_teardown(_self_: *mut ip_local_port_range) {}

#[repr(C)]
struct ip_local_port_range_variant {
    so_domain: i32,
    so_type: i32,
    so_protocol: i32,
}

static ip4_tcp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET,
    so_type: SOCK_STREAM,
    so_protocol: 0,
};

static ip4_udp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET,
    so_type: SOCK_DGRAM,
    so_protocol: 0,
};

static ip4_stcp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET,
    so_type: SOCK_STREAM,
    so_protocol: IPPROTO_SCTP,
};

static ip4_mptcp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET,
    so_type: SOCK_STREAM,
    so_protocol: IPPROTO_MPTCP,
};

static ip6_tcp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET6,
    so_type: SOCK_STREAM,
    so_protocol: 0,
};

static ip6_udp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET6,
    so_type: SOCK_DGRAM,
    so_protocol: 0,
};

static ip6_stcp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET6,
    so_type: SOCK_STREAM,
    so_protocol: IPPROTO_SCTP,
};

static ip6_mptcp: ip_local_port_range_variant = ip_local_port_range_variant {
    so_domain: AF_INET6,
    so_type: SOCK_STREAM,
    so_protocol: IPPROTO_MPTCP,
};

unsafe fn invalid_option_value(variant: *const ip_local_port_range_variant) {
    let mut val16: __u16;
    let mut val32: __u32;
    let mut val64: __u64;
    let fd: i32;
    let mut err: i32;

    fd = socket((*variant).so_domain, (*variant).so_type, (*variant).so_protocol);
    ASSERT_GE!(fd, 0, "socket failed");

    /* Too few bytes */
    val16 = 40000;
    err = setsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut val16 as *mut _ as *const _,
        mem::size_of_val(&val16) as socklen_t,
    );
    EXPECT_TRUE!(err != 0, "expected setsockopt(IP_LOCAL_PORT_RANGE) to fail");
    EXPECT_EQ!(*errno_location(), EINVAL);

    /* Empty range: low port > high port */
    val32 = pack_port_range(40222, 40111);
    err = setsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut val32 as *mut _ as *const _,
        mem::size_of_val(&val32) as socklen_t,
    );
    EXPECT_TRUE!(err != 0, "expected setsockopt(IP_LOCAL_PORT_RANGE) to fail");
    EXPECT_EQ!(*errno_location(), EINVAL);

    /* Too many bytes */
    val64 = pack_port_range(40333, 40444) as __u64;
    err = setsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut val64 as *mut _ as *const _,
        mem::size_of_val(&val64) as socklen_t,
    );
    EXPECT_TRUE!(err != 0, "expected setsockopt(IP_LOCAL_PORT_RANGE) to fail");
    EXPECT_EQ!(*errno_location(), EINVAL);

    err = close(fd);
    ASSERT_TRUE!(err == 0, "close failed");
}

unsafe fn port_range_out_of_netns_range(variant: *const ip_local_port_range_variant) {
    #[repr(C)]
    struct test {
        range_lo: __u16,
        range_hi: __u16,
    }
    let tests: [test; 2] = [
        test {
            range_lo: 30000,
            range_hi: 39999,
        }, /* socket range below netns range */
        test {
            range_lo: 50000,
            range_hi: 59999,
        }, /* socket range above netns range */
    ];
    let mut t: *const test;

    t = tests.as_ptr();
    while t < unsafe { tests.as_ptr().add(tests.len()) } {
        /* Bind a couple of sockets, not just one, to check
         * that the range wasn't clamped to a single port from
         * the netns range. That is [40000, 40000] or [49999,
         * 49999], respectively for each test case.
         */
        let mut fds: [i32; 2] = [0; 2];
        let mut i: i32;

        TH_LOG!("lo %5hu, hi %5hu", (*t).range_lo, (*t).range_hi);

        i = 0;
        while (i as usize) < fds.len() {
            let fd: i32;
            let mut err: i32;
            let port: i32;
            let mut range: __u32;

            fd = socket((*variant).so_domain, (*variant).so_type, (*variant).so_protocol);
            ASSERT_GE!(fd, 0, "#%d: socket failed", i);

            range = pack_port_range((*t).range_lo, (*t).range_hi);
            err = setsockopt(
                fd,
                SOL_IP,
                IP_LOCAL_PORT_RANGE,
                &mut range as *mut _ as *const _,
                mem::size_of_val(&range) as socklen_t,
            );
            ASSERT_TRUE!(err == 0, "#%d: setsockopt(IP_LOCAL_PORT_RANGE) failed", i);

            err = bind_to_loopback_any_port(fd);
            ASSERT_TRUE!(err == 0, "#%d: bind failed", i);

            /* Check that socket port range outside of ephemeral range is ignored */
            port = get_sock_port(fd);
            ASSERT_GE!(port, 40000, "#%d: expected port within netns range", i);
            ASSERT_LE!(port, 49999, "#%d: expected port within netns range", i);

            fds[i as usize] = fd;
            i += 1;
        }

        i = 0;
        while (i as usize) < fds.len() {
            ASSERT_TRUE!(close(fds[i as usize]) == 0, "#%d: close failed", i);
            i += 1;
        }
        t = t.add(1);
    }
}

unsafe fn single_port_range(variant: *const ip_local_port_range_variant) {
    #[repr(C)]
    struct test {
        range_lo: __u16,
        range_hi: __u16,
        expected: __u16,
    }
    let tests: [test; 3] = [
        /* single port range within ephemeral range */
        test {
            range_lo: 45000,
            range_hi: 45000,
            expected: 45000,
        },
        /* first port in the ephemeral range (clamp from above) */
        test {
            range_lo: 0,
            range_hi: 40000,
            expected: 40000,
        },
        /* last port in the ephemeral range (clamp from below)  */
        test {
            range_lo: 49999,
            range_hi: 0,
            expected: 49999,
        },
    ];
    let mut t: *const test;

    t = tests.as_ptr();
    while t < tests.as_ptr().add(tests.len()) {
        let fd: i32;
        let mut err: i32;
        let port: i32;
        let mut range: __u32;

        TH_LOG!(
            "lo %5hu, hi %5hu, expected %5hu",
            (*t).range_lo,
            (*t).range_hi,
            (*t).expected
        );

        fd = socket((*variant).so_domain, (*variant).so_type, (*variant).so_protocol);
        ASSERT_GE!(fd, 0, "socket failed");

        range = pack_port_range((*t).range_lo, (*t).range_hi);
        err = setsockopt(
            fd,
            SOL_IP,
            IP_LOCAL_PORT_RANGE,
            &mut range as *mut _ as *const _,
            mem::size_of_val(&range) as socklen_t,
        );
        ASSERT_TRUE!(err == 0, "setsockopt(IP_LOCAL_PORT_RANGE) failed");

        err = bind_to_loopback_any_port(fd);
        ASSERT_TRUE!(err == 0, "bind failed");

        port = get_sock_port(fd);
        ASSERT_EQ!(port, (*t).expected as i32, "unexpected local port");

        err = close(fd);
        ASSERT_TRUE!(err == 0, "close failed");
        t = t.add(1);
    }
}

unsafe fn exhaust_8_port_range(variant: *const ip_local_port_range_variant) {
    let mut port_set: __u8 = 0;
    let mut i: i32;
    let mut fd: i32;
    let mut err: i32;
    let mut range: __u32;
    let mut port: __u16;
    let mut fds: [i32; 8] = [0; 8];

    i = 0;
    while (i as usize) < fds.len() {
        fd = socket((*variant).so_domain, (*variant).so_type, (*variant).so_protocol);
        ASSERT_GE!(fd, 0, "socket failed");

        range = pack_port_range(40000, 40007);
        err = setsockopt(
            fd,
            SOL_IP,
            IP_LOCAL_PORT_RANGE,
            &mut range as *mut _ as *const _,
            mem::size_of_val(&range) as socklen_t,
        );
        ASSERT_TRUE!(err == 0, "setsockopt(IP_LOCAL_PORT_RANGE) failed");

        err = bind_to_loopback_any_port(fd);
        ASSERT_TRUE!(err == 0, "bind failed");

        port = get_sock_port(fd) as __u16;
        ASSERT_GE!(port, 40000, "expected port within sockopt range");
        ASSERT_LE!(port, 40007, "expected port within sockopt range");

        port_set |= (1u8).wrapping_shl((port - 40000) as u32);
        fds[i as usize] = fd;
        i += 1;
    }

    /* Check that all every port from the test range is in use */
    ASSERT_EQ!(port_set, 0xff, "expected all ports to be busy");

    /* Check that bind() fails because the whole range is busy */
    fd = socket((*variant).so_domain, (*variant).so_type, (*variant).so_protocol);
    ASSERT_GE!(fd, 0, "socket failed");

    range = pack_port_range(40000, 40007);
    err = setsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut range as *mut _ as *const _,
        mem::size_of_val(&range) as socklen_t,
    );
    ASSERT_TRUE!(err == 0, "setsockopt(IP_LOCAL_PORT_RANGE) failed");

    err = bind_to_loopback_any_port(fd);
    ASSERT_TRUE!(err != 0, "expected bind to fail");
    ASSERT_EQ!(*errno_location(), EADDRINUSE);

    err = close(fd);
    ASSERT_TRUE!(err == 0, "close failed");

    i = 0;
    while (i as usize) < fds.len() {
        err = close(fds[i as usize]);
        ASSERT_TRUE!(err == 0, "close failed");
        i += 1;
    }
}

unsafe fn late_bind(variant: *const ip_local_port_range_variant) {
    let mut addr: SockaddrStorage = mem::zeroed();
    let mut addr_len: socklen_t = 0;
    let one: i32 = 1;
    let fd: i32;
    let mut err: i32;
    let mut range: __u32;
    let mut port: __u16;

    fd = socket((*variant).so_domain, (*variant).so_type, 0);
    ASSERT_GE!(fd, 0, "socket failed");

    range = pack_port_range(40100, 40199);
    err = setsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut range as *mut _ as *const _,
        mem::size_of_val(&range) as socklen_t,
    );
    ASSERT_TRUE!(err == 0, "setsockopt(IP_LOCAL_PORT_RANGE) failed");

    err = setsockopt(
        fd,
        SOL_IP,
        IP_BIND_ADDRESS_NO_PORT,
        &one as *const _ as *const _,
        mem::size_of_val(&one) as socklen_t,
    );
    ASSERT_TRUE!(err == 0, "setsockopt(IP_BIND_ADDRESS_NO_PORT) failed");

    err = bind_to_loopback_any_port(fd);
    ASSERT_TRUE!(err == 0, "bind failed");

    port = get_sock_port(fd) as __u16;
    ASSERT_EQ!(port, 0, "getsockname failed");

    /* Invalid destination */
    addr = mem::zeroed();
    match (*variant).so_domain {
        AF_INET => {
            addr.v4.sin_family = AF_INET as _;
            addr.v4.sin_port = htons(0);
            addr.v4.sin_addr = in_addr {
                s_addr: htonl(INADDR_ANY),
            };
            addr_len = mem::size_of::<sockaddr_in>() as socklen_t;
        }
        AF_INET6 => {
            addr.v6.sin6_family = AF_INET6 as _;
            addr.v6.sin6_port = htons(0);
            addr.v6.sin6_addr = in6addr_any;
            addr_len = mem::size_of::<sockaddr_in6>() as socklen_t;
        }
        _ => {
            ASSERT_TRUE!(false, "unsupported socket domain");
        }
    }

    /* connect() doesn't need to succeed for late bind to happen */
    connect(fd, &addr.sa as *const sockaddr, addr_len);

    port = get_sock_port(fd) as __u16;
    ASSERT_GE!(port, 40100);
    ASSERT_LE!(port, 40199);

    err = close(fd);
    ASSERT_TRUE!(err == 0, "close failed");
}

/* XFAIL_ADD(ip_local_port_range, ip4_stcp, late_bind); */
/* XFAIL_ADD(ip_local_port_range, ip6_stcp, late_bind); */

unsafe fn get_port_range(variant: *const ip_local_port_range_variant) {
    let mut lo: __u16 = 0;
    let mut hi: __u16 = 0;
    let mut range: __u32 = 0;
    let fd: i32;
    let mut err: i32;

    fd = socket((*variant).so_domain, (*variant).so_type, (*variant).so_protocol);
    ASSERT_GE!(fd, 0, "socket failed");

    /* Get range before it will be set */
    err = get_ip_local_port_range(fd, &mut range);
    ASSERT_TRUE!(err == 0, "getsockopt(IP_LOCAL_PORT_RANGE) failed");

    unpack_port_range(range, &mut lo, &mut hi);
    ASSERT_EQ!(lo, 0, "unexpected low port");
    ASSERT_EQ!(hi, 0, "unexpected high port");

    range = pack_port_range(12345, 54321);
    err = setsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut range as *mut _ as *const _,
        mem::size_of_val(&range) as socklen_t,
    );
    ASSERT_TRUE!(err == 0, "setsockopt(IP_LOCAL_PORT_RANGE) failed");

    /* Get range after it has been set */
    err = get_ip_local_port_range(fd, &mut range);
    ASSERT_TRUE!(err == 0, "getsockopt(IP_LOCAL_PORT_RANGE) failed");

    unpack_port_range(range, &mut lo, &mut hi);
    ASSERT_EQ!(lo, 12345, "unexpected low port");
    ASSERT_EQ!(hi, 54321, "unexpected high port");

    /* Unset the port range  */
    range = pack_port_range(0, 0);
    err = setsockopt(
        fd,
        SOL_IP,
        IP_LOCAL_PORT_RANGE,
        &mut range as *mut _ as *const _,
        mem::size_of_val(&range) as socklen_t,
    );
    ASSERT_TRUE!(err == 0, "setsockopt(IP_LOCAL_PORT_RANGE) failed");

    /* Get range after it has been unset */
    err = get_ip_local_port_range(fd, &mut range);
    ASSERT_TRUE!(err == 0, "getsockopt(IP_LOCAL_PORT_RANGE) failed");

    unpack_port_range(range, &mut lo, &mut hi);
    ASSERT_EQ!(lo, 0, "unexpected low port");
    ASSERT_EQ!(hi, 0, "unexpected high port");

    err = close(fd);
    ASSERT_TRUE!(err == 0, "close failed");
}

/* TEST_HARNESS_MAIN */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
