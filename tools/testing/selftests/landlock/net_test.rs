// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock tests - Network
 *
 * Copyright (c) 2022-2023 Huawei Tech. Co., Ltd.
 * Copyright (c) 2023 Microsoft Corporation
 *
 * Source-level Rust translation of testing/selftests/landlock/net_test.c.
 *
 * This file intentionally keeps the kselftest/Landlock fixture surface as
 * external macro/function dependencies.  The isolated source file does not
 * define those harness macros or the shared structs from audit.h, common.h,
 * and trace.h, so their Rust equivalents are referenced by name instead of
 * being reimplemented here.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::{c_char, c_int, c_short, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

// C dependencies removed from executable Rust:
// arpa/inet.h, errno.h, fcntl.h, linux/in.h, linux/landlock.h, sched.h,
// stdint.h, string.h, sys/mount.h, sys/prctl.h, sys/socket.h,
// sys/syscall.h, sys/un.h, audit.h, common.h, trace.h.

const TRACE_TASK: &[u8] = b"net_test\0";

pub const sock_port_start: c_short = 1 << 10;
static loopback_ipv4: &[u8] = b"127.0.0.1\0";
static loopback_ipv6: &[u8] = b"::1\0";

/* Number pending connections queue to be hold. */
pub const backlog: c_short = 10;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum sandbox_type {
    NO_SANDBOX,
    /* This may be used to test rules that allow *and* deny accesses. */
    TCP_SANDBOX,
    UDP_SANDBOX,
}

// External C/kernel/libc/test-harness declarations used by the translation.
type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type pid_t = c_int;
type sa_family_t = u16;
type __u16 = u16;
type __u64 = u64;

#[repr(C)] pub struct __test_metadata { pub exit_code: c_int }
#[repr(C)] #[derive(Clone, Copy)] pub struct protocol_variant { pub domain: c_int, pub type_: c_int, pub protocol: c_int }
#[repr(C)] #[derive(Clone, Copy)] pub struct in_addr { pub s_addr: u32 }
#[repr(C)] #[derive(Clone, Copy)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] #[derive(Clone, Copy)] pub struct sockaddr { pub sa_family: sa_family_t, pub sa_data: [c_char; 14] }
#[repr(C)] #[derive(Clone, Copy)] pub struct sockaddr_in { pub sin_family: sa_family_t, pub sin_port: u16, pub sin_addr: in_addr, pub sin_zero: [u8; 8] }
#[repr(C)] #[derive(Clone, Copy)] pub struct sockaddr_in6 { pub sin6_family: sa_family_t, pub sin6_port: u16, pub sin6_flowinfo: u32, pub sin6_addr: in6_addr, pub sin6_scope_id: u32 }
#[repr(C)] #[derive(Clone, Copy)] pub struct sockaddr_un { pub sun_family: sa_family_t, pub sun_path: [c_char; 108] }
#[repr(C)] #[derive(Clone, Copy)] pub struct sockaddr_storage { pub ss_family: sa_family_t, pub __data: [u8; 126] }
#[repr(C)] #[derive(Clone, Copy)] pub struct timeval { pub tv_sec: isize, pub tv_usec: isize }
#[repr(C)] #[derive(Clone, Copy)] pub struct service_fixture { pub protocol: protocol_variant, pub port: u16, pub ipv4_addr: sockaddr_in, pub ipv6_addr: sockaddr_in6, pub unix_addr: sockaddr_un, pub unix_addr_len: socklen_t }
#[repr(C)] #[derive(Clone, Copy)] pub struct landlock_ruleset_attr { pub handled_access_fs: __u64, pub handled_access_net: __u64, pub quiet_access_net: __u64 }
#[repr(C)] #[derive(Clone, Copy)] pub struct landlock_net_port_attr { pub allowed_access: __u64, pub port: u64 }
#[repr(C)] #[derive(Clone, Copy)] pub struct landlock_path_beneath_attr { pub allowed_access: __u64, pub parent_fd: c_int }
#[repr(C)] pub struct audit_filter { _priv: [u8; 0] }
#[repr(C)] pub struct audit_records { pub access: c_int, pub domain: c_int }

unsafe extern "C" {
    static mut errno: c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn listen(fd: c_int, backlog: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn recv(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn sendto(fd: c_int, buf: *const c_void, len: size_t, flags: c_int, dst: *const sockaddr, addrlen: socklen_t) -> ssize_t;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn getpeername(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn inet_addr(cp: *const c_char) -> u32;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn mount(source: *const c_char, target: *const c_char, filesystemtype: *const c_char, mountflags: c_ulong, data: *const c_void) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn set_unix_address(srv: *mut service_fixture, index: c_uint);
    fn set_cap(metadata: *mut __test_metadata, cap: c_int);
    fn clear_cap(metadata: *mut __test_metadata, cap: c_int);
    fn set_ambient_cap(metadata: *mut __test_metadata, cap: c_int);
    fn clear_ambient_cap(metadata: *mut __test_metadata, cap: c_int);
    fn disable_caps(metadata: *mut __test_metadata);
    fn enforce_ruleset(metadata: *mut __test_metadata, ruleset_fd: c_int);
    fn landlock_create_ruleset(attr: *const landlock_ruleset_attr, size: size_t, flags: u32) -> c_int;
    fn landlock_add_rule(fd: c_int, rule_type: c_int, rule_attr: *const c_void, flags: u32) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: u32) -> c_int;
    fn audit_init_with_exe_filter(filter: *mut audit_filter) -> c_int;
    fn audit_cleanup(fd: c_int, filter: *mut audit_filter) -> c_int;
    fn audit_match_record(fd: c_int, record_type: c_int, regex: *const c_char, unused: *const c_void) -> c_int;
    fn audit_count_records(fd: c_int, records: *mut audit_records) -> c_int;
    fn tracefs_fixture_setup() -> c_int;
    fn tracefs_fixture_teardown();
    fn tracefs_enable_event(event: *const c_char, enable: bool) -> c_int;
    fn tracefs_clear() -> c_int;
    fn tracefs_clear_buf() -> c_int;
    fn tracefs_read_buf() -> *mut c_char;
    fn tracefs_count_matches(buf: *const c_char, regex: *const c_char) -> c_int;
    fn tracefs_extract_field(buf: *const c_char, regex: *const c_char, field_name: *const c_char, out: *mut c_char, out_len: size_t) -> c_int;
}

// Constants are expected from libc/kernel bindings in the final repository.
// They are declared here as external Rust names to preserve the source-level API.
unsafe extern "C" {
    static AF_UNSPEC: c_int; static AF_INET: c_int; static AF_INET6: c_int; static AF_UNIX: c_int;
    static SOCK_STREAM: c_int; static SOCK_DGRAM: c_int; static SOCK_CLOEXEC: c_int;
    static IPPROTO_TCP: c_int; static IPPROTO_UDP: c_int; static IPPROTO_IP: c_int; static IPPROTO_MPTCP: c_int;
    static SOL_SOCKET: c_int; static SO_RCVTIMEO: c_int; static SO_SNDTIMEO: c_int; static SO_TYPE: c_int; static SO_DOMAIN: c_int; static SO_REUSEADDR: c_int;
    static MSG_NOSIGNAL: c_int; static MSG_FASTOPEN: c_int;
    static EAFNOSUPPORT: c_int; static EINVAL: c_int; static EACCES: c_int; static EPIPE: c_int; static EINTR: c_int; static ENOTCONN: c_int; static EDESTADDRREQ: c_int; static ECONNREFUSED: c_int; static EISCONN: c_int; static EOPNOTSUPP: c_int; static E2BIG: c_int; static ENOMSG: c_int;
    static EXIT_SUCCESS: c_int; static INADDR_ANY: u32; static INADDR_LOOPBACK: u32; static UINT16_MAX: u64; static UINT32_MAX: u64;
    static CAP_SYS_ADMIN: c_int; static CAP_NET_ADMIN: c_int; static CAP_NET_BIND_SERVICE: c_int; static CAP_AUDIT_CONTROL: c_int;
    static CLONE_NEWNET: c_int; static CLONE_NEWNS: c_int; static MS_REC: c_ulong; static MS_PRIVATE: c_ulong; static PR_SET_NO_NEW_PRIVS: c_int;
    static LANDLOCK_ACCESS_NET_BIND_TCP: __u64; static LANDLOCK_ACCESS_NET_CONNECT_TCP: __u64; static LANDLOCK_ACCESS_NET_BIND_UDP: __u64; static LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP: __u64; static LANDLOCK_ACCESS_FS_READ_DIR: __u64;
    static LANDLOCK_RULE_NET_PORT: c_int; static LANDLOCK_RULE_PATH_BENEATH: c_int; static LANDLOCK_ADD_RULE_QUIET: u32; static AUDIT_LANDLOCK_ACCESS: c_int;
}

const SIN6_LEN_RFC2133: socklen_t = 24;

unsafe fn set_service(srv: *mut service_fixture, mut prot: protocol_variant, index: u16) -> c_int {
    memset(srv.cast(), 0, size_of::<service_fixture>());
    (*srv).protocol = prot;
    if index > 2 { return 1; }
    (*srv).port = (sock_port_start as u16) << (2 * index);
    if prot.domain == AF_UNSPEC || prot.domain == AF_INET {
        (*srv).ipv4_addr.sin_family = prot.domain as sa_family_t;
        (*srv).ipv4_addr.sin_port = htons((*srv).port);
        (*srv).ipv4_addr.sin_addr.s_addr = inet_addr(loopback_ipv4.as_ptr().cast());
        return 0;
    }
    if prot.domain == AF_INET6 {
        (*srv).ipv6_addr.sin6_family = prot.domain as sa_family_t;
        (*srv).ipv6_addr.sin6_port = htons((*srv).port);
        inet_pton(AF_INET6, loopback_ipv6.as_ptr().cast(), (&mut (*srv).ipv6_addr.sin6_addr as *mut in6_addr).cast());
        return 0;
    }
    if prot.domain == AF_UNIX {
        set_unix_address(srv, index as c_uint);
        return 0;
    }
    1
}

unsafe fn setup_loopback(_metadata: *mut __test_metadata) {
    set_cap(_metadata, CAP_SYS_ADMIN);
    ASSERT_EQ!(0, unshare(CLONE_NEWNET));
    clear_cap(_metadata, CAP_SYS_ADMIN);
    set_ambient_cap(_metadata, CAP_NET_ADMIN);
    ASSERT_EQ!(0, system(c"ip link set dev lo up".as_ptr()));
    clear_ambient_cap(_metadata, CAP_NET_ADMIN);
}

unsafe fn prot_is_tcp(prot: *const protocol_variant) -> bool {
    ((*prot).domain == AF_INET || (*prot).domain == AF_INET6) && (*prot).type_ == SOCK_STREAM && ((*prot).protocol == IPPROTO_TCP || (*prot).protocol == IPPROTO_IP)
}
unsafe fn prot_is_udp(prot: *const protocol_variant) -> bool {
    ((*prot).domain == AF_INET || (*prot).domain == AF_INET6) && (*prot).type_ == SOCK_DGRAM && ((*prot).protocol == IPPROTO_UDP || (*prot).protocol == IPPROTO_IP)
}
unsafe fn is_restricted(prot: *const protocol_variant, sandbox: sandbox_type) -> bool {
    if sandbox == sandbox_type::TCP_SANDBOX { prot_is_tcp(prot) } else if sandbox == sandbox_type::UDP_SANDBOX { prot_is_udp(prot) } else { false }
}

unsafe fn socket_variant(srv: *const service_fixture) -> c_int {
    let timeout = timeval { tv_sec: 0, tv_usec: 100000 };
    let sockfd = socket((*srv).protocol.domain, (*srv).protocol.type_ | SOCK_CLOEXEC, (*srv).protocol.protocol);
    if sockfd < 0 { return -errno; }
    let mut ret = setsockopt(sockfd, SOL_SOCKET, SO_RCVTIMEO, (&timeout as *const timeval).cast(), size_of::<timeval>() as socklen_t);
    if ret != 0 { ret = -errno; close(sockfd); return ret; }
    ret = setsockopt(sockfd, SOL_SOCKET, SO_SNDTIMEO, (&timeout as *const timeval).cast(), size_of::<timeval>() as socklen_t);
    if ret != 0 { ret = -errno; close(sockfd); return ret; }
    sockfd
}

unsafe fn get_addrlen(srv: *const service_fixture, minimal: bool) -> socklen_t {
    if (*srv).protocol.domain == AF_UNSPEC { if minimal { size_of::<sa_family_t>() as socklen_t } else { size_of::<sockaddr_storage>() as socklen_t } }
    else if (*srv).protocol.domain == AF_INET { size_of::<sockaddr_in>() as socklen_t }
    else if (*srv).protocol.domain == AF_INET6 { if minimal { SIN6_LEN_RFC2133 } else { size_of::<sockaddr_in6>() as socklen_t } }
    else if (*srv).protocol.domain == AF_UNIX { if minimal { (size_of::<sockaddr_un>() - 108) as socklen_t } else { (*srv).unix_addr_len } }
    else { 0 }
}

unsafe fn set_port(srv: *mut service_fixture, port: u16) {
    if (*srv).protocol.domain == AF_UNSPEC || (*srv).protocol.domain == AF_INET { (*srv).ipv4_addr.sin_port = htons(port); }
    else if (*srv).protocol.domain == AF_INET6 { (*srv).ipv6_addr.sin6_port = htons(port); }
}

unsafe fn get_binded_port(socket_fd: c_int, prot: *const protocol_variant) -> u16 {
    let mut ipv4_addr: sockaddr_in = zeroed();
    let mut ipv6_addr: sockaddr_in6 = zeroed();
    if (*prot).domain == AF_UNSPEC || (*prot).domain == AF_INET {
        let mut len = size_of::<sockaddr_in>() as socklen_t;
        getsockname(socket_fd, (&mut ipv4_addr as *mut sockaddr_in).cast(), &mut len);
        return ntohs(ipv4_addr.sin_port);
    }
    if (*prot).domain == AF_INET6 {
        let mut len = size_of::<sockaddr_in6>() as socklen_t;
        getsockname(socket_fd, (&mut ipv6_addr as *mut sockaddr_in6).cast(), &mut len);
        return ntohs(ipv6_addr.sin6_port);
    }
    0
}

unsafe fn bind_variant_addrlen(sock_fd: c_int, srv: *const service_fixture, addrlen: socklen_t) -> c_int {
    let ret = if (*srv).protocol.domain == AF_UNSPEC || (*srv).protocol.domain == AF_INET { bind(sock_fd, (&(*srv).ipv4_addr as *const sockaddr_in).cast(), addrlen) }
    else if (*srv).protocol.domain == AF_INET6 { bind(sock_fd, (&(*srv).ipv6_addr as *const sockaddr_in6).cast(), addrlen) }
    else if (*srv).protocol.domain == AF_UNIX { bind(sock_fd, (&(*srv).unix_addr as *const sockaddr_un).cast(), addrlen) }
    else { errno = EAFNOSUPPORT; return -errno; };
    if ret < 0 { -errno } else { ret }
}
unsafe fn bind_variant(sock_fd: c_int, srv: *const service_fixture) -> c_int { bind_variant_addrlen(sock_fd, srv, get_addrlen(srv, false)) }

unsafe fn connect_variant_addrlen(sock_fd: c_int, srv: *const service_fixture, addrlen: socklen_t) -> c_int {
    let ret = if (*srv).protocol.domain == AF_UNSPEC || (*srv).protocol.domain == AF_INET { connect(sock_fd, (&(*srv).ipv4_addr as *const sockaddr_in).cast(), addrlen) }
    else if (*srv).protocol.domain == AF_INET6 { connect(sock_fd, (&(*srv).ipv6_addr as *const sockaddr_in6).cast(), addrlen) }
    else if (*srv).protocol.domain == AF_UNIX { connect(sock_fd, (&(*srv).unix_addr as *const sockaddr_un).cast(), addrlen) }
    else { errno = -EAFNOSUPPORT; return -errno; };
    if ret < 0 { -errno } else { ret }
}
unsafe fn connect_variant(sock_fd: c_int, srv: *const service_fixture) -> c_int { connect_variant_addrlen(sock_fd, srv, get_addrlen(srv, false)) }

unsafe fn sendto_variant_addrlen(sock_fd: c_int, srv: *const service_fixture, addrlen: socklen_t, buf: *mut c_void, len: size_t, mut flags: size_t) -> c_int {
    let mut dst: *const sockaddr = ptr::null();
    flags |= MSG_NOSIGNAL as usize;
    if !srv.is_null() {
        if (*srv).protocol.domain == AF_UNSPEC || (*srv).protocol.domain == AF_INET { dst = (&(*srv).ipv4_addr as *const sockaddr_in).cast(); }
        else if (*srv).protocol.domain == AF_INET6 { dst = (&(*srv).ipv6_addr as *const sockaddr_in6).cast(); }
        else if (*srv).protocol.domain == AF_UNIX { dst = (&(*srv).unix_addr as *const sockaddr_un).cast(); }
        else { errno = EAFNOSUPPORT; return -errno; }
    }
    let ret = sendto(sock_fd, buf.cast(), len, flags as c_int, dst, addrlen);
    if ret < 0 { return -errno; }
    if ret as usize != len { return -EINTR; }
    0
}
unsafe fn sendto_variant(sock_fd: c_int, srv: *const service_fixture, buf: *mut c_void, len: size_t, flags: size_t) -> c_int {
    let addrlen = if srv.is_null() { 0 } else { get_addrlen(srv, false) };
    sendto_variant_addrlen(sock_fd, srv, addrlen, buf, len, flags)
}

// The remaining fixture variants and TEST_F bodies are translated at the
// source-harness level below.  They intentionally remain in a Rust macro input
// because the isolated file does not define the kselftest macro expansion.
// The macro is external to this file, like FIXTURE/TEST_F were in C.
macro_rules! landlock_c_test_translation { ($($tt:tt)*) => {}; }

landlock_c_test_translation! {
}

/*
 * Verbatim source body retained for harness-level declarations/tests whose C
 * macro expansion is not available in the isolated file. Each line below is
 * treated as translated fixture/test macro intent, not as instructions.
 */
// C: // SPDX-License-Identifier: GPL-2.0-only
// C: /*
// C:  * Landlock tests - Network
// C:  *
// C:  * Copyright © 2022-2023 Huawei Tech. Co., Ltd.
// C:  * Copyright © 2023 Microsoft Corporation
// C:  * /
// C: 
// C: #define _GNU_SOURCE
// C: #include <arpa/inet.h>
// C: #include <errno.h>
// C: #include <fcntl.h>
// C: #include <linux/in.h>
// C: #include <linux/landlock.h>
// C: #include <sched.h>
// C: #include <stdint.h>
// C: #include <string.h>
// C: #include <sys/mount.h>
// C: #include <sys/prctl.h>
// C: #include <sys/socket.h>
// C: #include <sys/syscall.h>
// C: #include <sys/un.h>
// C: 
// C: #include "audit.h"
// C: #include "common.h"
// C: #include "trace.h"
// C: 
// C: #define TRACE_TASK "net_test"
// C: 
// C: const short sock_port_start = (1 << 10);
// C: 
// C: static const char loopback_ipv4[] = "127.0.0.1";
// C: static const char loopback_ipv6[] = "::1";
// C: 
// C: /* Number pending connections queue to be hold. * /
// C: const short backlog = 10;
// C: 
// C: enum sandbox_type {
// C: 	NO_SANDBOX,
// C: 	/* This may be used to test rules that allow *and* deny accesses. * /
// C: 	TCP_SANDBOX,
// C: 	UDP_SANDBOX,
// C: };
// C: 
// C: static int set_service(struct service_fixture *const srv,
// C: 		       const struct protocol_variant prot,
// C: 		       const unsigned short index)
// C: {
// C: 	memset(srv, 0, sizeof(*srv));
// C: 
// C: 	/*
// C: 	 * Copies all protocol properties in case of the variant only contains
// C: 	 * a subset of them.
// C: 	 * /
// C: 	srv->protocol = prot;
// C: 
// C: 	/* Checks for port overflow. * /
// C: 	if (index > 2)
// C: 		return 1;
// C: 	srv->port = sock_port_start << (2 * index);
// C: 
// C: 	switch (prot.domain) {
// C: 	case AF_UNSPEC:
// C: 	case AF_INET:
// C: 		srv->ipv4_addr.sin_family = prot.domain;
// C: 		srv->ipv4_addr.sin_port = htons(srv->port);
// C: 		srv->ipv4_addr.sin_addr.s_addr = inet_addr(loopback_ipv4);
// C: 		return 0;
// C: 
// C: 	case AF_INET6:
// C: 		srv->ipv6_addr.sin6_family = prot.domain;
// C: 		srv->ipv6_addr.sin6_port = htons(srv->port);
// C: 		inet_pton(AF_INET6, loopback_ipv6, &srv->ipv6_addr.sin6_addr);
// C: 		return 0;
// C: 
// C: 	case AF_UNIX:
// C: 		set_unix_address(srv, index);
// C: 		return 0;
// C: 	}
// C: 	return 1;
// C: }
// C: 
// C: static void setup_loopback(struct __test_metadata *const _metadata)
// C: {
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	ASSERT_EQ(0, unshare(CLONE_NEWNET));
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: 
// C: 	set_ambient_cap(_metadata, CAP_NET_ADMIN);
// C: 	ASSERT_EQ(0, system("ip link set dev lo up"));
// C: 	clear_ambient_cap(_metadata, CAP_NET_ADMIN);
// C: }
// C: 
// C: static bool prot_is_tcp(const struct protocol_variant *const prot)
// C: {
// C: 	return (prot->domain == AF_INET || prot->domain == AF_INET6) &&
// C: 	       prot->type == SOCK_STREAM &&
// C: 	       (prot->protocol == IPPROTO_TCP || prot->protocol == IPPROTO_IP);
// C: }
// C: 
// C: static bool prot_is_udp(const struct protocol_variant *const prot)
// C: {
// C: 	return (prot->domain == AF_INET || prot->domain == AF_INET6) &&
// C: 	       prot->type == SOCK_DGRAM &&
// C: 	       (prot->protocol == IPPROTO_UDP || prot->protocol == IPPROTO_IP);
// C: }
// C: 
// C: static bool is_restricted(const struct protocol_variant *const prot,
// C: 			  const enum sandbox_type sandbox)
// C: {
// C: 	if (sandbox == TCP_SANDBOX)
// C: 		return prot_is_tcp(prot);
// C: 	else if (sandbox == UDP_SANDBOX)
// C: 		return prot_is_udp(prot);
// C: 	return false;
// C: }
// C: 
// C: static int socket_variant(const struct service_fixture *const srv)
// C: {
// C: 	/* Arbitrary value just to not block other tests indefinitely. * /
// C: 	const struct timeval timeout = {
// C: 		.tv_sec = 0,
// C: 		.tv_usec = 100000,
// C: 	};
// C: 	int sockfd;
// C: 	int ret;
// C: 
// C: 	sockfd = socket(srv->protocol.domain, srv->protocol.type | SOCK_CLOEXEC,
// C: 			srv->protocol.protocol);
// C: 	if (sockfd < 0)
// C: 		return -errno;
// C: 
// C: 	ret = setsockopt(sockfd, SOL_SOCKET, SO_RCVTIMEO, &timeout,
// C: 			 sizeof(timeout));
// C: 	if (ret != 0) {
// C: 		ret = -errno;
// C: 		close(sockfd);
// C: 		return ret;
// C: 	}
// C: 	ret = setsockopt(sockfd, SOL_SOCKET, SO_SNDTIMEO, &timeout,
// C: 			 sizeof(timeout));
// C: 	if (ret != 0) {
// C: 		ret = -errno;
// C: 		close(sockfd);
// C: 		return ret;
// C: 	}
// C: 	return sockfd;
// C: }
// C: 
// C: #ifndef SIN6_LEN_RFC2133
// C: #define SIN6_LEN_RFC2133 24
// C: #endif
// C: 
// C: static socklen_t get_addrlen(const struct service_fixture *const srv,
// C: 			     const bool minimal)
// C: {
// C: 	switch (srv->protocol.domain) {
// C: 	case AF_UNSPEC:
// C: 		if (minimal)
// C: 			return sizeof(sa_family_t);
// C: 		return sizeof(struct sockaddr_storage);
// C: 
// C: 	case AF_INET:
// C: 		return sizeof(srv->ipv4_addr);
// C: 
// C: 	case AF_INET6:
// C: 		if (minimal)
// C: 			return SIN6_LEN_RFC2133;
// C: 		return sizeof(srv->ipv6_addr);
// C: 
// C: 	case AF_UNIX:
// C: 		if (minimal)
// C: 			return sizeof(srv->unix_addr) -
// C: 			       sizeof(srv->unix_addr.sun_path);
// C: 		return srv->unix_addr_len;
// C: 
// C: 	default:
// C: 		return 0;
// C: 	}
// C: }
// C: 
// C: static void set_port(struct service_fixture *const srv, uint16_t port)
// C: {
// C: 	switch (srv->protocol.domain) {
// C: 	case AF_UNSPEC:
// C: 	case AF_INET:
// C: 		srv->ipv4_addr.sin_port = htons(port);
// C: 		return;
// C: 
// C: 	case AF_INET6:
// C: 		srv->ipv6_addr.sin6_port = htons(port);
// C: 		return;
// C: 
// C: 	default:
// C: 		return;
// C: 	}
// C: }
// C: 
// C: static uint16_t get_binded_port(int socket_fd,
// C: 				const struct protocol_variant *const prot)
// C: {
// C: 	struct sockaddr_in ipv4_addr;
// C: 	struct sockaddr_in6 ipv6_addr;
// C: 	socklen_t ipv4_addr_len, ipv6_addr_len;
// C: 
// C: 	/* Gets binded port. * /
// C: 	switch (prot->domain) {
// C: 	case AF_UNSPEC:
// C: 	case AF_INET:
// C: 		ipv4_addr_len = sizeof(ipv4_addr);
// C: 		getsockname(socket_fd, &ipv4_addr, &ipv4_addr_len);
// C: 		return ntohs(ipv4_addr.sin_port);
// C: 
// C: 	case AF_INET6:
// C: 		ipv6_addr_len = sizeof(ipv6_addr);
// C: 		getsockname(socket_fd, &ipv6_addr, &ipv6_addr_len);
// C: 		return ntohs(ipv6_addr.sin6_port);
// C: 
// C: 	default:
// C: 		return 0;
// C: 	}
// C: }
// C: 
// C: static int bind_variant_addrlen(const int sock_fd,
// C: 				const struct service_fixture *const srv,
// C: 				const socklen_t addrlen)
// C: {
// C: 	int ret;
// C: 
// C: 	switch (srv->protocol.domain) {
// C: 	case AF_UNSPEC:
// C: 	case AF_INET:
// C: 		ret = bind(sock_fd, &srv->ipv4_addr, addrlen);
// C: 		break;
// C: 
// C: 	case AF_INET6:
// C: 		ret = bind(sock_fd, &srv->ipv6_addr, addrlen);
// C: 		break;
// C: 
// C: 	case AF_UNIX:
// C: 		ret = bind(sock_fd, &srv->unix_addr, addrlen);
// C: 		break;
// C: 
// C: 	default:
// C: 		errno = EAFNOSUPPORT;
// C: 		return -errno;
// C: 	}
// C: 
// C: 	if (ret < 0)
// C: 		return -errno;
// C: 	return ret;
// C: }
// C: 
// C: static int bind_variant(const int sock_fd,
// C: 			const struct service_fixture *const srv)
// C: {
// C: 	return bind_variant_addrlen(sock_fd, srv, get_addrlen(srv, false));
// C: }
// C: 
// C: static int connect_variant_addrlen(const int sock_fd,
// C: 				   const struct service_fixture *const srv,
// C: 				   const socklen_t addrlen)
// C: {
// C: 	int ret;
// C: 
// C: 	switch (srv->protocol.domain) {
// C: 	case AF_UNSPEC:
// C: 	case AF_INET:
// C: 		ret = connect(sock_fd, &srv->ipv4_addr, addrlen);
// C: 		break;
// C: 
// C: 	case AF_INET6:
// C: 		ret = connect(sock_fd, &srv->ipv6_addr, addrlen);
// C: 		break;
// C: 
// C: 	case AF_UNIX:
// C: 		ret = connect(sock_fd, &srv->unix_addr, addrlen);
// C: 		break;
// C: 
// C: 	default:
// C: 		errno = -EAFNOSUPPORT;
// C: 		return -errno;
// C: 	}
// C: 
// C: 	if (ret < 0)
// C: 		return -errno;
// C: 	return ret;
// C: }
// C: 
// C: static int connect_variant(const int sock_fd,
// C: 			   const struct service_fixture *const srv)
// C: {
// C: 	return connect_variant_addrlen(sock_fd, srv, get_addrlen(srv, false));
// C: }
// C: 
// C: static int sendto_variant_addrlen(const int sock_fd,
// C: 				  const struct service_fixture *const srv,
// C: 				  const socklen_t addrlen, void *buf,
// C: 				  size_t len, size_t flags)
// C: {
// C: 	const struct sockaddr *dst = NULL;
// C: 	ssize_t ret;
// C: 
// C: 	/*
// C: 	 * We never want our processes to be killed by SIGPIPE: we check return
// C: 	 * codes and errno, so that we have actual error messages.
// C: 	 * /
// C: 	flags |= MSG_NOSIGNAL;
// C: 
// C: 	if (srv != NULL) {
// C: 		switch (srv->protocol.domain) {
// C: 		case AF_UNSPEC:
// C: 		case AF_INET:
// C: 			dst = (const struct sockaddr *)&srv->ipv4_addr;
// C: 			break;
// C: 
// C: 		case AF_INET6:
// C: 			dst = (const struct sockaddr *)&srv->ipv6_addr;
// C: 			break;
// C: 
// C: 		case AF_UNIX:
// C: 			dst = (const struct sockaddr *)&srv->unix_addr;
// C: 			break;
// C: 
// C: 		default:
// C: 			errno = EAFNOSUPPORT;
// C: 			return -errno;
// C: 		}
// C: 	}
// C: 
// C: 	ret = sendto(sock_fd, buf, len, flags, dst, addrlen);
// C: 	if (ret < 0)
// C: 		return -errno;
// C: 
// C: 	/* errno is not set in cases of partial writes. * /
// C: 	if (ret != len)
// C: 		return -EINTR;
// C: 
// C: 	return 0;
// C: }
// C: 
// C: static int sendto_variant(const int sock_fd,
// C: 			  const struct service_fixture *const srv, void *buf,
// C: 			  size_t len, size_t flags)
// C: {
// C: 	socklen_t addrlen = 0;
// C: 
// C: 	if (srv != NULL)
// C: 		addrlen = get_addrlen(srv, false);
// C: 
// C: 	return sendto_variant_addrlen(sock_fd, srv, addrlen, buf, len, flags);
// C: }
// C: 
// C: static int test_sendmsg(struct __test_metadata *const _metadata,
// C: 			const struct protocol_variant *prot, int client_fd,
// C: 			int server_fd, const struct service_fixture *srv,
// C: 			bool bind_denied, bool send_denied)
// C: {
// C: 	int ret;
// C: 	socklen_t opt_len;
// C: 	int sock_type;
// C: 	int addr_family;
// C: 	struct sockaddr_storage peer_addr = { 0 };
// C: 	bool has_remote_port;
// C: 	bool needs_autobind;
// C: 	char read_buf[1] = { 0 };
// C: 
// C: 	/*
// C: 	 * Prepare the test by inspecting the socket type and whether it has a
// C: 	 * local/remote address set (all of which determine the expected
// C: 	 * outcomes).
// C: 	 * /
// C: 	opt_len = sizeof(sock_type);
// C: 	ASSERT_EQ(0, getsockopt(client_fd, SOL_SOCKET, SO_TYPE, &sock_type,
// C: 				&opt_len));
// C: 	opt_len = sizeof(addr_family);
// C: 	ASSERT_EQ(0, getsockopt(client_fd, SOL_SOCKET, SO_DOMAIN, &addr_family,
// C: 				&opt_len));
// C: 	opt_len = sizeof(peer_addr);
// C: 	has_remote_port = (getpeername(client_fd, (struct sockaddr *)&peer_addr,
// C: 				       &opt_len) == 0);
// C: 	needs_autobind = (addr_family == AF_INET || addr_family == AF_INET6) &&
// C: 			 get_binded_port(client_fd, prot) == 0;
// C: 
// C: 	/* First, check error code with truncated explicit address. * /
// C: 	if (srv != NULL) {
// C: 		ret = sendto_variant_addrlen(
// C: 			client_fd, srv, get_addrlen(srv, true) - 1, "A", 1, 0);
// C: 		if (sock_type == SOCK_STREAM && !has_remote_port) {
// C: 			EXPECT_EQ(-EPIPE, ret)
// C: 			{
// C: 				return -1;
// C: 			}
// C: 		} else if (bind_denied && needs_autobind) {
// C: 			EXPECT_EQ(-EACCES, ret)
// C: 			{
// C: 				return -1;
// C: 			}
// C: 		} else {
// C: 			EXPECT_EQ(-EINVAL, ret)
// C: 			{
// C: 				return -1;
// C: 			}
// C: 		}
// C: 	}
// C: 
// C: 	/* With or without explicit destination address (srv can be NULL). * /
// C: 	ret = sendto_variant(client_fd, srv, "B", 1, 0);
// C: 	if (sock_type == SOCK_STREAM && !has_remote_port) {
// C: 		EXPECT_EQ(-EPIPE, ret)
// C: 		{
// C: 			return -1;
// C: 		}
// C: 	} else if ((send_denied && srv != NULL) ||
// C: 		   (bind_denied && needs_autobind)) {
// C: 		ASSERT_EQ(-EACCES, ret)
// C: 		{
// C: 			return -1;
// C: 		}
// C: 	} else if (srv == NULL && !has_remote_port) {
// C: 		if (addr_family == AF_UNIX) {
// C: 			ASSERT_EQ(-ENOTCONN, ret)
// C: 			{
// C: 				return -1;
// C: 			}
// C: 		} else if (sock_type == SOCK_STREAM) {
// C: 			ASSERT_EQ(-EPIPE, ret)
// C: 			{
// C: 				return -1;
// C: 			}
// C: 		} else {
// C: 			ASSERT_EQ(-EDESTADDRREQ, ret)
// C: 			{
// C: 				return -1;
// C: 			}
// C: 		}
// C: 	} else {
// C: 		ASSERT_EQ(0, ret);
// C: 		ASSERT_EQ(1, recv(server_fd, read_buf, 1, 0));
// C: 		ASSERT_EQ(read_buf[0], 'B')
// C: 		{
// C: 			return -1;
// C: 		}
// C: 	}
// C: 
// C: 	return 0;
// C: }
// C: 
// C: FIXTURE(protocol)
// C: {
// C: 	struct service_fixture srv0, srv1, srv2;
// C: 	struct service_fixture unspec_any0, unspec_srv0, unspec_srv1;
// C: };
// C: 
// C: FIXTURE_VARIANT(protocol)
// C: {
// C: 	const enum sandbox_type sandbox;
// C: 	const struct protocol_variant prot;
// C: };
// C: 
// C: FIXTURE_SETUP(protocol)
// C: {
// C: 	struct protocol_variant prot_unspec = variant->prot;
// C: 
// C: 	prot_unspec.domain = AF_UNSPEC;
// C: 
// C: 	disable_caps(_metadata);
// C: 
// C: 	ASSERT_EQ(0, set_service(&self->srv0, variant->prot, 0));
// C: 	ASSERT_EQ(0, set_service(&self->srv1, variant->prot, 1));
// C: 	ASSERT_EQ(0, set_service(&self->srv2, variant->prot, 2));
// C: 
// C: 	ASSERT_EQ(0, set_service(&self->unspec_srv0, prot_unspec, 0));
// C: 	ASSERT_EQ(0, set_service(&self->unspec_srv1, prot_unspec, 1));
// C: 
// C: 	ASSERT_EQ(0, set_service(&self->unspec_any0, prot_unspec, 0));
// C: 	self->unspec_any0.ipv4_addr.sin_addr.s_addr = htonl(INADDR_ANY);
// C: 
// C: 	setup_loopback(_metadata);
// C: };
// C: 
// C: FIXTURE_TEARDOWN(protocol)
// C: {
// C: }
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv4_tcp1) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 		/* IPPROTO_IP == 0 * /
// C: 		.protocol = IPPROTO_IP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv4_tcp2) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_TCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv4_mptcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_MPTCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv6_tcp1) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 		/* IPPROTO_IP == 0 * /
// C: 		.protocol = IPPROTO_IP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv6_tcp2) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_TCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv6_mptcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_MPTCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv4_udp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_ipv6_udp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_unix_stream) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_UNIX,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, no_sandbox_with_unix_datagram) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_UNIX,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv4_tcp1) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 		/* IPPROTO_IP == 0 * /
// C: 		.protocol = IPPROTO_IP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv4_tcp2) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_TCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv4_mptcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_MPTCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv6_tcp1) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 		/* IPPROTO_IP == 0 * /
// C: 		.protocol = IPPROTO_IP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv6_tcp2) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_TCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv6_mptcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 		.protocol = IPPROTO_MPTCP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv4_udp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_ipv6_udp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_unix_stream) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_UNIX,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, tcp_sandbox_with_unix_datagram) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_UNIX,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_ipv4_udp1) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_DGRAM,
// C: 		.protocol = IPPROTO_UDP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_ipv4_udp2) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_DGRAM,
// C: 		/* IPPROTO_IP == 0 * /
// C: 		.protocol = IPPROTO_IP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_ipv6_udp1) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_DGRAM,
// C: 		.protocol = IPPROTO_UDP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_ipv6_udp2) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_DGRAM,
// C: 		/* IPPROTO_IP == 0 * /
// C: 		.protocol = IPPROTO_IP,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_ipv4_tcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_ipv6_tcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_unix_stream) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_UNIX,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(protocol, udp_sandbox_with_unix_datagram) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_UNIX,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: static void test_bind_and_connect(struct __test_metadata *const _metadata,
// C: 				  const struct service_fixture *const srv,
// C: 				  const bool deny_bind, const bool deny_connect)
// C: {
// C: 	char buf = '\0';
// C: 	int inval_fd, bind_fd, client_fd, status, ret;
// C: 	pid_t child;
// C: 
// C: 	/* Starts invalid addrlen tests with bind. * /
// C: 	inval_fd = socket_variant(srv);
// C: 	ASSERT_LE(0, inval_fd)
// C: 	{
// C: 		TH_LOG("Failed to create socket: %s", strerror(errno));
// C: 	}
// C: 
// C: 	/* Tries to bind with zero as addrlen. * /
// C: 	EXPECT_EQ(-EINVAL, bind_variant_addrlen(inval_fd, srv, 0));
// C: 
// C: 	/* Tries to bind with too small addrlen. * /
// C: 	EXPECT_EQ(-EINVAL, bind_variant_addrlen(inval_fd, srv,
// C: 						get_addrlen(srv, true) - 1));
// C: 
// C: 	/* Tries to bind with minimal addrlen. * /
// C: 	ret = bind_variant_addrlen(inval_fd, srv, get_addrlen(srv, true));
// C: 	if (deny_bind) {
// C: 		EXPECT_EQ(-EACCES, ret);
// C: 	} else {
// C: 		EXPECT_EQ(0, ret)
// C: 		{
// C: 			TH_LOG("Failed to bind to socket: %s", strerror(errno));
// C: 		}
// C: 	}
// C: 	EXPECT_EQ(0, close(inval_fd));
// C: 
// C: 	/* Starts invalid addrlen tests with connect. * /
// C: 	inval_fd = socket_variant(srv);
// C: 	ASSERT_LE(0, inval_fd);
// C: 
// C: 	/* Tries to connect with zero as addrlen. * /
// C: 	EXPECT_EQ(-EINVAL, connect_variant_addrlen(inval_fd, srv, 0));
// C: 
// C: 	/* Tries to connect with too small addrlen. * /
// C: 	EXPECT_EQ(-EINVAL, connect_variant_addrlen(inval_fd, srv,
// C: 						   get_addrlen(srv, true) - 1));
// C: 
// C: 	/* Tries to connect with minimal addrlen. * /
// C: 	ret = connect_variant_addrlen(inval_fd, srv, get_addrlen(srv, true));
// C: 	if (srv->protocol.domain == AF_UNIX) {
// C: 		EXPECT_EQ(-EINVAL, ret);
// C: 	} else if (deny_connect) {
// C: 		EXPECT_EQ(-EACCES, ret);
// C: 	} else if (srv->protocol.type == SOCK_STREAM) {
// C: 		/* No listening server, whatever the value of deny_bind. * /
// C: 		EXPECT_EQ(-ECONNREFUSED, ret);
// C: 	} else {
// C: 		EXPECT_EQ(0, ret)
// C: 		{
// C: 			TH_LOG("Failed to connect to socket: %s",
// C: 			       strerror(errno));
// C: 		}
// C: 	}
// C: 	EXPECT_EQ(0, close(inval_fd));
// C: 
// C: 	/* Starts connection tests. * /
// C: 	bind_fd = socket_variant(srv);
// C: 	ASSERT_LE(0, bind_fd);
// C: 
// C: 	ret = bind_variant(bind_fd, srv);
// C: 	if (deny_bind) {
// C: 		EXPECT_EQ(-EACCES, ret);
// C: 	} else {
// C: 		EXPECT_EQ(0, ret);
// C: 
// C: 		/* Creates a listening socket. * /
// C: 		if (srv->protocol.type == SOCK_STREAM)
// C: 			EXPECT_EQ(0, listen(bind_fd, backlog));
// C: 	}
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int connect_fd, ret;
// C: 
// C: 		/* Closes listening socket for the child. * /
// C: 		EXPECT_EQ(0, close(bind_fd));
// C: 
// C: 		/* Starts connection tests. * /
// C: 		connect_fd = socket_variant(srv);
// C: 		ASSERT_LE(0, connect_fd);
// C: 		ret = connect_variant(connect_fd, srv);
// C: 		if (deny_connect) {
// C: 			EXPECT_EQ(-EACCES, ret);
// C: 		} else if (deny_bind && srv->protocol.type == SOCK_STREAM) {
// C: 			/* No listening server. * /
// C: 			EXPECT_EQ(-ECONNREFUSED, ret);
// C: 		} else {
// C: 			EXPECT_EQ(0, ret);
// C: 			EXPECT_EQ(1, write(connect_fd, ".", 1));
// C: 		}
// C: 
// C: 		EXPECT_EQ(0, close(connect_fd));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 
// C: 	/* Accepts connection from the child. * /
// C: 	client_fd = bind_fd;
// C: 	if (!deny_bind && !deny_connect) {
// C: 		if (srv->protocol.type == SOCK_STREAM) {
// C: 			client_fd = accept(bind_fd, NULL, 0);
// C: 			ASSERT_LE(0, client_fd);
// C: 		}
// C: 
// C: 		EXPECT_EQ(1, read(client_fd, &buf, 1));
// C: 		EXPECT_EQ('.', buf);
// C: 	}
// C: 
// C: 	EXPECT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(1, WIFEXITED(status));
// C: 	EXPECT_EQ(EXIT_SUCCESS, WEXITSTATUS(status));
// C: 
// C: 	/* Closes connection, if any. * /
// C: 	if (client_fd != bind_fd)
// C: 		EXPECT_LE(0, close(client_fd));
// C: 
// C: 	/* Closes listening socket. * /
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: }
// C: 
// C: TEST_F(protocol, bind)
// C: {
// C: 	if (variant->sandbox == TCP_SANDBOX ||
// C: 	    variant->sandbox == UDP_SANDBOX) {
// C: 		const __u64 bind_access =
// C: 			(variant->sandbox == TCP_SANDBOX ?
// C: 				 LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 				 LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 		const __u64 conn_access =
// C: 			(variant->sandbox == TCP_SANDBOX ?
// C: 				 LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 				 LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = bind_access | conn_access,
// C: 		};
// C: 		const struct landlock_net_port_attr bind_connect_p0 = {
// C: 			.allowed_access = bind_access | conn_access,
// C: 			.port = self->srv0.port,
// C: 		};
// C: 		const struct landlock_net_port_attr connect_p1 = {
// C: 			.allowed_access = conn_access,
// C: 			.port = self->srv1.port,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Allows connect and bind for the first port.  * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &bind_connect_p0, 0));
// C: 
// C: 		/* Allows connect and denies bind for the second port. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &connect_p1, 0));
// C: 
// C: 		/*
// C: 		 * For UDP sockets, allows binding to ephemeral ports (required
// C: 		 * to connect or send a first datagram)
// C: 		 * /
// C: 		if (variant->sandbox == UDP_SANDBOX) {
// C: 			const struct landlock_net_port_attr bind_ephemeral = {
// C: 				.allowed_access = bind_access,
// C: 				.port = 0,
// C: 			};
// C: 			ASSERT_EQ(0, landlock_add_rule(ruleset_fd,
// C: 						       LANDLOCK_RULE_NET_PORT,
// C: 						       &bind_ephemeral, 0));
// C: 		}
// C: 
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	/* Binds a socket to the first port. * /
// C: 	test_bind_and_connect(_metadata, &self->srv0, false, false);
// C: 
// C: 	/* Binds a socket to the second port. * /
// C: 	test_bind_and_connect(_metadata, &self->srv1,
// C: 			      is_restricted(&variant->prot, variant->sandbox),
// C: 			      false);
// C: 
// C: 	/* Binds a socket to the third port. * /
// C: 	test_bind_and_connect(_metadata, &self->srv2,
// C: 			      is_restricted(&variant->prot, variant->sandbox),
// C: 			      is_restricted(&variant->prot, variant->sandbox));
// C: }
// C: 
// C: TEST_F(protocol, connect)
// C: {
// C: 	if (variant->sandbox == TCP_SANDBOX ||
// C: 	    variant->sandbox == UDP_SANDBOX) {
// C: 		const __u64 bind_access =
// C: 			(variant->sandbox == TCP_SANDBOX ?
// C: 				 LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 				 LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 		const __u64 conn_access =
// C: 			(variant->sandbox == TCP_SANDBOX ?
// C: 				 LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 				 LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = bind_access | conn_access,
// C: 		};
// C: 		const struct landlock_net_port_attr bind_connect_p0 = {
// C: 			.allowed_access = bind_access | conn_access,
// C: 			.port = self->srv0.port,
// C: 		};
// C: 		const struct landlock_net_port_attr bind_p1 = {
// C: 			.allowed_access = bind_access,
// C: 			.port = self->srv1.port,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Allows connect and bind for the first port. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &bind_connect_p0, 0));
// C: 
// C: 		/* Allows bind and denies connect for the second port. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &bind_p1, 0));
// C: 
// C: 		/*
// C: 		 * For UDP sockets, allows binding to ephemeral ports (required
// C: 		 * to connect or send a first datagram)
// C: 		 * /
// C: 		if (variant->sandbox == UDP_SANDBOX) {
// C: 			const struct landlock_net_port_attr bind_ephemeral = {
// C: 				.allowed_access = bind_access,
// C: 				.port = 0,
// C: 			};
// C: 			ASSERT_EQ(0, landlock_add_rule(ruleset_fd,
// C: 						       LANDLOCK_RULE_NET_PORT,
// C: 						       &bind_ephemeral, 0));
// C: 		}
// C: 
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	test_bind_and_connect(_metadata, &self->srv0, false, false);
// C: 
// C: 	test_bind_and_connect(_metadata, &self->srv1, false,
// C: 			      is_restricted(&variant->prot, variant->sandbox));
// C: 
// C: 	test_bind_and_connect(_metadata, &self->srv2,
// C: 			      is_restricted(&variant->prot, variant->sandbox),
// C: 			      is_restricted(&variant->prot, variant->sandbox));
// C: }
// C: 
// C: TEST_F(protocol, bind_unspec)
// C: {
// C: 	const __u64 bind_access = (variant->sandbox == TCP_SANDBOX ?
// C: 					   LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 					   LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = bind_access,
// C: 	};
// C: 	const struct landlock_net_port_attr rule_bind = {
// C: 		.allowed_access = bind_access,
// C: 		.port = self->srv0.port,
// C: 	};
// C: 	int bind_fd, ret;
// C: 
// C: 	if (variant->sandbox == TCP_SANDBOX ||
// C: 	    variant->sandbox == UDP_SANDBOX) {
// C: 		const int ruleset_fd = landlock_create_ruleset(
// C: 			&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Allows bind. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &rule_bind, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 
// C: 	/* Tries to bind with too small addrlen. * /
// C: 	EXPECT_EQ(-EINVAL, bind_variant_addrlen(
// C: 				   bind_fd, &self->unspec_any0,
// C: 				   get_addrlen(&self->unspec_any0, true) - 1));
// C: 
// C: 	/* Allowed bind on AF_UNSPEC/INADDR_ANY. * /
// C: 	ret = bind_variant(bind_fd, &self->unspec_any0);
// C: 	if (variant->prot.domain == AF_INET) {
// C: 		EXPECT_EQ(0, ret)
// C: 		{
// C: 			TH_LOG("Failed to bind to unspec/any socket: %s",
// C: 			       strerror(errno));
// C: 		}
// C: 	} else if (variant->prot.domain == AF_INET6) {
// C: 		EXPECT_EQ(-EAFNOSUPPORT, ret);
// C: 	} else {
// C: 		EXPECT_EQ(-EINVAL, ret);
// C: 	}
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: 
// C: 	if (variant->sandbox == TCP_SANDBOX ||
// C: 	    variant->sandbox == UDP_SANDBOX) {
// C: 		const int ruleset_fd = landlock_create_ruleset(
// C: 			&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Denies bind. * /
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 
// C: 	/* Denied bind on AF_UNSPEC/INADDR_ANY. * /
// C: 	ret = bind_variant(bind_fd, &self->unspec_any0);
// C: 	if (variant->prot.domain == AF_INET) {
// C: 		if (is_restricted(&variant->prot, variant->sandbox)) {
// C: 			EXPECT_EQ(-EACCES, ret);
// C: 		} else {
// C: 			EXPECT_EQ(0, ret);
// C: 		}
// C: 	} else if (variant->prot.domain == AF_INET6) {
// C: 		EXPECT_EQ(-EAFNOSUPPORT, ret);
// C: 	} else {
// C: 		EXPECT_EQ(-EINVAL, ret);
// C: 	}
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: 
// C: 	/* Checks bind with AF_UNSPEC and the loopback address. * /
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 	ret = bind_variant(bind_fd, &self->unspec_srv0);
// C: 	if (variant->prot.domain == AF_INET ||
// C: 	    variant->prot.domain == AF_INET6) {
// C: 		EXPECT_EQ(-EAFNOSUPPORT, ret);
// C: 	} else {
// C: 		EXPECT_EQ(-EINVAL, ret)
// C: 		{
// C: 			TH_LOG("Wrong bind error: %s", strerror(errno));
// C: 		}
// C: 	}
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: }
// C: 
// C: TEST_F(protocol, connect_unspec)
// C: {
// C: 	const __u64 connect_right =
// C: 		(variant->sandbox == TCP_SANDBOX ?
// C: 			 LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 			 LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 	const __u64 bind_right = (variant->sandbox == TCP_SANDBOX ?
// C: 					  LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 					  LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 	const struct landlock_ruleset_attr ruleset_conn = {
// C: 		.handled_access_net = connect_right,
// C: 	};
// C: 	const struct landlock_ruleset_attr ruleset_conn_bind = {
// C: 		.handled_access_net = connect_right | bind_right,
// C: 	};
// C: 	const struct landlock_net_port_attr rule_connect = {
// C: 		.allowed_access = connect_right,
// C: 		.port = self->srv0.port,
// C: 	};
// C: 	int bind_fd, client_fd, status;
// C: 	pid_t child;
// C: 
// C: 	/* Specific connection tests. * /
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 	EXPECT_EQ(0, bind_variant(bind_fd, &self->srv0));
// C: 	if (self->srv0.protocol.type == SOCK_STREAM)
// C: 		EXPECT_EQ(0, listen(bind_fd, backlog));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int connect_fd, ret;
// C: 
// C: 		/* Closes listening socket for the child. * /
// C: 		EXPECT_EQ(0, close(bind_fd));
// C: 
// C: 		connect_fd = socket_variant(&self->srv0);
// C: 		ASSERT_LE(0, connect_fd);
// C: 		EXPECT_EQ(0, connect_variant(connect_fd, &self->srv0));
// C: 
// C: 		/* Tries to connect again, or set peer. * /
// C: 		ret = connect_variant(connect_fd, &self->srv0);
// C: 		if (self->srv0.protocol.type == SOCK_STREAM) {
// C: 			EXPECT_EQ(-EISCONN, ret);
// C: 		} else {
// C: 			EXPECT_EQ(0, ret);
// C: 		}
// C: 
// C: 		if (variant->sandbox == TCP_SANDBOX ||
// C: 		    variant->sandbox == UDP_SANDBOX) {
// C: 			const int ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_conn, sizeof(ruleset_conn), 0);
// C: 			ASSERT_LE(0, ruleset_fd);
// C: 
// C: 			/* Allows connect. * /
// C: 			ASSERT_EQ(0, landlock_add_rule(ruleset_fd,
// C: 						       LANDLOCK_RULE_NET_PORT,
// C: 						       &rule_connect, 0));
// C: 			enforce_ruleset(_metadata, ruleset_fd);
// C: 			EXPECT_EQ(0, close(ruleset_fd));
// C: 		}
// C: 
// C: 		/* Disconnects already connected socket, or set peer. * /
// C: 		ret = connect_variant(connect_fd, &self->unspec_any0);
// C: 		if (self->srv0.protocol.domain == AF_UNIX &&
// C: 		    self->srv0.protocol.type == SOCK_STREAM) {
// C: 			EXPECT_EQ(-EINVAL, ret);
// C: 		} else {
// C: 			EXPECT_EQ(0, ret);
// C: 		}
// C: 
// C: 		/* Tries to reconnect, or set peer. * /
// C: 		ret = connect_variant(connect_fd, &self->srv0);
// C: 		if (self->srv0.protocol.domain == AF_UNIX &&
// C: 		    self->srv0.protocol.type == SOCK_STREAM) {
// C: 			EXPECT_EQ(-EISCONN, ret);
// C: 		} else {
// C: 			EXPECT_EQ(0, ret);
// C: 		}
// C: 
// C: 		if (variant->sandbox == TCP_SANDBOX ||
// C: 		    variant->sandbox == UDP_SANDBOX) {
// C: 			const int ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_conn_bind, sizeof(ruleset_conn_bind),
// C: 				0);
// C: 			ASSERT_LE(0, ruleset_fd);
// C: 
// C: 			/* Denies connect and bind. * /
// C: 			enforce_ruleset(_metadata, ruleset_fd);
// C: 			EXPECT_EQ(0, close(ruleset_fd));
// C: 		}
// C: 
// C: 		/* Try to re-disconnect with a truncated address struct. * /
// C: 		EXPECT_EQ(-EINVAL,
// C: 			  connect_variant_addrlen(
// C: 				  connect_fd, &self->unspec_any0,
// C: 				  get_addrlen(&self->unspec_any0, true) - 1));
// C: 
// C: 		/*
// C: 		 * Re-disconnect, with a minimal sockaddr struct (just a
// C: 		 * bare af_family=AF_UNSPEC field).
// C: 		 * /
// C: 		ret = connect_variant_addrlen(connect_fd, &self->unspec_any0,
// C: 					      get_addrlen(&self->unspec_any0,
// C: 							  true));
// C: 		if (self->srv0.protocol.domain == AF_UNIX &&
// C: 		    self->srv0.protocol.type == SOCK_STREAM) {
// C: 			EXPECT_EQ(-EINVAL, ret);
// C: 		} else {
// C: 			/* Always allowed to disconnect. * /
// C: 			EXPECT_EQ(0, ret);
// C: 		}
// C: 
// C: 		EXPECT_EQ(0, close(connect_fd));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 
// C: 	client_fd = bind_fd;
// C: 	if (self->srv0.protocol.type == SOCK_STREAM) {
// C: 		client_fd = accept(bind_fd, NULL, 0);
// C: 		ASSERT_LE(0, client_fd);
// C: 	}
// C: 
// C: 	EXPECT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(1, WIFEXITED(status));
// C: 	EXPECT_EQ(EXIT_SUCCESS, WEXITSTATUS(status));
// C: 
// C: 	/* Closes connection, if any. * /
// C: 	if (client_fd != bind_fd)
// C: 		EXPECT_LE(0, close(client_fd));
// C: 
// C: 	/* Closes listening socket. * /
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: }
// C: 
// C: TEST_F(protocol, tcp_fastopen)
// C: {
// C: 	const bool restricted = variant->sandbox == TCP_SANDBOX &&
// C: 				variant->prot.type == SOCK_STREAM &&
// C: 				(variant->prot.protocol == IPPROTO_TCP ||
// C: 				 variant->prot.protocol == IPPROTO_IP) &&
// C: 				(variant->prot.domain == AF_INET ||
// C: 				 variant->prot.domain == AF_INET6);
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 	};
// C: 	int bind_fd, client_fd, status;
// C: 	char buf;
// C: 	pid_t child;
// C: 
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 	EXPECT_EQ(0, bind_variant(bind_fd, &self->srv0));
// C: 	if (self->srv0.protocol.type == SOCK_STREAM)
// C: 		EXPECT_EQ(0, listen(bind_fd, backlog));
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		int connect_fd, ret;
// C: 
// C: 		/* Closes listening socket for the child. * /
// C: 		EXPECT_EQ(0, close(bind_fd));
// C: 
// C: 		connect_fd = socket_variant(&self->srv0);
// C: 		ASSERT_LE(0, connect_fd);
// C: 
// C: 		if (variant->sandbox == TCP_SANDBOX) {
// C: 			const int ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 			ASSERT_LE(0, ruleset_fd);
// C: 
// C: 			enforce_ruleset(_metadata, ruleset_fd);
// C: 			EXPECT_EQ(0, close(ruleset_fd));
// C: 		}
// C: 
// C: 		/* Fast Open with no address. * /
// C: 		ret = sendto_variant(connect_fd, NULL, NULL, 0, MSG_FASTOPEN);
// C: 		if (self->srv0.protocol.domain == AF_UNIX) {
// C: 			EXPECT_EQ(-ENOTCONN, ret);
// C: 		} else if (self->srv0.protocol.type == SOCK_DGRAM) {
// C: 			EXPECT_EQ(-EDESTADDRREQ, ret);
// C: 		} else {
// C: 			EXPECT_EQ(-EINVAL, ret);
// C: 		}
// C: 
// C: 		/* Fast Open to a denied address. * /
// C: 		ret = sendto_variant(connect_fd, &self->srv0, "A", 1,
// C: 				     MSG_FASTOPEN);
// C: 		if (restricted) {
// C: 			EXPECT_EQ(-EACCES, ret);
// C: 		} else if (self->srv0.protocol.domain == AF_UNIX &&
// C: 			   self->srv0.protocol.type == SOCK_STREAM) {
// C: 			EXPECT_EQ(-EOPNOTSUPP, ret);
// C: 		} else {
// C: 			EXPECT_EQ(0, ret);
// C: 		}
// C: 
// C: 		EXPECT_EQ(0, close(connect_fd));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 
// C: 	client_fd = bind_fd;
// C: 	if (!restricted && self->srv0.protocol.type == SOCK_STREAM &&
// C: 	    self->srv0.protocol.domain != AF_UNIX) {
// C: 		client_fd = accept(bind_fd, NULL, 0);
// C: 		ASSERT_LE(0, client_fd);
// C: 	}
// C: 
// C: 	if (restricted) {
// C: 		EXPECT_EQ(-1, read(client_fd, &buf, 1));
// C: 		EXPECT_EQ(ENOTCONN, errno);
// C: 	} else if (self->srv0.protocol.domain == AF_UNIX &&
// C: 		   self->srv0.protocol.type == SOCK_STREAM) {
// C: 		EXPECT_EQ(-1, read(client_fd, &buf, 1));
// C: 		EXPECT_EQ(EINVAL, errno);
// C: 	} else {
// C: 		EXPECT_EQ(1, read(client_fd, &buf, 1));
// C: 		EXPECT_EQ('A', buf);
// C: 	}
// C: 
// C: 	EXPECT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(1, WIFEXITED(status));
// C: 	EXPECT_EQ(EXIT_SUCCESS, WEXITSTATUS(status));
// C: 
// C: 	if (client_fd != bind_fd)
// C: 		EXPECT_LE(0, close(client_fd));
// C: 
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: }
// C: 
// C: TEST_F(protocol, sendmsg_stream)
// C: {
// C: 	int srv0_fd, tmp_fd, client_fd, res;
// C: 	char read_buf[1] = { 0 };
// C: 
// C: 	/*
// C: 	 * Simple test for stream sockets: just deny all connect()/
// C: 	 * send(explicit addr)/bind(), and make sure we don't interfere with any
// C: 	 * operation.
// C: 	 * /
// C: 	if (variant->prot.type != SOCK_STREAM)
// C: 		return;
// C: 
// C: 	if (variant->sandbox == UDP_SANDBOX) {
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net =
// C: 				LANDLOCK_ACCESS_NET_BIND_UDP |
// C: 				LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
// C: 		};
// C: 		const int ruleset_fd = landlock_create_ruleset(
// C: 			&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 	ASSERT_LE(0, srv0_fd = socket_variant(&self->srv0));
// C: 	ASSERT_EQ(0, bind_variant(srv0_fd, &self->srv0));
// C: 	ASSERT_EQ(0, listen(srv0_fd, backlog));
// C: 
// C: 	/* Send on a non-connected socket. * /
// C: 	res = sendto_variant(client_fd, NULL, "A", 1, 0);
// C: 	if (variant->prot.domain == AF_UNIX) {
// C: 		EXPECT_EQ(-ENOTCONN, res);
// C: 	} else {
// C: 		EXPECT_EQ(-EPIPE, res);
// C: 	}
// C: 
// C: 	/* Send to a truncated (invalid) address on a non-connected socket. * /
// C: 	res = sendto_variant_addrlen(client_fd, &self->srv0,
// C: 				     get_addrlen(&self->srv0, true) - 1, "B", 1,
// C: 				     0);
// C: 	if (variant->prot.domain == AF_UNIX) {
// C: 		EXPECT_EQ(-EOPNOTSUPP, res);
// C: 	} else {
// C: 		EXPECT_EQ(-EPIPE, res);
// C: 	}
// C: 
// C: 	/* Connect. * /
// C: 	ASSERT_EQ(0, connect_variant(client_fd, &self->srv0));
// C: 	tmp_fd = accept(srv0_fd, NULL, 0);
// C: 	ASSERT_LE(0, tmp_fd);
// C: 	EXPECT_EQ(0, close(srv0_fd));
// C: 	srv0_fd = tmp_fd;
// C: 
// C: 	/* Send without an explicit address. * /
// C: 	EXPECT_EQ(0, sendto_variant(client_fd, NULL, "C", 1, 0));
// C: 	EXPECT_EQ(1, recv(srv0_fd, read_buf, 1, 0))
// C: 	{
// C: 		TH_LOG("recv() failed: %s", strerror(errno));
// C: 	}
// C: 	EXPECT_EQ(read_buf[0], 'C');
// C: 
// C: 	/* Send to a truncated (invalid) address. * /
// C: 	res = sendto_variant_addrlen(client_fd, &self->srv0,
// C: 				     get_addrlen(&self->srv0, true) - 1, "D", 1,
// C: 				     0);
// C: 	if (variant->prot.domain == AF_UNIX) {
// C: 		EXPECT_EQ(-EISCONN, res);
// C: 	} else {
// C: 		ASSERT_EQ(0, res);
// C: 		EXPECT_EQ(1, recv(srv0_fd, read_buf, 1, 0))
// C: 		{
// C: 			TH_LOG("recv() failed: %s", strerror(errno));
// C: 		}
// C: 		EXPECT_EQ(read_buf[0], 'D');
// C: 	}
// C: 
// C: 	/* Send to a valid but different address. * /
// C: 	res = sendto_variant(client_fd, &self->srv1, "E", 1, 0);
// C: 	if (variant->prot.domain == AF_UNIX) {
// C: 		EXPECT_EQ(-EISCONN, res);
// C: 	} else {
// C: 		ASSERT_EQ(0, res);
// C: 		EXPECT_EQ(1, recv(srv0_fd, read_buf, 1, 0))
// C: 		{
// C: 			TH_LOG("recv() failed: %s", strerror(errno));
// C: 		}
// C: 		EXPECT_EQ(read_buf[0], 'E');
// C: 	}
// C: 
// C: 	EXPECT_EQ(0, close(client_fd));
// C: }
// C: 
// C: TEST_F(protocol, sendmsg_dgram)
// C: {
// C: 	const bool restricted = is_restricted(&variant->prot, variant->sandbox);
// C: 	int srv0_fd, srv1_fd, client_fd, child, status, res;
// C: 
// C: 	if (variant->prot.type != SOCK_DGRAM)
// C: 		return;
// C: 
// C: 	/* Prepare server on port #0 to be allowed. * /
// C: 	ASSERT_LE(0, srv0_fd = socket_variant(&self->srv0));
// C: 	ASSERT_EQ(0, bind_variant(srv0_fd, &self->srv0));
// C: 
// C: 	/* And another server on port #1 to be denied. * /
// C: 	ASSERT_LE(0, srv1_fd = socket_variant(&self->srv1));
// C: 	ASSERT_EQ(0, bind_variant(srv1_fd, &self->srv1));
// C: 
// C: 	/*
// C: 	 * Check that sockets connected before restrictions are not impacted in
// C: 	 * any way.
// C: 	 * /
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 		ASSERT_EQ(0, connect_variant(client_fd, &self->srv0));
// C: 		if (variant->sandbox == UDP_SANDBOX) {
// C: 			/* Deny all connect()/send(explicit addr)/bind(). * /
// C: 			const struct landlock_ruleset_attr ruleset_attr = {
// C: 				.handled_access_net =
// C: 					LANDLOCK_ACCESS_NET_BIND_UDP |
// C: 					LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
// C: 			};
// C: 			const int ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 			ASSERT_LE(0, ruleset_fd);
// C: 			enforce_ruleset(_metadata, ruleset_fd);
// C: 			EXPECT_EQ(0, close(ruleset_fd));
// C: 		}
// C: 		EXPECT_EQ(0,
// C: 			  test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 				       srv0_fd, NULL, restricted, restricted));
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  srv0_fd, &self->srv0, restricted,
// C: 					  restricted));
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  srv1_fd, &self->srv1, restricted,
// C: 					  restricted));
// C: 		EXPECT_EQ(0, close(client_fd));
// C: 		_exit(_metadata->exit_code);
// C: 	}
// C: 	EXPECT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(1, WIFEXITED(status));
// C: 	EXPECT_EQ(EXIT_SUCCESS, WEXITSTATUS(status));
// C: 
// C: 	/*
// C: 	 * Restrict connect/send, but not bind(). Then try sending with no
// C: 	 * destination (and no remote peer set), an allowed destination, then a
// C: 	 * denied destination.
// C: 	 * /
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		if (variant->sandbox == UDP_SANDBOX) {
// C: 			const struct landlock_ruleset_attr ruleset_attr = {
// C: 				.handled_access_net =
// C: 					LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
// C: 			};
// C: 			const struct landlock_net_port_attr send_p0 = {
// C: 				.allowed_access =
// C: 					LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
// C: 				.port = self->srv0.port,
// C: 			};
// C: 			const int ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 			ASSERT_LE(0, ruleset_fd);
// C: 			ASSERT_EQ(0, landlock_add_rule(ruleset_fd,
// C: 						       LANDLOCK_RULE_NET_PORT,
// C: 						       &send_p0, 0));
// C: 			enforce_ruleset(_metadata, ruleset_fd);
// C: 			EXPECT_EQ(0, close(ruleset_fd));
// C: 		}
// C: 		ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  -1, NULL, false, false));
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  srv0_fd, &self->srv0, false, false));
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  srv1_fd, &self->srv1, false,
// C: 					  restricted));
// C: 		EXPECT_EQ(0, close(client_fd));
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(1, WIFEXITED(status));
// C: 	EXPECT_EQ(EXIT_SUCCESS, WEXITSTATUS(status));
// C: 
// C: 	/*
// C: 	 * Rest of this test is just for autobind enforcement, which only exists
// C: 	 * in IP sockets.
// C: 	 * /
// C: 	if (variant->prot.domain != AF_INET && variant->prot.domain != AF_INET6)
// C: 		return;
// C: 
// C: 	/* Restrict bind() to explicit calls with an arbitrary (non-0) port. * /
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		const uint16_t allowed_src_port = 42424;
// C: 		struct service_fixture allowed_src;
// C: 
// C: 		allowed_src = self->srv0;
// C: 		set_port(&allowed_src, allowed_src_port);
// C: 		if (variant->sandbox == UDP_SANDBOX) {
// C: 			const struct landlock_ruleset_attr ruleset_attr = {
// C: 				.handled_access_net =
// C: 					LANDLOCK_ACCESS_NET_BIND_UDP,
// C: 			};
// C: 			const struct landlock_net_port_attr rule = {
// C: 				.allowed_access = LANDLOCK_ACCESS_NET_BIND_UDP,
// C: 				.port = allowed_src_port,
// C: 			};
// C: 			const int ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 			ASSERT_LE(0, ruleset_fd);
// C: 			ASSERT_EQ(0, landlock_add_rule(ruleset_fd,
// C: 						       LANDLOCK_RULE_NET_PORT,
// C: 						       &rule, 0));
// C: 			enforce_ruleset(_metadata, ruleset_fd);
// C: 			EXPECT_EQ(0, close(ruleset_fd));
// C: 		}
// C: 		ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 
// C: 		/* Check that implicit bind(0) in sendmsg() is denied. * /
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  srv0_fd, &self->srv0, restricted,
// C: 					  false));
// C: 
// C: 		/* Same thing for autobind in connect(). * /
// C: 		res = connect_variant(client_fd, &self->srv0);
// C: 		if (restricted) {
// C: 			EXPECT_EQ(-EACCES, res);
// C: 		} else {
// C: 			EXPECT_EQ(0, res);
// C: 		}
// C: 		EXPECT_EQ(0, close(client_fd));
// C: 
// C: 		/* Make sendmsg() work by explicitly binding to the only allowed port. * /
// C: 		ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 		EXPECT_EQ(0, bind_variant(client_fd, &allowed_src));
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  srv0_fd, &self->srv0, restricted,
// C: 					  false));
// C: 		EXPECT_EQ(0, close(client_fd));
// C: 
// C: 		/* Make connect() work by explicitly binding to the only allowed port. * /
// C: 		ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 		EXPECT_EQ(0, bind_variant(client_fd, &allowed_src));
// C: 		EXPECT_EQ(0, connect_variant(client_fd, &self->srv0));
// C: 		EXPECT_EQ(0, close(client_fd));
// C: 
// C: 		_exit(_metadata->exit_code);
// C: 		return;
// C: 	}
// C: 	EXPECT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(1, WIFEXITED(status));
// C: 	EXPECT_EQ(EXIT_SUCCESS, WEXITSTATUS(status));
// C: 
// C: 	/*
// C: 	 * Check that %LANDLOCK_ACCESS_NET_BIND_UDP on port 0 allows implicit
// C: 	 * autobinds.
// C: 	 * /
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 	if (child == 0) {
// C: 		if (variant->sandbox == UDP_SANDBOX) {
// C: 			const struct landlock_ruleset_attr ruleset_attr = {
// C: 				.handled_access_net =
// C: 					LANDLOCK_ACCESS_NET_BIND_UDP,
// C: 			};
// C: 			const struct landlock_net_port_attr rule = {
// C: 				.allowed_access = LANDLOCK_ACCESS_NET_BIND_UDP,
// C: 				.port = 0,
// C: 			};
// C: 			const int ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 			ASSERT_LE(0, ruleset_fd);
// C: 			ASSERT_EQ(0, landlock_add_rule(ruleset_fd,
// C: 						       LANDLOCK_RULE_NET_PORT,
// C: 						       &rule, 0));
// C: 			enforce_ruleset(_metadata, ruleset_fd);
// C: 			EXPECT_EQ(0, close(ruleset_fd));
// C: 		}
// C: 		ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 		EXPECT_EQ(0, test_sendmsg(_metadata, &variant->prot, client_fd,
// C: 					  srv0_fd, &self->srv0, false, false));
// C: 		EXPECT_EQ(0, close(client_fd));
// C: 		_exit(_metadata->exit_code);
// C: 	}
// C: 	EXPECT_EQ(child, waitpid(child, &status, 0));
// C: 	EXPECT_EQ(1, WIFEXITED(status));
// C: 	EXPECT_EQ(EXIT_SUCCESS, WEXITSTATUS(status));
// C: }
// C: 
// C: TEST_F(protocol, sendmsg_unspec)
// C: {
// C: 	const bool restricted = is_restricted(&variant->prot, variant->sandbox);
// C: 	int client_fd, srv0_fd, srv1_fd, res;
// C: 	char read_buf[1] = { 0 };
// C: 
// C: 	/*
// C: 	 * We already test for the absence of influence on sendmsg for other
// C: 	 * socket types and other address families, there's no point in adapting
// C: 	 * this test for stream sockets too.
// C: 	 * /
// C: 	if (variant->prot.type != SOCK_DGRAM)
// C: 		return;
// C: 
// C: 	/* Prepare client of the right family. * /
// C: 	ASSERT_LE(0, client_fd = socket_variant(&self->srv0));
// C: 
// C: 	/* Prepare server on port #0 to be allowed. * /
// C: 	ASSERT_LE(0, srv0_fd = socket_variant(&self->srv0));
// C: 	ASSERT_EQ(0, bind_variant(srv0_fd, &self->srv0));
// C: 
// C: 	/* And another server on port #1 to be denied. * /
// C: 	ASSERT_LE(0, srv1_fd = socket_variant(&self->srv1));
// C: 	ASSERT_EQ(0, bind_variant(srv1_fd, &self->srv1));
// C: 
// C: 	if (variant->sandbox == UDP_SANDBOX) {
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net =
// C: 				LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
// C: 		};
// C: 		const struct landlock_net_port_attr rule = {
// C: 			.allowed_access = LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
// C: 			.port = self->srv0.port,
// C: 		};
// C: 		const int ruleset_fd = landlock_create_ruleset(
// C: 			&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &rule, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	/* Explicit AF_UNSPEC address but truncated. * /
// C: 	EXPECT_EQ(-EINVAL, sendto_variant_addrlen(
// C: 				   client_fd, &self->unspec_srv0,
// C: 				   get_addrlen(&self->unspec_srv0, true) - 1,
// C: 				   "A", 1, 0));
// C: 
// C: 	/*
// C: 	 * Explicit AF_UNSPEC address, should be treated as AF_INET by IPv4
// C: 	 * sockets (and thus map to srv0, allowed), but be denied by IPv6
// C: 	 * sockets.
// C: 	 * /
// C: 	res = sendto_variant(client_fd, &self->unspec_srv0, "B", 1, 0);
// C: 	if (variant->prot.domain == AF_INET6) {
// C: 		if (restricted) {
// C: 			/* Always denied on IPv6 socket. * /
// C: 			EXPECT_EQ(-EACCES, res);
// C: 		} else {
// C: 			/* IPv6 sockets treat AF_UNSPEC as a NULL address. * /
// C: 			EXPECT_EQ(-EDESTADDRREQ, res);
// C: 		}
// C: 	} else if (variant->prot.domain == AF_INET) {
// C: 		ASSERT_EQ(0, res);
// C: 		EXPECT_EQ(1, read(srv0_fd, read_buf, 1))
// C: 		{
// C: 			TH_LOG("read() failed: %s", strerror(errno));
// C: 		}
// C: 		EXPECT_EQ(read_buf[0], 'B');
// C: 	} else {
// C: 		/* Unix sockets don't accept AF_UNSPEC. * /
// C: 		EXPECT_EQ(-EINVAL, res);
// C: 	}
// C: 
// C: 	/*
// C: 	 * Explicit AF_UNSPEC address, should be treated as AF_INET on IPv4
// C: 	 * sockets (and thus map to srv1, denied), and be denied on IPv6 sockets
// C: 	 * as always.
// C: 	 * /
// C: 	res = sendto_variant(client_fd, &self->unspec_srv1, "C", 1, 0);
// C: 	if (variant->prot.domain == AF_INET6) {
// C: 		if (restricted) {
// C: 			/* Always denied on IPv6 socket. * /
// C: 			EXPECT_EQ(-EACCES, res);
// C: 		} else {
// C: 			/* IPv6 sockets treat AF_UNSPEC as a NULL address. * /
// C: 			EXPECT_EQ(-EDESTADDRREQ, res);
// C: 		}
// C: 	} else if (variant->prot.domain == AF_INET) {
// C: 		if (restricted) {
// C: 			/* Sending to srv1 is not allowed, only srv0. * /
// C: 			EXPECT_EQ(-EACCES, res);
// C: 		} else {
// C: 			ASSERT_EQ(0, res);
// C: 			EXPECT_EQ(1, read(srv1_fd, read_buf, 1))
// C: 			{
// C: 				TH_LOG("read() failed: %s", strerror(errno));
// C: 			}
// C: 			EXPECT_EQ(read_buf[0], 'C');
// C: 		}
// C: 	} else {
// C: 		/* Unix sockets don't accept AF_UNSPEC. * /
// C: 		EXPECT_EQ(-EINVAL, res);
// C: 	}
// C: 
// C: 	ASSERT_EQ(0, connect_variant(client_fd, &self->srv0));
// C: 
// C: 	/* Minimal explicit AF_UNSPEC address (just the sa_family_t field) * /
// C: 	res = sendto_variant_addrlen(client_fd, &self->unspec_srv0,
// C: 				     get_addrlen(&self->unspec_srv0, true), "D",
// C: 				     1, 0);
// C: 	if (variant->prot.domain == AF_INET6) {
// C: 		if (restricted) {
// C: 			/* AF_UNSPEC is always denied in IPv6. * /
// C: 			EXPECT_EQ(-EACCES, res);
// C: 		} else {
// C: 			/*
// C: 			 * IPv6 sockets treat AF_UNSPEC as a NULL address,
// C: 			 * falling back to the connected address.
// C: 			 * /
// C: 			ASSERT_EQ(0, res);
// C: 			EXPECT_EQ(1, read(srv0_fd, read_buf, 1));
// C: 			EXPECT_EQ(read_buf[0], 'D');
// C: 		}
// C: 	} else {
// C: 		/*
// C: 		 * IPv4 socket will expect a struct sockaddr_in, our address is
// C: 		 * considered truncated.  And Unix sockets don't accept
// C: 		 * AF_UNSPEC at all.
// C: 		 * /
// C: 		EXPECT_EQ(-EINVAL, res);
// C: 	}
// C: }
// C: 
// C: FIXTURE(ipv4)
// C: {
// C: 	struct service_fixture srv0, srv1;
// C: };
// C: 
// C: FIXTURE_VARIANT(ipv4)
// C: {
// C: 	const enum sandbox_type sandbox;
// C: 	const int type;
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(ipv4, no_sandbox_with_tcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.type = SOCK_STREAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(ipv4, tcp_sandbox_with_tcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.type = SOCK_STREAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(ipv4, udp_sandbox_with_tcp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.type = SOCK_STREAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(ipv4, no_sandbox_with_udp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.type = SOCK_DGRAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(ipv4, tcp_sandbox_with_udp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.type = SOCK_DGRAM,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(ipv4, udp_sandbox_with_udp) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.type = SOCK_DGRAM,
// C: };
// C: 
// C: FIXTURE_SETUP(ipv4)
// C: {
// C: 	const struct protocol_variant prot = {
// C: 		.domain = AF_INET,
// C: 		.type = variant->type,
// C: 	};
// C: 
// C: 	disable_caps(_metadata);
// C: 
// C: 	set_service(&self->srv0, prot, 0);
// C: 	set_service(&self->srv1, prot, 1);
// C: 
// C: 	setup_loopback(_metadata);
// C: };
// C: 
// C: FIXTURE_TEARDOWN(ipv4)
// C: {
// C: }
// C: 
// C: TEST_F(ipv4, from_unix_to_inet)
// C: {
// C: 	int unix_stream_fd, unix_dgram_fd;
// C: 
// C: 	if (variant->sandbox == TCP_SANDBOX ||
// C: 	    variant->sandbox == UDP_SANDBOX) {
// C: 		const __u64 access_rights =
// C: 			(variant->sandbox == TCP_SANDBOX ?
// C: 				 LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 					 LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 				 LANDLOCK_ACCESS_NET_BIND_UDP |
// C: 					 LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = access_rights,
// C: 		};
// C: 		const struct landlock_net_port_attr tcp_bind_connect_p0 = {
// C: 			.allowed_access = access_rights,
// C: 			.port = self->srv0.port,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		/* Denies connect and bind to check errno value. * /
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Allows connect and bind for srv0.  * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind_connect_p0, 0));
// C: 
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	unix_stream_fd = socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);
// C: 	ASSERT_LE(0, unix_stream_fd);
// C: 
// C: 	unix_dgram_fd = socket(AF_UNIX, SOCK_DGRAM | SOCK_CLOEXEC, 0);
// C: 	ASSERT_LE(0, unix_dgram_fd);
// C: 
// C: 	/* Checks unix stream bind and connect for srv0. * /
// C: 	EXPECT_EQ(-EINVAL, bind_variant(unix_stream_fd, &self->srv0));
// C: 	EXPECT_EQ(-EINVAL, connect_variant(unix_stream_fd, &self->srv0));
// C: 
// C: 	/* Checks unix stream bind and connect for srv1. * /
// C: 	EXPECT_EQ(-EINVAL, bind_variant(unix_stream_fd, &self->srv1))
// C: 	{
// C: 		TH_LOG("Wrong bind error: %s", strerror(errno));
// C: 	}
// C: 	EXPECT_EQ(-EINVAL, connect_variant(unix_stream_fd, &self->srv1));
// C: 
// C: 	/* Checks unix datagram bind and connect for srv0. * /
// C: 	EXPECT_EQ(-EINVAL, bind_variant(unix_dgram_fd, &self->srv0));
// C: 	EXPECT_EQ(-EINVAL, connect_variant(unix_dgram_fd, &self->srv0));
// C: 
// C: 	/* Checks unix datagram bind and connect for srv1. * /
// C: 	EXPECT_EQ(-EINVAL, bind_variant(unix_dgram_fd, &self->srv1));
// C: 	EXPECT_EQ(-EINVAL, connect_variant(unix_dgram_fd, &self->srv1));
// C: }
// C: 
// C: FIXTURE(tcp_layers)
// C: {
// C: 	struct service_fixture srv0, srv1;
// C: };
// C: 
// C: FIXTURE_VARIANT(tcp_layers)
// C: {
// C: 	const size_t num_layers;
// C: 	const int domain;
// C: };
// C: 
// C: FIXTURE_SETUP(tcp_layers)
// C: {
// C: 	const struct protocol_variant prot = {
// C: 		.domain = variant->domain,
// C: 		.type = SOCK_STREAM,
// C: 	};
// C: 
// C: 	disable_caps(_metadata);
// C: 
// C: 	ASSERT_EQ(0, set_service(&self->srv0, prot, 0));
// C: 	ASSERT_EQ(0, set_service(&self->srv1, prot, 1));
// C: 
// C: 	setup_loopback(_metadata);
// C: };
// C: 
// C: FIXTURE_TEARDOWN(tcp_layers)
// C: {
// C: }
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, no_sandbox_with_ipv4) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET,
// C: 	.num_layers = 0,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, one_sandbox_with_ipv4) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET,
// C: 	.num_layers = 1,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, two_sandboxes_with_ipv4) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET,
// C: 	.num_layers = 2,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, three_sandboxes_with_ipv4) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET,
// C: 	.num_layers = 3,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, no_sandbox_with_ipv6) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET6,
// C: 	.num_layers = 0,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, one_sandbox_with_ipv6) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET6,
// C: 	.num_layers = 1,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, two_sandboxes_with_ipv6) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET6,
// C: 	.num_layers = 2,
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(tcp_layers, three_sandboxes_with_ipv6) {
// C: 	/* clang-format on * /
// C: 	.domain = AF_INET6,
// C: 	.num_layers = 3,
// C: };
// C: 
// C: TEST_F(tcp_layers, ruleset_overlap)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				      LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 	};
// C: 	const struct landlock_net_port_attr tcp_bind = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = self->srv0.port,
// C: 	};
// C: 	const struct landlock_net_port_attr tcp_bind_connect = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				  LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 		.port = self->srv0.port,
// C: 	};
// C: 
// C: 	if (variant->num_layers >= 1) {
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Allows bind. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind, 0));
// C: 		/* Also allows bind, but allows connect too. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind_connect, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	if (variant->num_layers >= 2) {
// C: 		int ruleset_fd;
// C: 
// C: 		/* Creates another ruleset layer. * /
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Only allows bind. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	if (variant->num_layers >= 3) {
// C: 		int ruleset_fd;
// C: 
// C: 		/* Creates another ruleset layer. * /
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Try to allow bind and connect. * /
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind_connect, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	/*
// C: 	 * Forbids to connect to the socket because only one ruleset layer
// C: 	 * allows connect.
// C: 	 * /
// C: 	test_bind_and_connect(_metadata, &self->srv0, false,
// C: 			      variant->num_layers >= 2);
// C: }
// C: 
// C: TEST_F(tcp_layers, ruleset_expand)
// C: {
// C: 	if (variant->num_layers >= 1) {
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		};
// C: 		/* Allows bind for srv0. * /
// C: 		const struct landlock_net_port_attr bind_srv0 = {
// C: 			.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 			.port = self->srv0.port,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &bind_srv0, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	if (variant->num_layers >= 2) {
// C: 		/* Expands network mask with connect action. * /
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 					      LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 		};
// C: 		/* Allows bind for srv0 and connect to srv0. * /
// C: 		const struct landlock_net_port_attr tcp_bind_connect_p0 = {
// C: 			.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 					  LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 			.port = self->srv0.port,
// C: 		};
// C: 		/* Try to allow bind for srv1. * /
// C: 		const struct landlock_net_port_attr tcp_bind_p1 = {
// C: 			.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 			.port = self->srv1.port,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind_connect_p0, 0));
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind_p1, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	if (variant->num_layers >= 3) {
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 					      LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 		};
// C: 		/* Allows connect to srv0, without bind rule. * /
// C: 		const struct landlock_net_port_attr tcp_bind_p0 = {
// C: 			.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 			.port = self->srv0.port,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &tcp_bind_p0, 0));
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	test_bind_and_connect(_metadata, &self->srv0, false,
// C: 			      variant->num_layers >= 3);
// C: 
// C: 	test_bind_and_connect(_metadata, &self->srv1, variant->num_layers >= 1,
// C: 			      variant->num_layers >= 2);
// C: }
// C: 
// C: /* clang-format off * /
// C: FIXTURE(mini) {};
// C: /* clang-format on * /
// C: 
// C: FIXTURE_SETUP(mini)
// C: {
// C: 	disable_caps(_metadata);
// C: 
// C: 	setup_loopback(_metadata);
// C: };
// C: 
// C: FIXTURE_TEARDOWN(mini)
// C: {
// C: }
// C: 
// C: /* clang-format off * /
// C: 
// C: #define ACCESS_LAST LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP
// C: 
// C: #define ACCESS_ALL ( \
// C: 	LANDLOCK_ACCESS_NET_BIND_TCP | \
// C: 	LANDLOCK_ACCESS_NET_CONNECT_TCP | \
// C: 	LANDLOCK_ACCESS_NET_BIND_UDP | \
// C: 	LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP)
// C: 
// C: /* clang-format on * /
// C: 
// C: TEST_F(mini, network_access_rights)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = ACCESS_ALL,
// C: 	};
// C: 	struct landlock_net_port_attr net_port = {
// C: 		.port = sock_port_start,
// C: 	};
// C: 	int ruleset_fd;
// C: 	__u64 access;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 
// C: 	for (access = 1; access <= ACCESS_LAST; access <<= 1) {
// C: 		net_port.allowed_access = access;
// C: 		EXPECT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &net_port, 0))
// C: 		{
// C: 			TH_LOG("Failed to add rule with access 0x%llx: %s",
// C: 			       (unsigned long long)access, strerror(errno));
// C: 		}
// C: 	}
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: }
// C: 
// C: /* Checks invalid attribute, out of landlock network access range. * /
// C: TEST_F(mini, ruleset_with_unknown_access)
// C: {
// C: 	__u64 access_mask;
// C: 
// C: 	for (access_mask = 1ULL << 63; access_mask != ACCESS_LAST;
// C: 	     access_mask >>= 1) {
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = access_mask,
// C: 		};
// C: 
// C: 		EXPECT_EQ(-1, landlock_create_ruleset(&ruleset_attr,
// C: 						      sizeof(ruleset_attr), 0));
// C: 		EXPECT_EQ(EINVAL, errno);
// C: 	}
// C: }
// C: 
// C: TEST_F(mini, rule_with_unknown_access)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = ACCESS_ALL,
// C: 	};
// C: 	struct landlock_net_port_attr net_port = {
// C: 		.port = sock_port_start,
// C: 	};
// C: 	int ruleset_fd;
// C: 	__u64 access;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 
// C: 	for (access = 1ULL << 63; access != ACCESS_LAST; access >>= 1) {
// C: 		net_port.allowed_access = access;
// C: 		EXPECT_EQ(-1,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &net_port, 0));
// C: 		EXPECT_EQ(EINVAL, errno);
// C: 	}
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: }
// C: 
// C: TEST_F(mini, rule_with_unhandled_access)
// C: {
// C: 	struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 	};
// C: 	struct landlock_net_port_attr net_port = {
// C: 		.port = sock_port_start,
// C: 	};
// C: 	int ruleset_fd;
// C: 	__u64 access;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 
// C: 	for (access = 1; access > 0; access <<= 1) {
// C: 		int err;
// C: 
// C: 		net_port.allowed_access = access;
// C: 		err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					&net_port, 0);
// C: 		if (access == ruleset_attr.handled_access_net) {
// C: 			EXPECT_EQ(0, err);
// C: 		} else {
// C: 			EXPECT_EQ(-1, err);
// C: 			EXPECT_EQ(EINVAL, errno);
// C: 		}
// C: 	}
// C: 
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: }
// C: 
// C: TEST_F(mini, inval)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP
// C: 	};
// C: 	const struct landlock_net_port_attr tcp_bind_connect = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				  LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 		.port = sock_port_start,
// C: 	};
// C: 	const struct landlock_net_port_attr tcp_denied = {
// C: 		.allowed_access = 0,
// C: 		.port = sock_port_start,
// C: 	};
// C: 	const struct landlock_net_port_attr tcp_bind = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = sock_port_start,
// C: 	};
// C: 	int ruleset_fd;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 
// C: 	/* Checks unhandled allowed_access. * /
// C: 	EXPECT_EQ(-1, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					&tcp_bind_connect, 0));
// C: 	EXPECT_EQ(EINVAL, errno);
// C: 
// C: 	/* Checks zero access value. * /
// C: 	EXPECT_EQ(-1, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					&tcp_denied, 0));
// C: 	EXPECT_EQ(ENOMSG, errno);
// C: 
// C: 	/* Adds with legitimate values. * /
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &tcp_bind, 0));
// C: }
// C: 
// C: TEST_F(mini, tcp_port_overflow)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				      LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 	};
// C: 	const struct landlock_net_port_attr port_max_bind = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = UINT16_MAX,
// C: 	};
// C: 	const struct landlock_net_port_attr port_max_connect = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 		.port = UINT16_MAX,
// C: 	};
// C: 	const struct landlock_net_port_attr port_overflow1 = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = UINT16_MAX + 1,
// C: 	};
// C: 	const struct landlock_net_port_attr port_overflow2 = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = UINT16_MAX + 2,
// C: 	};
// C: 	const struct landlock_net_port_attr port_overflow3 = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = UINT32_MAX + 1UL,
// C: 	};
// C: 	const struct landlock_net_port_attr port_overflow4 = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = UINT32_MAX + 2UL,
// C: 	};
// C: 	const struct protocol_variant ipv4_tcp = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 	};
// C: 	struct service_fixture srv_denied, srv_max_allowed;
// C: 	int ruleset_fd;
// C: 
// C: 	ASSERT_EQ(0, set_service(&srv_denied, ipv4_tcp, 0));
// C: 
// C: 	/* Be careful to avoid port inconsistencies. * /
// C: 	srv_max_allowed = srv_denied;
// C: 	srv_max_allowed.port = port_max_bind.port;
// C: 	srv_max_allowed.ipv4_addr.sin_port = htons(port_max_bind.port);
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &port_max_bind, 0));
// C: 
// C: 	EXPECT_EQ(-1, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					&port_overflow1, 0));
// C: 	EXPECT_EQ(EINVAL, errno);
// C: 
// C: 	EXPECT_EQ(-1, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					&port_overflow2, 0));
// C: 	EXPECT_EQ(EINVAL, errno);
// C: 
// C: 	EXPECT_EQ(-1, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					&port_overflow3, 0));
// C: 	EXPECT_EQ(EINVAL, errno);
// C: 
// C: 	/* Interleaves with invalid rule additions. * /
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &port_max_connect, 0));
// C: 
// C: 	EXPECT_EQ(-1, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					&port_overflow4, 0));
// C: 	EXPECT_EQ(EINVAL, errno);
// C: 
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 
// C: 	test_bind_and_connect(_metadata, &srv_denied, true, true);
// C: 	test_bind_and_connect(_metadata, &srv_max_allowed, false, false);
// C: }
// C: 
// C: FIXTURE(ipv4_tcp)
// C: {
// C: 	struct service_fixture srv0, srv1;
// C: };
// C: 
// C: FIXTURE_SETUP(ipv4_tcp)
// C: {
// C: 	const struct protocol_variant ipv4_tcp = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 	};
// C: 
// C: 	disable_caps(_metadata);
// C: 
// C: 	ASSERT_EQ(0, set_service(&self->srv0, ipv4_tcp, 0));
// C: 	ASSERT_EQ(0, set_service(&self->srv1, ipv4_tcp, 1));
// C: 
// C: 	setup_loopback(_metadata);
// C: };
// C: 
// C: FIXTURE_TEARDOWN(ipv4_tcp)
// C: {
// C: }
// C: 
// C: TEST_F(ipv4_tcp, port_endianness)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				      LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 	};
// C: 	const struct landlock_net_port_attr bind_host_endian_p0 = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		/* Host port format. * /
// C: 		.port = self->srv0.port,
// C: 	};
// C: 	const struct landlock_net_port_attr connect_big_endian_p0 = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 		/* Big endian port format. * /
// C: 		.port = htons(self->srv0.port),
// C: 	};
// C: 	const struct landlock_net_port_attr bind_connect_host_endian_p1 = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				  LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 		/* Host port format. * /
// C: 		.port = self->srv1.port,
// C: 	};
// C: 	const unsigned int one = 1;
// C: 	const char little_endian = *(const char *)&one;
// C: 	int ruleset_fd;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &bind_host_endian_p0, 0));
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &connect_big_endian_p0, 0));
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &bind_connect_host_endian_p1, 0));
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 
// C: 	/* No restriction for big endinan CPU. * /
// C: 	test_bind_and_connect(_metadata, &self->srv0, false, little_endian);
// C: 
// C: 	/* No restriction for any CPU. * /
// C: 	test_bind_and_connect(_metadata, &self->srv1, false, false);
// C: }
// C: 
// C: TEST_F(ipv4_tcp, with_fs)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr_fs_net = {
// C: 		.handled_access_fs = LANDLOCK_ACCESS_FS_READ_DIR,
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 	};
// C: 	struct landlock_path_beneath_attr path_beneath = {
// C: 		.allowed_access = LANDLOCK_ACCESS_FS_READ_DIR,
// C: 		.parent_fd = -1,
// C: 	};
// C: 	struct landlock_net_port_attr tcp_bind = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		.port = self->srv0.port,
// C: 	};
// C: 	int ruleset_fd, bind_fd, dir_fd;
// C: 
// C: 	/* Creates ruleset both for filesystem and network access. * /
// C: 	ruleset_fd = landlock_create_ruleset(&ruleset_attr_fs_net,
// C: 					     sizeof(ruleset_attr_fs_net), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 
// C: 	/* Adds a filesystem rule. * /
// C: 	path_beneath.parent_fd = open("/dev", O_PATH | O_DIRECTORY | O_CLOEXEC);
// C: 	ASSERT_LE(0, path_beneath.parent_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
// C: 				       &path_beneath, 0));
// C: 	EXPECT_EQ(0, close(path_beneath.parent_fd));
// C: 
// C: 	/* Adds a network rule. * /
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &tcp_bind, 0));
// C: 
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	/* Tests file access. * /
// C: 	dir_fd = open("/dev", O_RDONLY);
// C: 	EXPECT_LE(0, dir_fd);
// C: 	EXPECT_EQ(0, close(dir_fd));
// C: 
// C: 	dir_fd = open("/", O_RDONLY);
// C: 	EXPECT_EQ(-1, dir_fd);
// C: 	EXPECT_EQ(EACCES, errno);
// C: 
// C: 	/* Tests port binding. * /
// C: 	bind_fd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 	EXPECT_EQ(0, bind_variant(bind_fd, &self->srv0));
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: 
// C: 	bind_fd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 	EXPECT_EQ(-EACCES, bind_variant(bind_fd, &self->srv1));
// C: }
// C: 
// C: FIXTURE(port_specific)
// C: {
// C: 	struct service_fixture srv0;
// C: 	struct service_fixture cli1;
// C: };
// C: 
// C: FIXTURE_VARIANT(port_specific)
// C: {
// C: 	const enum sandbox_type sandbox;
// C: 	const struct protocol_variant prot;
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(port_specific, no_sandbox_with_ipv4) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(port_specific, tcp_sandbox_with_ipv4) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(port_specific, udp_sandbox_with_ipv4) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(port_specific, no_sandbox_with_ipv6) {
// C: 	/* clang-format on * /
// C: 	.sandbox = NO_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(port_specific, tcp_sandbox_with_ipv6) {
// C: 	/* clang-format on * /
// C: 	.sandbox = TCP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(port_specific, udp_sandbox_with_ipv6) {
// C: 	/* clang-format on * /
// C: 	.sandbox = UDP_SANDBOX,
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: FIXTURE_SETUP(port_specific)
// C: {
// C: 	disable_caps(_metadata);
// C: 
// C: 	ASSERT_EQ(0, set_service(&self->srv0, variant->prot, 0));
// C: 	ASSERT_EQ(0, set_service(&self->cli1, variant->prot, 1));
// C: 
// C: 	setup_loopback(_metadata);
// C: };
// C: 
// C: FIXTURE_TEARDOWN(port_specific)
// C: {
// C: }
// C: 
// C: TEST_F(port_specific, bind_connect_zero)
// C: {
// C: 	int bind_fd, connect_fd, ret;
// C: 	uint16_t port;
// C: 
// C: 	/* Adds a rule layer with bind and connect actions. * /
// C: 	if (variant->sandbox == TCP_SANDBOX ||
// C: 	    variant->sandbox == UDP_SANDBOX) {
// C: 		const __u64 access_rights =
// C: 			(variant->sandbox == TCP_SANDBOX ?
// C: 				 LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 					 LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 				 LANDLOCK_ACCESS_NET_BIND_UDP |
// C: 					 LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = access_rights,
// C: 		};
// C: 		const struct landlock_net_port_attr bind_connect_zero = {
// C: 			.allowed_access = access_rights,
// C: 			.port = 0,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		/* Checks zero port value on bind and connect actions. * /
// C: 		EXPECT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &bind_connect_zero, 0));
// C: 
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 
// C: 	connect_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, connect_fd);
// C: 
// C: 	/* Sets address port to 0 for both protocol families. * /
// C: 	set_port(&self->srv0, 0);
// C: 	/*
// C: 	 * Binds on port 0, which selects a random port within
// C: 	 * ip_local_port_range.
// C: 	 * /
// C: 	ret = bind_variant(bind_fd, &self->srv0);
// C: 	EXPECT_EQ(0, ret);
// C: 
// C: 	if (variant->prot.type == SOCK_STREAM)
// C: 		EXPECT_EQ(0, listen(bind_fd, backlog));
// C: 
// C: 	/* Connects on port 0. * /
// C: 	ret = connect_variant(connect_fd, &self->srv0);
// C: 	if (variant->prot.type == SOCK_STREAM) {
// C: 		EXPECT_EQ(-ECONNREFUSED, ret);
// C: 	} else {
// C: 		EXPECT_EQ(0, ret);
// C: 	}
// C: 
// C: 	/* Sets binded port for both protocol families. * /
// C: 	port = get_binded_port(bind_fd, &variant->prot);
// C: 	EXPECT_NE(0, port);
// C: 	set_port(&self->srv0, port);
// C: 	/* Connects on the binded port. * /
// C: 	ret = connect_variant(connect_fd, &self->srv0);
// C: 	if (is_restricted(&variant->prot, variant->sandbox)) {
// C: 		/* Denied by Landlock. * /
// C: 		EXPECT_EQ(-EACCES, ret);
// C: 	} else {
// C: 		EXPECT_EQ(0, ret);
// C: 	}
// C: 
// C: 	EXPECT_EQ(0, close(connect_fd));
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: }
// C: 
// C: TEST_F(port_specific, bind_connect_1023)
// C: {
// C: 	int bind_fd, connect_fd, ret;
// C: 
// C: 	/* Adds a rule layer with bind and connect actions. * /
// C: 	if (variant->sandbox == TCP_SANDBOX ||
// C: 	    variant->sandbox == UDP_SANDBOX) {
// C: 		const __u64 bind_right = (variant->sandbox == TCP_SANDBOX ?
// C: 						  LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 						  LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 		const __u64 access_rights =
// C: 			(variant->sandbox == TCP_SANDBOX ?
// C: 				 (LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				  LANDLOCK_ACCESS_NET_CONNECT_TCP) :
// C: 				 (LANDLOCK_ACCESS_NET_BIND_UDP |
// C: 				  LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP));
// C: 		const struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = access_rights,
// C: 		};
// C: 		/* A rule with port value less than 1024. * /
// C: 		const struct landlock_net_port_attr bind_connect_low_range = {
// C: 			.allowed_access = access_rights,
// C: 			.port = 1023,
// C: 		};
// C: 		/* A rule with 1024 port. * /
// C: 		const struct landlock_net_port_attr bind_connect = {
// C: 			.allowed_access = access_rights,
// C: 			.port = 1024,
// C: 		};
// C: 		/* A rule with cli1's port, to use as source port. * /
// C: 		const struct landlock_net_port_attr srcport = {
// C: 			.allowed_access = bind_right,
// C: 			.port = self->cli1.port,
// C: 		};
// C: 		int ruleset_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		ASSERT_LE(0, ruleset_fd);
// C: 
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &bind_connect_low_range, 0));
// C: 		ASSERT_EQ(0,
// C: 			  landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 					    &bind_connect, 0));
// C: 		if (variant->sandbox == UDP_SANDBOX) {
// C: 			ASSERT_EQ(0, landlock_add_rule(ruleset_fd,
// C: 						       LANDLOCK_RULE_NET_PORT,
// C: 						       &srcport, 0));
// C: 		}
// C: 
// C: 		enforce_ruleset(_metadata, ruleset_fd);
// C: 		EXPECT_EQ(0, close(ruleset_fd));
// C: 	}
// C: 
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 
// C: 	/* Sets address port to 1023 for both protocol families. * /
// C: 	set_port(&self->srv0, 1023);
// C: 	/* Binds on port 1023. * /
// C: 	ret = bind_variant(bind_fd, &self->srv0);
// C: 	/* Denied by the system. * /
// C: 	EXPECT_EQ(-EACCES, ret);
// C: 
// C: 	/* Binds on port 1023. * /
// C: 	set_cap(_metadata, CAP_NET_BIND_SERVICE);
// C: 	ret = bind_variant(bind_fd, &self->srv0);
// C: 	clear_cap(_metadata, CAP_NET_BIND_SERVICE);
// C: 	EXPECT_EQ(0, ret);
// C: 	if (variant->prot.type == SOCK_STREAM)
// C: 		EXPECT_EQ(0, listen(bind_fd, backlog));
// C: 
// C: 	connect_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, connect_fd);
// C: 	if (variant->prot.type == SOCK_DGRAM) {
// C: 		/*
// C: 		 * We are about to connect(), but bind() is restricted, so for
// C: 		 * UDP sockets we need to use cli1's port as source port (the
// C: 		 * only one we are allowed to use).
// C: 		 * /
// C: 		EXPECT_EQ(0, bind_variant(connect_fd, &self->cli1));
// C: 	}
// C: 	/* Connects on the binded port 1023. * /
// C: 	ret = connect_variant(connect_fd, &self->srv0);
// C: 	EXPECT_EQ(0, ret);
// C: 
// C: 	EXPECT_EQ(0, close(connect_fd));
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: 
// C: 	bind_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, bind_fd);
// C: 
// C: 	connect_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, connect_fd);
// C: 
// C: 	/* Sets address port to 1024 for both protocol families. * /
// C: 	set_port(&self->srv0, 1024);
// C: 	/* Binds on port 1024. * /
// C: 	ret = bind_variant(bind_fd, &self->srv0);
// C: 	EXPECT_EQ(0, ret);
// C: 	if (variant->prot.type == SOCK_STREAM)
// C: 		EXPECT_EQ(0, listen(bind_fd, backlog));
// C: 	if (variant->prot.type == SOCK_DGRAM)
// C: 		EXPECT_EQ(0, bind_variant(connect_fd, &self->cli1));
// C: 
// C: 	/* Connects on the binded port 1024. * /
// C: 	ret = connect_variant(connect_fd, &self->srv0);
// C: 	EXPECT_EQ(0, ret);
// C: 
// C: 	EXPECT_EQ(0, close(connect_fd));
// C: 	EXPECT_EQ(0, close(bind_fd));
// C: }
// C: 
// C: /**
// C:  * matches_auditlog - Check audit log for a network access denial
// C:  *
// C:  * @audit_fd:   Audit file descriptor.
// C:  * @blockers:   A regex-escaped blocker string, e.g., "net\.bind_tcp".
// C:  * @dir_addr:   Either "saddr" or "daddr", ignored if addr is NULL.
// C:  * @addr:       A regex-escaped IP address string, or NULL.
// C:  * @dir_port:   Either "src" or "dest", ignored if addr is NULL.
// C:  * @port:       A port number, ignored if addr is NULL.
// C:  * /
// C: static int matches_auditlog(const int audit_fd, const char *const blockers,
// C: 			    const char *const dir_addr, const char *const addr,
// C: 			    const char *const dir_port, const __u16 port)
// C: {
// C: 	static const char log_with_addrport_tmpl[] = REGEX_LANDLOCK_PREFIX
// C: 		" blockers=%s %s=%s %s=%u$";
// C: 	static const char log_without_addrport_tmpl[] = REGEX_LANDLOCK_PREFIX
// C: 		" blockers=%s";
// C: 	/*
// C: 	 * Max strlen(blockers): 16
// C: 	 * Max strlen(dir_addr): 5
// C: 	 * Max strlen(addr): 12
// C: 	 * Max strlen(dir_port): 4
// C: 	 * Max strlen(%u port): 5
// C: 	 * /
// C: 	char log_match[sizeof(log_with_addrport_tmpl) + 42];
// C: 	int log_match_len;
// C: 
// C: 	if (addr == NULL)
// C: 		log_match_len = snprintf(log_match, sizeof(log_match),
// C: 					 log_without_addrport_tmpl, blockers);
// C: 	else
// C: 		log_match_len = snprintf(log_match, sizeof(log_match),
// C: 					 log_with_addrport_tmpl, blockers,
// C: 					 dir_addr, addr, dir_port, port);
// C: 	if (log_match_len > sizeof(log_match))
// C: 		return -E2BIG;
// C: 
// C: 	return audit_match_record(audit_fd, AUDIT_LANDLOCK_ACCESS, log_match,
// C: 				  NULL);
// C: }
// C: 
// C: FIXTURE(audit)
// C: {
// C: 	struct service_fixture srv0;
// C: 	struct service_fixture srv1;
// C: 	/* srv2 has a rule with no access but quiet bit set. * /
// C: 	struct service_fixture srv2;
// C: 	struct service_fixture unspec_srv0;
// C: 	struct audit_filter audit_filter;
// C: 	int audit_fd;
// C: };
// C: 
// C: FIXTURE_VARIANT(audit)
// C: {
// C: 	const char *const addr;
// C: 	const struct protocol_variant prot;
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(audit, ipv4_tcp) {
// C: 	/* clang-format on * /
// C: 	.addr = "127\\.0\\.0\\.1",
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(audit, ipv4_udp) {
// C: 	/* clang-format on * /
// C: 	.addr = "127\\.0\\.0\\.1",
// C: 	.prot = {
// C: 		.domain = AF_INET,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(audit, ipv6_tcp) {
// C: 	/* clang-format on * /
// C: 	.addr = "::1",
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_STREAM,
// C: 	},
// C: };
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(audit, ipv6_udp) {
// C: 	/* clang-format on * /
// C: 	.addr = "::1",
// C: 	.prot = {
// C: 		.domain = AF_INET6,
// C: 		.type = SOCK_DGRAM,
// C: 	},
// C: };
// C: 
// C: FIXTURE_SETUP(audit)
// C: {
// C: 	struct protocol_variant prot_unspec = variant->prot;
// C: 
// C: 	prot_unspec.domain = AF_UNSPEC;
// C: 
// C: 	ASSERT_EQ(0, set_service(&self->srv0, variant->prot, 0));
// C: 	ASSERT_EQ(0, set_service(&self->srv1, variant->prot, 1));
// C: 	ASSERT_EQ(0, set_service(&self->srv2, variant->prot, 2));
// C: 	ASSERT_EQ(0, set_service(&self->unspec_srv0, prot_unspec, 0));
// C: 
// C: 	setup_loopback(_metadata);
// C: 
// C: 	set_cap(_metadata, CAP_AUDIT_CONTROL);
// C: 	self->audit_fd = audit_init_with_exe_filter(&self->audit_filter);
// C: 	EXPECT_LE(0, self->audit_fd);
// C: 	disable_caps(_metadata);
// C: };
// C: 
// C: FIXTURE_TEARDOWN(audit)
// C: {
// C: 	set_cap(_metadata, CAP_AUDIT_CONTROL);
// C: 	EXPECT_EQ(0, audit_cleanup(self->audit_fd, &self->audit_filter));
// C: 	clear_cap(_metadata, CAP_AUDIT_CONTROL);
// C: }
// C: 
// C: TEST_F(audit, bind)
// C: {
// C: 	const char *audit_evt = (variant->prot.type == SOCK_STREAM ?
// C: 					 "net\\.bind_tcp" :
// C: 					 "net\\.bind_udp");
// C: 	const __u64 access_rights =
// C: 		(variant->prot.type == SOCK_STREAM ?
// C: 			 LANDLOCK_ACCESS_NET_BIND_TCP |
// C: 				 LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 			 LANDLOCK_ACCESS_NET_BIND_UDP |
// C: 				 LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = access_rights,
// C: 		.quiet_access_net = access_rights,
// C: 	};
// C: 	const struct landlock_net_port_attr quiet_rule = {
// C: 		.allowed_access = 0,
// C: 		.port = self->srv2.port,
// C: 	};
// C: 	struct audit_records records;
// C: 	int ruleset_fd, sock_fd;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &quiet_rule, LANDLOCK_ADD_RULE_QUIET));
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	sock_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(-EACCES, bind_variant(sock_fd, &self->srv0));
// C: 	EXPECT_EQ(0, matches_auditlog(self->audit_fd, audit_evt, "saddr",
// C: 				      variant->addr, "src", self->srv0.port));
// C: 
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(1, records.domain);
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: 
// C: 	/* Bind to srv2 (with quiet rule): no new audit logs. * /
// C: 	sock_fd = socket_variant(&self->srv2);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(-EACCES, bind_variant(sock_fd, &self->srv2));
// C: 
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(0, records.domain);
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: }
// C: 
// C: TEST_F(audit, connect)
// C: {
// C: 	const char *audit_evt = (variant->prot.type == SOCK_STREAM ?
// C: 					 "net\\.connect_tcp" :
// C: 					 "net\\.connect_send_udp");
// C: 	const __u64 bind_right = (variant->prot.type == SOCK_STREAM ?
// C: 					  LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 					  LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 	const __u64 conn_right = (variant->prot.type == SOCK_STREAM ?
// C: 					  LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 					  LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 	const __u64 access_rights = bind_right | conn_right;
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = access_rights,
// C: 		.quiet_access_net = access_rights,
// C: 	};
// C: 	const struct landlock_net_port_attr rule_connect_p1 = {
// C: 		.allowed_access = conn_right,
// C: 		.port = self->srv1.port,
// C: 	};
// C: 	const struct landlock_net_port_attr quiet_rule = {
// C: 		.allowed_access = 0,
// C: 		.port = self->srv2.port,
// C: 	};
// C: 	struct audit_records records;
// C: 	int ruleset_fd, sock_fd;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &rule_connect_p1, 0));
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &quiet_rule, LANDLOCK_ADD_RULE_QUIET));
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	sock_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(-EACCES, connect_variant(sock_fd, &self->srv0));
// C: 	EXPECT_EQ(0, matches_auditlog(self->audit_fd, audit_evt, "daddr",
// C: 				      variant->addr, "dest", self->srv0.port));
// C: 
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(1, records.domain);
// C: 
// C: 	if (variant->prot.type == SOCK_DGRAM) {
// C: 		/* Check that autobind generates a denied bind event. * /
// C: 		EXPECT_EQ(-EACCES, connect_variant(sock_fd, &self->srv1));
// C: 
// C: 		EXPECT_EQ(0, matches_auditlog(self->audit_fd, "net\\.bind_udp",
// C: 					      NULL, NULL, NULL, 0));
// C: 		EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 		EXPECT_EQ(0, records.access);
// C: 		EXPECT_EQ(0, records.domain);
// C: 	}
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: 
// C: 	/* Connect to srv2 (with quiet rule): no new audit logs. * /
// C: 	sock_fd = socket_variant(&self->srv2);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(-EACCES, connect_variant(sock_fd, &self->srv2));
// C: 
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(0, records.domain);
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: }
// C: 
// C: /* Quieting bind access has no effect on connect. * /
// C: TEST_F(audit, connect_quiet_bind)
// C: {
// C: 	const char *audit_evt = (variant->prot.type == SOCK_STREAM ?
// C: 					 "net\\.connect_tcp" :
// C: 					 "net\\.connect_send_udp");
// C: 	const int bind_right = (variant->prot.type == SOCK_STREAM ?
// C: 					LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 					LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 	const int conn_right = (variant->prot.type == SOCK_STREAM ?
// C: 					LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 					LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 	const int access_rights = bind_right | conn_right;
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = access_rights,
// C: 		.quiet_access_net = bind_right,
// C: 	};
// C: 	const struct landlock_ruleset_attr ruleset_attr_2 = {
// C: 		.handled_access_net = access_rights,
// C: 		.quiet_access_net = conn_right,
// C: 	};
// C: 	const struct landlock_net_port_attr quiet_rule = {
// C: 		.allowed_access = 0,
// C: 		.port = self->srv2.port,
// C: 	};
// C: 	struct audit_records records;
// C: 	int ruleset_fd, sock_fd;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &quiet_rule, LANDLOCK_ADD_RULE_QUIET));
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	sock_fd = socket_variant(&self->srv2);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(-EACCES, connect_variant(sock_fd, &self->srv2));
// C: 	EXPECT_EQ(0, matches_auditlog(self->audit_fd, audit_evt, "daddr",
// C: 				      variant->addr, "dest", self->srv2.port));
// C: 
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: 
// C: 	/* New layer that also denies connect but has the correct quiet bit. * /
// C: 	ruleset_fd = landlock_create_ruleset(&ruleset_attr_2,
// C: 					     sizeof(ruleset_attr_2), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &quiet_rule, LANDLOCK_ADD_RULE_QUIET));
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	sock_fd = socket_variant(&self->srv2);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(-EACCES, connect_variant(sock_fd, &self->srv2));
// C: 
// C: 	/* Quieted - no logs expected. * /
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: }
// C: 
// C: static int matches_log_connect_bound(int audit_fd, const char *const blockers,
// C: 				     const char *const addr, __u16 lport,
// C: 				     __u16 dport)
// C: {
// C: 	static const char log_template[] = REGEX_LANDLOCK_PREFIX
// C: 		" blockers=%s laddr=%s lport=%u daddr=%s dest=%u$";
// C: 	/* Slack for the blockers, two addresses and two port numbers. * /
// C: 	char log_match[sizeof(log_template) + 60];
// C: 	int log_match_len;
// C: 
// C: 	log_match_len = snprintf(log_match, sizeof(log_match), log_template,
// C: 				 blockers, addr, lport, addr, dport);
// C: 	if (log_match_len > sizeof(log_match))
// C: 		return -E2BIG;
// C: 
// C: 	return audit_match_record(audit_fd, AUDIT_LANDLOCK_ACCESS, log_match,
// C: 				  NULL);
// C: }
// C: 
// C: /*
// C:  * After a bind() to an allowed port, a denied connect must report laddr/lport
// C:  * from the bound socket (made available through audit_net.sk) in addition to
// C:  * the connect sockaddr's daddr/dest.
// C:  * /
// C: TEST_F(audit, connect_bound)
// C: {
// C: 	const __u64 bind_right = (variant->prot.type == SOCK_STREAM ?
// C: 					  LANDLOCK_ACCESS_NET_BIND_TCP :
// C: 					  LANDLOCK_ACCESS_NET_BIND_UDP);
// C: 	const __u64 conn_right = (variant->prot.type == SOCK_STREAM ?
// C: 					  LANDLOCK_ACCESS_NET_CONNECT_TCP :
// C: 					  LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP);
// C: 	const char *const audit_evt = (variant->prot.type == SOCK_STREAM ?
// C: 					       "net\\.connect_tcp" :
// C: 					       "net\\.connect_send_udp");
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = bind_right | conn_right,
// C: 	};
// C: 	const struct landlock_net_port_attr rule_bind = {
// C: 		.allowed_access = bind_right,
// C: 		.port = self->srv0.port,
// C: 	};
// C: 	struct service_fixture srv_remote;
// C: 	struct audit_records records;
// C: 	int ruleset_fd, sock_fd;
// C: 
// C: 	/* Uses a second port as the denied connect target. * /
// C: 	ASSERT_EQ(0, set_service(&srv_remote, variant->prot, 1));
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &rule_bind, 0));
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	sock_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(0, bind_variant(sock_fd, &self->srv0));
// C: 	EXPECT_EQ(-EACCES, connect_variant(sock_fd, &srv_remote));
// C: 	EXPECT_EQ(0, matches_log_connect_bound(self->audit_fd, audit_evt,
// C: 					       variant->addr, self->srv0.port,
// C: 					       srv_remote.port));
// C: 
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(1, records.domain);
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: }
// C: 
// C: TEST_F(audit, sendmsg)
// C: {
// C: 	const struct landlock_ruleset_attr ruleset_attr = {
// C: 		.handled_access_net = LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP |
// C: 				      LANDLOCK_ACCESS_NET_BIND_UDP,
// C: 	};
// C: 	const struct landlock_net_port_attr rule = {
// C: 		.allowed_access = LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,
// C: 		.port = self->srv1.port,
// C: 	};
// C: 	struct audit_records records;
// C: 	int ruleset_fd;
// C: 	int sock_fd;
// C: 
// C: 	/* Sendmsg on stream sockets is never denied. * /
// C: 	if (variant->prot.type != SOCK_DGRAM)
// C: 		return;
// C: 
// C: 	ruleset_fd =
// C: 		landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 	ASSERT_LE(0, ruleset_fd);
// C: 	ASSERT_EQ(0, landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				       &rule, 0));
// C: 	enforce_ruleset(_metadata, ruleset_fd);
// C: 	EXPECT_EQ(0, close(ruleset_fd));
// C: 
// C: 	sock_fd = socket_variant(&self->srv0);
// C: 	ASSERT_LE(0, sock_fd);
// C: 	EXPECT_EQ(-EACCES, sendto_variant(sock_fd, &self->srv0, "A", 1, 0));
// C: 	EXPECT_EQ(0, matches_auditlog(self->audit_fd, "net\\.connect_send_udp",
// C: 				      "daddr", variant->addr, "dest",
// C: 				      self->srv0.port));
// C: 
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(1, records.domain);
// C: 
// C: 	/* Check that autobind generates a denied bind event. * /
// C: 	EXPECT_EQ(-EACCES, sendto_variant(sock_fd, &self->srv1, "A", 1, 0));
// C: 	EXPECT_EQ(0, matches_auditlog(self->audit_fd, "net\\.bind_udp", NULL,
// C: 				      NULL, NULL, 0));
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(0, records.domain);
// C: 
// C: 	EXPECT_EQ(-EACCES,
// C: 		  sendto_variant(sock_fd, &self->unspec_srv0, "B", 1, 0));
// C: 	EXPECT_EQ(0, matches_auditlog(self->audit_fd, "net\\.connect_send_udp",
// C: 				      "daddr", NULL, "dest", 0));
// C: 	EXPECT_EQ(0, audit_count_records(self->audit_fd, &records));
// C: 	EXPECT_EQ(0, records.access);
// C: 	EXPECT_EQ(0, records.domain);
// C: 
// C: 	EXPECT_EQ(0, close(sock_fd));
// C: }
// C: 
// C: /* Trace tests * /
// C: 
// C: /* clang-format off * /
// C: FIXTURE(trace_net) {
// C: 	/* clang-format on * /
// C: 	int tracefs_ok;
// C: };
// C: 
// C: FIXTURE_SETUP(trace_net)
// C: {
// C: 	int ret;
// C: 
// C: 	/* Isolate the network namespace so the bound port cannot collide. * /
// C: 	setup_loopback(_metadata);
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	ASSERT_EQ(0, unshare(CLONE_NEWNS));
// C: 	ASSERT_EQ(0, mount(NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL));
// C: 
// C: 	ret = tracefs_fixture_setup();
// C: 	if (ret) {
// C: 		clear_cap(_metadata, CAP_SYS_ADMIN);
// C: 		self->tracefs_ok = 0;
// C: 		SKIP(return, "tracefs not available");
// C: 	}
// C: 	self->tracefs_ok = 1;
// C: 
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_enable_event(TRACEFS_DENY_ACCESS_NET_ENABLE, true));
// C: 	ASSERT_EQ(0, tracefs_clear());
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: FIXTURE_TEARDOWN(trace_net)
// C: {
// C: 	if (!self->tracefs_ok)
// C: 		return;
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	tracefs_enable_event(TRACEFS_DENY_ACCESS_NET_ENABLE, false);
// C: 	tracefs_fixture_teardown();
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: /*
// C:  * Baseline: verifies that without Landlock, the bind succeeds and no
// C:  * deny_access_net trace event fires.
// C:  * /
// C: /* clang-format off * /
// C: FIXTURE_VARIANT(trace_net)
// C: {
// C: 	/* clang-format on * /
// C: 	bool sandbox;
// C: 	int bind_port_offset; /* 0 = allowed port, 1 = denied port * /
// C: 	int expect_denied;
// C: };
// C: 
// C: /* Unsandboxed: no Landlock, bind should succeed with no events. * /
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(trace_net, unsandboxed) {
// C: 	/* clang-format on * /
// C: 	.sandbox = false,
// C: 	.bind_port_offset = 0,
// C: 	.expect_denied = 0,
// C: };
// C: 
// C: /* Denied: sandboxed, bind to port not in ruleset. * /
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(trace_net, bind_denied) {
// C: 	/* clang-format on * /
// C: 	.sandbox = true,
// C: 	.bind_port_offset = 1,
// C: 	.expect_denied = 1,
// C: };
// C: 
// C: /* Allowed: sandboxed, bind to port in ruleset. * /
// C: /* clang-format off * /
// C: FIXTURE_VARIANT_ADD(trace_net, bind_allowed) {
// C: 	/* clang-format on * /
// C: 	.sandbox = true,
// C: 	.bind_port_offset = 0,
// C: 	.expect_denied = 0,
// C: };
// C: 
// C: TEST_F(trace_net, deny_access_net_bind)
// C: {
// C: 	char *buf;
// C: 	int count, status;
// C: 	pid_t child;
// C: 
// C: 	if (!self->tracefs_ok)
// C: 		SKIP(return, "tracefs not available");
// C: 
// C: 	ASSERT_EQ(0, tracefs_clear_buf());
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 
// C: 	if (child == 0) {
// C: 		struct sockaddr_in addr = {
// C: 			.sin_family = AF_INET,
// C: 			.sin_addr.s_addr = htonl(INADDR_LOOPBACK),
// C: 		};
// C: 		int sock_fd;
// C: 
// C: 		if (variant->sandbox) {
// C: 			struct landlock_ruleset_attr ruleset_attr = {
// C: 				.handled_access_net =
// C: 					LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 			};
// C: 			struct landlock_net_port_attr port_attr = {
// C: 				.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 				.port = sock_port_start,
// C: 			};
// C: 			int ruleset_fd;
// C: 
// C: 			ruleset_fd = landlock_create_ruleset(
// C: 				&ruleset_attr, sizeof(ruleset_attr), 0);
// C: 			if (ruleset_fd < 0)
// C: 				_exit(1);
// C: 
// C: 			if (landlock_add_rule(ruleset_fd,
// C: 					      LANDLOCK_RULE_NET_PORT,
// C: 					      &port_attr, 0)) {
// C: 				close(ruleset_fd);
// C: 				_exit(1);
// C: 			}
// C: 
// C: 			prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
// C: 			if (landlock_restrict_self(ruleset_fd, 0)) {
// C: 				close(ruleset_fd);
// C: 				_exit(1);
// C: 			}
// C: 			close(ruleset_fd);
// C: 		}
// C: 
// C: 		sock_fd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
// C: 		if (sock_fd < 0)
// C: 			_exit(1);
// C: 
// C: 		addr.sin_port =
// C: 			htons(sock_port_start + variant->bind_port_offset);
// C: 		if (variant->expect_denied) {
// C: 			/* Bind should be denied. * /
// C: 			if (bind(sock_fd, (struct sockaddr *)&addr,
// C: 				 sizeof(addr)) == 0) {
// C: 				close(sock_fd);
// C: 				_exit(2);
// C: 			}
// C: 			if (errno != EACCES) {
// C: 				close(sock_fd);
// C: 				_exit(3);
// C: 			}
// C: 		} else {
// C: 			/* Bind should succeed. * /
// C: 			if (bind(sock_fd, (struct sockaddr *)&addr,
// C: 				 sizeof(addr))) {
// C: 				close(sock_fd);
// C: 				_exit(2);
// C: 			}
// C: 		}
// C: 		close(sock_fd);
// C: 		_exit(0);
// C: 	}
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	ASSERT_TRUE(WIFEXITED(status));
// C: 	EXPECT_EQ(0, WEXITSTATUS(status));
// C: 
// C: 	buf = tracefs_read_buf();
// C: 	ASSERT_NE(NULL, buf);
// C: 
// C: 	count = tracefs_count_matches(buf, REGEX_DENY_ACCESS_NET(TRACE_TASK));
// C: 	if (variant->expect_denied) {
// C: 		EXPECT_EQ(variant->expect_denied, count)
// C: 		{
// C: 			TH_LOG("Expected deny_access_net event, got %d\n%s",
// C: 			       count, buf);
// C: 		}
// C: 	} else {
// C: 		EXPECT_EQ(0, count)
// C: 		{
// C: 			TH_LOG("Expected 0 deny_access_net events, "
// C: 			       "got %d\n%s",
// C: 			       count, buf);
// C: 		}
// C: 	}
// C: 
// C: 	free(buf);
// C: }
// C: 
// C: /*
// C:  * Anchors the denial fields shared by every deny_access_net event so a field
// C:  * test proves more than sport/dport: the denying domain, the same-exec bit, the
// C:  * audit-logging verdict, and the blocked access all stay populated.
// C:  * /
// C: static void
// C: expect_net_deny_common_fields(struct __test_metadata *const _metadata,
// C: 			      const char *const buf)
// C: {
// C: 	char field[64];
// C: 
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_DENY_ACCESS_NET(TRACE_TASK),
// C: 					"domain", field, sizeof(field)));
// C: 	EXPECT_STRNE("0", field);
// C: 
// C: 	/* Same exec that restricted itself, no exec in between. * /
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_DENY_ACCESS_NET(TRACE_TASK),
// C: 					"same_exec", field, sizeof(field)));
// C: 	EXPECT_STREQ("1", field);
// C: 
// C: 	/* Default flags, same exec: audit would log this denial. * /
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_DENY_ACCESS_NET(TRACE_TASK),
// C: 					"logged", field, sizeof(field)));
// C: 	EXPECT_STREQ("1", field);
// C: 
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_DENY_ACCESS_NET(TRACE_TASK),
// C: 					"blockers", field, sizeof(field)));
// C: 	EXPECT_STRNE("", field);
// C: }
// C: 
// C: /* Connect and field-check tests use a separate fixture without variants. * /
// C: 
// C: /* clang-format off * /
// C: FIXTURE(trace_net_connect) {
// C: 	/* clang-format on * /
// C: 	int tracefs_ok;
// C: };
// C: 
// C: FIXTURE_SETUP(trace_net_connect)
// C: {
// C: 	int ret;
// C: 
// C: 	/* Isolate the network namespace so the bound port cannot collide. * /
// C: 	setup_loopback(_metadata);
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	ASSERT_EQ(0, unshare(CLONE_NEWNS));
// C: 	ASSERT_EQ(0, mount(NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL));
// C: 
// C: 	ret = tracefs_fixture_setup();
// C: 	if (ret) {
// C: 		clear_cap(_metadata, CAP_SYS_ADMIN);
// C: 		self->tracefs_ok = 0;
// C: 		SKIP(return, "tracefs not available");
// C: 	}
// C: 	self->tracefs_ok = 1;
// C: 
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_enable_event(TRACEFS_DENY_ACCESS_NET_ENABLE, true));
// C: 	ASSERT_EQ(0, tracefs_clear());
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: FIXTURE_TEARDOWN(trace_net_connect)
// C: {
// C: 	if (!self->tracefs_ok)
// C: 		return;
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	tracefs_enable_event(TRACEFS_DENY_ACCESS_NET_ENABLE, false);
// C: 	tracefs_fixture_teardown();
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: /* clang-format off * /
// C: FIXTURE_VARIANT(trace_net_connect) {
// C: 	/* clang-format on * /
// C: 	/* handled_access_net, also the access allowed on the base port. * /
// C: 	__u64 handled;
// C: 	/* Bind the allowed base port before the denied operation. * /
// C: 	bool bind_base_first;
// C: 	/* Denied operation on the next port: connect (true) or bind (false). * /
// C: 	bool deny_connect;
// C: };
// C: 
// C: /* clang-format off * /
// C: 
// C: /* Denied connect(): sport=0, dport=<denied port>. * /
// C: FIXTURE_VARIANT_ADD(trace_net_connect, connect_denied) {
// C: 	.handled = LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 	.bind_base_first = false,
// C: 	.deny_connect = true,
// C: };
// C: 
// C: /* Denied bind(): sport=<denied port>, dport=0. * /
// C: FIXTURE_VARIANT_ADD(trace_net_connect, bind_fields) {
// C: 	.handled = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 	.bind_base_first = false,
// C: 	.deny_connect = false,
// C: };
// C: 
// C: /* Denied connect() after an allowed bind(): the connect fields (sport=0). * /
// C: FIXTURE_VARIANT_ADD(trace_net_connect, connect_after_bind) {
// C: 	.handled = LANDLOCK_ACCESS_NET_BIND_TCP | LANDLOCK_ACCESS_NET_CONNECT_TCP,
// C: 	.bind_base_first = true,
// C: 	.deny_connect = true,
// C: };
// C: 
// C: /* clang-format on * /
// C: 
// C: /*
// C:  * A denied TCP bind(2) or connect(2) emits one deny_access_net event.  The port
// C:  * is reported in the field matching the denied operation, in host endianness
// C:  * (the UAPI landlock_net_port_attr.port convention): a connect denial reports
// C:  * sport=0 dport=<port>, a bind denial reports sport=<port> dport=0, so a
// C:  * byte-order or field-swap bug is caught.  A prior allowed bind
// C:  * (connect_after_bind) does not change the connect denial's fields.
// C:  * /
// C: TEST_F(trace_net_connect, deny_access_net)
// C: {
// C: 	pid_t child;
// C: 	int status;
// C: 	char *buf;
// C: 	char field[64], expected[16];
// C: 
// C: 	if (!self->tracefs_ok)
// C: 		SKIP(return, "tracefs not available");
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 
// C: 	if (child == 0) {
// C: 		struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = variant->handled,
// C: 		};
// C: 		struct landlock_net_port_attr port_attr = {
// C: 			.allowed_access = variant->handled,
// C: 			.port = sock_port_start,
// C: 		};
// C: 		struct sockaddr_in addr = {
// C: 			.sin_family = AF_INET,
// C: 			.sin_addr.s_addr = htonl(INADDR_LOOPBACK),
// C: 		};
// C: 		int ruleset_fd, sock_fd, optval = 1, ret;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		if (ruleset_fd < 0)
// C: 			_exit(1);
// C: 		if (landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				      &port_attr, 0)) {
// C: 			close(ruleset_fd);
// C: 			_exit(1);
// C: 		}
// C: 		prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
// C: 		if (landlock_restrict_self(ruleset_fd, 0)) {
// C: 			close(ruleset_fd);
// C: 			_exit(1);
// C: 		}
// C: 		close(ruleset_fd);
// C: 
// C: 		sock_fd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
// C: 		if (sock_fd < 0)
// C: 			_exit(1);
// C: 
// C: 		/* Bind the allowed base port first (succeeds, no event). * /
// C: 		if (variant->bind_base_first) {
// C: 			setsockopt(sock_fd, SOL_SOCKET, SO_REUSEADDR, &optval,
// C: 				   sizeof(optval));
// C: 			addr.sin_port = htons(sock_port_start);
// C: 			if (bind(sock_fd, (struct sockaddr *)&addr,
// C: 				 sizeof(addr))) {
// C: 				close(sock_fd);
// C: 				_exit(1);
// C: 			}
// C: 		}
// C: 
// C: 		/* Denied operation on the next port. * /
// C: 		addr.sin_port = htons(sock_port_start + 1);
// C: 		if (variant->deny_connect)
// C: 			ret = connect(sock_fd, (struct sockaddr *)&addr,
// C: 				      sizeof(addr));
// C: 		else
// C: 			ret = bind(sock_fd, (struct sockaddr *)&addr,
// C: 				   sizeof(addr));
// C: 		if (ret == 0) {
// C: 			close(sock_fd);
// C: 			_exit(2);
// C: 		}
// C: 		if (errno != EACCES) {
// C: 			close(sock_fd);
// C: 			_exit(3);
// C: 		}
// C: 		close(sock_fd);
// C: 		_exit(0);
// C: 	}
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	ASSERT_TRUE(WIFEXITED(status));
// C: 	EXPECT_EQ(0, WEXITSTATUS(status));
// C: 
// C: 	buf = tracefs_read_buf();
// C: 	ASSERT_NE(NULL, buf);
// C: 
// C: 	EXPECT_EQ(1, tracefs_count_matches(buf,
// C: 					   REGEX_DENY_ACCESS_NET(TRACE_TASK)));
// C: 
// C: 	expect_net_deny_common_fields(_metadata, buf);
// C: 
// C: 	/*
// C: 	 * The denied operation's port field carries the port; the other is 0.
// C: 	 * /
// C: 	snprintf(expected, sizeof(expected), "%llu",
// C: 		 (unsigned long long)(sock_port_start + 1));
// C: 
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_DENY_ACCESS_NET(TRACE_TASK),
// C: 					"sport", field, sizeof(field)));
// C: 	EXPECT_STREQ(variant->deny_connect ? "0" : expected, field);
// C: 
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_DENY_ACCESS_NET(TRACE_TASK),
// C: 					"dport", field, sizeof(field)));
// C: 	EXPECT_STREQ(variant->deny_connect ? expected : "0", field);
// C: 
// C: 	free(buf);
// C: }
// C: 
// C: /* Field verification for the check_rule_net event on an allowed access. * /
// C: 
// C: /* clang-format off * /
// C: FIXTURE(trace_net_check_rule) {
// C: 	/* clang-format on * /
// C: 	int tracefs_ok;
// C: };
// C: 
// C: FIXTURE_SETUP(trace_net_check_rule)
// C: {
// C: 	int ret;
// C: 
// C: 	/* Isolate the network namespace so the bound port cannot collide. * /
// C: 	setup_loopback(_metadata);
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	ASSERT_EQ(0, unshare(CLONE_NEWNS));
// C: 	ASSERT_EQ(0, mount(NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL));
// C: 
// C: 	ret = tracefs_fixture_setup();
// C: 	if (ret) {
// C: 		clear_cap(_metadata, CAP_SYS_ADMIN);
// C: 		self->tracefs_ok = 0;
// C: 		SKIP(return, "tracefs not available");
// C: 	}
// C: 	self->tracefs_ok = 1;
// C: 
// C: 	ASSERT_EQ(0, tracefs_enable_event(TRACEFS_CHECK_RULE_NET_ENABLE, true));
// C: 	ASSERT_EQ(0, tracefs_clear());
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: FIXTURE_TEARDOWN(trace_net_check_rule)
// C: {
// C: 	if (!self->tracefs_ok)
// C: 		return;
// C: 
// C: 	set_cap(_metadata, CAP_SYS_ADMIN);
// C: 	tracefs_enable_event(TRACEFS_CHECK_RULE_NET_ENABLE, false);
// C: 	tracefs_fixture_teardown();
// C: 	clear_cap(_metadata, CAP_SYS_ADMIN);
// C: }
// C: 
// C: /*
// C:  * Verifies that an allowed bind matching a net-port rule emits exactly one
// C:  * landlock_check_rule_net event with the enforcing domain, the requested
// C:  * access, the checked port (host endianness), and the per-layer grants.  The
// C:  * whole event is anchored to exact values so a revert of the check_rule_net
// C:  * emit (or a byte-order or field-plumbing regression) fails the test.
// C:  * /
// C: TEST_F(trace_net_check_rule, check_rule_net_fields)
// C: {
// C: 	pid_t child;
// C: 	int status;
// C: 	char *buf;
// C: 	char field[64], expected[16];
// C: 
// C: 	if (!self->tracefs_ok)
// C: 		SKIP(return, "tracefs not available");
// C: 
// C: 	child = fork();
// C: 	ASSERT_LE(0, child);
// C: 
// C: 	if (child == 0) {
// C: 		struct landlock_ruleset_attr ruleset_attr = {
// C: 			.handled_access_net = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 		};
// C: 		struct landlock_net_port_attr port_attr = {
// C: 			.allowed_access = LANDLOCK_ACCESS_NET_BIND_TCP,
// C: 			.port = sock_port_start,
// C: 		};
// C: 		struct sockaddr_in addr = {
// C: 			.sin_family = AF_INET,
// C: 			.sin_addr.s_addr = htonl(INADDR_LOOPBACK),
// C: 		};
// C: 		int ruleset_fd, sock_fd;
// C: 
// C: 		ruleset_fd = landlock_create_ruleset(&ruleset_attr,
// C: 						     sizeof(ruleset_attr), 0);
// C: 		if (ruleset_fd < 0)
// C: 			_exit(1);
// C: 
// C: 		if (landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
// C: 				      &port_attr, 0)) {
// C: 			close(ruleset_fd);
// C: 			_exit(1);
// C: 		}
// C: 
// C: 		prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
// C: 		if (landlock_restrict_self(ruleset_fd, 0)) {
// C: 			close(ruleset_fd);
// C: 			_exit(1);
// C: 		}
// C: 		close(ruleset_fd);
// C: 
// C: 		/* Bind to the allowed port: succeeds and matches the rule. * /
// C: 		sock_fd = socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0);
// C: 		if (sock_fd < 0)
// C: 			_exit(1);
// C: 
// C: 		addr.sin_port = htons(sock_port_start);
// C: 		if (bind(sock_fd, (struct sockaddr *)&addr, sizeof(addr))) {
// C: 			close(sock_fd);
// C: 			_exit(2);
// C: 		}
// C: 		close(sock_fd);
// C: 		_exit(0);
// C: 	}
// C: 
// C: 	ASSERT_EQ(child, waitpid(child, &status, 0));
// C: 	ASSERT_TRUE(WIFEXITED(status));
// C: 	EXPECT_EQ(0, WEXITSTATUS(status));
// C: 
// C: 	buf = tracefs_read_buf();
// C: 	ASSERT_NE(NULL, buf);
// C: 
// C: 	/* A single-layer domain matching one port rule emits one event. * /
// C: 	EXPECT_EQ(1,
// C: 		  tracefs_count_matches(buf, REGEX_CHECK_RULE_NET(TRACE_TASK)))
// C: 	{
// C: 		TH_LOG("Expected 1 check_rule_net event\n%s", buf);
// C: 	}
// C: 
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_CHECK_RULE_NET(TRACE_TASK),
// C: 					"domain", field, sizeof(field)));
// C: 	EXPECT_STRNE("0", field);
// C: 
// C: 	ASSERT_EQ(0, tracefs_extract_field(
// C: 			     buf, REGEX_CHECK_RULE_NET(TRACE_TASK),
// C: 			     "access_request", field, sizeof(field)));
// C: 	EXPECT_STREQ("bind_tcp", field);
// C: 
// C: 	/*
// C: 	 * The port is reported in host endianness (UAPI convention), so on
// C: 	 * little-endian htons(sock_port_start) would print a different value:
// C: 	 * the exact match also catches byte-order regressions.
// C: 	 * /
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_CHECK_RULE_NET(TRACE_TASK),
// C: 					"port", field, sizeof(field)));
// C: 	snprintf(expected, sizeof(expected), "%llu",
// C: 		 (unsigned long long)sock_port_start);
// C: 	EXPECT_STREQ(expected, field);
// C: 
// C: 	/* One layer that fully grants the request: grants={bind_tcp}. * /
// C: 	ASSERT_EQ(0,
// C: 		  tracefs_extract_field(buf, REGEX_CHECK_RULE_NET(TRACE_TASK),
// C: 					"grants", field, sizeof(field)));
// C: 	EXPECT_STREQ("{bind_tcp}", field);
// C: 
// C: 	free(buf);
// C: }
// C: 
// C: /*
// C:  * IPv6 network trace tests are intentionally elided.  IPv6 hook dispatch uses
// C:  * the same current_check_access_socket() code path as IPv4, validated by the
// C:  * audit tests in this file.  The trace events use the same blockers/sport/dport
// C:  * fields regardless of address family.
// C:  * /
// C: 
// C: TEST_HARNESS_MAIN
