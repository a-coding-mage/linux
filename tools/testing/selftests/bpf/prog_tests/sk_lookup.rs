// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2020 Cloudflare
/*
 * Test BPF attach point for INET socket lookup (BPF_SK_LOOKUP).
 *
 * Tests exercise:
 *  - attaching/detaching/querying programs to BPF_SK_LOOKUP hook,
 *  - redirecting socket lookup to a socket selected by BPF program,
 *  - failing a socket lookup on BPF program's request,
 *  - error scenarios for selecting a socket from BPF program,
 *  - accessing BPF program context,
 *  - attaching and running multiple BPF programs.
 *
 * Tests run in a dedicated network namespace.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = bool;
type ssize_t = isize;
type socklen_t = u32;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const EXT_IP4: &[u8] = b"127.0.0.1\0";
const EXT_IP6: &[u8] = b"fd00::1\0";
const EXT_PORT: c_int = 7007;

const INT_IP4: &[u8] = b"127.0.0.2\0";
const INT_IP4_V6: &[u8] = b"::ffff:127.0.0.2\0";
const INT_IP6: &[u8] = b"fd00::2\0";
const INT_PORT: c_int = 8008;

const SERVER_A: c_int = 0;
const SERVER_B: c_int = 1;
const MAX_SERVERS: usize = 2;

const PROG1: c_int = 0;
const PROG2: c_int = 1;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SOL_IP: c_int = 0;
const SOL_IPV6: c_int = 41;
const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
const SO_REUSEADDR: c_int = 2;
const SO_REUSEPORT: c_int = 15;
const SO_COOKIE: c_int = 57;
const IP_RECVORIGDSTADDR: c_int = 20;
const IP_ORIGDSTADDR: c_int = 20;
const IPV6_RECVORIGDSTADDR: c_int = 74;
const IPV6_ORIGDSTADDR: c_int = 74;
const IPPROTO_TCP: c_int = 6;
const SOMAXCONN: c_int = 4096;
const O_RDONLY: c_int = 0;
const MSG_CTRUNC: c_int = 8;
const BPF_SK_LOOKUP: c_int = 43;
const BPF_NOEXIST: u64 = 1;
const BPF_ANY: u64 = 0;
const ECONNREFUSED: c_int = 111;
const CLONE_NEWNET: c_int = 0x40000000;

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct inet_addr {
    ip: *const c_char,
    port: c_ushort,
}
type c_ushort = u16;

#[repr(C)]
struct test {
    desc: *const c_char,
    lookup_prog: *mut bpf_program,
    reuseport_prog: *mut bpf_program,
    sock_map: *mut bpf_map,
    sotype: c_int,
    connect_to: inet_addr,
    listen_at: inet_addr,
    accept_on: c_int,
    reuseport_has_conns: bool_,
}

#[repr(C)]
struct cb_opts {
    family: c_int,
    sotype: c_int,
    reuseport: bool_,
}

#[repr(C)]
struct test_multi_prog {
    desc: *const c_char,
    prog1: *mut bpf_program,
    prog2: *mut bpf_program,
    redir_map: *mut bpf_map,
    run_map: *mut bpf_map,
    expect_errno: c_int,
    listen_at: inet_addr,
}

#[repr(C)]
struct test_sk_lookup_progs {
    lookup_pass: *mut bpf_program,
    lookup_drop: *mut bpf_program,
    redir_port: *mut bpf_program,
    redir_ip4: *mut bpf_program,
    select_sock_a: *mut bpf_program,
    select_sock_b: *mut bpf_program,
    select_sock_a_no_reuseport: *mut bpf_program,
    redir_ip6: *mut bpf_program,
    check_ifindex: *mut bpf_program,
    reuseport_drop: *mut bpf_program,
    sk_assign_esocknosupport: *mut bpf_program,
    sk_assign_eexist: *mut bpf_program,
    sk_assign_replace_flag: *mut bpf_program,
    sk_assign_null: *mut bpf_program,
    access_ctx_sk: *mut bpf_program,
    ctx_narrow_access: *mut bpf_program,
    multi_prog_pass1: *mut bpf_program,
    multi_prog_pass2: *mut bpf_program,
    multi_prog_drop1: *mut bpf_program,
    multi_prog_drop2: *mut bpf_program,
    multi_prog_redir2: *mut bpf_program,
    multi_prog_redir1: *mut bpf_program,
}

#[repr(C)]
struct test_sk_lookup_maps {
    redir_map: *mut bpf_map,
    run_map: *mut bpf_map,
}

#[repr(C)]
struct test_sk_lookup {
    progs: test_sk_lookup_progs,
    maps: test_sk_lookup_maps,
}

#[repr(C)]
struct bpf_sk_lookup {
    family: __u32,
    protocol: __u32,
    remote_ip4: __u32,
    remote_ip6: [__u32; 4],
    remote_port: __u32,
    local_ip4: __u32,
    local_ip6: [__u32; 4],
    local_port: __u32,
    ingress_ifindex: __u32,
    cookie: __u64,
}

#[repr(C)]
struct sockaddr_storage {
    ss_family: c_ushort,
    __data: [u8; 126],
}
#[repr(C)]
#[derive(Copy, Clone)]
struct in_addr {
    s_addr: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct sockaddr_in {
    sin_family: c_ushort,
    sin_port: c_ushort,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}
#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}
#[repr(C)]
struct sockaddr_in6 {
    sin6_family: c_ushort,
    sin6_port: c_ushort,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}
#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: usize,
}
#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: usize,
    msg_control: *mut c_void,
    msg_controllen: usize,
    msg_flags: c_int,
}
#[repr(C)]
struct cmsghdr {
    cmsg_len: usize,
    cmsg_level: c_int,
    cmsg_type: c_int,
}
#[repr(C)]
struct network_helper_opts {
    backlog: c_int,
    post_socket_cb: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_int>,
    cb_opts: *mut c_void,
}
#[repr(C)]
struct bpf_link_info_netns {
    netns_ino: __u32,
}
#[repr(C)]
struct bpf_link_info {
    netns: bpf_link_info_netns,
}
#[repr(C)]
struct bpf_test_run_opts {
    ctx_in: *mut c_void,
    ctx_size_in: __u32,
    ctx_out: *mut c_void,
    ctx_size_out: __u32,
}

static mut duration: __u32 = 0;

unsafe extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn htons(hostshort: u16) -> u16;
    fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> ssize_t;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> ssize_t;
    fn accept(fd: c_int, addr: *mut c_void, len: *mut socklen_t) -> c_int;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const c_void, len: socklen_t) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn system(cmd: *const c_char) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    static mut errno: c_int;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__attach_netns(prog: *mut bpf_program, net_fd: c_int) -> *mut bpf_link;
    fn bpf_program__name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_link__detach(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_prog_query(target_fd: c_int, attach_type: c_int, query_flags: __u32, attach_flags: *mut __u32, prog_ids: *mut __u32, prog_cnt: *mut __u32) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn start_server_str(family: c_int, sotype: c_int, ip: *const c_char, port: c_int, opts: *mut network_helper_opts) -> c_int;
    fn start_server_addr(sotype: c_int, addr: *mut sockaddr_storage, len: socklen_t, opts: *mut c_void) -> c_int;
    fn connect_to_addr_str(family: c_int, sotype: c_int, ip: *const c_char, port: c_int, opts: *mut c_void) -> c_int;
    fn connect_fd_to_fd(fd: c_int, target_fd: c_int, timeout_ms: c_int) -> c_int;
    fn client_socket(family: c_int, sotype: c_int, opts: *mut c_void) -> c_int;
    fn make_sockaddr(family: c_int, ip: *const c_char, port: c_int, dst: *mut sockaddr_storage, len: *mut socklen_t) -> c_int;
    fn link_info_prog_id(link: *mut bpf_link, info: *mut bpf_link_info) -> __u32;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test_sk_lookup__open_and_load() -> *mut test_sk_lookup;
    fn test_sk_lookup__destroy(skel: *mut test_sk_lookup);
    fn log_err(fmt: *const c_char, ...);
    fn CHECK(cond: bool, tag: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut bpf_link, name: *const c_char) -> bool;
    fn PTR_ERR(ptr: *mut bpf_link) -> c_long;
    fn CMSG_FIRSTHDR(msg: *mut msghdr) -> *mut cmsghdr;
    fn CMSG_NXTHDR(msg: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr;
    fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut u8;
}

const fn cmsg_align(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}
const fn cmsg_space(len: usize) -> usize {
    cmsg_align(size_of::<cmsghdr>()) + cmsg_align(len)
}

unsafe fn is_ipv6(ip: *const c_char) -> bool {
    !strchr(ip, ':' as c_int).is_null()
}

unsafe extern "C" fn setsockopts(fd: c_int, opts: *mut c_void) -> c_int {
    let co = opts as *mut cb_opts;
    let one: c_int = 1;
    let mut err: c_int = 0;

    /* Enabled for UDPv6 sockets for IPv4-mapped IPv6 to work. */
    if (*co).sotype == SOCK_DGRAM {
        err = setsockopt(fd, SOL_IP, IP_RECVORIGDSTADDR, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
        if CHECK(err != 0, c"setsockopt(IP_RECVORIGDSTADDR)".as_ptr(), c"failed\n".as_ptr()) {
            log_err(c"failed to enable IP_RECVORIGDSTADDR".as_ptr());
            return err;
        }
    }
    if (*co).sotype == SOCK_DGRAM && (*co).family == AF_INET6 {
        err = setsockopt(fd, SOL_IPV6, IPV6_RECVORIGDSTADDR, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
        if CHECK(err != 0, c"setsockopt(IPV6_RECVORIGDSTADDR)".as_ptr(), c"failed\n".as_ptr()) {
            log_err(c"failed to enable IPV6_RECVORIGDSTADDR".as_ptr());
            return err;
        }
    }
    if (*co).sotype == SOCK_STREAM {
        err = setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
        if CHECK(err != 0, c"setsockopt(SO_REUSEADDR)".as_ptr(), c"failed\n".as_ptr()) {
            log_err(c"failed to enable SO_REUSEADDR".as_ptr());
            return err;
        }
    }
    if (*co).reuseport {
        err = setsockopt(fd, SOL_SOCKET, SO_REUSEPORT, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
        if CHECK(err != 0, c"setsockopt(SO_REUSEPORT)".as_ptr(), c"failed\n".as_ptr()) {
            log_err(c"failed to enable SO_REUSEPORT".as_ptr());
            return err;
        }
    }
    err
}

unsafe fn attach_reuseport(sock_fd: c_int, reuseport_prog: *mut bpf_program) -> c_int {
    let prog_fd = bpf_program__fd(reuseport_prog);
    if prog_fd < 0 {
        errno = -prog_fd;
        return -1;
    }
    let err = setsockopt(sock_fd, SOL_SOCKET, SO_ATTACH_REUSEPORT_EBPF, &prog_fd as *const _ as *const c_void, size_of::<c_int>() as socklen_t);
    if err != 0 { -1 } else { 0 }
}

unsafe fn make_server(sotype: c_int, ip: *const c_char, port: c_int, reuseport_prog: *mut bpf_program) -> c_int {
    let mut cb_opts = cb_opts {
        family: if is_ipv6(ip) { AF_INET6 } else { AF_INET },
        sotype,
        reuseport: !reuseport_prog.is_null(),
    };
    let mut opts = network_helper_opts {
        backlog: SOMAXCONN,
        post_socket_cb: Some(setsockopts),
        cb_opts: &mut cb_opts as *mut _ as *mut c_void,
    };
    let fd = start_server_str(cb_opts.family, sotype, ip, port, &mut opts);
    if !ASSERT_OK_FD(fd, c"start_server_str".as_ptr()) {
        return -1;
    }
    /* Late attach reuseport prog so we can have one init path */
    if !reuseport_prog.is_null() {
        let err = attach_reuseport(fd, reuseport_prog);
        if CHECK(err != 0, c"attach_reuseport".as_ptr(), c"failed\n".as_ptr()) {
            log_err(c"failed to attach reuseport prog".as_ptr());
            close(fd);
            return -1;
        }
    }
    fd
}

unsafe fn socket_cookie(fd: c_int) -> __u64 {
    let mut cookie: __u64 = 0;
    let mut cookie_len: socklen_t = size_of::<__u64>() as socklen_t;
    if CHECK(getsockopt(fd, SOL_SOCKET, SO_COOKIE, &mut cookie as *mut _ as *mut c_void, &mut cookie_len) < 0,
             c"getsockopt(SO_COOKIE)".as_ptr(), c"%s\n".as_ptr(), strerror(errno)) {
        return 0;
    }
    cookie
}

unsafe fn fill_sk_lookup_ctx(ctx: *mut bpf_sk_lookup, local_ip: *const c_char, local_port: __u16, remote_ip: *const c_char, remote_port: __u16) -> c_int {
    memset(ctx as *mut c_void, 0, size_of::<bpf_sk_lookup>());
    (*ctx).local_port = local_port as __u32;
    (*ctx).remote_port = htons(remote_port) as __u32;
    let (local, remote) = if is_ipv6(local_ip) {
        (*ctx).family = AF_INET6 as __u32;
        (&mut (*ctx).local_ip6[0] as *mut _ as *mut c_void, &mut (*ctx).remote_ip6[0] as *mut _ as *mut c_void)
    } else {
        (*ctx).family = AF_INET as __u32;
        (&mut (*ctx).local_ip4 as *mut _ as *mut c_void, &mut (*ctx).remote_ip4 as *mut _ as *mut c_void)
    };
    let mut err = inet_pton((*ctx).family as c_int, local_ip, local);
    if CHECK(err != 1, c"inet_pton".as_ptr(), c"local_ip failed\n".as_ptr()) { return 1; }
    err = inet_pton((*ctx).family as c_int, remote_ip, remote);
    if CHECK(err != 1, c"inet_pton".as_ptr(), c"remote_ip failed\n".as_ptr()) { return 1; }
    0
}

unsafe fn send_byte(fd: c_int) -> c_int {
    errno = 0;
    let n = send(fd, c"a".as_ptr() as *const c_void, 1, 0);
    if CHECK(n <= 0, c"send_byte".as_ptr(), c"send".as_ptr()) {
        log_err(c"failed/partial send".as_ptr());
        return -1;
    }
    0
}

unsafe fn recv_byte(fd: c_int) -> c_int {
    let mut buf = [0i8; 1];
    let n = recv(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[i8; 1]>(), 0);
    if CHECK(n <= 0, c"recv_byte".as_ptr(), c"recv".as_ptr()) {
        log_err(c"failed/partial recv".as_ptr());
        return -1;
    }
    0
}

unsafe fn tcp_recv_send(server_fd: c_int) -> c_int {
    let mut buf = [0i8; 1];
    let fd = accept(server_fd, null_mut(), null_mut());
    if CHECK(fd < 0, c"accept".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed to accept".as_ptr());
        return -1;
    }
    let mut n = recv(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[i8; 1]>(), 0);
    if CHECK(n <= 0, c"recv".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed/partial recv".as_ptr());
        close(fd);
        return -1;
    }
    n = send(fd, buf.as_ptr() as *const c_void, n as usize, 0);
    if CHECK(n <= 0, c"send".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed/partial send".as_ptr());
        close(fd);
        return -1;
    }
    close(fd);
    0
}

unsafe fn v4_to_v6(ss: *mut sockaddr_storage) {
    let v6 = ss as *mut sockaddr_in6;
    let v4 = *(ss as *mut sockaddr_in);
    (*v6).sin6_family = AF_INET6 as c_ushort;
    (*v6).sin6_port = v4.sin_port;
    (*v6).sin6_addr.s6_addr[10] = 0xff;
    (*v6).sin6_addr.s6_addr[11] = 0xff;
    memcpy(&mut (*v6).sin6_addr.s6_addr[12] as *mut _ as *mut c_void, &v4.sin_addr.s_addr as *const _ as *const c_void, 4);
    memset(&mut (*v6).sin6_addr.s6_addr[0] as *mut _ as *mut c_void, 0, 10);
}

unsafe fn udp_recv_send(server_fd: c_int) -> c_int {
    let mut cmsg_buf = [0u8; cmsg_space(size_of::<sockaddr_storage>())];
    let mut src_storage: sockaddr_storage = zeroed();
    let src_addr = &mut src_storage as *mut sockaddr_storage;
    let mut dst_addr: *mut sockaddr_storage = null_mut();
    let mut msg: msghdr = zeroed();
    let mut iov: iovec = zeroed();
    let mut buf = [0i8; 1];
    iov.iov_base = buf.as_mut_ptr() as *mut c_void;
    iov.iov_len = size_of::<[i8; 1]>();
    msg.msg_name = src_addr as *mut c_void;
    msg.msg_namelen = size_of::<sockaddr_storage>() as socklen_t;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = cmsg_buf.as_mut_ptr() as *mut c_void;
    msg.msg_controllen = cmsg_buf.len();
    errno = 0;
    let mut n = recvmsg(server_fd, &mut msg, 0);
    if CHECK(n <= 0, c"recvmsg".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed to receive".as_ptr());
        return -1;
    }
    if CHECK((msg.msg_flags & MSG_CTRUNC) != 0, c"recvmsg".as_ptr(), c"truncated cmsg\n".as_ptr()) { return -1; }
    let mut cm = CMSG_FIRSTHDR(&mut msg);
    while !cm.is_null() {
        if ((*cm).cmsg_level == SOL_IP && (*cm).cmsg_type == IP_ORIGDSTADDR) ||
           ((*cm).cmsg_level == SOL_IPV6 && (*cm).cmsg_type == IPV6_ORIGDSTADDR) {
            dst_addr = CMSG_DATA(cm) as *mut sockaddr_storage;
            break;
        }
        log_err(c"warning: ignored cmsg at level %d type %d".as_ptr(), (*cm).cmsg_level, (*cm).cmsg_type);
        cm = CMSG_NXTHDR(&mut msg, cm);
    }
    if CHECK(dst_addr.is_null(), c"recvmsg".as_ptr(), c"missing ORIGDSTADDR\n".as_ptr()) { return -1; }
    /* Server socket bound to IPv4-mapped IPv6 address */
    if (*src_addr).ss_family as c_int == AF_INET6 && (*dst_addr).ss_family as c_int == AF_INET {
        v4_to_v6(dst_addr);
    }
    /* Reply from original destination address. */
    let fd = start_server_addr(SOCK_DGRAM, dst_addr, size_of::<sockaddr_storage>() as socklen_t, null_mut());
    if !ASSERT_OK_FD(fd, c"start_server_addr".as_ptr()) {
        log_err(c"failed to create tx socket".as_ptr());
        return -1;
    }
    msg.msg_control = null_mut();
    msg.msg_controllen = 0;
    n = sendmsg(fd, &msg, 0);
    if CHECK(n <= 0, c"sendmsg".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed to send echo reply".as_ptr());
        close(fd);
        return -1;
    }
    close(fd);
    0
}

unsafe fn tcp_echo_test(client_fd: c_int, server_fd: c_int) -> c_int {
    if send_byte(client_fd) != 0 { return -1; }
    if tcp_recv_send(server_fd) != 0 { return -1; }
    if recv_byte(client_fd) != 0 { return -1; }
    0
}

unsafe fn udp_echo_test(client_fd: c_int, server_fd: c_int) -> c_int {
    if send_byte(client_fd) != 0 { return -1; }
    if udp_recv_send(server_fd) != 0 { return -1; }
    if recv_byte(client_fd) != 0 { return -1; }
    0
}

unsafe fn attach_lookup_prog(prog: *mut bpf_program) -> *mut bpf_link {
    let net_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    if CHECK(net_fd < 0, c"open".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed to open /proc/self/ns/net".as_ptr());
        return null_mut();
    }
    let mut link = bpf_program__attach_netns(prog, net_fd);
    if !ASSERT_OK_PTR(link, c"bpf_program__attach_netns".as_ptr()) {
        errno = -PTR_ERR(link) as c_int;
        log_err(c"failed to attach program '%s' to netns".as_ptr(), bpf_program__name(prog));
        link = null_mut();
    }
    close(net_fd);
    link
}

unsafe fn update_lookup_map(map: *mut bpf_map, index: c_int, sock_fd: c_int) -> c_int {
    let map_fd = bpf_map__fd(map);
    if CHECK(map_fd < 0, c"bpf_map__fd".as_ptr(), c"failed\n".as_ptr()) {
        errno = -map_fd;
        log_err(c"failed to get map FD".as_ptr());
        return -1;
    }
    let value: u64 = sock_fd as u64;
    let err = bpf_map_update_elem(map_fd, &index as *const _ as *const c_void, &value as *const _ as *const c_void, BPF_NOEXIST);
    if CHECK(err != 0, c"bpf_map_update_elem".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed to update redir_map @ %d".as_ptr(), index);
        return -1;
    }
    0
}

unsafe fn query_lookup_prog(skel: *mut test_sk_lookup) {
    let mut link: [*mut bpf_link; 3] = [null_mut(); 3];
    let mut info: bpf_link_info = zeroed();
    let mut attach_flags: __u32 = 0;
    let mut prog_ids: [__u32; 3] = [0; 3];
    let mut prog_cnt: __u32 = 3;
    let net_fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
    if CHECK(net_fd < 0, c"open".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed to open /proc/self/ns/net".as_ptr());
        return;
    }
    link[0] = attach_lookup_prog((*skel).progs.lookup_pass);
    if link[0].is_null() { close(net_fd); return; }
    link[1] = attach_lookup_prog((*skel).progs.lookup_pass);
    if link[1].is_null() { goto_detach(&mut link); close(net_fd); return; }
    link[2] = attach_lookup_prog((*skel).progs.lookup_drop);
    if link[2].is_null() { goto_detach(&mut link); close(net_fd); return; }
    let mut err = bpf_prog_query(net_fd, BPF_SK_LOOKUP, 0, &mut attach_flags, prog_ids.as_mut_ptr(), &mut prog_cnt);
    if CHECK(err != 0, c"bpf_prog_query".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"failed to query lookup prog".as_ptr());
        goto_detach(&mut link);
        close(net_fd);
        return;
    }
    errno = 0;
    if CHECK(attach_flags != 0, c"bpf_prog_query".as_ptr(), c"wrong attach_flags on query: %u".as_ptr(), attach_flags) { goto_detach(&mut link); close(net_fd); return; }
    if CHECK(prog_cnt != 3, c"bpf_prog_query".as_ptr(), c"wrong program count on query: %u".as_ptr(), prog_cnt) { goto_detach(&mut link); close(net_fd); return; }
    for i in 0..3 {
        let prog_id = link_info_prog_id(link[i], &mut info);
        CHECK(prog_ids[i] != prog_id, c"bpf_prog_query".as_ptr(), c"invalid program #%u id on query: %u != %u\n".as_ptr(), i as c_uint, prog_ids[i], prog_id);
        CHECK(info.netns.netns_ino == 0, c"netns_ino".as_ptr(), c"unexpected netns_ino: %u\n".as_ptr(), info.netns.netns_ino);
    }
    err = bpf_link__detach(link[0]);
    if CHECK(err != 0, c"link_detach".as_ptr(), c"failed %d\n".as_ptr(), err) {
        goto_detach(&mut link);
        close(net_fd);
        return;
    }
    /* prog id is still there, but netns_ino is zeroed out */
    let prog_id = link_info_prog_id(link[0], &mut info);
    CHECK(prog_ids[0] != prog_id, c"bpf_prog_query".as_ptr(), c"invalid program #0 id on query: %u != %u\n".as_ptr(), prog_ids[0], prog_id);
    CHECK(info.netns.netns_ino != 0, c"netns_ino".as_ptr(), c"unexpected netns_ino: %u\n".as_ptr(), info.netns.netns_ino);
    goto_detach(&mut link);
    close(net_fd);
}

unsafe fn goto_detach(link: &mut [*mut bpf_link; 3]) {
    if !link[2].is_null() { bpf_link__destroy(link[2]); }
    if !link[1].is_null() { bpf_link__destroy(link[1]); }
    if !link[0].is_null() { bpf_link__destroy(link[0]); }
}

unsafe fn run_lookup_prog(t: *const test) {
    let mut server_fds = [-1; MAX_SERVERS];
    let mut client_fd: c_int;
    let mut reuse_conn_fd: c_int = -1;
    let lookup_link = attach_lookup_prog((*t).lookup_prog);
    if lookup_link.is_null() { return; }
    for i in 0..MAX_SERVERS {
        server_fds[i] = make_server((*t).sotype, (*t).listen_at.ip, (*t).listen_at.port as c_int, (*t).reuseport_prog);
        if server_fds[i] < 0 { break; }
        if update_lookup_map((*t).sock_map, i as c_int, server_fds[i]) != 0 { break; }
        /* want just one server for non-reuseport test */
        if (*t).reuseport_prog.is_null() { break; }
    }
    /* Regular UDP socket lookup with reuseport behaves differently when reuseport group contains connected sockets. */
    if (*t).reuseport_has_conns {
        /* Add an extra socket to reuseport group */
        reuse_conn_fd = make_server((*t).sotype, (*t).listen_at.ip, (*t).listen_at.port as c_int, (*t).reuseport_prog);
        if reuse_conn_fd >= 0 {
            /* Connect the extra socket to itself */
            let err = connect_fd_to_fd(reuse_conn_fd, reuse_conn_fd, 0);
            ASSERT_OK(err, c"connect_fd_to_fd".as_ptr());
        }
    }
    client_fd = connect_to_addr_str(if is_ipv6((*t).connect_to.ip) { AF_INET6 } else { AF_INET }, (*t).sotype, (*t).connect_to.ip, (*t).connect_to.port as c_int, null_mut());
    if ASSERT_OK_FD(client_fd, c"connect_to_addr_str".as_ptr()) {
        if (*t).sotype == SOCK_STREAM {
            tcp_echo_test(client_fd, server_fds[(*t).accept_on as usize]);
        } else {
            udp_echo_test(client_fd, server_fds[(*t).accept_on as usize]);
        }
        close(client_fd);
    }
    if reuse_conn_fd != -1 { close(reuse_conn_fd); }
    for fd in server_fds { if fd != -1 { close(fd); } }
    bpf_link__destroy(lookup_link);
}

macro_rules! ia { ($ip:expr, $port:expr) => { inet_addr { ip: $ip.as_ptr() as *const c_char, port: $port as c_ushort } }; }
macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }

