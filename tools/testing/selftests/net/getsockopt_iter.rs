// SPDX-License-Identifier: GPL-2.0
/*
 * Quick test for getsockopt{_iter} tests.
 *
 * Each fixture targets one converted protocol and pins down the
 * returned-length / errno semantics across buffer-size variations,
 * an unknown optname and a bogus level.
 *
 * - netlink: NETLINK_PKTINFO covers the flag-style int path; the
 *   NETLINK_LIST_MEMBERSHIPS cases cover the size-discovery path
 *   that always reports the required buffer length back via optlen,
 *   even when the user buffer is too small to receive any group bits.
 * - vsock:   SO_VM_SOCKETS_BUFFER_SIZE covers the u64 path.
 * - raw:     ICMP_FILTER covers a fixed-size struct payload that clamps
 *            the length down on a short buffer instead of failing.
 *
 * Author: Breno Leitao <leitao@debian.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type socklen_t = u32;
type __u32 = u32;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_NETLINK: c_int = 16;
const AF_VSOCK: c_int = 40;
const SOCK_STREAM: c_int = 1;
const SOCK_RAW: c_int = 3;
const SOL_SOCKET: c_int = 1;
const SOL_RAW: c_int = 255;
const SOL_NETLINK: c_int = 270;
const SOL_TLS: c_int = 282;
const IPPROTO_ICMP: c_int = 1;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_IPV6: c_int = 41;
const NETLINK_ROUTE: c_int = 0;
const NETLINK_ADD_MEMBERSHIP: c_int = 1;
const NETLINK_PKTINFO: c_int = 3;
const NETLINK_LIST_MEMBERSHIPS: c_int = 9;
const RTNLGRP_LINK: c_int = 1;
const SO_VM_SOCKETS_BUFFER_SIZE: c_int = 0;
const SO_VM_SOCKETS_CONNECT_TIMEOUT_OLD: c_int = 6;
const SO_VM_SOCKETS_CONNECT_TIMEOUT_NEW: c_int = 8;
const ICMP_FILTER: c_int = 1;
const IPV6_CHECKSUM: c_int = 7;
const IPV6_HDRINCL: c_int = 36;
const TCP_ULP: c_int = 31;
const TLS_TX: c_int = 1;
const TLS_TX_ZEROCOPY_RO: c_int = 3;
const TLS_1_2_VERSION: u16 = 0x0303;
const TLS_CIPHER_AES_GCM_128: u16 = 51;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const EINVAL: c_int = 22;
const ENOPROTOOPT: c_int = 92;
const EOPNOTSUPP: c_int = 95;
const EBUSY: c_int = 16;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct __kernel_sock_timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
struct __kernel_old_timeval {
    tv_sec: i32,
    tv_usec: i32,
}

#[repr(C)]
struct icmp_filter {
    data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct tls_crypto_info {
    version: u16,
    cipher_type: u16,
}

#[repr(C)]
struct tls12_crypto_info_aes_gcm_128 {
    info: tls_crypto_info,
    iv: [u8; 8],
    key: [u8; 16],
    salt: [u8; 4],
    rec_seq: [u8; 8],
}

unsafe extern "C" {
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn htonl(hostlong: u32) -> u32;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

/* ---------- netlink ---------- */

struct netlink {
    fd: c_int,
}

unsafe fn netlink_setup(self_: *mut netlink) {
    let group: c_int = RTNLGRP_LINK;

    unsafe {
        (*self_).fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
        if (*self_).fd < 0 {
            return;
        }

        /* Joining a multicast group grows nlk->ngroups so the
         * NETLINK_LIST_MEMBERSHIPS path has a non-zero size to report.
         */
        if setsockopt(
            (*self_).fd,
            SOL_NETLINK,
            NETLINK_ADD_MEMBERSHIP,
            &group as *const _ as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) < 0
        {
            return;
        }
    }
}

unsafe fn netlink_teardown(self_: *mut netlink) {
    unsafe {
        if (*self_).fd >= 0 {
            close((*self_).fd);
        }
    }
}

unsafe fn netlink_pktinfo_exact(self_: *mut netlink) {
    let mut optlen: socklen_t;
    let mut val: c_int = -1;

    optlen = size_of::<c_int>() as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_NETLINK, NETLINK_PKTINFO, &mut val as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<c_int>() as socklen_t, optlen);
    assert!(val == 0 || val == 1);
}

