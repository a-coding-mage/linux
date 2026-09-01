// SPDX-License-Identifier: GPL-2.0
/* nettest - used for functional tests of networking APIs
 *
 * Copyright (c) 2013-2019 David Ahern <dsahern@gmail.com>. All rights reserved.
 *
 * Source-level Rust translation of testing/selftests/net/nettest.c.
 * C system headers are represented by libc-style items and raw FFI calls.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;
type time_t = i64;
type pid_t = c_int;

const IPV6_UNICAST_IF: c_int = 76;
const IPV6_MULTICAST_IF: c_int = 17;
const DEFAULT_PORT: u16 = 12345;
const NS_PREFIX: &[u8] = b"/run/netns/\0";

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const PF_INET: c_int = AF_INET;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_RAW: c_int = 3;
const SOL_SOCKET: c_int = 1;
const SOL_IP: c_int = 0;
const SOL_IPV6: c_int = 41;
const IPPROTO_IP: c_int = 0;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const IPPROTO_RAW: c_int = 255;
const SO_REUSEADDR: c_int = 2;
const SO_DONTROUTE: c_int = 5;
const SO_BROADCAST: c_int = 6;
const SO_ERROR: c_int = 4;
const SO_BINDTODEVICE: c_int = 25;
const SO_REUSEPORT: c_int = 15;
const IP_PKTINFO: c_int = 8;
const IP_RECVERR: c_int = 11;
const IP_TOS: c_int = 1;
const IP_FREEBIND: c_int = 15;
const IP_UNICAST_IF: c_int = 50;
const IP_MULTICAST_IF: c_int = 32;
const IP_ADD_MEMBERSHIP: c_int = 35;
const IPV6_RECVPKTINFO: c_int = 49;
const IPV6_PKTINFO: c_int = 50;
const IPV6_RECVERR: c_int = 25;
const IPV6_TCLASS: c_int = 67;
const IPV6_FREEBIND: c_int = 78;
const TCP_MD5SIG: c_int = 14;
const TCP_MD5SIG_EXT: c_int = 32;
const TCP_MD5SIG_FLAG_PREFIX: u8 = 1;
const TCP_MD5SIG_FLAG_IFINDEX: u8 = 2;
const UDP_ENCAP: c_int = 100;
const UDP_ENCAP_ESPINUDP: c_int = 2;
const IP_XFRM_POLICY: c_int = 17;
const IPV6_XFRM_POLICY: c_int = 34;
const XFRM_POLICY_ALLOW: u8 = 0;
const XFRM_POLICY_OUT: u8 = 1;
const XFRM_POLICY_IN: u8 = 0;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const F_SETFD: c_int = 2;
const FD_CLOEXEC: c_int = 1;
const O_NONBLOCK: c_int = 0o4000;
const CLONE_NEWNET: c_int = 0x40000000;
const SIOCGIFINDEX: c_ulong = 0x8933;
const PATH_MAX: usize = 4096;
const INT_MAX: c_int = 2147483647;
const INADDR_ANY: u32 = 0;
const ENOENT: c_int = 2;
const EINTR: c_int = 4;
const EACCES: c_int = 13;
const ENOMEM: c_int = 12;
const ERANGE: c_int = 34;
const ENOTSUP: c_int = 95;
const EINPROGRESS: c_int = 115;
const SIGKILL: c_int = 9;
const FD_SETSIZE: c_int = 1024;

const GETOPT_STR: &[u8] = b"sr:l:c:Q:p:t:g:P:DRn:M:X:m:d:I:BN:O:SUCi6xL:0:1:2:3:Fbqf\0";
const OPT_FORCE_BIND_KEY_IFINDEX: c_int = 1001;
const OPT_NO_BIND_KEY_IFINDEX: c_int = 1002;
const OPT_CLIENT_DONTROUTE: c_int = 1003;
const OPT_SERVER_DONTROUTE: c_int = 1004;

#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ip_mreqn {
    imr_multiaddr: in_addr,
    imr_address: in_addr,
    imr_ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in_pktinfo {
    ipi_ifindex: c_int,
    ipi_spec_dst: in_addr,
    ipi_addr: in_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_pktinfo {
    ipi6_addr: in6_addr,
    ipi6_ifindex: c_uint,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}

#[repr(C)]
struct cmsghdr {
    cmsg_len: size_t,
    cmsg_level: c_int,
    cmsg_type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
union SockAddrStorage {
    v4: sockaddr_in,
    v6: sockaddr_in6,
}

#[repr(C)]
#[derive(Copy, Clone)]
union InAddrUnion {
    in_: in_addr,
    in6: in6_addr,
}

#[repr(C)]
struct tcp_md5sig {
    tcpm_addr: sockaddr,
    __tcpm_pad1: u16,
    tcpm_keylen: u16,
    __tcpm_pad2: u32,
    tcpm_key: [u8; 80],
    tcpm_flags: u8,
    tcpm_prefixlen: u8,
    tcpm_ifindex: c_int,
}

#[repr(C)]
struct xfrm_selector {
    daddr: [u8; 16],
    saddr: [u8; 16],
    dport: u16,
    dport_mask: u16,
    sport: u16,
    sport_mask: u16,
    family: u16,
    prefixlen_d: u8,
    prefixlen_s: u8,
    proto: u8,
    ifindex: c_int,
    user: u32,
}

#[repr(C)]
struct xfrm_userpolicy_info {
    sel: xfrm_selector,
    lft: [u64; 8],
    curlft: [u64; 4],
    priority: u32,
    index: u32,
    dir: u8,
    action: u8,
    flags: u8,
    share: u8,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: time_t,
}

#[repr(C)]
struct fd_set {
    fds_bits: [c_long; 16],
}

#[repr(C)]
struct ifreq {
    ifr_name: [c_char; 16],
    ifr_ifindex: c_int,
}

#[repr(C)]
struct protoent {
    p_name: *mut c_char,
    p_aliases: *mut *mut c_char,
    p_proto: c_int,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct sock_args {
    /* local address */
    local_addr_str: *const c_char,
    client_local_addr_str: *const c_char,
    local_addr: InAddrUnion,

    /* remote address */
    remote_addr_str: *const c_char,
    remote_addr: InAddrUnion,
    scope_id: c_int, /* remote scope; v6 send only */

    grp: in_addr, /* multicast group */

    has_local_ip: c_uint,
    has_remote_ip: c_uint,
    has_grp: c_uint,
    has_expected_laddr: c_uint,
    has_expected_raddr: c_uint,
    bind_test_only: c_uint,
    client_dontroute: c_uint,
    server_dontroute: c_uint,

    port: u16,

    type_: c_int, /* DGRAM, STREAM, RAW */
    protocol: c_int,
    version: c_int, /* AF_INET/AF_INET6 */

    use_setsockopt: c_int,
    use_freebind: c_int,
    use_cmsg: c_int,
    dsfield: u8,
    dev: *const c_char,
    server_dev: *const c_char,
    ifindex: c_int,

    clientns: *const c_char,
    serverns: *const c_char,

    password: *const c_char,
    client_pw: *const c_char,
    /* prefix for MD5 password */
    md5_prefix_str: *const c_char,
    md5_prefix: SockAddrStorage,
    prefix_len: c_uint,
    /* 0: default, -1: force off, +1: force on */
    bind_key_ifindex: c_int,

    /* expected addresses and device index for connection */
    expected_dev: *const c_char,
    expected_server_dev: *const c_char,
    expected_ifindex: c_int,

    /* local address */
    expected_laddr_str: *const c_char,
    expected_laddr: InAddrUnion,

    /* remote address */
    expected_raddr_str: *const c_char,
    expected_raddr: InAddrUnion,

    /* ESP in UDP encap test */
    use_xfrm: c_int,

    /* use send() and connect() instead of sendto */
    datagram_connect: c_int,
}

#[repr(C)]
enum addr_type {
    ADDR_TYPE_LOCAL,
    ADDR_TYPE_REMOTE,
    ADDR_TYPE_MCAST,
    ADDR_TYPE_EXPECTED_LOCAL,
    ADDR_TYPE_EXPECTED_REMOTE,
    ADDR_TYPE_MD5_PREFIX,
}

static mut server_mode: c_int = 0;
static mut prog_timeout: c_uint = 5;
static mut interactive: c_uint = 0;
static mut iter: c_int = 1;
static mut msg: *mut c_char = b"Hello world!\0".as_ptr() as *mut c_char;
static mut msglen: c_int = 0;
static mut quiet: c_int = 0;
static mut try_broadcast: c_int = 1;

static mut long_opts: [option; 5] = [
    option { name: b"force-bind-key-ifindex\0".as_ptr() as *const c_char, has_arg: 0, flag: null_mut(), val: OPT_FORCE_BIND_KEY_IFINDEX },
    option { name: b"no-bind-key-ifindex\0".as_ptr() as *const c_char, has_arg: 0, flag: null_mut(), val: OPT_NO_BIND_KEY_IFINDEX },
    option { name: b"client-dontroute\0".as_ptr() as *const c_char, has_arg: 0, flag: null_mut(), val: OPT_CLIENT_DONTROUTE },
    option { name: b"server-dontroute\0".as_ptr() as *const c_char, has_arg: 0, flag: null_mut(), val: OPT_SERVER_DONTROUTE },
    option { name: null(), has_arg: 0, flag: null_mut(), val: 0 },
];

