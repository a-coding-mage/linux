// SPDX-License-Identifier: GPL-2.0
/*
 * Test functionality of BPF filters with SO_REUSEPORT.  This program creates
 * an SO_REUSEPORT receiver group containing one socket per CPU core. It then
 * creates a BPF program that will select a socket from this group based
 * on the core id that receives the packet.  The sending code artificially
 * moves itself to run on different core ids and sends one message from
 * each core.  Since these packets are delivered over loopback, they should
 * arrive on the same core that sent them.  The receiving code then ensures
 * that the packet was received on the socket for the corresponding core id.
 * This entire process is done for several different core id permutations
 * and for each IPv4/IPv6 and TCP/UDP combination.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type socklen_t = u32;
type sa_family_t = u16;
type in_port_t = u16;
type in_addr_t = u32;
type __u16 = u16;
type __u32 = u32;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const SO_ATTACH_REUSEPORT_CBPF: c_int = 51;
const EPOLLIN: u32 = 0x00000001;
const EPOLL_CTL_ADD: c_int = 1;
const INADDR_ANY: u32 = 0x00000000;
const INADDR_LOOPBACK: u32 = 0x7f000001;
const CLONE_NEWNET: c_int = 0x40000000;
const _SC_NPROCESSORS_ONLN: c_int = 84;

const BPF_LD: u16 = 0x00;
const BPF_W: u16 = 0x00;
const BPF_ABS: u16 = 0x20;
const BPF_RET: u16 = 0x06;
const BPF_A: u16 = 0x10;
const SKF_AD_OFF: u32 = 0xfffff000;
const SKF_AD_CPU: u32 = 36;

static PORT: c_int = 8888;

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
struct sock_filter {
    code: __u16,
    jt: u8,
    jf: u8,
    k: __u32,
}

#[repr(C)]
struct sock_fprog {
    len: u16,
    filter: *mut sock_filter,
}

#[repr(C)]
union epoll_data {
    ptr: *mut c_void,
    fd: c_int,
    u32_: u32,
    u64_: u64,
}

#[repr(C)]
struct epoll_event {
    events: u32,
    data: epoll_data,
}

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; 1024 / (8 * size_of::<c_ulong>())],
}

extern "C" {
    static in6addr_any: in6_addr;
    static in6addr_loopback: in6_addr;

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
    fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    fn send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> isize;
    fn close(fd: c_int) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int) -> c_int;
    fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    fn recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> isize;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn sysconf(name: c_int) -> isize;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn __errno_location() -> *mut c_int;
    fn error(status: c_int, errnum: c_int, format: *const c_char, ...);
}

extern "C" {
    static mut stderr: *mut c_void;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn CPU_SET(cpu: c_int, cpusetp: *mut cpu_set_t) {
    let cpu = cpu as usize;
    let bits_per_word = 8 * size_of::<c_ulong>();
    (*cpusetp).__bits[cpu / bits_per_word] |= (1 as c_ulong) << (cpu % bits_per_word);
}

unsafe fn build_rcv_group(rcv_fd: *mut c_int, len: size_t, family: c_int, proto: c_int) {
    let mut addr: sockaddr_storage = zeroed();
    let addr4: *mut sockaddr_in;
    let addr6: *mut sockaddr_in6;
    let mut i: size_t;
    let mut opt: c_int;

    match family {
        AF_INET => {
            addr4 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in;
            (*addr4).sin_family = AF_INET as sa_family_t;
            (*addr4).sin_addr.s_addr = htonl(INADDR_ANY);
            (*addr4).sin_port = htons(PORT as u16);
        }
        AF_INET6 => {
            addr6 = &mut addr as *mut sockaddr_storage as *mut sockaddr_in6;
            (*addr6).sin6_family = AF_INET6 as sa_family_t;
            (*addr6).sin6_addr = in6addr_any;
            (*addr6).sin6_port = htons(PORT as u16);
        }
        _ => {
            error(1, 0, b"Unsupported family %d\0".as_ptr() as *const c_char, family);
        }
    }

    i = 0;
    while i < len {
        *rcv_fd.add(i) = socket(family, proto, 0);
        if *rcv_fd.add(i) < 0 {
            error(1, errno(), b"failed to create receive socket\0".as_ptr() as *const c_char);
        }

        opt = 1;
        if setsockopt(
            *rcv_fd.add(i),
            SOL_SOCKET,
            SO_REUSEPORT,
            &opt as *const c_int as *const c_void,
            size_of::<c_int>() as socklen_t,
        ) != 0
        {
            error(1, errno(), b"failed to set SO_REUSEPORT\0".as_ptr() as *const c_char);
        }

        if bind(
            *rcv_fd.add(i),
            &addr as *const sockaddr_storage as *const sockaddr,
            size_of::<sockaddr_storage>() as socklen_t,
        ) != 0
        {
            error(1, errno(), b"failed to bind receive socket\0".as_ptr() as *const c_char);
        }

        if proto == SOCK_STREAM && listen(*rcv_fd.add(i), (len * 10) as c_int) != 0 {
            error(1, errno(), b"failed to listen on receive port\0".as_ptr() as *const c_char);
        }
        i += 1;
    }
}

unsafe fn attach_bpf(fd: c_int) {
    let mut code = [
        sock_filter {
            /* A = raw_smp_processor_id() */
            code: BPF_LD | BPF_W | BPF_ABS,
            jt: 0,
            jf: 0,
            k: SKF_AD_OFF + SKF_AD_CPU,
        },
        sock_filter {
            /* return A */
            code: BPF_RET | BPF_A,
            jt: 0,
            jf: 0,
            k: 0,
        },
    ];
    let mut p = sock_fprog {
        len: 2,
        filter: code.as_mut_ptr(),
    };

    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_ATTACH_REUSEPORT_CBPF,
        &mut p as *mut sock_fprog as *const c_void,
        size_of::<sock_fprog>() as socklen_t,
    ) != 0
    {
        error(1, errno(), b"failed to set SO_ATTACH_REUSEPORT_CBPF\0".as_ptr() as *const c_char);
    }
}

