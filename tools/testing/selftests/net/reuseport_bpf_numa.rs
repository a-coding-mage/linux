// SPDX-License-Identifier: GPL-2.0
/*
 * Test functionality of BPF filters with SO_REUSEPORT. Same test as
 * in reuseport_bpf_cpu, only as one socket per NUMA node.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

const PORT: c_int = 8888;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
const INADDR_ANY: u32 = 0x00000000;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const EPOLLIN: u32 = 0x001;
const EPOLL_CTL_ADD: c_int = 1;
const CLONE_NEWNET: c_int = 0x40000000;
const __NR_bpf: c_long = 321;
const BPF_PROG_LOAD: c_int = 5;
const BPF_PROG_TYPE_SOCKET_FILTER: u32 = 1;
const BPF_FUNC_get_numa_node_id: i32 = 42;
const BPF_JMP: u16 = 0x05;
const BPF_CALL: u16 = 0x80;
const BPF_EXIT: u16 = 0x90;

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type sa_family_t = u16;
type in_port_t = u16;
type in_addr_t = u32;

#[repr(C)]
struct in_addr {
    s_addr: in_addr_t,
}

#[repr(C)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: sa_family_t,
    sin_port: in_port_t,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct sockaddr_in6 {
    sin6_family: sa_family_t,
    sin6_port: in_port_t,
    sin6_flowinfo: u32,
    sin6_addr: in6_addr,
    sin6_scope_id: u32,
}

#[repr(C, align(8))]
struct sockaddr_storage {
    ss_family: sa_family_t,
    __ss_padding: [u8; 118],
    __ss_align: c_ulong,
}

#[repr(C)]
struct epoll_data {
    fd: c_int,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

#[repr(C)]
struct bpf_insn {
    code: u8,
    dst_src: u8,
    off: i16,
    imm: i32,
}

impl bpf_insn {
    const fn new(code: u16, dst_reg: u8, src_reg: u8, off: i16, imm: i32) -> Self {
        Self {
            code: code as u8,
            dst_src: (dst_reg & 0x0f) | ((src_reg & 0x0f) << 4),
            off,
            imm,
        }
    }
}

#[repr(C)]
struct bpf_attr {
    prog_type: u32,
    insn_cnt: u32,
    insns: u64,
    license: u64,
    log_level: u32,
    log_size: u32,
    log_buf: u64,
    kern_version: u32,
    prog_flags: u32,
    prog_name: [u8; 16],
    prog_ifindex: u32,
    expected_attach_type: u32,
}

#[repr(C)]
struct bitmask {
    size: c_ulong,
    maskp: *mut c_ulong,
}

unsafe extern "C" {
    static mut errno: c_int;
    static in6addr_any: in6_addr;
    static in6addr_loopback: in6_addr;
    static mut numa_nodes_ptr: *mut bitmask;

    fn htonl(hostlong: u32) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn numa_allocate_cpumask() -> *mut bitmask;
    fn numa_node_to_cpus(node: c_int, mask: *mut bitmask) -> c_int;
    fn numa_bitmask_weight(mask: *const bitmask) -> c_uint;
    fn numa_bitmask_free(mask: *mut bitmask);
    fn numa_run_on_node(node: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int)
        -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn numa_available() -> c_int;
    fn numa_max_node() -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn numa_bitmask_isbitset(mask: *const bitmask, n: c_uint) -> c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn ksft_exit_skip(msg: *const c_char, ...);
}

unsafe fn build_rcv_group(rcv_fd: *mut c_int, len: size_t, family: c_int, proto: c_int) {
    let mut addr: sockaddr_storage = mem::zeroed();
    let mut i: size_t;
    let mut opt: c_int;

    match family {
        AF_INET => {
            let addr4 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in;
            (*addr4).sin_family = AF_INET as sa_family_t;
            (*addr4).sin_addr.s_addr = htonl(INADDR_ANY);
            (*addr4).sin_port = htons(PORT as u16);
        }
        AF_INET6 => {
            let addr6 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in6;
            (*addr6).sin6_family = AF_INET6 as sa_family_t;
            (*addr6).sin6_addr = in6addr_any;
            (*addr6).sin6_port = htons(PORT as u16);
        }
        _ => error(1, 0, c"Unsupported family %d".as_ptr(), family),
    }

    i = 0;
    while i < len {
        *rcv_fd.add(i) = socket(family, proto, 0);
        if *rcv_fd.add(i) < 0 {
            error(1, errno, c"failed to create receive socket".as_ptr());
        }

        opt = 1;
        if setsockopt(
            *rcv_fd.add(i),
            SOL_SOCKET,
            SO_REUSEPORT,
            &opt as *const c_int as *const c_void,
            mem::size_of_val(&opt) as socklen_t,
        ) != 0
        {
            error(1, errno, c"failed to set SO_REUSEPORT".as_ptr());
        }

        if bind(
            *rcv_fd.add(i),
            &addr as *const sockaddr_storage as *const sockaddr,
            mem::size_of_val(&addr) as socklen_t,
        ) != 0
        {
            error(1, errno, c"failed to bind receive socket".as_ptr());
        }

        if proto == SOCK_STREAM && listen(*rcv_fd.add(i), (len * 10) as c_int) != 0 {
            error(1, errno, c"failed to listen on receive port".as_ptr());
        }

        i += 1;
    }
}

unsafe fn attach_bpf(fd: c_int) {
    static mut BPF_LOG_BUF: [c_char; 65536] = [0; 65536];
    static BPF_LICENSE: [c_char; 1] = [0];

    let prog = [
        /* R0 = bpf_get_numa_node_id() */
        bpf_insn::new(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_get_numa_node_id),
        /* return R0 */
        bpf_insn::new(BPF_JMP | BPF_EXIT, 0, 0, 0, 0),
    ];
    let mut attr: bpf_attr = mem::zeroed();

    attr.prog_type = BPF_PROG_TYPE_SOCKET_FILTER;
    attr.insn_cnt = prog.len() as u32;
    attr.insns = prog.as_ptr() as c_ulong as u64;
    attr.license = BPF_LICENSE.as_ptr() as c_ulong as u64;
    attr.log_buf = BPF_LOG_BUF.as_mut_ptr() as c_ulong as u64;
    attr.log_size = mem::size_of_val(&BPF_LOG_BUF) as u32;
    attr.log_level = 1;

    let bpf_fd = syscall(
        __NR_bpf,
        BPF_PROG_LOAD,
        &mut attr as *mut bpf_attr,
        mem::size_of_val(&attr),
    ) as c_int;
    if bpf_fd < 0 {
        error(
            1,
            errno,
            c"ebpf error. log:\n%s\n".as_ptr(),
            BPF_LOG_BUF.as_ptr(),
        );
    }

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ATTACH_REUSEPORT_EBPF,
        &bpf_fd as *const c_int as *const c_void,
        mem::size_of_val(&bpf_fd) as socklen_t,
    ) != 0
    {
        error(
            1,
            errno,
            c"failed to set SO_ATTACH_REUSEPORT_EBPF".as_ptr(),
        );
    }

    close(bpf_fd);
}