unsafe fn test_redirect_lookup(skel: *mut test_sk_lookup) {
    let tests = [
        test { desc: cstr!("TCP IPv4 redir port"), lookup_prog: (*skel).progs.redir_port, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(EXT_IP4, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv4 redir addr"), lookup_prog: (*skel).progs.redir_ip4, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv4 redir with reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, INT_PORT), accept_on: SERVER_B, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv4 redir skip reuseport"), lookup_prog: (*skel).progs.select_sock_a_no_reuseport, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 redir port"), lookup_prog: (*skel).progs.redir_port, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(EXT_IP6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 redir addr"), lookup_prog: (*skel).progs.redir_ip6, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv4->IPv6 redir port"), lookup_prog: (*skel).progs.redir_port, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4_V6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 redir with reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, INT_PORT), accept_on: SERVER_B, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 redir skip reuseport"), lookup_prog: (*skel).progs.select_sock_a_no_reuseport, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4 redir port"), lookup_prog: (*skel).progs.redir_port, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(EXT_IP4, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4 redir addr"), lookup_prog: (*skel).progs.redir_ip4, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4 redir with reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, INT_PORT), accept_on: SERVER_B, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4 redir and reuseport with conns"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, INT_PORT), accept_on: SERVER_B, reuseport_has_conns: true },
        test { desc: cstr!("UDP IPv4 redir skip reuseport"), lookup_prog: (*skel).progs.select_sock_a_no_reuseport, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv6 redir port"), lookup_prog: (*skel).progs.redir_port, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(EXT_IP6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv6 redir addr"), lookup_prog: (*skel).progs.redir_ip6, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4->IPv6 redir port"), lookup_prog: (*skel).progs.redir_port, reuseport_prog: null_mut(), sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4_V6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv6 redir and reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, INT_PORT), accept_on: SERVER_B, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv6 redir and reuseport with conns"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, INT_PORT), accept_on: SERVER_B, reuseport_has_conns: true },
        test { desc: cstr!("UDP IPv6 redir skip reuseport"), lookup_prog: (*skel).progs.select_sock_a_no_reuseport, reuseport_prog: (*skel).progs.select_sock_b, sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
    ];
    for t in &tests {
        if test__start_subtest(t.desc) { run_lookup_prog(t); }
    }
}