unsafe fn netlink_pktinfo_oversize_clamped(self_: *mut netlink) {
    let mut buf = [0 as c_char; 16];
    let mut optlen: socklen_t;

    optlen = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_NETLINK, NETLINK_PKTINFO, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<c_int>() as socklen_t, optlen);
}

unsafe fn netlink_pktinfo_undersize(self_: *mut netlink) {
    let mut buf = [0 as c_char; 2];
    let mut optlen: socklen_t;

    optlen = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_NETLINK, NETLINK_PKTINFO, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    unsafe {
        assert_eq!(EINVAL, errno());
    }
    assert_eq!(size_of_val(&buf) as socklen_t, optlen);
}

unsafe fn netlink_list_memberships_size_discovery(self_: *mut netlink) {
    let mut optlen: socklen_t = 0;
    let mut dummy: c_char = 0;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_NETLINK, NETLINK_LIST_MEMBERSHIPS, &mut dummy as *mut _ as *mut c_void, &mut optlen));
    }
    assert!(optlen > 0);
    assert_eq!(0, optlen % size_of::<__u32>() as socklen_t);
}

unsafe fn netlink_list_memberships_full_read(self_: *mut netlink) {
    let mut buf = [0 as __u32; 64];
    let mut optlen: socklen_t;

    optlen = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_NETLINK, NETLINK_LIST_MEMBERSHIPS, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    assert!(optlen > 0);
    assert!(optlen <= size_of_val(&buf) as socklen_t);
    assert_eq!(0, optlen % size_of::<__u32>() as socklen_t);
}

unsafe fn netlink_bad_level(self_: *mut netlink) {
    let mut optlen: socklen_t;
    let mut val: c_int = 0;

    optlen = size_of::<c_int>() as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_SOCKET + 1, NETLINK_PKTINFO, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(ENOPROTOOPT, errno());
    }
    assert_eq!(size_of_val(&val) as socklen_t, optlen);
}

unsafe fn netlink_bad_optname(self_: *mut netlink) {
    let mut optlen: socklen_t;
    let mut val: c_int = 0;

    optlen = size_of::<c_int>() as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_NETLINK, 0x7fff, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(ENOPROTOOPT, errno());
    }
    assert_eq!(size_of_val(&val) as socklen_t, optlen);
}

/* ---------- vsock ---------- */

struct vsock {
    fd: c_int,
}

unsafe fn vsock_setup(self_: *mut vsock) {
    unsafe {
        (*self_).fd = socket(AF_VSOCK, SOCK_STREAM, 0);
        if (*self_).fd < 0 {
            return;
        }
    }
}

unsafe fn vsock_teardown(self_: *mut vsock) {
    unsafe {
        if (*self_).fd >= 0 {
            close((*self_).fd);
        }
    }
}

unsafe fn vsock_buffer_size_exact(self_: *mut vsock) {
    let mut optlen: socklen_t;
    let mut val: u64 = 0;

    optlen = size_of::<u64>() as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, &mut val as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<u64>() as socklen_t, optlen);
    assert!(val > 0);
}

unsafe fn vsock_buffer_size_oversize_clamped(self_: *mut vsock) {
    let mut buf = [0 as c_char; 16];
    let mut optlen: socklen_t;

    optlen = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<u64>() as socklen_t, optlen);
}

unsafe fn vsock_buffer_size_undersize(self_: *mut vsock) {
    let mut buf = [0 as c_char; 4];
    let mut optlen: socklen_t;

    optlen = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, buf.as_mut_ptr() as *mut c_void, &mut optlen));
        assert_eq!(EINVAL, errno());
    }
    assert_eq!(size_of_val(&buf) as socklen_t, optlen);
}

unsafe fn vsock_bad_level(self_: *mut vsock) {
    let mut optlen: socklen_t;
    let mut val: u64 = 0;

    optlen = size_of::<u64>() as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_SOCKET + 1, SO_VM_SOCKETS_BUFFER_SIZE, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(ENOPROTOOPT, errno());
    }
    assert_eq!(size_of_val(&val) as socklen_t, optlen);
}

unsafe fn vsock_bad_optname(self_: *mut vsock) {
    let mut optlen: socklen_t;
    let mut val: u64 = 0;

    optlen = size_of::<u64>() as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, AF_VSOCK, 0x7fff, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(ENOPROTOOPT, errno());
    }
    assert_eq!(size_of_val(&val) as socklen_t, optlen);
}