/*
 * Return true if it is a cpuless node. Return false if it isn't or any
 * error (very unlikely) happens during the libnuma calls.
 */
unsafe fn is_cpuless_node(node_id: c_int) -> bool {
    let cpumask: *mut bitmask;
    let mut ret = false;

    cpumask = numa_allocate_cpumask();
    if cpumask.is_null() {
        return ret;
    }

    if numa_node_to_cpus(node_id, cpumask) == 0 && numa_bitmask_weight(cpumask) == 0 {
        ret = true;
    }

    numa_bitmask_free(cpumask);
    ret
}

unsafe fn send_from_node(node_id: c_int, family: c_int, proto: c_int) {
    let mut saddr: sockaddr_storage = mem::zeroed();
    let mut daddr: sockaddr_storage = mem::zeroed();
    let fd: c_int;

    match family {
        AF_INET => {
            let saddr4 = &mut saddr as *mut sockaddr_storage as *mut sockaddr_in;
            (*saddr4).sin_family = AF_INET as sa_family_t;
            (*saddr4).sin_addr.s_addr = htonl(INADDR_ANY);
            (*saddr4).sin_port = 0;

            let daddr4 = &mut daddr as *mut sockaddr_storage as *mut sockaddr_in;
            (*daddr4).sin_family = AF_INET as sa_family_t;
            (*daddr4).sin_addr.s_addr = htonl(INADDR_LOOPBACK);
            (*daddr4).sin_port = htons(PORT as u16);
        }
        AF_INET6 => {
            let saddr6 = &mut saddr as *mut sockaddr_storage as *mut sockaddr_in6;
            (*saddr6).sin6_family = AF_INET6 as sa_family_t;
            (*saddr6).sin6_addr = in6addr_any;
            (*saddr6).sin6_port = 0;

            let daddr6 = &mut daddr as *mut sockaddr_storage as *mut sockaddr_in6;
            (*daddr6).sin6_family = AF_INET6 as sa_family_t;
            (*daddr6).sin6_addr = in6addr_loopback;
            (*daddr6).sin6_port = htons(PORT as u16);
        }
        _ => error(1, 0, c"Unsupported family %d".as_ptr(), family),
    }

    if numa_run_on_node(node_id) < 0 {
        error(1, errno, c"failed to pin to node".as_ptr());
    }

    fd = socket(family, proto, 0);
    if fd < 0 {
        error(1, errno, c"failed to create send socket".as_ptr());
    }

    if bind(
        fd,
        &saddr as *const sockaddr_storage as *const sockaddr,
        mem::size_of_val(&saddr) as socklen_t,
    ) != 0
    {
        error(1, errno, c"failed to bind send socket".as_ptr());
    }

    if connect(
        fd,
        &daddr as *const sockaddr_storage as *const sockaddr,
        mem::size_of_val(&daddr) as socklen_t,
    ) != 0
    {
        error(1, errno, c"failed to connect send socket".as_ptr());
    }

    if send(fd, c"a".as_ptr() as *const c_void, 1, 0) < 0 {
        error(1, errno, c"failed to send message".as_ptr());
    }

    close(fd);
}