unsafe fn drop_on_lookup(t: *const test) {
    let family = if is_ipv6((*t).connect_to.ip) { AF_INET6 } else { AF_INET };
    let mut dst: sockaddr_storage = zeroed();
    let mut len: socklen_t = 0;
    let lookup_link = attach_lookup_prog((*t).lookup_prog);
    if lookup_link.is_null() { return; }
    let server_fd = make_server((*t).sotype, (*t).listen_at.ip, (*t).listen_at.port as c_int, (*t).reuseport_prog);
    if server_fd < 0 { bpf_link__destroy(lookup_link); return; }
    let client_fd = client_socket(family, (*t).sotype, null_mut());
    if ASSERT_OK_FD(client_fd, c"client_socket".as_ptr()) {
        let mut err = make_sockaddr(family, (*t).connect_to.ip, (*t).connect_to.port as c_int, &mut dst, &mut len);
        if ASSERT_OK(err, c"make_sockaddr".as_ptr()) {
            err = connect(client_fd, &dst as *const _ as *const c_void, len);
            if (*t).sotype == SOCK_DGRAM {
                err = send_byte(client_fd);
                if err == 0 {
                    /* Read out asynchronous error */
                    let n = recv(client_fd, null_mut(), 0, 0);
                    err = (n == -1) as c_int;
                }
            }
            if CHECK(err == 0 || errno != ECONNREFUSED, c"connect".as_ptr(), c"unexpected success or error\n".as_ptr()) {
                log_err(c"expected ECONNREFUSED on connect".as_ptr());
            }
        }
        close(client_fd);
    }
    close(server_fd);
    bpf_link__destroy(lookup_link);
}

