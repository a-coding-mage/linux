// SPDX-License-Identifier: GPL-2.0
/* Copyright 2025 Google LLC */

/* Translated from:
 * #include <test_progs.h>
 * #include "sk_bypass_prot_mem.skel.h"
 * #include "network_helpers.h"
 *
 * PAGE_SIZE is getpagesize() when not provided by the build environment.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

type Bool = bool;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOL_SOCKET: c_int = 1;
const SO_RCVBUF: c_int = 8;
const MSG_DONTWAIT: c_int = 0x40;
const MSG_TRUNC: c_int = 0x20;
const EAGAIN: c_int = 11;

const NR_PAGES: usize = 32;
const NR_SOCKETS: usize = 2;
const BUF_SINGLE: usize = 1024;

#[repr(C)]
struct test_case {
    name: [c_char; 8],
    family: c_int,
    type_: c_int,
    create_sockets: unsafe extern "C" fn(*mut test_case, *mut c_int, c_int) -> c_int,
    get_memory_allocated: unsafe extern "C" fn(*mut test_case, *mut sk_bypass_prot_mem) -> c_long,
}

#[repr(C)]
struct sk_bypass_prot_mem {
    bss: *mut sk_bypass_prot_mem_bss,
    links: sk_bypass_prot_mem_links,
    progs: sk_bypass_prot_mem_progs,
}

#[repr(C)]
struct sk_bypass_prot_mem_bss {
    tcp_activated: Bool,
    tcp_memory_allocated: c_long,
    udp_activated: Bool,
    udp_memory_allocated: c_long,
    nr_cpus: c_int,
}

#[repr(C)]
struct sk_bypass_prot_mem_links {
    sock_create: *mut c_void,
}

#[repr(C)]
struct sk_bypass_prot_mem_progs {
    sock_create: *mut c_void,
}

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn getpagesize() -> c_int;
    fn start_server(family: c_int, type_: c_int, addr: *const c_void, port: c_int, timeout: c_int) -> c_int;
    fn connect_to_fd(fd: c_int, timeout: c_int) -> c_int;
    fn connect_fd_to_fd(fd1: c_int, fd2: c_int, timeout: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: c_uint) -> c_int;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;

    fn sk_bypass_prot_mem__open_and_load() -> *mut sk_bypass_prot_mem;
    fn sk_bypass_prot_mem__attach(skel: *mut sk_bypass_prot_mem) -> c_int;
    fn sk_bypass_prot_mem__destroy(skel: *mut sk_bypass_prot_mem);
    fn bpf_program__attach_cgroup(prog: *mut c_void, cgroup: c_int) -> *mut c_void;
    fn libbpf_num_possible_cpus() -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn make_netns(name: *const c_char) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    #[link_name = "close_netns"]
    fn close_netns_fn(token: *mut nstoken);
    #[link_name = "remove_netns"]
    fn remove_netns_fn(name: *const c_char);
    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> Bool;

    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> Bool;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> Bool;
    fn ASSERT_LE(actual: c_long, expected: c_long, name: *const c_char) -> Bool;
    fn ASSERT_GT(actual: c_long, expected: c_long, name: *const c_char) -> Bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> Bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> Bool;
    fn PRINT_FAIL(fmt: *const c_char, ...);
}

const fn c_name(bytes: &[u8; 8]) -> [c_char; 8] {
    [
        bytes[0] as c_char,
        bytes[1] as c_char,
        bytes[2] as c_char,
        bytes[3] as c_char,
        bytes[4] as c_char,
        bytes[5] as c_char,
        bytes[6] as c_char,
        bytes[7] as c_char,
    ]
}

unsafe fn page_size() -> usize {
    getpagesize() as usize
}

unsafe fn buf_total() -> usize {
    NR_PAGES * page_size() / NR_SOCKETS
}

unsafe fn nr_send() -> usize {
    buf_total() / BUF_SINGLE
}

unsafe extern "C" fn tcp_create_sockets(test_case: *mut test_case, sk: *mut c_int, len: c_int) -> c_int {
    let mut err: c_int = 0;

    let server = start_server((*test_case).family, (*test_case).type_, ptr::null(), 0, 0);
    if !ASSERT_GE(server as c_long, 0, c"start_server_str".as_ptr()) {
        return server;
    }

    /* Keep for-loop so we can change NR_SOCKETS easily. */
    let mut i = 0;
    while i < len {
        *sk.add(i as usize) = connect_to_fd(server, 0);
        if *sk.add(i as usize) < 0 {
            ASSERT_GE(*sk.add(i as usize) as c_long, 0, c"connect_to_fd".as_ptr());
            err = *sk.add(i as usize);
            break;
        }

        *sk.add((i + 1) as usize) = accept(server, ptr::null_mut(), ptr::null_mut());
        if *sk.add((i + 1) as usize) < 0 {
            ASSERT_GE(*sk.add((i + 1) as usize) as c_long, 0, c"accept".as_ptr());
            err = *sk.add((i + 1) as usize);
            break;
        }
        i += 2;
    }

    close(server);

    err
}