extern "C" {
    static mut errno: c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;

    fn time(tloc: *mut time_t) -> time_t;
    fn localtime(timep: *const time_t) -> *mut c_void;
    fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const c_void) -> size_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn atoi(nptr: *const c_char) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fflush(stream: *mut FILE) -> c_int;
    fn setbuffer(stream: *mut FILE, buf: *mut c_char, size: size_t);
    fn fileno(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(socket: c_int, level: c_int, option_name: c_int, option_value: *const c_void, option_len: socklen_t) -> c_int;
    fn getsockopt(socket: c_int, level: c_int, option_name: c_int, option_value: *mut c_void, option_len: *mut socklen_t) -> c_int;
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn getpeername(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn bind(sockfd: c_int, addr: *const c_void, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut socklen_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const c_void, addrlen: socklen_t) -> c_int;
    fn sendto(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int, dest_addr: *const c_void, addrlen: socklen_t) -> ssize_t;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn geteuid() -> c_uint;
    fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *mut timeval) -> c_int;
    fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: socklen_t) -> *const c_char;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn htons(hostshort: u16) -> u16;
    fn ntohs(netshort: u16) -> u16;
    fn htonl(hostlong: u32) -> u32;
    fn get_ifidx(ifname: *const c_char) -> c_int;
    fn getprotobyname(name: *const c_char) -> *mut protoent;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn wait(wstatus: *mut c_int) -> pid_t;
    static in6addr_any: in6_addr;
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    (*set).fds_bits = [0; 16];
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let idx = (fd / (8 * size_of::<c_long>() as c_int)) as usize;
    let bit = fd % (8 * size_of::<c_long>() as c_int);
    (*set).fds_bits[idx] |= 1 << bit;
}

unsafe fn FD_ISSET(fd: c_int, set: *mut fd_set) -> bool {
    let idx = (fd / (8 * size_of::<c_long>() as c_int)) as usize;
    let bit = fd % (8 * size_of::<c_long>() as c_int);
    ((*set).fds_bits[idx] & (1 << bit)) != 0
}

fn CMSG_ALIGN(len: usize) -> usize {
    let align = size_of::<usize>();
    (len + align - 1) & !(align - 1)
}

fn CMSG_LEN(len: usize) -> usize {
    CMSG_ALIGN(size_of::<cmsghdr>()) + len
}

unsafe fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(CMSG_ALIGN(size_of::<cmsghdr>()))
}

unsafe fn CMSG_FIRSTHDR(m: *mut msghdr) -> *mut cmsghdr {
    if (*m).msg_controllen >= size_of::<cmsghdr>() {
        (*m).msg_control as *mut cmsghdr
    } else {
        null_mut()
    }
}

unsafe fn CMSG_NXTHDR(m: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let next = (cmsg as *mut u8).add(CMSG_ALIGN((*cmsg).cmsg_len)) as *mut cmsghdr;
    let max = ((*m).msg_control as *mut u8).add((*m).msg_controllen);
    if (next as *mut u8).add(size_of::<cmsghdr>()) > max {
        null_mut()
    } else {
        next
    }
}

unsafe fn timestamp(timebuf: *mut c_char, buflen: c_int) -> *mut c_char {
    let mut now: time_t = 0;
    now = time(null_mut());
    if strftime(timebuf, buflen as size_t, b"%T\0".as_ptr() as *const c_char, localtime(&now)) == 0 {
        memset(timebuf as *mut c_void, 0, buflen as size_t);
        strncpy(timebuf, b"00:00:00\0".as_ptr() as *const c_char, (buflen - 1) as size_t);
    }
    timebuf
}

unsafe fn log_prefix(stream: *mut FILE) {
    let mut timebuf = [0 as c_char; 64];
    fprintf(
        stream,
        b"%s %s:\0".as_ptr() as *const c_char,
        timestamp(timebuf.as_mut_ptr(), timebuf.len() as c_int),
        if server_mode != 0 { b"server\0".as_ptr() } else { b"client\0".as_ptr() },
    );
}

macro_rules! log_msg {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if quiet == 0 {
                log_prefix(stdout);
                fprintf(stdout, $fmt.as_ptr() as *const c_char $(, $arg)*);
                fflush(stdout);
            }
        }
    }};
}

macro_rules! log_error {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if quiet == 0 {
                log_prefix(stderr);
                fprintf(stderr, $fmt.as_ptr() as *const c_char $(, $arg)*);
                fflush(stderr);
            }
        }
    }};
}

macro_rules! log_err_errno {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if quiet == 0 {
                let mut timebuf = [0 as c_char; 64];
                fprintf(stderr, b"%s %s: \0".as_ptr() as *const c_char,
                    timestamp(timebuf.as_mut_ptr(), timebuf.len() as c_int),
                    if server_mode != 0 { b"server\0".as_ptr() } else { b"client\0".as_ptr() });
                fprintf(stderr, $fmt.as_ptr() as *const c_char $(, $arg)*);
                fprintf(stderr, b": %d: %s\n\0".as_ptr() as *const c_char, errno, strerror(errno));
                fflush(stderr);
            }
        }
    }};
}

unsafe fn log_address(desc: *const c_char, sa: *mut sockaddr) {
    let mut addrstr = [0 as c_char; 64];
    if quiet != 0 {
        return;
    }
    if (*sa).sa_family as c_int == AF_INET {
        let s = sa as *mut sockaddr_in;
        log_msg!(b"%s %s:%d\n\0", desc, inet_ntop(AF_INET, &(*s).sin_addr as *const _ as *const c_void, addrstr.as_mut_ptr(), addrstr.len() as socklen_t), ntohs((*s).sin_port) as c_int);
    } else if (*sa).sa_family as c_int == AF_INET6 {
        let s6 = sa as *mut sockaddr_in6;
        log_msg!(b"%s [%s]:%d\n\0", desc, inet_ntop(AF_INET6, &(*s6).sin6_addr as *const _ as *const c_void, addrstr.as_mut_ptr(), addrstr.len() as socklen_t), ntohs((*s6).sin6_port) as c_int);
    }
    fflush(stdout);
}

unsafe fn switch_ns(ns: *const c_char) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    if geteuid() != 0 {
        log_error!(b"warning: likely need root to set netns %s!\n\0", ns);
    }
    snprintf(path.as_mut_ptr(), path.len(), b"%s%s\0".as_ptr() as *const c_char, NS_PREFIX.as_ptr(), ns);
    let fd = open(path.as_ptr(), 0);
    if fd < 0 {
        log_err_errno!(b"Failed to open netns path; can not switch netns\0");
        return 1;
    }
    let ret = setns(fd, CLONE_NEWNET);
    close(fd);
    ret
}

unsafe fn tcp_md5sig_fn(sd: c_int, mut addr: *mut c_void, alen: socklen_t, args: *mut sock_args) -> c_int {
    let keylen = strlen((*args).password);
    let mut md5sig: tcp_md5sig = zeroed();
    let mut opt = TCP_MD5SIG;
    let mut rc: c_int;
    md5sig.tcpm_keylen = keylen as u16;
    memcpy(md5sig.tcpm_key.as_mut_ptr() as *mut c_void, (*args).password as *const c_void, keylen);
    if (*args).prefix_len != 0 {
        opt = TCP_MD5SIG_EXT;
        md5sig.tcpm_flags |= TCP_MD5SIG_FLAG_PREFIX;
        md5sig.tcpm_prefixlen = (*args).prefix_len as u8;
        addr = &mut (*args).md5_prefix as *mut _ as *mut c_void;
    }
    memcpy(&mut md5sig.tcpm_addr as *mut _ as *mut c_void, addr, alen as size_t);
    if (((*args).ifindex != 0) && (*args).bind_key_ifindex >= 0) || (*args).bind_key_ifindex >= 1 {
        opt = TCP_MD5SIG_EXT;
        md5sig.tcpm_flags |= TCP_MD5SIG_FLAG_IFINDEX;
        md5sig.tcpm_ifindex = (*args).ifindex;
        log_msg!(b"TCP_MD5SIG_FLAG_IFINDEX set tcpm_ifindex=%d\n\0", md5sig.tcpm_ifindex);
    } else {
        log_msg!(b"TCP_MD5SIG_FLAG_IFINDEX off\n\0", md5sig.tcpm_ifindex);
    }
    rc = setsockopt(sd, IPPROTO_TCP, opt, &md5sig as *const _ as *const c_void, size_of::<tcp_md5sig>() as socklen_t);
    if rc < 0 {
        /* ENOENT is harmless. Returned when a password is cleared */
        if errno == ENOENT {
            rc = 0;
        } else {
            log_err_errno!(b"setsockopt(TCP_MD5SIG)\0");
        }
    }
    rc
}

unsafe fn tcp_md5_remote(sd: c_int, args: *mut sock_args) -> c_int {
    let mut sin: sockaddr_in = zeroed();
    sin.sin_family = AF_INET as u16;
    let mut sin6: sockaddr_in6 = zeroed();
    sin6.sin6_family = AF_INET6 as u16;
    let addr: *mut c_void;
    let alen: c_int;
    match (*args).version {
        AF_INET => {
            sin.sin_port = htons((*args).port);
            sin.sin_addr = (*args).md5_prefix.v4.sin_addr;
            addr = &mut sin as *mut _ as *mut c_void;
            alen = size_of::<sockaddr_in>() as c_int;
        }
        AF_INET6 => {
            sin6.sin6_port = htons((*args).port);
            sin6.sin6_addr = (*args).md5_prefix.v6.sin6_addr;
            addr = &mut sin6 as *mut _ as *mut c_void;
            alen = size_of::<sockaddr_in6>() as c_int;
        }
        _ => {
            log_error!(b"unknown address family\n\0");
            std::process::exit(1);
        }
    }
    if tcp_md5sig_fn(sd, addr, alen as socklen_t, args) != 0 { -1 } else { 0 }
}