/* SO_VM_SOCKETS_CONNECT_TIMEOUT_{NEW,OLD} return a sock_timeval-shaped
 * payload, which is wider than u64 on 64-bit. They exercise the path
 * where the protocol's reported lv (16 bytes) is larger than the
 * common 8-byte u64 case covered above.
 */
unsafe fn vsock_connect_timeout_new_exact(self_: *mut vsock) {
    let mut tv = __kernel_sock_timeval { tv_sec: 0, tv_usec: 0 };
    let mut optlen: socklen_t;

    optlen = size_of_val(&tv) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, AF_VSOCK, SO_VM_SOCKETS_CONNECT_TIMEOUT_NEW, &mut tv as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of_val(&tv) as socklen_t, optlen);
}

unsafe fn vsock_connect_timeout_new_oversize_clamped(self_: *mut vsock) {
    let mut buf = [0 as c_char; size_of::<__kernel_sock_timeval>() * 2];
    let mut optlen: socklen_t;

    optlen = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, AF_VSOCK, SO_VM_SOCKETS_CONNECT_TIMEOUT_NEW, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<__kernel_sock_timeval>() as socklen_t, optlen);
}

unsafe fn vsock_connect_timeout_new_undersize(self_: *mut vsock) {
    let mut optlen: socklen_t;
    let mut val: u64 = 0;

    optlen = size_of_val(&val) as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, AF_VSOCK, SO_VM_SOCKETS_CONNECT_TIMEOUT_NEW, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(EINVAL, errno());
    }
    assert_eq!(size_of_val(&val) as socklen_t, optlen);
}

unsafe fn vsock_connect_timeout_old_exact(self_: *mut vsock) {
    let mut tv = __kernel_old_timeval { tv_sec: 0, tv_usec: 0 };
    let mut optlen: socklen_t;

    optlen = size_of_val(&tv) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, AF_VSOCK, SO_VM_SOCKETS_CONNECT_TIMEOUT_OLD, &mut tv as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of_val(&tv) as socklen_t, optlen);
}

/* ---------- raw (ipv4) ---------- */

struct raw {
    fd: c_int,
}

unsafe fn raw_setup(self_: *mut raw) {
    let filt = icmp_filter { data: 0xdeadbeef };

    unsafe {
        (*self_).fd = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP);
        if (*self_).fd < 0 {
            return;
        }

        if setsockopt((*self_).fd, SOL_RAW, ICMP_FILTER, &filt as *const _ as *const c_void, size_of_val(&filt) as socklen_t) < 0 {
            return;
        }
    }
}

unsafe fn raw_teardown(self_: *mut raw) {
    unsafe {
        if (*self_).fd >= 0 {
            close((*self_).fd);
        }
    }
}

unsafe fn raw_icmpfilter_exact(self_: *mut raw) {
    let mut filt = icmp_filter { data: 0 };
    let mut optlen: socklen_t = size_of_val(&filt) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_RAW, ICMP_FILTER, &mut filt as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of_val(&filt) as socklen_t, optlen);
    assert_eq!(0xdeadbeef, filt.data);
}

unsafe fn raw_icmpfilter_oversize_clamped(self_: *mut raw) {
    let mut buf = [0 as c_char; 16];
    let mut optlen: socklen_t = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_RAW, ICMP_FILTER, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<icmp_filter>() as socklen_t, optlen);
}

/* Unlike the int/u64 options above, ICMP_FILTER clamps the length down
 * to the user buffer instead of returning EINVAL: a short buffer
 * succeeds and reports the truncated length back via optlen.
 */
unsafe fn raw_icmpfilter_undersize_clamped(self_: *mut raw) {
    let mut buf = [0 as c_char; 2];
    let mut optlen: socklen_t = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_RAW, ICMP_FILTER, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of_val(&buf) as socklen_t, optlen);
}