unsafe extern "C" fn udp_create_sockets(test_case: *mut test_case, sk: *mut c_int, len: c_int) -> c_int {
    let mut err: c_int;
    let rcvbuf: c_int = buf_total() as c_int;

    /* Keep for-loop so we can change NR_SOCKETS easily. */
    let mut i = 0;
    while i < len {
        *sk.add(i as usize) = start_server((*test_case).family, (*test_case).type_, ptr::null(), 0, 0);
        if *sk.add(i as usize) < 0 {
            ASSERT_GE(*sk.add(i as usize) as c_long, 0, c"start_server".as_ptr());
            return *sk.add(i as usize);
        }

        *sk.add((i + 1) as usize) = connect_to_fd(*sk.add(i as usize), 0);
        if *sk.add((i + 1) as usize) < 0 {
            ASSERT_GE(*sk.add((i + 1) as usize) as c_long, 0, c"connect_to_fd".as_ptr());
            return *sk.add((i + 1) as usize);
        }

        err = connect_fd_to_fd(*sk.add(i as usize), *sk.add((i + 1) as usize), 0);
        if err != 0 {
            ASSERT_EQ(err as c_long, 0, c"connect_fd_to_fd".as_ptr());
            return err;
        }

        let mut j = 0;
        while j < 2 {
            err = setsockopt(
                *sk.add((i + j) as usize),
                SOL_SOCKET,
                SO_RCVBUF,
                &rcvbuf as *const c_int as *const c_void,
                size_of::<c_int>() as c_uint,
            );
            if err != 0 {
                ASSERT_EQ(err as c_long, 0, c"setsockopt(SO_RCVBUF)".as_ptr());
                return err;
            }
            j += 1;
        }
        i += 2;
    }

    0
}

unsafe fn get_memory_allocated(
    test_case: *mut test_case,
    activated: *mut Bool,
    memory_allocated: *mut c_long,
) -> c_long {
    *activated = true;

    /* AF_INET and AF_INET6 share the same memory_allocated.
     * tcp_init_sock() is called by AF_INET and AF_INET6,
     * but udp_lib_init_sock() is inline.
     */
    let sk = socket(AF_INET, (*test_case).type_, 0);
    if !ASSERT_GE(sk as c_long, 0, c"get_memory_allocated".as_ptr()) {
        return -1;
    }

    close(sk);

    *memory_allocated
}

unsafe extern "C" fn tcp_get_memory_allocated(
    test_case: *mut test_case,
    skel: *mut sk_bypass_prot_mem,
) -> c_long {
    get_memory_allocated(
        test_case,
        &mut (*(*skel).bss).tcp_activated,
        &mut (*(*skel).bss).tcp_memory_allocated,
    )
}

unsafe extern "C" fn udp_get_memory_allocated(
    test_case: *mut test_case,
    skel: *mut sk_bypass_prot_mem,
) -> c_long {
    get_memory_allocated(
        test_case,
        &mut (*(*skel).bss).udp_activated,
        &mut (*(*skel).bss).udp_memory_allocated,
    )
}

unsafe fn check_bypass(test_case: *mut test_case, skel: *mut sk_bypass_prot_mem, bypass: Bool) -> c_int {
    let mut buf = [0 as c_char; BUF_SINGLE];
    let mut memory_allocated = [0 as c_long; 2];
    let mut sk = [0 as c_int; NR_SOCKETS];
    let mut err: c_int = 0;

    for i in 0..sk.len() {
        sk[i] = -1;
    }

    err = ((*test_case).create_sockets)(test_case, sk.as_mut_ptr(), sk.len() as c_int);
    if err == 0 {
        memory_allocated[0] = ((*test_case).get_memory_allocated)(test_case, skel);

        /* allocate pages >= NR_PAGES */
        'send_loop: for i in 0..sk.len() {
            for _j in 0..nr_send() {
                let bytes = send(sk[i], buf.as_ptr() as *const c_void, size_of_val(&buf), 0);

                /* Avoid too noisy logs when something failed. */
                if bytes != size_of_val(&buf) as isize {
                    ASSERT_EQ(bytes as c_long, size_of_val(&buf) as c_long, c"send".as_ptr());
                    if bytes < 0 {
                        err = bytes as c_int;
                        break 'send_loop;
                    }
                }
            }
        }

        if err == 0 {
            memory_allocated[1] = ((*test_case).get_memory_allocated)(test_case, skel);

            if bypass {
                ASSERT_LE(memory_allocated[1], memory_allocated[0] + 10, c"bypass".as_ptr());
            } else {
                ASSERT_GT(memory_allocated[1], memory_allocated[0] + NR_PAGES as c_long, c"no bypass".as_ptr());
            }
        }
    }

    if (*test_case).type_ == SOCK_DGRAM {
        /* UDP starts purging sk->sk_receive_queue after one RCU
         * grace period, then udp_memory_allocated goes down,
         * so drain the queue before close().
         */
        for i in 0..sk.len() {
            for _j in 0..nr_send() {
                let bytes = recv(
                    sk[i],
                    buf.as_mut_ptr() as *mut c_void,
                    1,
                    MSG_DONTWAIT | MSG_TRUNC,
                );

                if bytes == size_of_val(&buf) as isize {
                    continue;
                }
                if bytes != -1 || *__errno_location() != EAGAIN {
                    PRINT_FAIL(
                        c"bytes: %d, errno: %s\n".as_ptr(),
                        bytes as c_int,
                        strerror(*__errno_location()),
                    );
                }
                break;
            }
        }
    }

    for i in 0..sk.len() {
        if sk[i] < 0 {
            break;
        }

        close(sk[i]);
    }

    err
}