unsafe fn get_ifidx_local(ifname: *const c_char) -> c_int {
    let mut ifdata: ifreq = zeroed();
    if ifname.is_null() || *ifname == 0 {
        return -1;
    }
    strcpy(ifdata.ifr_name.as_mut_ptr(), ifname);
    let sd = socket(PF_INET, SOCK_DGRAM, IPPROTO_IP);
    if sd < 0 {
        log_err_errno!(b"socket failed\0");
        return -1;
    }
    let rc = ioctl(sd, SIOCGIFINDEX, &mut ifdata as *mut _ as *mut c_char);
    close(sd);
    if rc != 0 {
        log_err_errno!(b"ioctl(SIOCGIFINDEX) failed\0");
        return -1;
    }
    ifdata.ifr_ifindex
}

unsafe fn bind_to_device(sd: c_int, name: *const c_char) -> c_int {
    let rc = setsockopt(sd, SOL_SOCKET, SO_BINDTODEVICE, name as *const c_void, (strlen(name) + 1) as socklen_t);
    if rc < 0 {
        log_err_errno!(b"setsockopt(SO_BINDTODEVICE)\0");
    }
    rc
}

unsafe fn get_bind_to_device(sd: c_int, name: *mut c_char, len: size_t) -> c_int {
    let mut optlen = len as socklen_t;
    *name = 0;
    let rc = getsockopt(sd, SOL_SOCKET, SO_BINDTODEVICE, name as *mut c_void, &mut optlen);
    if rc < 0 {
        log_err_errno!(b"getsockopt(SO_BINDTODEVICE)\0");
    }
    rc
}

unsafe fn check_device(sd: c_int, args: *mut sock_args) -> c_int {
    let mut ifindex = 0;
    let mut name = [0 as c_char; 32];
    if get_bind_to_device(sd, name.as_mut_ptr(), name.len()) != 0 {
        name[0] = 0;
    } else {
        ifindex = get_ifidx_local(name.as_ptr());
    }
    log_msg!(b"    bound to device %s/%d\n\0", if name[0] != 0 { name.as_ptr() } else { b"<none>\0".as_ptr() as *const c_char }, ifindex);
    if (*args).expected_ifindex == 0 {
        return 0;
    }
    if (*args).expected_ifindex != ifindex {
        log_error!(b"Device index mismatch: expected %d have %d\n\0", (*args).expected_ifindex, ifindex);
        return 1;
    }
    log_msg!(b"Device index matches: expected %d have %d\n\0", (*args).expected_ifindex, ifindex);
    0
}

unsafe fn set_pktinfo_v4(sd: c_int) -> c_int { let one: c_int = 1; let rc = setsockopt(sd, SOL_IP, IP_PKTINFO, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t); if rc < 0 && rc != -ENOTSUP { log_err_errno!(b"setsockopt(IP_PKTINFO)\0"); } rc }
unsafe fn set_recvpktinfo_v6(sd: c_int) -> c_int { let one: c_int = 1; let rc = setsockopt(sd, SOL_IPV6, IPV6_RECVPKTINFO, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t); if rc < 0 && rc != -ENOTSUP { log_err_errno!(b"setsockopt(IPV6_RECVPKTINFO)\0"); } rc }
unsafe fn set_recverr_v4(sd: c_int) -> c_int { let one: c_int = 1; let rc = setsockopt(sd, SOL_IP, IP_RECVERR, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t); if rc < 0 && rc != -ENOTSUP { log_err_errno!(b"setsockopt(IP_RECVERR)\0"); } rc }
unsafe fn set_recverr_v6(sd: c_int) -> c_int { let one: c_int = 1; let rc = setsockopt(sd, SOL_IPV6, IPV6_RECVERR, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t); if rc < 0 && rc != -ENOTSUP { log_err_errno!(b"setsockopt(IPV6_RECVERR)\0"); } rc }