unsafe fn send_from_cpu(cpu_id: c_int, family: c_int, proto: c_int) {
    let mut saddr: sockaddr_storage = zeroed();
    let mut daddr: sockaddr_storage = zeroed();
    let saddr4: *mut sockaddr_in;
    let daddr4: *mut sockaddr_in;
    let saddr6: *mut sockaddr_in6;
    let daddr6: *mut sockaddr_in6;
    let mut cpu_set: cpu_set_t;
    let fd: c_int;

    match family {
        AF_INET => {
            saddr4 = &mut saddr as *mut sockaddr_storage as *mut sockaddr_in;
            (*saddr4).sin_family = AF_INET as sa_family_t;
            (*saddr4).sin_addr.s_addr = htonl(INADDR_ANY);
            (*saddr4).sin_port = 0;

            daddr4 = &mut daddr as *mut sockaddr_storage as *mut sockaddr_in;
            (*daddr4).sin_family = AF_INET as sa_family_t;
            (*daddr4).sin_addr.s_addr = htonl(INADDR_LOOPBACK);
            (*daddr4).sin_port = htons(PORT as u16);
        }
        AF_INET6 => {
            saddr6 = &mut saddr as *mut sockaddr_storage as *mut sockaddr_in6;
            (*saddr6).sin6_family = AF_INET6 as sa_family_t;
            (*saddr6).sin6_addr = in6addr_any;
            (*saddr6).sin6_port = 0;

            daddr6 = &mut daddr as *mut sockaddr_storage as *mut sockaddr_in6;
            (*daddr6).sin6_family = AF_INET6 as sa_family_t;
            (*daddr6).sin6_addr = in6addr_loopback;
            (*daddr6).sin6_port = htons(PORT as u16);
        }
        _ => {
            error(1, 0, b"Unsupported family %d\0".as_ptr() as *const c_char, family);
        }
    }

    cpu_set = zeroed();
    CPU_SET(cpu_id, &mut cpu_set);
    if sched_setaffinity(0, size_of::<cpu_set_t>(), &cpu_set) < 0 {
        error(1, errno(), b"failed to pin to cpu\0".as_ptr() as *const c_char);
    }

    fd = socket(family, proto, 0);
    if fd < 0 {
        error(1, errno(), b"failed to create send socket\0".as_ptr() as *const c_char);
    }

    if bind(
        fd,
        &saddr as *const sockaddr_storage as *const sockaddr,
        size_of::<sockaddr_storage>() as socklen_t,
    ) != 0
    {
        error(1, errno(), b"failed to bind send socket\0".as_ptr() as *const c_char);
    }

    if connect(
        fd,
        &daddr as *const sockaddr_storage as *const sockaddr,
        size_of::<sockaddr_storage>() as socklen_t,
    ) != 0
    {
        error(1, errno(), b"failed to connect send socket\0".as_ptr() as *const c_char);
    }

    if send(fd, b"a\0".as_ptr() as *const c_void, 1, 0) < 0 {
        error(1, errno(), b"failed to send message\0".as_ptr() as *const c_char);
    }

    close(fd);
}

