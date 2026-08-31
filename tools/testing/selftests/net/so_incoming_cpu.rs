// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */
/* Translated from C. Original includes:
 * #define _GNU_SOURCE
 * <sched.h>, <fcntl.h>, <netinet/in.h>, <sys/socket.h>,
 * <sys/sysinfo.h>, "kselftest_harness.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type socklen_t = u32;
type size_t = usize;
type ssize_t = isize;

const O_WRONLY: c_int = 1;
const CLONE_NEWNET: c_int = 0x40000000;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOCK_NONBLOCK: c_int = 0o0004000;
const SOL_SOCKET: c_int = 1;
const SO_REUSEPORT: c_int = 15;
const SO_INCOMING_CPU: c_int = 49;
const INADDR_LOOPBACK: u32 = 0x7f000001;

const NR_PORT: c_int = 60001 - 10000 - 1;
const NR_CLIENT_PER_SERVER_DEFAULT: c_int = 32;

static mut nr_client_per_server: c_int = 0;
static mut nr_server: c_int = 0;
static mut nr_client: c_int = 0;

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub union so_incoming_cpu_addr {
    pub addr: sockaddr,
    pub in_addr: sockaddr_in,
}

#[repr(C)]
pub struct so_incoming_cpu {
    pub servers: *mut c_int,
    pub u: so_incoming_cpu_addr,
    pub addrlen: socklen_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum when_to_set {
    BEFORE_REUSEPORT,
    BEFORE_LISTEN,
    AFTER_LISTEN,
    AFTER_ALL_LISTEN,
}

#[repr(C)]
pub struct so_incoming_cpu_variant {
    pub when_to_set: c_int,
}

pub static before_reuseport: so_incoming_cpu_variant = so_incoming_cpu_variant {
    when_to_set: when_to_set::BEFORE_REUSEPORT as c_int,
};

pub static before_listen: so_incoming_cpu_variant = so_incoming_cpu_variant {
    when_to_set: when_to_set::BEFORE_LISTEN as c_int,
};

pub static after_listen: so_incoming_cpu_variant = so_incoming_cpu_variant {
    when_to_set: when_to_set::AFTER_LISTEN as c_int,
};

pub static after_all_listen: so_incoming_cpu_variant = so_incoming_cpu_variant {
    when_to_set: when_to_set::AFTER_ALL_LISTEN as c_int,
};

#[repr(C)]
pub struct cpu_set_t {
    __bits: [usize; 16],
}

extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn get_nprocs() -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
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
    fn getsockname(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;

    fn ASSERT_NE_int(left: c_int, right: c_int);
    fn ASSERT_EQ_int(left: c_int, right: c_int);
    fn ASSERT_LE_int(left: c_int, right: c_int);
    fn ASSERT_NE_ptr(left: *const c_void, right: *const c_void);
    fn TH_LOG(fmt: *const c_char, ...);
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    let set = &mut *set;
    let mut i: usize = 0;

    while i < set.__bits.len() {
        set.__bits[i] = 0;
        i += 1;
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let cpu = cpu as usize;
    let bits_per_word = core::mem::size_of::<usize>() * 8;

    (*set).__bits[cpu / bits_per_word] |= 1usize << (cpu % bits_per_word);
}

unsafe fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> c_int {
    let cpu = cpu as usize;
    let bits_per_word = core::mem::size_of::<usize>() * 8;

    (((*set).__bits[cpu / bits_per_word] & (1usize << (cpu % bits_per_word))) != 0) as c_int
}

unsafe fn CPU_COUNT(set: *const cpu_set_t) -> c_int {
    let mut count: c_uint = 0;
    let mut i: usize = 0;

    while i < (*set).__bits.len() {
        count = count.wrapping_add((*set).__bits[i].count_ones());
        i += 1;
    }

    count as c_int
}

unsafe fn write_sysctl(_metadata: *mut __test_metadata, filename: *mut c_char, string: *mut c_char) {
    let fd: c_int;
    let len: c_int;
    let ret: c_int;

    fd = open(filename, O_WRONLY);
    ASSERT_NE_int(fd, -1);

    len = strlen(string) as c_int;
    ret = write(fd, string as *const c_void, len as size_t) as c_int;
    ASSERT_EQ_int(ret, len);
}

unsafe fn setup_netns(_metadata: *mut __test_metadata) {
    ASSERT_EQ_int(unshare(CLONE_NEWNET), 0);
    ASSERT_EQ_int(system(c"ip link set lo up".as_ptr()), 0);

    write_sysctl(
        _metadata,
        c"/proc/sys/net/ipv4/ip_local_port_range".as_ptr() as *mut c_char,
        c"10000 60001".as_ptr() as *mut c_char,
    );
    write_sysctl(
        _metadata,
        c"/proc/sys/net/ipv4/tcp_tw_reuse".as_ptr() as *mut c_char,
        c"0".as_ptr() as *mut c_char,
    );
}

/* FIXTURE_SETUP(so_incoming_cpu) */
pub unsafe fn so_incoming_cpu_setup(_metadata: *mut __test_metadata, self_: *mut so_incoming_cpu) {
    setup_netns(_metadata);

    nr_server = get_nprocs();
    ASSERT_LE_int(2, nr_server);

    if NR_CLIENT_PER_SERVER_DEFAULT * nr_server < NR_PORT {
        nr_client_per_server = NR_CLIENT_PER_SERVER_DEFAULT;
    } else {
        nr_client_per_server = NR_PORT / nr_server;
    }

    nr_client = nr_client_per_server * nr_server;

    (*self_).servers = malloc(core::mem::size_of::<c_int>() * nr_server as usize) as *mut c_int;
    ASSERT_NE_ptr((*self_).servers as *const c_void, core::ptr::null());

    (*self_).u.in_addr.sin_family = AF_INET as u16;
    (*self_).u.in_addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    (*self_).u.in_addr.sin_port = htons(0);
    (*self_).addrlen = core::mem::size_of::<sockaddr_in>() as socklen_t;
}