unsafe fn set_unicast_if(sd: c_int, mut ifindex: c_int, version: c_int) -> c_int {
    let mut opt = IP_UNICAST_IF;
    let mut level = SOL_IP;
    ifindex = htonl(ifindex as u32) as c_int;
    if version == AF_INET6 {
        opt = IPV6_UNICAST_IF;
        level = SOL_IPV6;
    }
    let rc = setsockopt(sd, level, opt, &ifindex as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if rc < 0 { log_err_errno!(b"setsockopt(IP_UNICAST_IF)\0"); }
    rc
}

unsafe fn set_multicast_if(sd: c_int, ifindex: c_int) -> c_int {
    let mreq = ip_mreqn { imr_multiaddr: in_addr { s_addr: 0 }, imr_address: in_addr { s_addr: 0 }, imr_ifindex: ifindex };
    let rc = setsockopt(sd, SOL_IP, IP_MULTICAST_IF, &mreq as *const _ as *const c_void, size_of::<ip_mreqn>() as socklen_t);
    if rc < 0 { log_err_errno!(b"setsockopt(IP_MULTICAST_IF)\0"); }
    rc
}

unsafe fn set_membership(sd: c_int, grp: u32, addr: u32, ifindex: c_int) -> c_int {
    if addr == htonl(INADDR_ANY) && ifindex == 0 {
        log_error!(b"Either local address or device needs to be given for multicast membership\n\0");
        return -1;
    }
    let mreq = ip_mreqn { imr_multiaddr: in_addr { s_addr: grp }, imr_address: in_addr { s_addr: addr }, imr_ifindex: ifindex };
    let rc = setsockopt(sd, IPPROTO_IP, IP_ADD_MEMBERSHIP, &mreq as *const _ as *const c_void, size_of::<ip_mreqn>() as socklen_t);
    if rc < 0 {
        log_err_errno!(b"setsockopt(IP_ADD_MEMBERSHIP)\0");
        return -1;
    }
    0
}

unsafe fn set_freebind(sd: c_int, version: c_int) -> c_int {
    let one: c_uint = 1;
    let mut rc = 0;
    match version {
        AF_INET => if setsockopt(sd, SOL_IP, IP_FREEBIND, &one as *const _ as *const c_void, size_of::<c_uint>() as socklen_t) != 0 { log_err_errno!(b"setsockopt(IP_FREEBIND)\0"); rc = -1; },
        AF_INET6 => if setsockopt(sd, SOL_IPV6, IPV6_FREEBIND, &one as *const _ as *const c_void, size_of::<c_uint>() as socklen_t) != 0 { log_err_errno!(b"setsockopt(IPV6_FREEBIND)\0"); rc = -1; },
        _ => {}
    }
    rc
}

unsafe fn set_broadcast(sd: c_int) -> c_int { let one: c_uint = 1; if setsockopt(sd, SOL_SOCKET, SO_BROADCAST, &one as *const _ as *const c_void, size_of::<c_uint>() as socklen_t) != 0 { log_err_errno!(b"setsockopt(SO_BROADCAST)\0"); -1 } else { 0 } }
unsafe fn set_reuseport(sd: c_int) -> c_int { let one: c_uint = 1; if setsockopt(sd, SOL_SOCKET, SO_REUSEPORT, &one as *const _ as *const c_void, size_of::<c_uint>() as socklen_t) != 0 { log_err_errno!(b"setsockopt(SO_REUSEPORT)\0"); -1 } else { 0 } }
unsafe fn set_reuseaddr(sd: c_int) -> c_int { let one: c_uint = 1; if setsockopt(sd, SOL_SOCKET, SO_REUSEADDR, &one as *const _ as *const c_void, size_of::<c_uint>() as socklen_t) != 0 { log_err_errno!(b"setsockopt(SO_REUSEADDR)\0"); -1 } else { 0 } }

unsafe fn set_dsfield(sd: c_int, version: c_int, dsfield: c_int) -> c_int {
    if dsfield == 0 { return 0; }
    match version {
        AF_INET => if setsockopt(sd, SOL_IP, IP_TOS, &dsfield as *const _ as *const c_void, size_of::<c_int>() as socklen_t) < 0 { log_err_errno!(b"setsockopt(IP_TOS)\0"); return -1; },
        AF_INET6 => if setsockopt(sd, SOL_IPV6, IPV6_TCLASS, &dsfield as *const _ as *const c_void, size_of::<c_int>() as socklen_t) < 0 { log_err_errno!(b"setsockopt(IPV6_TCLASS)\0"); return -1; },
        _ => { log_error!(b"Invalid address family\n\0"); return -1; }
    }
    0
}

unsafe fn set_dontroute(sd: c_int) -> c_int {
    let one: c_uint = 1;
    if setsockopt(sd, SOL_SOCKET, SO_DONTROUTE, &one as *const _ as *const c_void, size_of::<c_uint>() as socklen_t) < 0 {
        log_err_errno!(b"setsockopt(SO_DONTROUTE)\0");
        return -1;
    }
    0
}

unsafe fn str_to_uint(str_: *const c_char, min: c_int, max: c_int, value: *mut c_uint) -> c_int {
    let mut end: *mut c_char = null_mut();
    errno = 0;
    let number = strtoul(str_, &mut end, 0) as c_uint as c_int;
    /* entire string should be consumed by conversion and value should be between min and max */
    if ((*end == 0) || (*end == b'\n' as c_char)) && end != str_ as *mut c_char && errno != ERANGE && min <= number && number <= max {
        *value = number as c_uint;
        return 0;
    }
    -1
}

unsafe fn resolve_devices(args: *mut sock_args) -> c_int {
    if !(*args).dev.is_null() {
        (*args).ifindex = get_ifidx_local((*args).dev);
        if (*args).ifindex < 0 {
            log_error!(b"Invalid device name\n\0");
            return 1;
        }
    }
    if !(*args).expected_dev.is_null() {
        let mut tmp: c_uint = 0;
        if str_to_uint((*args).expected_dev, 0, INT_MAX, &mut tmp) == 0 {
            (*args).expected_ifindex = tmp as c_int;
        } else {
            (*args).expected_ifindex = get_ifidx_local((*args).expected_dev);
            if (*args).expected_ifindex < 0 {
                fprintf(stderr, b"Invalid expected device\n\0".as_ptr() as *const c_char);
                return 1;
            }
        }
    }
    0
}

unsafe fn expected_addr_match(sa: *mut sockaddr, expected: *mut c_void, desc: *const c_char) -> c_int {
    let mut addrstr = [0 as c_char; 64];
    let mut rc = 0;
    if (*sa).sa_family as c_int == AF_INET {
        let s = sa as *mut sockaddr_in;
        let exp_in = expected as *mut in_addr;
        if (*s).sin_addr.s_addr != (*exp_in).s_addr {
            log_error!(b"%s address does not match expected %s\n\0", desc, inet_ntop(AF_INET, exp_in as *const c_void, addrstr.as_mut_ptr(), addrstr.len() as socklen_t));
            rc = 1;
        }
    } else if (*sa).sa_family as c_int == AF_INET6 {
        let s6 = sa as *mut sockaddr_in6;
        let exp_in = expected as *mut in6_addr;
        if memcmp(&(*s6).sin6_addr as *const _ as *const c_void, exp_in as *const c_void, size_of::<in6_addr>()) != 0 {
            log_error!(b"%s address does not match expected %s\n\0", desc, inet_ntop(AF_INET6, exp_in as *const c_void, addrstr.as_mut_ptr(), addrstr.len() as socklen_t));
            rc = 1;
        }
    } else {
        log_error!(b"%s address does not match expected - unknown family\n\0", desc);
        rc = 1;
    }
    if rc == 0 { log_msg!(b"%s address matches expected\n\0", desc); }
    rc
}

unsafe fn show_sockstat(sd: c_int, args: *mut sock_args) -> c_int {
    let mut local_addr: sockaddr_in6 = zeroed();
    let mut remote_addr: sockaddr_in6 = zeroed();
    let mut alen = size_of::<sockaddr_in6>() as socklen_t;
    let mut rc = 0;
    let mut sa = &mut local_addr as *mut _ as *mut sockaddr;
    let mut desc = if server_mode != 0 { b"server local:\0".as_ptr() } else { b"client local:\0".as_ptr() } as *const c_char;
    if getsockname(sd, sa, &mut alen) == 0 {
        log_address(desc, sa);
        if (*args).has_expected_laddr != 0 {
            rc = expected_addr_match(sa, &mut (*args).expected_laddr as *mut _ as *mut c_void, b"local\0".as_ptr() as *const c_char);
        }
    } else { log_err_errno!(b"getsockname failed\0"); }
    sa = &mut remote_addr as *mut _ as *mut sockaddr;
    desc = if server_mode != 0 { b"server peer:\0".as_ptr() } else { b"client peer:\0".as_ptr() } as *const c_char;
    if getpeername(sd, sa, &mut alen) == 0 {
        log_address(desc, sa);
        if (*args).has_expected_raddr != 0 {
            rc |= expected_addr_match(sa, &mut (*args).expected_raddr as *mut _ as *mut c_void, b"remote\0".as_ptr() as *const c_char);
        }
    } else { log_err_errno!(b"getpeername failed\0"); }
    rc
}

unsafe fn convert_addr(args: *mut sock_args, _str: *const c_char, atype: addr_type) -> c_int {
    let pfx_len_max = if (*args).version == AF_INET6 { 128 } else { 32 };
    let family = (*args).version;
    let mut dev: *mut c_char = null_mut();
    let mut sep: *mut c_char;
    let desc: *const c_char;
    let addr: *mut c_void;
    let str_ = strdup(_str);
    if str_.is_null() { return -ENOMEM; }
    match atype {
        addr_type::ADDR_TYPE_LOCAL => { desc = b"local\0".as_ptr() as *const c_char; addr = &mut (*args).local_addr as *mut _ as *mut c_void; }
        addr_type::ADDR_TYPE_REMOTE => { desc = b"remote\0".as_ptr() as *const c_char; addr = &mut (*args).remote_addr as *mut _ as *mut c_void; }
        addr_type::ADDR_TYPE_MCAST => { desc = b"mcast grp\0".as_ptr() as *const c_char; addr = &mut (*args).grp as *mut _ as *mut c_void; }
        addr_type::ADDR_TYPE_EXPECTED_LOCAL => { desc = b"expected local\0".as_ptr() as *const c_char; addr = &mut (*args).expected_laddr as *mut _ as *mut c_void; }
        addr_type::ADDR_TYPE_EXPECTED_REMOTE => { desc = b"expected remote\0".as_ptr() as *const c_char; addr = &mut (*args).expected_raddr as *mut _ as *mut c_void; }
        addr_type::ADDR_TYPE_MD5_PREFIX => {
            desc = b"md5 prefix\0".as_ptr() as *const c_char;
            if family == AF_INET {
                (*args).md5_prefix.v4.sin_family = AF_INET as u16;
                addr = &mut (*args).md5_prefix.v4.sin_addr as *mut _ as *mut c_void;
            } else if family == AF_INET6 {
                (*args).md5_prefix.v6.sin6_family = AF_INET6 as u16;
                addr = &mut (*args).md5_prefix.v6.sin6_addr as *mut _ as *mut c_void;
            } else { return 1; }
            sep = strchr(str_, b'/' as c_int);
            if !sep.is_null() {
                *sep = 0;
                sep = sep.add(1);
                if str_to_uint(sep, 1, pfx_len_max, &mut (*args).prefix_len) != 0 {
                    fprintf(stderr, b"Invalid prefix length\n\0".as_ptr() as *const c_char);
                    return 1;
                }
            } else { (*args).prefix_len = 0; }
        }
    }
    let mut rc = 0;
    match family {
        AF_INET => {
            let in_ = addr as *mut in_addr;
            if inet_pton(AF_INET, str_, in_ as *mut c_void) == 0 {
                log_error!(b"Invalid %s IP address\n\0", desc);
                rc = -1;
            }
        }
        AF_INET6 => {
            dev = strchr(str_, b'%' as c_int);
            if !dev.is_null() { *dev = 0; dev = dev.add(1); }
            let in6 = addr as *mut in6_addr;
            if inet_pton(AF_INET6, str_, in6 as *mut c_void) == 0 {
                log_error!(b"Invalid %s IPv6 address\n\0", desc);
                rc = -1;
            } else if !dev.is_null() {
                (*args).scope_id = get_ifidx_local(dev);
                if (*args).scope_id < 0 {
                    log_error!(b"Invalid scope on %s IPv6 address\n\0", desc);
                    rc = -1;
                }
            }
        }
        _ => log_error!(b"Invalid address family\n\0"),
    }
    free(str_ as *mut c_void);
    rc
}

unsafe fn validate_addresses(args: *mut sock_args) -> c_int {
    if !(*args).local_addr_str.is_null() && convert_addr(args, (*args).local_addr_str, addr_type::ADDR_TYPE_LOCAL) < 0 { return 1; }
    if !(*args).remote_addr_str.is_null() && convert_addr(args, (*args).remote_addr_str, addr_type::ADDR_TYPE_REMOTE) < 0 { return 1; }
    if !(*args).md5_prefix_str.is_null() && convert_addr(args, (*args).md5_prefix_str, addr_type::ADDR_TYPE_MD5_PREFIX) < 0 { return 1; }
    if !(*args).expected_laddr_str.is_null() && convert_addr(args, (*args).expected_laddr_str, addr_type::ADDR_TYPE_EXPECTED_LOCAL) != 0 { return 1; }
    if !(*args).expected_raddr_str.is_null() && convert_addr(args, (*args).expected_raddr_str, addr_type::ADDR_TYPE_EXPECTED_REMOTE) != 0 { return 1; }
    0
}

unsafe fn get_index_from_cmsg(m: *mut msghdr) -> c_int {
    let mut cm = CMSG_FIRSTHDR(m);
    let mut ifindex = 0;
    let mut buf = [0 as c_char; 64];
    while (*m).msg_controllen != 0 && !cm.is_null() {
        if (*cm).cmsg_level == SOL_IP && (*cm).cmsg_type == IP_PKTINFO {
            let pi = CMSG_DATA(cm) as *mut in_pktinfo;
            inet_ntop(AF_INET, &(*pi).ipi_addr as *const _ as *const c_void, buf.as_mut_ptr(), buf.len() as socklen_t);
            ifindex = (*pi).ipi_ifindex;
        } else if (*cm).cmsg_level == SOL_IPV6 && (*cm).cmsg_type == IPV6_PKTINFO {
            let pi6 = CMSG_DATA(cm) as *mut in6_pktinfo;
            inet_ntop(AF_INET6, &(*pi6).ipi6_addr as *const _ as *const c_void, buf.as_mut_ptr(), buf.len() as socklen_t);
            ifindex = (*pi6).ipi6_ifindex as c_int;
        }
        cm = CMSG_NXTHDR(m, cm);
    }
    if ifindex != 0 { log_msg!(b"    pktinfo: ifindex %d dest addr %s\n\0", ifindex, buf.as_ptr()); }
    ifindex
}

unsafe fn send_msg_no_cmsg(sd: c_int, addr: *mut c_void, alen: socklen_t) -> c_int {
    loop {
        let err = sendto(sd, msg as *const c_void, msglen as size_t, 0, addr, alen);
        if err < 0 {
            if errno == EACCES && try_broadcast != 0 {
                try_broadcast = 0;
                if set_broadcast(sd) == 0 { continue; }
                errno = EACCES;
            }
            log_err_errno!(b"sendto failed\0");
            return 1;
        }
        return 0;
    }
}

unsafe fn send_msg_cmsg(sd: c_int, addr: *mut c_void, alen: socklen_t, ifindex: c_int, version: c_int) -> c_int {
    let mut cmsgbuf = [0u8; 64];
    let mut iov: [iovec; 2] = zeroed();
    let mut m: msghdr = zeroed();
    iov[0].iov_base = msg as *mut c_void;
    iov[0].iov_len = msglen as size_t;
    m.msg_iov = iov.as_mut_ptr();
    m.msg_iovlen = 1;
    m.msg_name = addr;
    m.msg_namelen = alen;
    let cm = cmsgbuf.as_mut_ptr() as *mut cmsghdr;
    m.msg_control = cm as *mut c_void;
    if version == AF_INET {
        (*cm).cmsg_level = SOL_IP;
        (*cm).cmsg_type = IP_PKTINFO;
        (*cm).cmsg_len = CMSG_LEN(size_of::<in_pktinfo>());
        (*(CMSG_DATA(cm) as *mut in_pktinfo)).ipi_ifindex = ifindex;
        m.msg_controllen = (*cm).cmsg_len;
    } else if version == AF_INET6 {
        (*cm).cmsg_level = SOL_IPV6;
        (*cm).cmsg_type = IPV6_PKTINFO;
        (*cm).cmsg_len = CMSG_LEN(size_of::<in6_pktinfo>());
        (*(CMSG_DATA(cm) as *mut in6_pktinfo)).ipi6_ifindex = ifindex as c_uint;
        m.msg_controllen = (*cm).cmsg_len;
    }
    loop {
        let err = sendmsg(sd, &m, 0);
        if err < 0 {
            if errno == EACCES && try_broadcast != 0 {
                try_broadcast = 0;
                if set_broadcast(sd) == 0 { continue; }
                errno = EACCES;
            }
            log_err_errno!(b"sendmsg failed\0");
            return 1;
        }
        return 0;
    }
}

unsafe fn send_msg(sd: c_int, addr: *mut c_void, alen: socklen_t, args: *mut sock_args) -> c_int {
    if (*args).type_ == SOCK_STREAM {
        if write(sd, msg as *const c_void, msglen as size_t) < 0 { log_err_errno!(b"write failed sending msg to peer\0"); return 1; }
    } else if (*args).datagram_connect != 0 {
        if send(sd, msg as *const c_void, msglen as size_t, 0) < 0 { log_err_errno!(b"send failed sending msg to peer\0"); return 1; }
    } else if (*args).ifindex != 0 && (*args).use_cmsg != 0 {
        if send_msg_cmsg(sd, addr, alen, (*args).ifindex, (*args).version) != 0 { return 1; }
    } else if send_msg_no_cmsg(sd, addr, alen) != 0 { return 1; }
    log_msg!(b"Sent message:\n\0");
    log_msg!(b"    %.24s%s\n\0", msg, if msglen > 24 { b" ...\0".as_ptr() } else { b"\0".as_ptr() });
    0
}

unsafe fn socket_read_dgram(sd: c_int, args: *mut sock_args) -> c_int {
    let mut addr = [0u8; size_of::<sockaddr_in6>()];
    let sa = addr.as_mut_ptr() as *mut sockaddr;
    let mut alen = addr.len() as socklen_t;
    let mut iov: [iovec; 2] = zeroed();
    let mut m: msghdr = zeroed();
    let mut cmsgbuf = [0u8; 256];
    let mut buf = [0 as c_char; 16 * 1024 + 1];
    iov[0].iov_base = buf.as_mut_ptr() as *mut c_void;
    iov[0].iov_len = 16 * 1024;
    m.msg_name = addr.as_mut_ptr() as *mut c_void;
    m.msg_namelen = alen;
    m.msg_iov = iov.as_mut_ptr();
    m.msg_iovlen = 1;
    m.msg_control = cmsgbuf.as_mut_ptr() as *mut c_void;
    m.msg_controllen = cmsgbuf.len();
    let len = recvmsg(sd, &mut m, 0) as c_int;
    if len == 0 { log_msg!(b"peer closed connection.\n\0"); return 0; }
    if len < 0 { log_msg!(b"failed to read message: %d: %s\n\0", errno, strerror(errno)); return -1; }
    buf[len as usize] = 0;
    log_address(b"Message from:\0".as_ptr() as *const c_char, sa);
    log_msg!(b"    %.24s%s\n\0", buf.as_ptr(), if len > 24 { b" ...\0".as_ptr() } else { b"\0".as_ptr() });
    let ifindex = get_index_from_cmsg(&mut m);
    if (*args).expected_ifindex != 0 {
        if (*args).expected_ifindex != ifindex {
            log_error!(b"Device index mismatch: expected %d have %d\n\0", (*args).expected_ifindex, ifindex);
            return -1;
        }
        log_msg!(b"Device index matches: expected %d have %d\n\0", (*args).expected_ifindex, ifindex);
    }
    if interactive == 0 && server_mode != 0 {
        if (*args).version == AF_INET6 {
            let s6 = sa as *mut sockaddr_in6;
            if (*args).dev.is_null() {
                (*s6).sin6_scope_id = ifindex as u32;
                if sendmsg(sd, &m, 0) < 0 { log_err_errno!(b"failed to send msg to peer\0"); return -1; }
            } else if sendto(sd, buf.as_ptr() as *const c_void, len as size_t, 0, addr.as_mut_ptr() as *mut c_void, alen) < 0 {
                log_err_errno!(b"failed to send msg to peer\0");
                return -1;
            }
        } else {
            loop {
                let err = sendmsg(sd, &m, 0);
                if err < 0 {
                    if errno == EACCES && try_broadcast != 0 {
                        try_broadcast = 0;
                        if set_broadcast(sd) == 0 { continue; }
                        errno = EACCES;
                    }
                    log_err_errno!(b"failed to send msg to peer\0");
                    return -1;
                }
                break;
            }
        }
        log_msg!(b"Sent message:\n\0");
        log_msg!(b"    %.24s%s\n\0", buf.as_ptr(), if len > 24 { b" ...\0".as_ptr() } else { b"\0".as_ptr() });
    }
    1
}

unsafe fn socket_read_stream(sd: c_int) -> c_int {
    let mut buf = [0 as c_char; 1024];
    let len = read(sd, buf.as_mut_ptr() as *mut c_void, buf.len() - 1) as c_int;
    if len == 0 { log_msg!(b"client closed connection.\n\0"); return 0; }
    if len < 0 { log_msg!(b"failed to read message\n\0"); return -1; }
    buf[len as usize] = 0;
    log_msg!(b"Incoming message:\n\0");
    log_msg!(b"    %.24s%s\n\0", buf.as_ptr(), if len > 24 { b" ...\0".as_ptr() } else { b"\0".as_ptr() });
    if interactive == 0 && server_mode != 0 {
        if write(sd, buf.as_ptr() as *const c_void, len as size_t) < 0 { log_err_errno!(b"failed to send buf\0"); return -1; }
        log_msg!(b"Sent message:\n\0");
        log_msg!(b"     %.24s%s\n\0", buf.as_ptr(), if len > 24 { b" ...\0".as_ptr() } else { b"\0".as_ptr() });
    }
    1
}

unsafe fn socket_read(sd: c_int, args: *mut sock_args) -> c_int {
    if (*args).type_ == SOCK_STREAM { socket_read_stream(sd) } else { socket_read_dgram(sd, args) }
}

unsafe fn stdin_to_socket(sd: c_int, type_: c_int, addr: *mut c_void, alen: socklen_t) -> c_int {
    let mut buf = [0 as c_char; 1024];
    if fgets(buf.as_mut_ptr(), buf.len() as c_int, stdin).is_null() { return 0; }
    let len = strlen(buf.as_ptr());
    if type_ == SOCK_STREAM {
        if write(sd, buf.as_ptr() as *const c_void, len) < 0 { log_err_errno!(b"failed to send buf\0"); return -1; }
    } else {
        loop {
            let err = sendto(sd, buf.as_ptr() as *const c_void, len, 0, addr, alen);
            if err < 0 {
                if errno == EACCES && try_broadcast != 0 {
                    try_broadcast = 0;
                    if set_broadcast(sd) == 0 { continue; }
                    errno = EACCES;
                }
                log_err_errno!(b"failed to send msg to peer\0");
                return -1;
            }
            break;
        }
    }
    log_msg!(b"Sent message:\n\0");
    log_msg!(b"    %.24s%s\n\0", buf.as_ptr(), if len > 24 { b" ...\0".as_ptr() } else { b"\0".as_ptr() });
    1
}

unsafe fn set_recv_attr(sd: c_int, version: c_int) {
    if version == AF_INET6 { set_recvpktinfo_v6(sd); set_recverr_v6(sd); } else { set_pktinfo_v4(sd); set_recverr_v4(sd); }
}

unsafe fn msg_loop(client: c_int, sd: c_int, addr: *mut c_void, alen: socklen_t, args: *mut sock_args) -> c_int {
    let mut timeout = timeval { tv_sec: prog_timeout as time_t, tv_usec: 0 };
    let mut ptval: *mut timeval = null_mut();
    let mut rfds: fd_set = zeroed();
    if (*args).type_ != SOCK_STREAM { set_recv_attr(sd, (*args).version); }
    if !msg.is_null() {
        msglen = strlen(msg) as c_int;
        if client != 0 && send_msg(sd, addr, alen, args) != 0 { return 1; }
        if interactive == 0 {
            ptval = &mut timeout;
            if prog_timeout == 0 { timeout.tv_sec = 5; }
        }
    }
    let nfds = if interactive != 0 { core::cmp::max(fileno(stdin), sd) + 1 } else { sd + 1 };
    loop {
        FD_ZERO(&mut rfds);
        FD_SET(sd, &mut rfds);
        if interactive != 0 { FD_SET(fileno(stdin), &mut rfds); }
        let mut rc = select(nfds, &mut rfds, null_mut(), null_mut(), ptval);
        if rc < 0 {
            if errno == EINTR { continue; }
            log_err_errno!(b"select failed\0");
            return 1;
        } else if rc == 0 {
            log_error!(b"Timed out waiting for response\n\0");
            return 2;
        }
        if FD_ISSET(sd, &mut rfds) {
            rc = socket_read(sd, args);
            if rc < 0 { return 1; }
            if rc == 0 { break; }
        }
        rc = 0;
        if FD_ISSET(fileno(stdin), &mut rfds) && stdin_to_socket(sd, (*args).type_, addr, alen) <= 0 { break; }
        if interactive != 0 { continue; }
        if iter != -1 {
            iter -= 1;
            if iter == 0 { break; }
        }
        log_msg!(b"Going into quiet mode\n\0");
        quiet = 1;
        if client != 0 && send_msg(sd, addr, alen, args) != 0 { rc = 1; break; }
    }
    0
}

unsafe fn msock_init(args: *mut sock_args, server: c_int) -> c_int {
    let mut if_addr = htonl(INADDR_ANY);
    let mut laddr: sockaddr_in = zeroed();
    laddr.sin_family = AF_INET as u16;
    laddr.sin_port = htons((*args).port);
    let one: c_int = 1;
    if server == 0 && (*args).has_local_ip != 0 { if_addr = (*args).local_addr.in_.s_addr; }
    let sd = socket(PF_INET, SOCK_DGRAM, 0);
    if sd < 0 { log_err_errno!(b"socket\0"); return -1; }
    if setsockopt(sd, SOL_SOCKET, SO_REUSEADDR, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t) < 0 { log_err_errno!(b"Setting SO_REUSEADDR error\0"); close(sd); return -1; }
    if setsockopt(sd, SOL_SOCKET, SO_BROADCAST, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t) < 0 { log_err_errno!(b"Setting SO_BROADCAST error\0"); }
    if set_dsfield(sd, AF_INET, (*args).dsfield as c_int) != 0 { close(sd); return -1; }
    if server != 0 {
        if (*args).server_dontroute != 0 && set_dontroute(sd) != 0 { close(sd); return -1; }
    } else if (*args).client_dontroute != 0 && set_dontroute(sd) != 0 { close(sd); return -1; }
    if !(*args).dev.is_null() && bind_to_device(sd, (*args).dev) != 0 { close(sd); return -1; }
    else if (*args).use_setsockopt != 0 && set_multicast_if(sd, (*args).ifindex) != 0 { close(sd); return -1; }
    laddr.sin_addr.s_addr = if_addr;
    if bind(sd, &laddr as *const _ as *const c_void, size_of::<sockaddr_in>() as socklen_t) < 0 { log_err_errno!(b"bind failed\0"); close(sd); return -1; }
    if server != 0 && set_membership(sd, (*args).grp.s_addr, (*args).local_addr.in_.s_addr, (*args).ifindex) != 0 { close(sd); return -1; }
    sd
}

unsafe fn msock_server(args: *mut sock_args) -> c_int { msock_init(args, 1) }
unsafe fn msock_client(args: *mut sock_args) -> c_int { msock_init(args, 0) }

unsafe fn bind_socket(sd: c_int, args: *mut sock_args) -> c_int {
    let mut serv_addr: sockaddr_in = zeroed();
    serv_addr.sin_family = AF_INET as u16;
    let mut serv6_addr: sockaddr_in6 = zeroed();
    serv6_addr.sin6_family = AF_INET6 as u16;
    if (*args).has_local_ip == 0 && (*args).type_ == SOCK_RAW { return 0; }
    let (addr, alen): (*mut c_void, socklen_t) = match (*args).version {
        AF_INET => { serv_addr.sin_port = htons((*args).port); serv_addr.sin_addr = (*args).local_addr.in_; (&mut serv_addr as *mut _ as *mut c_void, size_of::<sockaddr_in>() as socklen_t) }
        AF_INET6 => { serv6_addr.sin6_port = htons((*args).port); serv6_addr.sin6_addr = (*args).local_addr.in6; (&mut serv6_addr as *mut _ as *mut c_void, size_of::<sockaddr_in6>() as socklen_t) }
        _ => { log_error!(b"Invalid address family\n\0"); return -1; }
    };
    if bind(sd, addr, alen) < 0 { log_err_errno!(b"error binding socket\0"); return -1; }
    0
}

unsafe fn config_xfrm_policy(sd: c_int, args: *mut sock_args) -> c_int {
    let mut policy: xfrm_userpolicy_info = zeroed();
    let type_ = UDP_ENCAP_ESPINUDP;
    let mut xfrm_af = IP_XFRM_POLICY;
    let mut level = SOL_IP;
    if (*args).type_ != SOCK_DGRAM { log_error!(b"Invalid socket type. Only DGRAM could be used for XFRM\n\0"); return 1; }
    policy.action = XFRM_POLICY_ALLOW;
    policy.sel.family = (*args).version as u16;
    if (*args).version == AF_INET6 { xfrm_af = IPV6_XFRM_POLICY; level = SOL_IPV6; }
    policy.dir = XFRM_POLICY_OUT;
    if setsockopt(sd, level, xfrm_af, &policy as *const _ as *const c_void, size_of::<xfrm_userpolicy_info>() as socklen_t) < 0 { return 1; }
    policy.dir = XFRM_POLICY_IN;
    if setsockopt(sd, level, xfrm_af, &policy as *const _ as *const c_void, size_of::<xfrm_userpolicy_info>() as socklen_t) < 0 { return 1; }
    if setsockopt(sd, IPPROTO_UDP, UDP_ENCAP, &type_ as *const _ as *const c_void, size_of::<c_int>() as socklen_t) < 0 { log_err_errno!(b"Failed to set xfrm encap\0"); return 1; }
    0
}

unsafe fn lsock_init(args: *mut sock_args) -> c_int {
    let sd = socket((*args).version, (*args).type_, (*args).protocol);
    if sd < 0 { log_err_errno!(b"Error opening socket\0"); return -1; }
    if set_reuseaddr(sd) != 0 || set_reuseport(sd) != 0 || set_dsfield(sd, (*args).version, (*args).dsfield as c_int) != 0 { close(sd); return -1; }
    if (*args).server_dontroute != 0 && set_dontroute(sd) != 0 { close(sd); return -1; }
    if !(*args).dev.is_null() && bind_to_device(sd, (*args).dev) != 0 { close(sd); return -1; }
    else if (*args).use_setsockopt != 0 && set_unicast_if(sd, (*args).ifindex, (*args).version) != 0 { close(sd); return -1; }
    if (*args).use_freebind != 0 && set_freebind(sd, (*args).version) != 0 { close(sd); return -1; }
    if bind_socket(sd, args) != 0 { close(sd); return -1; }
    if (*args).bind_test_only != 0 { return sd; }
    if (*args).type_ == SOCK_STREAM && listen(sd, 1) < 0 { log_err_errno!(b"listen failed\0"); close(sd); return -1; }
    let flags = fcntl(sd, F_GETFL);
    if flags < 0 || fcntl(sd, F_SETFL, flags | O_NONBLOCK) < 0 { log_err_errno!(b"Failed to set non-blocking option\0"); close(sd); return -1; }
    if fcntl(sd, F_SETFD, FD_CLOEXEC) < 0 { log_err_errno!(b"Failed to set close-on-exec flag\0"); }
    if (*args).use_xfrm != 0 && config_xfrm_policy(sd, args) != 0 { log_err_errno!(b"Failed to set xfrm policy\0"); close(sd); return -1; }
    sd
}

unsafe fn ipc_write(fd: c_int, message: c_int) {
    /* Not in both_mode, so there's no process to signal */
    if fd < 0 { return; }
    if write(fd, &message as *const _ as *const c_void, size_of::<c_int>()) < 0 { log_err_errno!(b"Failed to send client status\0"); }
}

unsafe fn do_server(args: *mut sock_args, ipc_fd: c_int) -> c_int {
    /* ipc_fd = -1 if no parent process to signal */
    let mut timeout = timeval { tv_sec: prog_timeout as time_t, tv_usec: 0 };
    let mut ptval: *mut timeval = null_mut();
    let mut addr = [0u8; size_of::<sockaddr_in6>()];
    let mut alen = addr.len() as socklen_t;
    let mut rfds: fd_set = zeroed();
    if !(*args).serverns.is_null() {
        if switch_ns((*args).serverns) != 0 { log_error!(b"Could not set server netns to %s\n\0", (*args).serverns); ipc_write(ipc_fd, 0); return 1; }
        log_msg!(b"Switched server netns\n\0");
    }
    (*args).dev = (*args).server_dev;
    (*args).expected_dev = (*args).expected_server_dev;
    if resolve_devices(args) != 0 || validate_addresses(args) != 0 { ipc_write(ipc_fd, 0); return 1; }
    if prog_timeout != 0 { ptval = &mut timeout; }
    let lsd = if (*args).has_grp != 0 { msock_server(args) } else { lsock_init(args) };
    if lsd < 0 { ipc_write(ipc_fd, 0); return 1; }
    if (*args).bind_test_only != 0 { close(lsd); ipc_write(ipc_fd, 1); return 0; }
    if (*args).type_ != SOCK_STREAM {
        ipc_write(ipc_fd, 1);
        let rc = msg_loop(0, lsd, addr.as_mut_ptr() as *mut c_void, alen, args);
        close(lsd);
        return rc;
    }
    if !(*args).password.is_null() && tcp_md5_remote(lsd, args) != 0 { close(lsd); ipc_write(ipc_fd, 0); return 1; }
    ipc_write(ipc_fd, 1);
    let mut rc = 0;
    loop {
        log_msg!(b"waiting for client connection.\n\0");
        FD_ZERO(&mut rfds);
        FD_SET(lsd, &mut rfds);
        rc = select(lsd + 1, &mut rfds, null_mut(), null_mut(), ptval);
        if rc == 0 { rc = 2; break; }
        if rc < 0 {
            if errno == EINTR { continue; }
            log_err_errno!(b"select failed\0");
            break;
        }
        let mut csd = -1;
        if FD_ISSET(lsd, &mut rfds) {
            csd = accept(lsd, addr.as_mut_ptr() as *mut c_void, &mut alen);
            if csd < 0 { log_err_errno!(b"accept failed\0"); break; }
            rc = show_sockstat(csd, args);
            if rc != 0 { break; }
            rc = check_device(csd, args);
            if rc != 0 { break; }
        }
        rc = msg_loop(0, csd, addr.as_mut_ptr() as *mut c_void, alen, args);
        close(csd);
        if interactive == 0 { break; }
    }
    close(lsd);
    rc
}

unsafe fn wait_for_connect(sd: c_int) -> c_int {
    let mut _tv = timeval { tv_sec: prog_timeout as time_t, tv_usec: 0 };
    let mut tv: *mut timeval = null_mut();
    let mut wfd: fd_set = zeroed();
    let mut val: c_int = 0;
    let mut sz = size_of::<c_int>() as socklen_t;
    FD_ZERO(&mut wfd);
    FD_SET(sd, &mut wfd);
    if prog_timeout != 0 { tv = &mut _tv; }
    let rc = select(FD_SETSIZE, null_mut(), &mut wfd, null_mut(), tv);
    if rc == 0 { log_error!(b"connect timed out\n\0"); return -2; }
    if rc < 0 { log_err_errno!(b"select failed\0"); return -3; }
    if getsockopt(sd, SOL_SOCKET, SO_ERROR, &mut val as *mut _ as *mut c_void, &mut sz) < 0 { log_err_errno!(b"getsockopt(SO_ERROR) failed\0"); return -4; }
    if val != 0 { log_error!(b"connect failed: %d: %s\n\0", val, strerror(val)); return -1; }
    0
}

unsafe fn connectsock(addr: *mut c_void, alen: socklen_t, args: *mut sock_args) -> c_int {
    let sd = socket((*args).version, (*args).type_, (*args).protocol);
    let mut rc = -1;
    if sd < 0 { log_err_errno!(b"Failed to create socket\0"); return -1; }
    let flags = fcntl(sd, F_GETFL);
    if flags < 0 || fcntl(sd, F_SETFL, flags | O_NONBLOCK) < 0 { log_err_errno!(b"Failed to set non-blocking option\0"); close(sd); return -1; }
    if set_reuseport(sd) != 0 || set_dsfield(sd, (*args).version, (*args).dsfield as c_int) != 0 { close(sd); return -1; }
    if (*args).client_dontroute != 0 && set_dontroute(sd) != 0 { close(sd); return -1; }
    if !(*args).dev.is_null() && bind_to_device(sd, (*args).dev) != 0 { close(sd); return -1; }
    else if (*args).use_setsockopt != 0 && set_unicast_if(sd, (*args).ifindex, (*args).version) != 0 { close(sd); return -1; }
    if (*args).has_local_ip != 0 && bind_socket(sd, args) != 0 { close(sd); return -1; }
    if (*args).type_ != SOCK_STREAM && (*args).datagram_connect == 0 { return sd; }
    if !(*args).password.is_null() && tcp_md5sig_fn(sd, addr, alen, args) != 0 { close(sd); return -1; }
    if (*args).bind_test_only != 0 { return sd; }
    if connect(sd, addr, alen) < 0 {
        if errno != EINPROGRESS { log_err_errno!(b"Failed to connect to remote host\0"); rc = -1; close(sd); return rc; }
        rc = wait_for_connect(sd);
        if rc < 0 { close(sd); return rc; }
    }
    sd
}

unsafe fn do_client(args: *mut sock_args) -> c_int {
    let mut sin: sockaddr_in = zeroed();
    sin.sin_family = AF_INET as u16;
    let mut sin6: sockaddr_in6 = zeroed();
    sin6.sin6_family = AF_INET6 as u16;
    let addr: *mut c_void;
    let alen: c_int;
    let mut rc = 0;
    if (*args).has_remote_ip == 0 && (*args).has_grp == 0 {
        fprintf(stderr, b"remote IP or multicast group not given\n\0".as_ptr() as *const c_char);
        return 1;
    }
    if !(*args).clientns.is_null() {
        if switch_ns((*args).clientns) != 0 { log_error!(b"Could not set client netns to %s\n\0", (*args).clientns); return 1; }
        log_msg!(b"Switched client netns\n\0");
    }
    (*args).local_addr_str = (*args).client_local_addr_str;
    if resolve_devices(args) != 0 || validate_addresses(args) != 0 { return 1; }
    if ((*args).use_setsockopt != 0 || (*args).use_cmsg != 0) && (*args).ifindex == 0 {
        fprintf(stderr, b"Device binding not specified\n\0".as_ptr() as *const c_char);
        return 1;
    }
    if (*args).use_setsockopt != 0 || (*args).use_cmsg != 0 { (*args).dev = null(); }
    match (*args).version {
        AF_INET => {
            sin.sin_port = htons((*args).port);
            sin.sin_addr = if (*args).has_grp != 0 { (*args).grp } else { (*args).remote_addr.in_ };
            addr = &mut sin as *mut _ as *mut c_void;
            alen = size_of::<sockaddr_in>() as c_int;
        }
        AF_INET6 => {
            sin6.sin6_port = htons((*args).port);
            sin6.sin6_addr = (*args).remote_addr.in6;
            sin6.sin6_scope_id = (*args).scope_id as u32;
            addr = &mut sin6 as *mut _ as *mut c_void;
            alen = size_of::<sockaddr_in6>() as c_int;
        }
        _ => return 1,
    }
    (*args).password = (*args).client_pw;
    let sd = if (*args).has_grp != 0 { msock_client(args) } else { connectsock(addr, alen as socklen_t, args) };
    if sd < 0 { return -sd; }
    if (*args).bind_test_only == 0 {
        if (*args).type_ == SOCK_STREAM {
            rc = show_sockstat(sd, args);
            if rc != 0 { close(sd); return rc; }
        }
        rc = msg_loop(1, sd, addr, alen as socklen_t, args);
    }
    close(sd);
    rc
}

unsafe fn random_msg(mut len: c_int) -> *mut c_char {
    let mut n = 0;
    let olen = len + 1;
    if len <= 0 { return null_mut(); }
    let m = malloc(olen as size_t) as *mut c_char;
    if m.is_null() { return null_mut(); }
    while len > 26 {
        let i = snprintf(m.add(n as usize), (olen - n) as size_t, b"%.26s\0".as_ptr() as *const c_char, b"abcdefghijklmnopqrstuvwxyz\0".as_ptr());
        n += i;
        len -= i;
    }
    snprintf(m.add(n as usize), (olen - n) as size_t, b"%.*s\0".as_ptr() as *const c_char, len, b"abcdefghijklmnopqrstuvwxyz\0".as_ptr());
    m
}

unsafe fn ipc_child(fd: c_int, args: *mut sock_args) -> c_int {
    let outbuf = malloc(4096) as *mut c_char;
    let errbuf = malloc(4096) as *mut c_char;
    let mut rc = 1;
    if outbuf.is_null() || errbuf.is_null() {
        fprintf(stderr, b"server: Failed to allocate buffers for stdout and stderr\n\0".as_ptr() as *const c_char);
    } else {
        setbuffer(stdout, outbuf, 4096);
        setbuffer(stderr, errbuf, 4096);
        server_mode = 1; /* to tell log_msg in case we are in both_mode */
        /* when running in both mode, address validation applies solely to client side */
        (*args).has_expected_laddr = 0;
        (*args).has_expected_raddr = 0;
        rc = do_server(args, fd);
    }
    free(outbuf as *mut c_void);
    free(errbuf as *mut c_void);
    rc
}

unsafe fn ipc_parent(cpid: c_int, fd: c_int, args: *mut sock_args) -> c_int {
    let mut buf: c_int = 0;
    if read(fd, &mut buf as *mut _ as *mut c_void, size_of::<c_int>()) <= 0 {
        log_err_errno!(b"Failed to read IPC status from pipe\0");
        return 1;
    }
    if buf == 0 {
        log_error!(b"Server failed; can not continue\n\0");
        return 1;
    }
    log_msg!(b"Server is ready\n\0");
    let client_status = do_client(args);
    log_msg!(b"parent is done!\n\0");
    if kill(cpid, 0) == 0 { kill(cpid, SIGKILL); }
    let mut status: c_int = 0;
    wait(&mut status);
    client_status
}

unsafe fn print_usage(prog: *mut c_char) {
    printf(
        b"usage: %s OPTS\nRequired:\n    -r addr       remote address to connect to (client mode only)\n    -p port       port to connect to (client mode)/listen on (server mode)\n                  (default: %d)\n    -s            server mode (default: client mode)\n    -t            timeout seconds (default: none)\n\nOptional:\n    -B            do both client and server via fork and IPC\n    -N ns         set client to network namespace ns (requires root)\n    -O ns         set server to network namespace ns (requires root)\n    -F            Restart server loop\n    -6            IPv6 (default is IPv4)\n    -P proto      protocol for socket: icmp, ospf (default: none)\n    -D|R          datagram (D) / raw (R) socket (default stream)\n    -l addr       local address to bind to in server mode\n    -c addr       local address to bind to in client mode\n    -Q dsfield    DS Field value of the socket (the IP_TOS or\n                  IPV6_TCLASS socket option)\n    -x            configure XFRM policy on socket\n\n    -d dev        bind socket to given device name\n    -I dev        bind socket to given device name - server mode\n    -S            use setsockopt (IP_UNICAST_IF or IP_MULTICAST_IF)\n                  to set device binding\n    -U            Use connect() and send() for datagram sockets\n    -f            bind socket with the IP[V6]_FREEBIND option\n    -C            use cmsg and IP_PKTINFO to specify device binding\n\n    -L len        send random message of given length\n    -n num        number of times to send message\n\n    -M password   use MD5 sum protection\n    -X password   MD5 password for client mode\n    -m prefix/len prefix and length to use for MD5 key\n    --no-bind-key-ifindex: Force TCP_MD5SIG_FLAG_IFINDEX off\n    --force-bind-key-ifindex: Force TCP_MD5SIG_FLAG_IFINDEX on\n        (default: only if -I is passed)\n    --client-dontroute: don't use gateways for client socket: send\n                        packets only if destination is on link (see\n                        SO_DONTROUTE in socket(7))\n    --server-dontroute: don't use gateways for server socket: send\n                        packets only if destination is on link (see\n                        SO_DONTROUTE in socket(7))\n\n    -g grp        multicast group (e.g., 239.1.1.1)\n    -i            interactive mode (default is echo and terminate)\n\n    -0 addr       Expected local address\n    -1 addr       Expected remote address\n    -2 dev        Expected device name (or index) to receive packet\n    -3 dev        Expected device name (or index) to receive packets - server mode\n\n    -b            Bind test only.\n    -q            Be quiet. Run test without printing anything.\n\0".as_ptr() as *const c_char,
        prog,
        DEFAULT_PORT as c_int,
    );
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut args: sock_args = zeroed();
    args.version = AF_INET;
    args.type_ = SOCK_STREAM;
    args.port = DEFAULT_PORT;
    let mut both_mode = 0;
    let mut tmp: c_uint = 0;
    let mut forever = 0;
    let mut fd = [0 as c_int; 2];
    let mut rc = 0;
    loop {
        rc = getopt_long(argc, argv, GETOPT_STR.as_ptr() as *const c_char, long_opts.as_ptr(), null_mut());
        if rc == -1 { break; }
        match rc {
            x if x == b'B' as c_int => both_mode = 1,
            x if x == b's' as c_int => server_mode = 1,
            x if x == b'F' as c_int => forever = 1,
            x if x == b'l' as c_int => { args.has_local_ip = 1; args.local_addr_str = optarg; }
            x if x == b'r' as c_int => { args.has_remote_ip = 1; args.remote_addr_str = optarg; }
            x if x == b'c' as c_int => { args.has_local_ip = 1; args.client_local_addr_str = optarg; }
            x if x == b'Q' as c_int => { if str_to_uint(optarg, 0, 255, &mut tmp) != 0 { fprintf(stderr, b"Invalid DS Field\n\0".as_ptr() as *const c_char); return 1; } args.dsfield = tmp as u8; }
            x if x == b'p' as c_int => { if str_to_uint(optarg, 1, 65535, &mut tmp) != 0 { fprintf(stderr, b"Invalid port\n\0".as_ptr() as *const c_char); return 1; } args.port = tmp as u16; }
            x if x == b't' as c_int => { if str_to_uint(optarg, 0, INT_MAX, &mut prog_timeout) != 0 { fprintf(stderr, b"Invalid timeout\n\0".as_ptr() as *const c_char); return 1; } }
            x if x == b'D' as c_int => args.type_ = SOCK_DGRAM,
            x if x == b'R' as c_int => { args.type_ = SOCK_RAW; args.port = 0; if args.protocol == 0 { args.protocol = IPPROTO_RAW; } }
            x if x == b'P' as c_int => {
                let pe = getprotobyname(optarg);
                if !pe.is_null() { args.protocol = (*pe).p_proto; }
                else { if str_to_uint(optarg, 0, 0xffff, &mut tmp) != 0 { fprintf(stderr, b"Invalid protocol\n\0".as_ptr() as *const c_char); return 1; } args.protocol = tmp as c_int; }
            }
            x if x == b'n' as c_int => iter = atoi(optarg),
            x if x == b'N' as c_int => args.clientns = optarg,
            x if x == b'O' as c_int => args.serverns = optarg,
            x if x == b'L' as c_int => msg = random_msg(atoi(optarg)),
            x if x == b'M' as c_int => args.password = optarg,
            OPT_FORCE_BIND_KEY_IFINDEX => args.bind_key_ifindex = 1,
            OPT_NO_BIND_KEY_IFINDEX => args.bind_key_ifindex = -1,
            OPT_CLIENT_DONTROUTE => args.client_dontroute = 1,
            OPT_SERVER_DONTROUTE => args.server_dontroute = 1,
            x if x == b'X' as c_int => args.client_pw = optarg,
            x if x == b'm' as c_int => args.md5_prefix_str = optarg,
            x if x == b'S' as c_int => args.use_setsockopt = 1,
            x if x == b'f' as c_int => args.use_freebind = 1,
            x if x == b'C' as c_int => args.use_cmsg = 1,
            x if x == b'd' as c_int => args.dev = optarg,
            x if x == b'I' as c_int => args.server_dev = optarg,
            x if x == b'i' as c_int => interactive = 1,
            x if x == b'g' as c_int => { args.has_grp = 1; if convert_addr(&mut args, optarg, addr_type::ADDR_TYPE_MCAST) < 0 { return 1; } args.type_ = SOCK_DGRAM; }
            x if x == b'6' as c_int => args.version = AF_INET6,
            x if x == b'b' as c_int => args.bind_test_only = 1,
            x if x == b'0' as c_int => { args.has_expected_laddr = 1; args.expected_laddr_str = optarg; }
            x if x == b'1' as c_int => { args.has_expected_raddr = 1; args.expected_raddr_str = optarg; }
            x if x == b'2' as c_int => args.expected_dev = optarg,
            x if x == b'3' as c_int => args.expected_server_dev = optarg,
            x if x == b'q' as c_int => quiet = 1,
            x if x == b'x' as c_int => args.use_xfrm = 1,
            x if x == b'U' as c_int => args.datagram_connect = 1,
            _ => { print_usage(*argv); return 1; }
        }
    }
    if !args.password.is_null() && ((args.has_remote_ip == 0 && args.md5_prefix_str.is_null()) || args.type_ != SOCK_STREAM) {
        log_error!(b"MD5 passwords apply to TCP only and require a remote ip for the password\n\0");
        return 1;
    }
    if !args.md5_prefix_str.is_null() && args.password.is_null() {
        log_error!(b"Prefix range for MD5 protection specified without a password\n\0");
        return 1;
    }
    if iter == 0 { fprintf(stderr, b"Invalid number of messages to send\n\0".as_ptr() as *const c_char); return 1; }
    if args.type_ == SOCK_STREAM && args.protocol == 0 { args.protocol = IPPROTO_TCP; }
    if args.type_ == SOCK_DGRAM && args.protocol == 0 { args.protocol = IPPROTO_UDP; }
    if (args.type_ == SOCK_STREAM || args.type_ == SOCK_DGRAM) && args.port == 0 {
        fprintf(stderr, b"Invalid port number\n\0".as_ptr() as *const c_char);
        return 1;
    }
    if (both_mode != 0 || server_mode == 0) && args.has_grp == 0 && args.has_remote_ip == 0 && args.has_local_ip == 0 {
        fprintf(stderr, b"Local (server mode) or remote IP (client IP) required\n\0".as_ptr() as *const c_char);
        return 1;
    }
    if interactive != 0 {
        prog_timeout = 0;
        msg = null_mut();
    }
    if both_mode != 0 {
        if pipe(fd.as_mut_ptr()) < 0 { perror(b"pipe\0".as_ptr() as *const c_char); std::process::exit(1); }
        let cpid = fork();
        if cpid < 0 { perror(b"fork\0".as_ptr() as *const c_char); std::process::exit(1); }
        if cpid != 0 { return ipc_parent(cpid, fd[0], &mut args); }
        return ipc_child(fd[1], &mut args);
    }
    if server_mode != 0 {
        loop {
            rc = do_server(&mut args, -1);
            if forever == 0 { break; }
        }
        return rc;
    }
    do_client(&mut args)
}

fn main() {
    unsafe {
        let mut args: Vec<*mut c_char> = std::env::args()
            .map(|s| std::ffi::CString::new(s).unwrap().into_raw())
            .collect();
        args.push(null_mut());
        let rc = main_impl((args.len() - 1) as c_int, args.as_mut_ptr());
        std::process::exit(rc);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
