// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// <test_progs.h>, <network_helpers.h>, <cgroup_helpers.h>,
// <linux/errqueue.h>, <poll.h>, <unistd.h>, "icmp_send.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_short, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};

const TIMEOUT_MS: c_int = 1000;

const ICMP_DEST_UNREACH: c_int = 3;
const ICMPV6_DEST_UNREACH: c_int = 1;

const ICMP_HOST_UNREACH: c_int = 1;
const ICMP_FRAG_NEEDED: c_int = 4;
const NR_ICMP_UNREACH: c_int = 15;
const ICMPV6_REJECT_ROUTE: c_int = 6;

const KFUNC_RET_UNSET: c_int = -1;

const SOCK_STREAM: c_int = 1;
const SOCK_NONBLOCK: c_int = 0o4000;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const IPPROTO_IP: c_int = 0;
const IPPROTO_IPV6: c_int = 41;
const IP_RECVERR: c_int = 11;
const IPV6_RECVERR: c_int = 25;
const SO_EE_ORIGIN_ICMP: c_int = 2;
const SO_EE_ORIGIN_ICMP6: c_int = 3;
const EINPROGRESS: c_int = 115;
const EINVAL: c_int = 22;
const ENETUNREACH: c_int = 101;
const EBUSY: c_int = 16;
const POLLERR: c_short = 0x008;
const MSG_ERRQUEUE: c_int = 0x2000;
const INADDR_LOOPBACK: c_uint = 0x7f000001;

type socklen_t = c_uint;
type ssize_t = isize;

#[repr(C)]
pub struct sockaddr {
    pub sa_family: c_ushort,
    pub sa_data: [c_char; 14],
}

type c_ushort = u16;

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: c_ushort,
    pub __data: [u8; 126],
}

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut c_void,
    pub msg_controllen: usize,
    pub msg_flags: c_int,
}

#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