/* FIXTURE_TEARDOWN(so_incoming_cpu) */
pub unsafe fn so_incoming_cpu_teardown(
    _metadata: *mut __test_metadata,
    self_: *mut so_incoming_cpu,
) {
    let mut i: c_int;

    i = 0;
    while i < nr_server {
        close(*(*self_).servers.add(i as usize));
        i += 1;
    }

    free((*self_).servers as *mut c_void);
}

pub unsafe fn set_so_incoming_cpu(_metadata: *mut __test_metadata, fd: c_int, cpu: c_int) {
    let ret: c_int;

    ret = setsockopt(
        fd,
        SOL_SOCKET,
        SO_INCOMING_CPU,
        &cpu as *const c_int as *const c_void,
        core::mem::size_of::<c_int>() as socklen_t,
    );
    ASSERT_EQ_int(ret, 0);
}

pub unsafe fn create_server(
    _metadata: *mut __test_metadata,
    self_: *mut so_incoming_cpu,
    variant: *const so_incoming_cpu_variant,
    cpu: c_int,
) -> c_int {
    let fd: c_int;
    let mut ret: c_int;

    fd = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
    ASSERT_NE_int(fd, -1);

    if (*variant).when_to_set == when_to_set::BEFORE_REUSEPORT as c_int {
        set_so_incoming_cpu(_metadata, fd, cpu);
    }

    ret = setsockopt(
        fd,
        SOL_SOCKET,
        SO_REUSEPORT,
        &(1 as c_int) as *const c_int as *const c_void,
        core::mem::size_of::<c_int>() as socklen_t,
    );
    ASSERT_EQ_int(ret, 0);

    ret = bind(fd, &(*self_).u.addr as *const sockaddr, (*self_).addrlen);
    ASSERT_EQ_int(ret, 0);

    if (*variant).when_to_set == when_to_set::BEFORE_LISTEN as c_int {
        set_so_incoming_cpu(_metadata, fd, cpu);
    }

    /* We don't use nr_client_per_server here not to block
     * this test at connect() if SO_INCOMING_CPU is broken.
     */
    ret = listen(fd, nr_client);
    ASSERT_EQ_int(ret, 0);

    if (*variant).when_to_set == when_to_set::AFTER_LISTEN as c_int {
        set_so_incoming_cpu(_metadata, fd, cpu);
    }

    fd
}