unsafe fn receive_on_node(
    rcv_fd: *mut c_int,
    len: c_int,
    epfd: c_int,
    node_id: c_int,
    proto: c_int,
) {
    let mut ev: epoll_event = mem::zeroed();
    let mut i: c_int;
    let fd: c_int;
    let mut buf = [0 as c_char; 8];

    i = epoll_wait(epfd, &mut ev, 1, -1);
    if i < 0 {
        error(1, errno, c"epoll_wait failed".as_ptr());
    }

    if proto == SOCK_STREAM {
        fd = accept(ev.data.fd, ptr::null_mut(), ptr::null_mut());
        if fd < 0 {
            error(1, errno, c"failed to accept".as_ptr());
        }
        i = recv(fd, buf.as_mut_ptr() as *mut c_void, mem::size_of_val(&buf), 0) as c_int;
        close(fd);
    } else {
        i = recv(
            ev.data.fd,
            buf.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&buf),
            0,
        ) as c_int;
    }

    if i < 0 {
        error(1, errno, c"failed to recv".as_ptr());
    }

    i = 0;
    while i < len {
        if ev.data.fd == *rcv_fd.add(i as usize) {
            break;
        }
        i += 1;
    }
    if i == len {
        error(1, 0, c"failed to find socket".as_ptr());
    }
    fprintf(
        stderr,
        c"send node %d, receive socket %d\n".as_ptr(),
        node_id,
        i,
    );
    if node_id != i {
        error(1, 0, c"node id/receive socket mismatch".as_ptr());
    }
}

unsafe fn test(rcv_fd: *mut c_int, len: c_int, family: c_int, proto: c_int) {
    let mut ev: epoll_event = mem::zeroed();
    let epfd: c_int;
    let mut node: c_int;

    build_rcv_group(rcv_fd, len as size_t, family, proto);
    attach_bpf(*rcv_fd.add(0));

    epfd = epoll_create(1);
    if epfd < 0 {
        error(1, errno, c"failed to create epoll".as_ptr());
    }
    node = 0;
    while node < len {
        ev.events = EPOLLIN;
        ev.data.fd = *rcv_fd.add(node as usize);
        if epoll_ctl(epfd, EPOLL_CTL_ADD, *rcv_fd.add(node as usize), &mut ev) != 0 {
            error(1, errno, c"failed to register sock epoll".as_ptr());
        }
        node += 1;
    }

    /* Forward iterate */
    node = 0;
    while node < len {
        if numa_bitmask_isbitset(numa_nodes_ptr, node as c_uint) == 0 {
            node += 1;
            continue;
        }
        if is_cpuless_node(node) {
            node += 1;
            continue;
        }
        send_from_node(node, family, proto);
        receive_on_node(rcv_fd, len, epfd, node, proto);
        node += 1;
    }

    /* Reverse iterate */
    node = len - 1;
    while node >= 0 {
        if numa_bitmask_isbitset(numa_nodes_ptr, node as c_uint) == 0 {
            node -= 1;
            continue;
        }
        if is_cpuless_node(node) {
            node -= 1;
            continue;
        }
        send_from_node(node, family, proto);
        receive_on_node(rcv_fd, len, epfd, node, proto);
        node -= 1;
    }

    close(epfd);
    node = 0;
    while node < len {
        close(*rcv_fd.add(node as usize));
        node += 1;
    }
}

unsafe fn setup_netns() {
    if unshare(CLONE_NEWNET) != 0 {
        error(1, errno, c"failed to unshare netns".as_ptr());
    }
    if system(c"ip link set lo up".as_ptr()) != 0 {
        error(
            1,
            0,
            c"failed to bring up lo interface in netns".as_ptr(),
        );
    }
}

fn main() -> c_int {
    unsafe {
        let rcv_fd: *mut c_int;
        let nodes: c_int;

        setup_netns();

        if numa_available() < 0 {
            ksft_exit_skip(c"no numa api support\n".as_ptr());
        }

        nodes = numa_max_node() + 1;

        rcv_fd = calloc(nodes as size_t, mem::size_of::<c_int>()) as *mut c_int;
        if rcv_fd.is_null() {
            error(1, 0, c"failed to allocate array".as_ptr());
        }

        fprintf(stderr, c"---- IPv4 UDP ----\n".as_ptr());
        test(rcv_fd, nodes, AF_INET, SOCK_DGRAM);

        fprintf(stderr, c"---- IPv6 UDP ----\n".as_ptr());
        test(rcv_fd, nodes, AF_INET6, SOCK_DGRAM);

        fprintf(stderr, c"---- IPv4 TCP ----\n".as_ptr());
        test(rcv_fd, nodes, AF_INET, SOCK_STREAM);

        fprintf(stderr, c"---- IPv6 TCP ----\n".as_ptr());
        test(rcv_fd, nodes, AF_INET6, SOCK_STREAM);

        free(rcv_fd as *mut c_void);

        fprintf(stderr, c"SUCCESS\n".as_ptr());
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