#[repr(C)]
pub struct sock_extended_err {
    pub ee_errno: u32,
    pub ee_origin: u8,
    pub ee_type: u8,
    pub ee_code: u8,
    pub ee_pad: u8,
    pub ee_info: u32,
    pub ee_data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv4_hdr {
    pub version: u8,
    pub daddr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_hdr {
    pub version: u8,
    pub daddr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_hdr {
    pub dest: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv4_packet {
    pub iph: ipv4_hdr,
    pub tcp: tcp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_packet {
    pub iph: ipv6_hdr,
    pub tcp: tcp_hdr,
}

#[repr(C)]
pub union icmp_packet {
    pub v4: ipv4_packet,
    pub v6: ipv6_packet,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_size_in: c_uint,
}

#[repr(C)]
pub struct icmp_send_bss {
    pub server_port: c_int,
    pub unreach_type: c_int,
    pub unreach_code: c_int,
    pub rec_count: c_int,
}

#[repr(C)]
pub struct icmp_send_data {
    pub kfunc_ret: c_int,
    pub target_pid: c_int,
    pub rec_kfunc_rets: [c_int; 2],
}

#[repr(C)]
pub struct icmp_send_progs {
    pub egress: *mut bpf_program,
    pub recursion: *mut bpf_program,
}

#[repr(C)]
pub struct icmp_send_links {
    pub egress: *mut bpf_link,
    pub recursion: *mut bpf_link,
}

#[repr(C)]
pub struct icmp_send {
    pub bss: *mut icmp_send_bss,
    pub data: *mut icmp_send_data,
    pub progs: icmp_send_progs,
    pub links: icmp_send_links,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static pkt_v4: ipv4_packet;
    static pkt_v6: ipv6_packet;
    static in6addr_loopback: in6_addr;

    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_uint, timeout: c_int) -> c_int;
    fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn CMSG_FIRSTHDR(mhdr: *mut msghdr) -> *mut cmsghdr;
    fn CMSG_NXTHDR(mhdr: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr;
    fn CMSG_DATA(cmsg: *mut cmsghdr) -> *mut u8;
    fn ntohs(netshort: u16) -> u16;
    fn htons(hostshort: u16) -> u16;
    fn htonl(hostlong: c_uint) -> c_uint;
    fn getpid() -> c_int;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn get_socket_local_port(fd: c_int) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn icmp_send__open_and_load() -> *mut icmp_send;
    fn icmp_send__destroy(obj: *mut icmp_send);
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn setup_cgroup_environment() -> c_int;
    fn get_root_cgroup() -> c_int;
    fn cleanup_cgroup_environment();

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE_ssize(actual: ssize_t, expected: ssize_t, name: *const c_char) -> bool;
    fn ASSERT_NEQ_ptr(actual: *const c_void, expected: *const c_void, name: *const c_char)
        -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ_u8(actual: u8, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_FAIL(name: *const c_char);
}

unsafe fn connect_to_fd_nonblock(server_fd: c_int) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let mut len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let mut on: c_int = 1;

    if getsockname(
        server_fd,
        addr_of_mut!(addr).cast::<sockaddr>(),
        addr_of_mut!(len),
    ) != 0
    {
        return -1;
    }

    let fd = socket(addr.ss_family as c_int, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if fd < 0 {
        return -1;
    }

    if addr.ss_family as c_int == AF_INET6
        && setsockopt(
            fd,
            IPPROTO_IPV6,
            IPV6_RECVERR,
            addr_of_mut!(on).cast::<c_void>(),
            size_of::<c_int>() as socklen_t,
        ) < 0
    {
        close(fd);
        return -1;
    }

    let err = connect(fd, addr_of_mut!(addr).cast::<sockaddr>(), len);
    if err < 0 && errno != EINPROGRESS {
        close(fd);
        return -1;
    }

    fd
}

unsafe fn read_icmp_errqueue(sockfd: c_int, expected_code: c_int, af: c_int) {
    let expected_ee_type = if af == AF_INET {
        ICMP_DEST_UNREACH
    } else {
        ICMPV6_DEST_UNREACH
    };
    let expected_origin = if af == AF_INET {
        SO_EE_ORIGIN_ICMP
    } else {
        SO_EE_ORIGIN_ICMP6
    };
    let expected_level = if af == AF_INET {
        IPPROTO_IP
    } else {
        IPPROTO_IPV6
    };
    let expected_type = if af == AF_INET {
        IP_RECVERR
    } else {
        IPV6_RECVERR
    };
    let mut ctrl_buf = [0 as c_char; 512];
    let mut msg: msghdr = zeroed();
    msg.msg_control = ctrl_buf.as_mut_ptr().cast::<c_void>();
    msg.msg_controllen = size_of::<[c_char; 512]>();
    let mut pfd = pollfd {
        fd: sockfd,
        events: POLLERR,
        revents: 0,
    };

    if !ASSERT_GE(poll(addr_of_mut!(pfd), 1, TIMEOUT_MS), 1, c"poll_errqueue".as_ptr()) {
        return;
    }

    let n = recvmsg(sockfd, addr_of_mut!(msg), MSG_ERRQUEUE);
    if !ASSERT_GE_ssize(n, 0, c"recvmsg_errqueue".as_ptr()) {
        return;
    }

    let mut cm = CMSG_FIRSTHDR(addr_of_mut!(msg));
    if !ASSERT_NEQ_ptr(
        cm.cast::<c_void>(),
        null_mut(),
        c"cm_firsthdr_null".as_ptr(),
    ) {
        return;
    }

    while !cm.is_null() {
        if (*cm).cmsg_level != expected_level || (*cm).cmsg_type != expected_type {
            cm = CMSG_NXTHDR(addr_of_mut!(msg), cm);
            continue;
        }

        let sock_err = CMSG_DATA(cm).cast::<sock_extended_err>();

        if !ASSERT_EQ_u8(
            (*sock_err).ee_origin,
            expected_origin,
            c"sock_err_origin".as_ptr(),
        ) {
            return;
        }
        if !ASSERT_EQ_u8(
            (*sock_err).ee_type,
            expected_ee_type,
            c"sock_err_type_dest_unreach".as_ptr(),
        ) {
            return;
        }
        ASSERT_EQ(
            (*sock_err).ee_code as c_int,
            expected_code,
            c"sock_err_code".as_ptr(),
        );
        return;
    }

    ASSERT_FAIL(c"no IP_RECVERR/IPV6_RECVERR control message found".as_ptr());
}

unsafe fn valid_unreach_code(code: c_int, af: c_int) -> bool {
    if code < 0 {
        return false;
    }

    if af == AF_INET {
        return code <= NR_ICMP_UNREACH && code != ICMP_FRAG_NEEDED;
    }

    code <= ICMPV6_REJECT_ROUTE
}

unsafe fn trigger_prog_read_icmp_errqueue(
    skel: *mut icmp_send,
    code: c_int,
    af: c_int,
    ip: *const c_char,
) {
    let mut srv_fd: c_int = -1;
    let mut client_fd: c_int;

    srv_fd = start_server(af, SOCK_STREAM, ip, 0, TIMEOUT_MS);
    if !ASSERT_OK_FD(srv_fd, c"start_server".as_ptr()) {
        return;
    }

    let port = get_socket_local_port(srv_fd);
    if !ASSERT_GE(port, 0, c"get_socket_local_port".as_ptr()) {
        close(srv_fd);
        return;
    }

    (*(*skel).bss).server_port = ntohs(port as u16) as c_int;
    (*(*skel).bss).unreach_type = if af == AF_INET {
        ICMP_DEST_UNREACH
    } else {
        ICMPV6_DEST_UNREACH
    };
    (*(*skel).bss).unreach_code = code;
    (*(*skel).data).kfunc_ret = KFUNC_RET_UNSET;

    client_fd = connect_to_fd_nonblock(srv_fd);
    if !ASSERT_OK_FD(client_fd, c"client_connect_nonblock".as_ptr()) {
        close(srv_fd);
        return;
    }

    if valid_unreach_code(code, af) {
        read_icmp_errqueue(client_fd, code, af);
    }

    close(client_fd);
    close(srv_fd);
}

unsafe fn run_icmp_test(
    skel: *mut icmp_send,
    af: c_int,
    ip: *const c_char,
    max_code: c_int,
) {
    let mut code = 0;
    while code <= max_code {
        if af == AF_INET && code == ICMP_FRAG_NEEDED {
            code += 1;
            continue;
        }

        trigger_prog_read_icmp_errqueue(skel, code, af, ip);
        ASSERT_EQ((*(*skel).data).kfunc_ret, 0, c"kfunc_ret".as_ptr());
        code += 1;
    }

    /* Test invalid codes */
    trigger_prog_read_icmp_errqueue(skel, -1, af, ip);
    ASSERT_EQ((*(*skel).data).kfunc_ret, -EINVAL, c"kfunc_ret".as_ptr());

    trigger_prog_read_icmp_errqueue(skel, max_code + 1, af, ip);
    ASSERT_EQ((*(*skel).data).kfunc_ret, -EINVAL, c"kfunc_ret".as_ptr());

    if af == AF_INET {
        trigger_prog_read_icmp_errqueue(skel, ICMP_FRAG_NEEDED, af, ip);
        ASSERT_EQ((*(*skel).data).kfunc_ret, -EINVAL, c"kfunc_ret".as_ptr());
    }
}

unsafe fn run_icmp_no_route_test(skel: *mut icmp_send, af: c_int) {
    let mut pkt: icmp_packet = zeroed();
    let mut opts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: addr_of_mut!(pkt).cast::<c_void>(),
        data_size_in: 0,
    };
    let err: c_int;

    match af {
        AF_INET => {
            pkt.v4 = pkt_v4;
            pkt.v4.iph.version = 4;
            pkt.v4.iph.daddr = htonl(INADDR_LOOPBACK);
            pkt.v4.tcp.dest = htons(80);
            opts.data_size_in = size_of::<ipv4_packet>() as c_uint;
            (*(*skel).bss).unreach_type = ICMP_DEST_UNREACH;
        }
        AF_INET6 => {
            pkt.v6 = pkt_v6;
            pkt.v6.iph.version = 6;
            pkt.v6.iph.daddr = in6addr_loopback;
            pkt.v6.tcp.dest = htons(80);
            opts.data_size_in = size_of::<ipv6_packet>() as c_uint;
            (*(*skel).bss).unreach_type = ICMPV6_DEST_UNREACH;
        }
        _ => {
            ASSERT_FAIL(c"af_not_supported".as_ptr());
            return;
        }
    }

    (*(*skel).bss).server_port = 80;
    (*(*skel).data).kfunc_ret = KFUNC_RET_UNSET;

    err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.egress), addr_of_mut!(opts));
    if !ASSERT_OK(err, c"test_run".as_ptr()) {
        return;
    }

    ASSERT_EQ(
        (*(*skel).data).kfunc_ret,
        -ENETUNREACH,
        c"kfunc_ret_no_route".as_ptr(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn test_icmp_send_unreach_cgroup() {
    let mut skel: *mut icmp_send;
    let mut cgroup_fd: c_int = -1;

    skel = icmp_send__open_and_load();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"skel_open".as_ptr()) {
        goto_cleanup_icmp_send_unreach_cgroup(skel, cgroup_fd);
        return;
    }

    cgroup_fd = test__join_cgroup(c"/icmp_send_unreach_cgroup".as_ptr());
    if !ASSERT_OK_FD(cgroup_fd, c"join_cgroup".as_ptr()) {
        goto_cleanup_icmp_send_unreach_cgroup(skel, cgroup_fd);
        return;
    }

    (*skel).links.egress = bpf_program__attach_cgroup((*skel).progs.egress, cgroup_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.egress.cast::<c_void>(),
        c"prog_attach_cgroup".as_ptr(),
    ) {
        goto_cleanup_icmp_send_unreach_cgroup(skel, cgroup_fd);
        return;
    }

    if test__start_subtest(c"ipv4".as_ptr()) {
        run_icmp_test(skel, AF_INET, c"127.0.0.1".as_ptr(), NR_ICMP_UNREACH);
    }

    if test__start_subtest(c"ipv6".as_ptr()) {
        run_icmp_test(skel, AF_INET6, c"::1".as_ptr(), ICMPV6_REJECT_ROUTE);
    }

    if test__start_subtest(c"no_route_ipv4".as_ptr()) {
        run_icmp_no_route_test(skel, AF_INET);
    }

    if test__start_subtest(c"no_route_ipv6".as_ptr()) {
        run_icmp_no_route_test(skel, AF_INET6);
    }

    goto_cleanup_icmp_send_unreach_cgroup(skel, cgroup_fd);
}

unsafe fn goto_cleanup_icmp_send_unreach_cgroup(skel: *mut icmp_send, cgroup_fd: c_int) {
    icmp_send__destroy(skel);
    if cgroup_fd >= 0 {
        close(cgroup_fd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_icmp_send_unreach_recursion() {
    let mut skel: *mut icmp_send = null_mut();
    let mut cgroup_fd: c_int = -1;
    let mut err: c_int;

    err = setup_cgroup_environment();
    if !ASSERT_OK(err, c"setup_cgroup_environment".as_ptr()) {
        return;
    }

    skel = icmp_send__open_and_load();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"skel_open".as_ptr()) {
        goto_cleanup_icmp_send_unreach_recursion(skel, cgroup_fd);
        return;
    }

    cgroup_fd = get_root_cgroup();
    if !ASSERT_OK_FD(cgroup_fd, c"get_root_cgroup".as_ptr()) {
        goto_cleanup_icmp_send_unreach_recursion(skel, cgroup_fd);
        return;
    }

    (*(*skel).data).target_pid = getpid();
    (*skel).links.recursion = bpf_program__attach_cgroup((*skel).progs.recursion, cgroup_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.recursion.cast::<c_void>(),
        c"prog_attach_cgroup".as_ptr(),
    ) {
        goto_cleanup_icmp_send_unreach_recursion(skel, cgroup_fd);
        return;
    }

    trigger_prog_read_icmp_errqueue(skel, ICMP_HOST_UNREACH, AF_INET, c"127.0.0.1".as_ptr());

    /*
     * Because there's recursion involved, the first call will return at
     * index 1 since it will return the second, and the second call will
     * return at index 0 since it will return the first.
     */
    ASSERT_EQ((*(*skel).bss).rec_count, 2, c"rec_count".as_ptr());
    ASSERT_EQ(
        (*(*skel).data).rec_kfunc_rets[0],
        -EBUSY,
        c"kfunc_rets[0]".as_ptr(),
    );
    ASSERT_EQ(
        (*(*skel).data).rec_kfunc_rets[1],
        0,
        c"kfunc_rets[1]".as_ptr(),
    );

    goto_cleanup_icmp_send_unreach_recursion(skel, cgroup_fd);
}

unsafe fn goto_cleanup_icmp_send_unreach_recursion(skel: *mut icmp_send, cgroup_fd: c_int) {
    icmp_send__destroy(skel);
    if cgroup_fd >= 0 {
        close(cgroup_fd);
    }
    cleanup_cgroup_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