pub unsafe fn create_servers(
    _metadata: *mut __test_metadata,
    self_: *mut so_incoming_cpu,
    variant: *const so_incoming_cpu_variant,
) {
    let mut i: c_int;
    let mut ret: c_int;

    i = 0;
    while i < nr_server {
        *(*self_).servers.add(i as usize) = create_server(_metadata, self_, variant, i);

        if i == 0 {
            ret = getsockname(
                *(*self_).servers.add(i as usize),
                &mut (*self_).u.addr as *mut sockaddr,
                &mut (*self_).addrlen as *mut socklen_t,
            );
            ASSERT_EQ_int(ret, 0);
        }

        i += 1;
    }

    if (*variant).when_to_set == when_to_set::AFTER_ALL_LISTEN as c_int {
        i = 0;
        while i < nr_server {
            set_so_incoming_cpu(_metadata, *(*self_).servers.add(i as usize), i);
            i += 1;
        }
    }
}

pub unsafe fn create_clients(_metadata: *mut __test_metadata, self_: *mut so_incoming_cpu) {
    let mut cpu_set: cpu_set_t = core::mem::zeroed();
    let mut i: c_int;
    let mut j: c_int;
    let mut fd: c_int;
    let mut ret: c_int;

    i = 0;
    while i < nr_server {
        CPU_ZERO(&mut cpu_set as *mut cpu_set_t);

        CPU_SET(i, &mut cpu_set as *mut cpu_set_t);
        ASSERT_EQ_int(CPU_COUNT(&cpu_set as *const cpu_set_t), 1);
        ASSERT_NE_int(CPU_ISSET(i, &cpu_set as *const cpu_set_t), 0);

        /* Make sure SYN will be processed on the i-th CPU
         * and finally distributed to the i-th listener.
         */
        ret = sched_setaffinity(
            0,
            core::mem::size_of::<cpu_set_t>(),
            &cpu_set as *const cpu_set_t,
        );
        ASSERT_EQ_int(ret, 0);

        j = 0;
        while j < nr_client_per_server {
            fd = socket(AF_INET, SOCK_STREAM, 0);
            ASSERT_NE_int(fd, -1);

            ret = connect(fd, &(*self_).u.addr as *const sockaddr, (*self_).addrlen);
            ASSERT_EQ_int(ret, 0);

            close(fd);
            j += 1;
        }

        i += 1;
    }
}

pub unsafe fn verify_incoming_cpu(_metadata: *mut __test_metadata, self_: *mut so_incoming_cpu) {
    let mut i: c_int;
    let mut j: c_int;
    let mut fd: c_int;
    let mut cpu: c_int = 0;
    let mut ret: c_int;
    let mut total: c_int = 0;
    let mut len: socklen_t = core::mem::size_of::<c_int>() as socklen_t;

    i = 0;
    while i < nr_server {
        j = 0;
        while j < nr_client_per_server {
            /* If we see -EAGAIN here, SO_INCOMING_CPU is broken */
            fd = accept(
                *(*self_).servers.add(i as usize),
                &mut (*self_).u.addr as *mut sockaddr,
                &mut (*self_).addrlen as *mut socklen_t,
            );
            ASSERT_NE_int(fd, -1);

            ret = getsockopt(
                fd,
                SOL_SOCKET,
                SO_INCOMING_CPU,
                &mut cpu as *mut c_int as *mut c_void,
                &mut len as *mut socklen_t,
            );
            ASSERT_EQ_int(ret, 0);
            ASSERT_EQ_int(cpu, i);

            close(fd);
            total += 1;
            j += 1;
        }

        i += 1;
    }

    ASSERT_EQ_int(total, nr_client);
    TH_LOG(
        c"SO_INCOMING_CPU is very likely to be working correctly with %d sockets.".as_ptr(),
        total,
    );
}

/* TEST_F(so_incoming_cpu, test1) */
pub unsafe fn test1(
    _metadata: *mut __test_metadata,
    self_: *mut so_incoming_cpu,
    variant: *const so_incoming_cpu_variant,
) {
    create_servers(_metadata, self_, variant);
    create_clients(_metadata, self_);
    verify_incoming_cpu(_metadata, self_);
}