unsafe fn test_drop_on_lookup(skel: *mut test_sk_lookup) {
    let tests = [
        test { desc: cstr!("TCP IPv4 drop on lookup"), lookup_prog: (*skel).progs.lookup_drop, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(EXT_IP4, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 drop on lookup"), lookup_prog: (*skel).progs.lookup_drop, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(EXT_IP6, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4 drop on lookup"), lookup_prog: (*skel).progs.lookup_drop, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(EXT_IP4, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv6 drop on lookup"), lookup_prog: (*skel).progs.lookup_drop, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(EXT_IP6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        /* The program will drop on success, meaning that the ifindex was 1. */
        test { desc: cstr!("TCP IPv4 drop on valid ifindex"), lookup_prog: (*skel).progs.check_ifindex, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(EXT_IP4, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 drop on valid ifindex"), lookup_prog: (*skel).progs.check_ifindex, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(EXT_IP6, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4 drop on valid ifindex"), lookup_prog: (*skel).progs.check_ifindex, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(EXT_IP4, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv6 drop on valid ifindex"), lookup_prog: (*skel).progs.check_ifindex, reuseport_prog: null_mut(), sock_map: null_mut(), sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(EXT_IP6, EXT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
    ];
    for t in &tests {
        if test__start_subtest(t.desc) { drop_on_lookup(t); }
    }
}

unsafe fn drop_on_reuseport(t: *const test) {
    let family = if is_ipv6((*t).connect_to.ip) { AF_INET6 } else { AF_INET };
    let mut dst: sockaddr_storage = zeroed();
    let mut len: socklen_t = 0;
    let lookup_link = attach_lookup_prog((*t).lookup_prog);
    if lookup_link.is_null() { return; }
    let server1 = make_server((*t).sotype, (*t).listen_at.ip, (*t).listen_at.port as c_int, (*t).reuseport_prog);
    if server1 < 0 { bpf_link__destroy(lookup_link); return; }
    if update_lookup_map((*t).sock_map, SERVER_A, server1) == 0 {
        /* second server on destination address we should never reach */
        let server2 = make_server((*t).sotype, (*t).connect_to.ip, (*t).connect_to.port as c_int, null_mut());
        if server2 >= 0 {
            let client = client_socket(family, (*t).sotype, null_mut());
            if ASSERT_OK_FD(client, c"client_socket".as_ptr()) {
                let mut err = make_sockaddr(family, (*t).connect_to.ip, (*t).connect_to.port as c_int, &mut dst, &mut len);
                if ASSERT_OK(err, c"make_sockaddr".as_ptr()) {
                    err = connect(client, &dst as *const _ as *const c_void, len);
                    if (*t).sotype == SOCK_DGRAM {
                        err = send_byte(client);
                        if err == 0 {
                            /* Read out asynchronous error */
                            let n = recv(client, null_mut(), 0, 0);
                            err = (n == -1) as c_int;
                        }
                    }
                    if CHECK(err == 0 || errno != ECONNREFUSED, c"connect".as_ptr(), c"unexpected success or error\n".as_ptr()) {
                        log_err(c"expected ECONNREFUSED on connect".as_ptr());
                    }
                }
                close(client);
            }
            close(server2);
        }
    }
    close(server1);
    bpf_link__destroy(lookup_link);
}

unsafe fn test_drop_on_reuseport(skel: *mut test_sk_lookup) {
    let tests = [
        test { desc: cstr!("TCP IPv4 drop on reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.reuseport_drop, sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 drop on reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.reuseport_drop, sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("UDP IPv4 drop on reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.reuseport_drop, sock_map: (*skel).maps.redir_map, sotype: SOCK_DGRAM, connect_to: ia!(EXT_IP4, EXT_PORT), listen_at: ia!(INT_IP4, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
        test { desc: cstr!("TCP IPv6 drop on reuseport"), lookup_prog: (*skel).progs.select_sock_a, reuseport_prog: (*skel).progs.reuseport_drop, sock_map: (*skel).maps.redir_map, sotype: SOCK_STREAM, connect_to: ia!(EXT_IP6, EXT_PORT), listen_at: ia!(INT_IP6, INT_PORT), accept_on: SERVER_A, reuseport_has_conns: false },
    ];
    for t in &tests {
        if test__start_subtest(t.desc) { drop_on_reuseport(t); }
    }
}

unsafe fn run_sk_assign(skel: *mut test_sk_lookup, lookup_prog: *mut bpf_program, remote_ip: *const c_char, local_ip: *const c_char) {
    let mut server_fds = [-1; MAX_SERVERS];
    let mut ctx: bpf_sk_lookup = zeroed();
    let mut opts = bpf_test_run_opts {
        ctx_in: &mut ctx as *mut _ as *mut c_void,
        ctx_size_in: size_of::<bpf_sk_lookup>() as __u32,
        ctx_out: &mut ctx as *mut _ as *mut c_void,
        ctx_size_out: size_of::<bpf_sk_lookup>() as __u32,
    };
    if fill_sk_lookup_ctx(&mut ctx, local_ip, EXT_PORT as __u16, remote_ip, INT_PORT as __u16) != 0 { return; }
    ctx.protocol = IPPROTO_TCP as __u32;
    for i in 0..MAX_SERVERS {
        server_fds[i] = make_server(SOCK_STREAM, local_ip, 0, null_mut());
        if server_fds[i] < 0 { break; }
        if update_lookup_map((*skel).maps.redir_map, i as c_int, server_fds[i]) != 0 { break; }
    }
    let server_cookie = socket_cookie(server_fds[SERVER_B as usize]);
    if server_cookie == 0 { return; }
    let err = bpf_prog_test_run_opts(bpf_program__fd(lookup_prog), &mut opts);
    if CHECK(err != 0, c"test_run".as_ptr(), c"failed with error %d\n".as_ptr(), errno) {
    } else if !CHECK(ctx.cookie == 0, c"ctx.cookie".as_ptr(), c"no socket selected\n".as_ptr()) {
        CHECK(ctx.cookie != server_cookie, c"ctx.cookie".as_ptr(), c"selected sk %llu instead of %llu\n".as_ptr(), ctx.cookie, server_cookie);
    }
    for fd in server_fds { if fd != -1 { close(fd); } }
}

unsafe fn run_sk_assign_v4(skel: *mut test_sk_lookup, lookup_prog: *mut bpf_program) {
    run_sk_assign(skel, lookup_prog, INT_IP4.as_ptr() as *const c_char, EXT_IP4.as_ptr() as *const c_char);
}
unsafe fn run_sk_assign_v6(skel: *mut test_sk_lookup, lookup_prog: *mut bpf_program) {
    run_sk_assign(skel, lookup_prog, INT_IP6.as_ptr() as *const c_char, EXT_IP6.as_ptr() as *const c_char);
}

unsafe fn run_sk_assign_connected(skel: *mut test_sk_lookup, sotype: c_int) {
    let server_fd = make_server(sotype, EXT_IP4.as_ptr() as *const c_char, EXT_PORT, null_mut());
    if server_fd < 0 { return; }
    let connected_fd = connect_to_addr_str(AF_INET, sotype, EXT_IP4.as_ptr() as *const c_char, EXT_PORT, null_mut());
    if ASSERT_OK_FD(connected_fd, c"connect_to_addr_str".as_ptr()) {
        /* Put a connected socket in redirect map */
        if update_lookup_map((*skel).maps.redir_map, SERVER_A, connected_fd) == 0 {
            let lookup_link = attach_lookup_prog((*skel).progs.sk_assign_esocknosupport);
            if !lookup_link.is_null() {
                /* Try to redirect TCP SYN / UDP packet to a connected socket */
                let client_fd = connect_to_addr_str(AF_INET, sotype, EXT_IP4.as_ptr() as *const c_char, EXT_PORT, null_mut());
                if ASSERT_OK_FD(client_fd, c"connect_to_addr_str".as_ptr()) {
                    if sotype == SOCK_DGRAM {
                        send_byte(client_fd);
                        recv_byte(server_fd);
                    }
                    close(client_fd);
                }
                bpf_link__destroy(lookup_link);
            }
        }
        close(connected_fd);
    }
    close(server_fd);
}

unsafe fn test_sk_assign_helper(skel: *mut test_sk_lookup) {
    if test__start_subtest(c"sk_assign returns EEXIST".as_ptr()) { run_sk_assign_v4(skel, (*skel).progs.sk_assign_eexist); }
    if test__start_subtest(c"sk_assign honors F_REPLACE".as_ptr()) { run_sk_assign_v4(skel, (*skel).progs.sk_assign_replace_flag); }
    if test__start_subtest(c"sk_assign accepts NULL socket".as_ptr()) { run_sk_assign_v4(skel, (*skel).progs.sk_assign_null); }
    if test__start_subtest(c"access ctx->sk".as_ptr()) { run_sk_assign_v4(skel, (*skel).progs.access_ctx_sk); }
    if test__start_subtest(c"narrow access to ctx v4".as_ptr()) { run_sk_assign_v4(skel, (*skel).progs.ctx_narrow_access); }
    if test__start_subtest(c"narrow access to ctx v6".as_ptr()) { run_sk_assign_v6(skel, (*skel).progs.ctx_narrow_access); }
    if test__start_subtest(c"sk_assign rejects TCP established".as_ptr()) { run_sk_assign_connected(skel, SOCK_STREAM); }
    if test__start_subtest(c"sk_assign rejects UDP connected".as_ptr()) { run_sk_assign_connected(skel, SOCK_DGRAM); }
}

unsafe fn run_multi_prog_lookup(t: *const test_multi_prog) {
    let mut dst: sockaddr_storage = zeroed();
    let map_fd = bpf_map__fd((*t).run_map);
    let mut done: c_int = 0;
    let mut prog_idx = PROG1;
    let mut err = bpf_map_update_elem(map_fd, &prog_idx as *const _ as *const c_void, &done as *const _ as *const c_void, BPF_ANY);
    if CHECK(err != 0, c"bpf_map_update_elem".as_ptr(), c"failed\n".as_ptr()) { return; }
    prog_idx = PROG2;
    err = bpf_map_update_elem(map_fd, &prog_idx as *const _ as *const c_void, &done as *const _ as *const c_void, BPF_ANY);
    if CHECK(err != 0, c"bpf_map_update_elem".as_ptr(), c"failed\n".as_ptr()) { return; }
    let link1 = attach_lookup_prog((*t).prog1);
    if link1.is_null() { return; }
    let link2 = attach_lookup_prog((*t).prog2);
    if !link2.is_null() {
        let server_fd = make_server(SOCK_STREAM, (*t).listen_at.ip, (*t).listen_at.port as c_int, null_mut());
        if server_fd >= 0 {
            if update_lookup_map((*t).redir_map, SERVER_A, server_fd) == 0 {
                let client_fd = client_socket(AF_INET, SOCK_STREAM, null_mut());
                if ASSERT_OK_FD(client_fd, c"client_socket".as_ptr()) {
                    let mut len: socklen_t = 0;
                    err = make_sockaddr(AF_INET, EXT_IP4.as_ptr() as *const c_char, EXT_PORT, &mut dst, &mut len);
                    if ASSERT_OK(err, c"make_sockaddr".as_ptr()) {
                        err = connect(client_fd, &dst as *const _ as *const c_void, len);
                        if !CHECK(err != 0 && (*t).expect_errno == 0, c"connect".as_ptr(), c"unexpected error %d\n".as_ptr(), errno) &&
                           !CHECK(err != 0 && (*t).expect_errno != 0 && errno != (*t).expect_errno, c"connect".as_ptr(), c"unexpected error %d\n".as_ptr(), errno) {
                            done = 0;
                            prog_idx = PROG1;
                            err = bpf_map_lookup_elem(map_fd, &prog_idx as *const _ as *const c_void, &mut done as *mut _ as *mut c_void);
                            CHECK(err != 0, c"bpf_map_lookup_elem".as_ptr(), c"failed\n".as_ptr());
                            CHECK(done == 0, c"bpf_map_lookup_elem".as_ptr(), c"PROG1 !done\n".as_ptr());
                            done = 0;
                            prog_idx = PROG2;
                            err = bpf_map_lookup_elem(map_fd, &prog_idx as *const _ as *const c_void, &mut done as *mut _ as *mut c_void);
                            CHECK(err != 0, c"bpf_map_lookup_elem".as_ptr(), c"failed\n".as_ptr());
                            CHECK(done == 0, c"bpf_map_lookup_elem".as_ptr(), c"PROG2 !done\n".as_ptr());
                        }
                    }
                    close(client_fd);
                }
            }
            close(server_fd);
        }
        bpf_link__destroy(link2);
    }
    bpf_link__destroy(link1);
}

unsafe fn test_multi_prog_lookup(skel: *mut test_sk_lookup) {
    let mut tests = [
        test_multi_prog { desc: cstr!("multi prog - pass, pass"), prog1: (*skel).progs.multi_prog_pass1, prog2: (*skel).progs.multi_prog_pass2, redir_map: null_mut(), run_map: null_mut(), expect_errno: 0, listen_at: ia!(EXT_IP4, EXT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - drop, drop"), prog1: (*skel).progs.multi_prog_drop1, prog2: (*skel).progs.multi_prog_drop2, redir_map: null_mut(), run_map: null_mut(), expect_errno: ECONNREFUSED, listen_at: ia!(EXT_IP4, EXT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - pass, drop"), prog1: (*skel).progs.multi_prog_pass1, prog2: (*skel).progs.multi_prog_drop2, redir_map: null_mut(), run_map: null_mut(), expect_errno: ECONNREFUSED, listen_at: ia!(EXT_IP4, EXT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - drop, pass"), prog1: (*skel).progs.multi_prog_drop1, prog2: (*skel).progs.multi_prog_pass2, redir_map: null_mut(), run_map: null_mut(), expect_errno: ECONNREFUSED, listen_at: ia!(EXT_IP4, EXT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - pass, redir"), prog1: (*skel).progs.multi_prog_pass1, prog2: (*skel).progs.multi_prog_redir2, redir_map: null_mut(), run_map: null_mut(), expect_errno: 0, listen_at: ia!(INT_IP4, INT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - redir, pass"), prog1: (*skel).progs.multi_prog_redir1, prog2: (*skel).progs.multi_prog_pass2, redir_map: null_mut(), run_map: null_mut(), expect_errno: 0, listen_at: ia!(INT_IP4, INT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - drop, redir"), prog1: (*skel).progs.multi_prog_drop1, prog2: (*skel).progs.multi_prog_redir2, redir_map: null_mut(), run_map: null_mut(), expect_errno: 0, listen_at: ia!(INT_IP4, INT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - redir, drop"), prog1: (*skel).progs.multi_prog_redir1, prog2: (*skel).progs.multi_prog_drop2, redir_map: null_mut(), run_map: null_mut(), expect_errno: 0, listen_at: ia!(INT_IP4, INT_PORT) },
        test_multi_prog { desc: cstr!("multi prog - redir, redir"), prog1: (*skel).progs.multi_prog_redir1, prog2: (*skel).progs.multi_prog_redir2, redir_map: null_mut(), run_map: null_mut(), expect_errno: 0, listen_at: ia!(INT_IP4, INT_PORT) },
    ];
    for t in &mut tests {
        t.redir_map = (*skel).maps.redir_map;
        t.run_map = (*skel).maps.run_map;
        if test__start_subtest(t.desc) { run_multi_prog_lookup(t); }
    }
}

unsafe fn run_tests(skel: *mut test_sk_lookup) {
    if test__start_subtest(c"query lookup prog".as_ptr()) { query_lookup_prog(skel); }
    test_redirect_lookup(skel);
    test_drop_on_lookup(skel);
    test_drop_on_reuseport(skel);
    test_sk_assign_helper(skel);
    test_multi_prog_lookup(skel);
}

unsafe fn switch_netns() -> c_int {
    static SETUP0: &[u8] = b"ip -6 addr add dev lo fd00::1/128\0";
    static SETUP1: &[u8] = b"ip -6 addr add dev lo fd00::2/128\0";
    static SETUP2: &[u8] = b"ip link set dev lo up\0";
    let setup_script = [
        SETUP0.as_ptr() as *const c_char,
        SETUP1.as_ptr() as *const c_char,
        SETUP2.as_ptr() as *const c_char,
        null(),
    ];
    let mut err = unshare(CLONE_NEWNET);
    if CHECK(err != 0, c"unshare".as_ptr(), c"failed\n".as_ptr()) {
        log_err(c"unshare(CLONE_NEWNET)".as_ptr());
        return -1;
    }
    for &cmd in &setup_script {
        if cmd.is_null() { break; }
        err = system(cmd);
        if CHECK(err != 0, c"system".as_ptr(), c"failed\n".as_ptr()) {
            log_err(c"system(%s)".as_ptr(), cmd);
            return -1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn test_sk_lookup() {
    let err = switch_netns();
    if err != 0 { return; }
    let skel = test_sk_lookup__open_and_load();
    if CHECK(skel.is_null(), c"skel open_and_load".as_ptr(), c"failed\n".as_ptr()) { return; }
    run_tests(skel);
    test_sk_lookup__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