unsafe fn run_test(test_case: *mut test_case) {
    let skel = sk_bypass_prot_mem__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"open_and_load".as_ptr()) {
        return;
    }

    (*(*skel).bss).nr_cpus = libbpf_num_possible_cpus();

    let mut err = sk_bypass_prot_mem__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        sk_bypass_prot_mem__destroy(skel);
        return;
    }

    let cgroup = test__join_cgroup(c"/sk_bypass_prot_mem".as_ptr());
    if !ASSERT_GE(cgroup as c_long, 0, c"join_cgroup".as_ptr()) {
        sk_bypass_prot_mem__destroy(skel);
        return;
    }

    err = make_netns(c"sk_bypass_prot_mem".as_ptr());
    if !ASSERT_EQ(err as c_long, 0, c"make_netns".as_ptr()) {
        close(cgroup);
        sk_bypass_prot_mem__destroy(skel);
        return;
    }

    let nstoken = open_netns(c"sk_bypass_prot_mem".as_ptr());
    if ASSERT_OK_PTR(nstoken as *const c_void, c"open_netns".as_ptr()) {
        err = check_bypass(test_case, skel, false);
        if ASSERT_EQ(err as c_long, 0, c"test_bypass(false)".as_ptr()) {
            err = write_sysctl(c"/proc/sys/net/core/bypass_prot_mem".as_ptr(), c"1".as_ptr());
            if ASSERT_EQ(err as c_long, 0, c"write_sysctl(1)".as_ptr()) {
                err = check_bypass(test_case, skel, true);
                if ASSERT_EQ(err as c_long, 0, c"test_bypass(true by sysctl)".as_ptr()) {
                    err = write_sysctl(c"/proc/sys/net/core/bypass_prot_mem".as_ptr(), c"0".as_ptr());
                    if ASSERT_EQ(err as c_long, 0, c"write_sysctl(0)".as_ptr()) {
                        (*skel).links.sock_create =
                            bpf_program__attach_cgroup((*skel).progs.sock_create, cgroup);
                        if ASSERT_OK_PTR((*skel).links.sock_create as *const c_void, c"attach_cgroup(sock_create)".as_ptr()) {
                            err = check_bypass(test_case, skel, true);
                            ASSERT_EQ(err as c_long, 0, c"test_bypass(true by bpf)".as_ptr());
                        }
                    }
                }
            }
        }

        close_netns_fn(nstoken);
    }
    remove_netns_fn(c"sk_bypass_prot_mem".as_ptr());
    close(cgroup);
    sk_bypass_prot_mem__destroy(skel);
}

static mut TEST_CASES: [test_case; 4] = [
    test_case {
        name: c_name(b"TCP  \0\0\0"),
        family: AF_INET,
        type_: SOCK_STREAM,
        create_sockets: tcp_create_sockets,
        get_memory_allocated: tcp_get_memory_allocated,
    },
    test_case {
        name: c_name(b"UDP  \0\0\0"),
        family: AF_INET,
        type_: SOCK_DGRAM,
        create_sockets: udp_create_sockets,
        get_memory_allocated: udp_get_memory_allocated,
    },
    test_case {
        name: c_name(b"TCPv6\0\0\0"),
        family: AF_INET6,
        type_: SOCK_STREAM,
        create_sockets: tcp_create_sockets,
        get_memory_allocated: tcp_get_memory_allocated,
    },
    test_case {
        name: c_name(b"UDPv6\0\0\0"),
        family: AF_INET6,
        type_: SOCK_DGRAM,
        create_sockets: udp_create_sockets,
        get_memory_allocated: udp_get_memory_allocated,
    },
];

#[no_mangle]
pub unsafe extern "C" fn serial_test_sk_bypass_prot_mem() {
    let mut i = 0;

    while i < TEST_CASES.len() {
        if test__start_subtest(TEST_CASES[i].name.as_ptr()) {
            run_test(&mut TEST_CASES[i]);
        }
        i += 1;
    }
}