unsafe fn raw_icmpfilter_wrong_proto(_self_: *mut raw) {
    let mut filt = icmp_filter { data: 0 };
    let mut optlen: socklen_t = size_of_val(&filt) as socklen_t;
    let fd: c_int;

    unsafe {
        fd = socket(AF_INET, SOCK_RAW, IPPROTO_UDP);
        if fd < 0 {
            return;
        }

        assert_eq!(-1, getsockopt(fd, SOL_RAW, ICMP_FILTER, &mut filt as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(EOPNOTSUPP, errno());
        close(fd);
    }
}

unsafe fn raw_bad_optname(self_: *mut raw) {
    let mut optlen: socklen_t;
    let mut val: c_int = 0;

    optlen = size_of_val(&val) as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_RAW, 0x7fff, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(ENOPROTOOPT, errno());
    }
    assert_eq!(size_of_val(&val) as socklen_t, optlen);
}

/* ---------- raw (ipv6) ---------- */

struct rawv6 {
    fd: c_int,
}

unsafe fn rawv6_setup(self_: *mut rawv6) {
    unsafe {
        (*self_).fd = socket(AF_INET6, SOCK_RAW, IPPROTO_UDP);
        if (*self_).fd < 0 {
            return;
        }
    }
}

unsafe fn rawv6_teardown(self_: *mut rawv6) {
    unsafe {
        if (*self_).fd >= 0 {
            close((*self_).fd);
        }
    }
}

unsafe fn rawv6_hdrincl_exact(self_: *mut rawv6) {
    let mut optlen: socklen_t;
    let mut val: c_int = -1;

    optlen = size_of_val(&val) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, IPPROTO_IPV6, IPV6_HDRINCL, &mut val as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<c_int>() as socklen_t, optlen);
    assert!(val == 0 || val == 1);
}

unsafe fn rawv6_hdrincl_oversize_clamped(self_: *mut rawv6) {
    let mut buf = [0 as c_char; 16];
    let mut optlen: socklen_t = size_of_val(&buf) as socklen_t;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, IPPROTO_IPV6, IPV6_HDRINCL, buf.as_mut_ptr() as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<c_int>() as socklen_t, optlen);
}

/* Raw int options clamp the reported length down to the user buffer
 * instead of returning EINVAL on a short buffer.
 */
unsafe fn rawv6_hdrincl_undersize_clamped(self_: *mut rawv6) {
    let mut optlen: socklen_t = 2;
    let mut val: c_int = 0;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, IPPROTO_IPV6, IPV6_HDRINCL, &mut val as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(2, optlen);
}

unsafe fn rawv6_checksum_default(self_: *mut rawv6) {
    let mut optlen: socklen_t;
    let mut val: c_int = 0;

    optlen = size_of_val(&val) as socklen_t;

    /* A non-ICMPv6 raw socket has the checksum disabled, reported as -1. */
    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, IPPROTO_IPV6, IPV6_CHECKSUM, &mut val as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<c_int>() as socklen_t, optlen);
    assert_eq!(-1, val);
}

unsafe fn rawv6_bad_optname(self_: *mut rawv6) {
    let mut optlen: socklen_t;
    let mut val: c_int = 0;

    optlen = size_of_val(&val) as socklen_t;

    /* SOL_RAW reaches do_rawv6_getsockopt() directly. */
    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_RAW, 0x7fff, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(ENOPROTOOPT, errno());
    }
    assert_eq!(size_of_val(&val) as socklen_t, optlen);
}

/* ---------- tls ---------- */

struct tls {
    fd: c_int,
    sfd: c_int,
}

unsafe fn tls_setup(self_: *mut tls) {
    let mut a = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr {
            s_addr: unsafe { htonl(INADDR_LOOPBACK) },
        },
        sin_zero: [0; 8],
    };
    let mut alen: socklen_t = size_of_val(&a) as socklen_t;
    let lfd: c_int;

    unsafe {
        (*self_).fd = -1;
        (*self_).sfd = -1;

        lfd = socket(AF_INET, SOCK_STREAM, 0);
        if lfd < 0 {
            return;
        }
        if bind(lfd, &a as *const _ as *const sockaddr, size_of_val(&a) as socklen_t) != 0
            || listen(lfd, 1) != 0
            || getsockname(lfd, &mut a as *mut _ as *mut sockaddr, &mut alen) != 0
        {
            close(lfd);
            return;
        }
        (*self_).fd = socket(AF_INET, SOCK_STREAM, 0);
        if (*self_).fd < 0 {
            close(lfd);
            return;
        }
        if connect((*self_).fd, &a as *const _ as *const sockaddr, size_of_val(&a) as socklen_t) != 0 {
            close(lfd);
            return;
        }
        (*self_).sfd = accept(lfd, ptr::null_mut(), ptr::null_mut());
        close(lfd);
        if setsockopt((*self_).fd, IPPROTO_TCP, TCP_ULP, c"tls".as_ptr() as *const c_void, size_of_val(c"tls".to_bytes_with_nul()) as socklen_t) != 0 {
            return;
        }
    }
}