unsafe fn receive_on_cpu(rcv_fd: *mut c_int, len: c_int, epfd: c_int, cpu_id: c_int, proto: c_int) {
    let mut ev: epoll_event = zeroed();
    let mut i: c_int;
    let fd: c_int;
    let mut buf = [0 as c_char; 8];

    i = epoll_wait(epfd, &mut ev, 1, -1);
    if i < 0 {
        error(1, errno(), b"epoll_wait failed\0".as_ptr() as *const c_char);
    }

    if proto == SOCK_STREAM {
        fd = accept(ev.data.fd, ptr::null_mut(), ptr::null_mut());
        if fd < 0 {
            error(1, errno(), b"failed to accept\0".as_ptr() as *const c_char);
        }
        i = recv(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 8]>(), 0) as c_int;
        close(fd);
    } else {
        i = recv(
            ev.data.fd,
            buf.as_mut_ptr() as *mut c_void,
            size_of::<[c_char; 8]>(),
            0,
        ) as c_int;
    }

    if i < 0 {
        error(1, errno(), b"failed to recv\0".as_ptr() as *const c_char);
    }

    i = 0;
    while i < len {
        if ev.data.fd == *rcv_fd.add(i as usize) {
            break;
        }
        i += 1;
    }
    if i == len {
        error(1, 0, b"failed to find socket\0".as_ptr() as *const c_char);
    }
    fprintf(
        stderr,
        b"send cpu %d, receive socket %d\n\0".as_ptr() as *const c_char,
        cpu_id,
        i,
    );
    if cpu_id != i {
        error(1, 0, b"cpu id/receive socket mismatch\0".as_ptr() as *const c_char);
    }
}

unsafe fn test(rcv_fd: *mut c_int, len: c_int, family: c_int, proto: c_int) {
    let mut ev: epoll_event = zeroed();
    let epfd: c_int;
    let mut cpu: c_int;

    build_rcv_group(rcv_fd, len as size_t, family, proto);
    attach_bpf(*rcv_fd.add(0));

    epfd = epoll_create(1);
    if epfd < 0 {
        error(1, errno(), b"failed to create epoll\0".as_ptr() as *const c_char);
    }
    cpu = 0;
    while cpu < len {
        ev.events = EPOLLIN;
        ev.data.fd = *rcv_fd.add(cpu as usize);
        if epoll_ctl(epfd, EPOLL_CTL_ADD, *rcv_fd.add(cpu as usize), &mut ev) != 0 {
            error(1, errno(), b"failed to register sock epoll\0".as_ptr() as *const c_char);
        }
        cpu += 1;
    }

    /* Forward iterate */
    cpu = 0;
    while cpu < len {
        send_from_cpu(cpu, family, proto);
        receive_on_cpu(rcv_fd, len, epfd, cpu, proto);
        cpu += 1;
    }

    /* Reverse iterate */
    cpu = len - 1;
    while cpu >= 0 {
        send_from_cpu(cpu, family, proto);
        receive_on_cpu(rcv_fd, len, epfd, cpu, proto);
        cpu -= 1;
    }

    /* Even cores */
    cpu = 0;
    while cpu < len {
        send_from_cpu(cpu, family, proto);
        receive_on_cpu(rcv_fd, len, epfd, cpu, proto);
        cpu += 2;
    }

    /* Odd cores */
    cpu = 1;
    while cpu < len {
        send_from_cpu(cpu, family, proto);
        receive_on_cpu(rcv_fd, len, epfd, cpu, proto);
        cpu += 2;
    }

    close(epfd);
    cpu = 0;
    while cpu < len {
        close(*rcv_fd.add(cpu as usize));
        cpu += 1;
    }
}

unsafe fn setup_netns() {
    if unshare(CLONE_NEWNET) != 0 {
        error(1, errno(), b"failed to unshare netns\0".as_ptr() as *const c_char);
    }
    if system(b"ip link set lo up\0".as_ptr() as *const c_char) != 0 {
        error(
            1,
            0,
            b"failed to bring up lo interface in netns\0".as_ptr() as *const c_char,
        );
    }
}

fn main() -> c_int {
    unsafe {
        let rcv_fd: *mut c_int;
        let cpus: c_int;

        setup_netns();

        cpus = sysconf(_SC_NPROCESSORS_ONLN) as c_int;
        if cpus <= 0 {
            error(1, errno(), b"failed counting cpus\0".as_ptr() as *const c_char);
        }

        rcv_fd = calloc(cpus as size_t, size_of::<c_int>()) as *mut c_int;
        if rcv_fd.is_null() {
            error(1, 0, b"failed to allocate array\0".as_ptr() as *const c_char);
        }

        fprintf(stderr, b"---- IPv4 UDP ----\n\0".as_ptr() as *const c_char);
        test(rcv_fd, cpus, AF_INET, SOCK_DGRAM);

        fprintf(stderr, b"---- IPv6 UDP ----\n\0".as_ptr() as *const c_char);
        test(rcv_fd, cpus, AF_INET6, SOCK_DGRAM);

        fprintf(stderr, b"---- IPv4 TCP ----\n\0".as_ptr() as *const c_char);
        test(rcv_fd, cpus, AF_INET, SOCK_STREAM);

        fprintf(stderr, b"---- IPv6 TCP ----\n\0".as_ptr() as *const c_char);
        test(rcv_fd, cpus, AF_INET6, SOCK_STREAM);

        free(rcv_fd as *mut c_void);

        fprintf(stderr, b"SUCCESS\n\0".as_ptr() as *const c_char);
        0
    }
}