unsafe fn tls_teardown(self_: *mut tls) {
    unsafe {
        if (*self_).fd >= 0 {
            close((*self_).fd);
        }
        if (*self_).sfd >= 0 {
            close((*self_).sfd);
        }
    }
}

/* do_tls_getsockopt_tx_zc(): fixed-size int, exact length required. */
unsafe fn tls_tx_zerocopy_exact(self_: *mut tls) {
    let mut optlen: socklen_t = size_of::<c_int>() as socklen_t;
    let mut val: c_int = -1;

    unsafe {
        assert_eq!(0, getsockopt((*self_).fd, SOL_TLS, TLS_TX_ZEROCOPY_RO, &mut val as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of::<c_int>() as socklen_t, optlen);
    assert!(val == 0 || val == 1);
}

unsafe fn tls_tx_zerocopy_wrong_len(self_: *mut tls) {
    let mut optlen: socklen_t = 2;
    let mut val: c_int = 0;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_TLS, TLS_TX_ZEROCOPY_RO, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(EINVAL, errno());
    }
}

/* do_tls_getsockopt_conf(): NULL optval still yields EINVAL -- the
 * converted code tests opt->iter_out.ubuf in place of optval.
 */
unsafe fn tls_conf_null_optval(self_: *mut tls) {
    let mut optlen: socklen_t = 64;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_TLS, TLS_TX, ptr::null_mut(), &mut optlen));
        assert_eq!(EINVAL, errno());
    }
}

unsafe fn tls_conf_short(self_: *mut tls) {
    let mut optlen: socklen_t = 2;
    let mut buf = [0 as c_char; 2];

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_TLS, TLS_TX, buf.as_mut_ptr() as *mut c_void, &mut optlen));
        assert_eq!(EINVAL, errno());
    }
}

/* TLS_TX before crypto is set reports not-ready. */
unsafe fn tls_conf_not_ready(self_: *mut tls) {
    let mut info = tls_crypto_info { version: 0, cipher_type: 0 };
    let mut optlen: socklen_t = size_of_val(&info) as socklen_t;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_TLS, TLS_TX, &mut info as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(EBUSY, errno());
    }
}

/* Set TX crypto, then read it back at the base and full sizes, exercising
 * both copy_to_iter() branches. SKIP if AES-GCM is unavailable.
 */
unsafe fn tls_conf_crypto_roundtrip(self_: *mut tls) {
    let tx = tls12_crypto_info_aes_gcm_128 {
        info: tls_crypto_info {
            version: TLS_1_2_VERSION,
            cipher_type: TLS_CIPHER_AES_GCM_128,
        },
        iv: [0; 8],
        key: [0; 16],
        salt: [0; 4],
        rec_seq: [0; 8],
    };
    let mut full = tls12_crypto_info_aes_gcm_128 {
        info: tls_crypto_info { version: 0, cipher_type: 0 },
        iv: [0; 8],
        key: [0; 16],
        salt: [0; 4],
        rec_seq: [0; 8],
    };
    let mut base = tls_crypto_info { version: 0, cipher_type: 0 };
    let mut optlen: socklen_t;

    unsafe {
        if setsockopt((*self_).fd, SOL_TLS, TLS_TX, &tx as *const _ as *const c_void, size_of_val(&tx) as socklen_t) != 0 {
            return;
        }

        optlen = size_of_val(&base) as socklen_t;
        assert_eq!(0, getsockopt((*self_).fd, SOL_TLS, TLS_TX, &mut base as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of_val(&base) as socklen_t, optlen);
    assert_eq!(TLS_1_2_VERSION, base.version);
    assert_eq!(TLS_CIPHER_AES_GCM_128, base.cipher_type);

    unsafe {
        optlen = size_of_val(&full) as socklen_t;
        assert_eq!(0, getsockopt((*self_).fd, SOL_TLS, TLS_TX, &mut full as *mut _ as *mut c_void, &mut optlen));
    }
    assert_eq!(size_of_val(&full) as socklen_t, optlen);
    assert_eq!(TLS_CIPHER_AES_GCM_128, full.info.cipher_type);
}

unsafe fn tls_bad_optname(self_: *mut tls) {
    let mut optlen: socklen_t = size_of::<c_int>() as socklen_t;
    let mut val: c_int = 0;

    unsafe {
        assert_eq!(-1, getsockopt((*self_).fd, SOL_TLS, 0x7fff, &mut val as *mut _ as *mut c_void, &mut optlen));
        assert_eq!(ENOPROTOOPT, errno());
    }
}

fn main() {}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
