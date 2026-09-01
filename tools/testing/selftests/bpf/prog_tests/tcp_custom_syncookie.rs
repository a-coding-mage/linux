// SPDX-License-Identifier: GPL-2.0
/* Copyright Amazon.com Inc. or its affiliates. */

/* Translated from C implementation source.
 * C includes required: sched.h, stdlib.h, net/if.h, test_progs.h,
 * cgroup_helpers.h, network_helpers.h, test_tcp_custom_syncookie.skel.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const CLONE_NEWNET: c_int = 0x40000000;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const BPF_TC_INGRESS: c_int = 1;

const MSG: &[u8; 11] = b"Hello World";
const MSGLEN: usize = 11;

#[repr(C)]
struct test_tcp_custom_syncookie_case {
    family: c_int,
    type_: c_int,
    addr: [c_char; 16],
    name: [c_char; 10],
}

#[repr(C)]
struct test_tcp_custom_syncookie {
    progs: test_tcp_custom_syncookie__progs,
    bss: *mut test_tcp_custom_syncookie__bss,
}

#[repr(C)]
struct test_tcp_custom_syncookie__progs {
    tcp_custom_syncookie: *mut bpf_program,
}

#[repr(C)]
struct test_tcp_custom_syncookie__bss {
    handled_syn: bool,
    handled_ack: bool,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_tc_hook {
    sz: usize,
    ifindex: c_int,
    attach_point: c_int,
}

#[repr(C)]
struct bpf_tc_opts {
    sz: usize,
    prog_fd: c_int,
}

unsafe extern "C" {
    fn unshare(flags: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn write_sysctl(path: *const c_char, value: *const c_char) -> c_int;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;

    fn send(sockfd: c_int, buf: *const c_void, len: usize, flags: c_int) -> isize;
    fn recv(sockfd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn accept(sockfd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn start_server(family: c_int, type_: c_int, addr: *const c_char, port: c_int, timeout_ms: c_int) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;

    fn test_tcp_custom_syncookie__open_and_load() -> *mut test_tcp_custom_syncookie;
    fn test_tcp_custom_syncookie__destroy(skel: *mut test_tcp_custom_syncookie);
}

/* C test assertion helpers are macros in test_progs.h. They are kept as
 * external declarations here to preserve the source-level control flow.
 */
unsafe extern "C" {
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

static mut test_cases: [test_tcp_custom_syncookie_case; 2] = [
    test_tcp_custom_syncookie_case {
        family: AF_INET,
        type_: SOCK_STREAM,
        addr: [
            b'1' as c_char,
            b'2' as c_char,
            b'7' as c_char,
            b'.' as c_char,
            b'0' as c_char,
            b'.' as c_char,
            b'0' as c_char,
            b'.' as c_char,
            b'1' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        name: [
            b'I' as c_char,
            b'P' as c_char,
            b'v' as c_char,
            b'4' as c_char,
            b' ' as c_char,
            b'T' as c_char,
            b'C' as c_char,
            b'P' as c_char,
            0,
            0,
        ],
    },
    test_tcp_custom_syncookie_case {
        family: AF_INET6,
        type_: SOCK_STREAM,
        addr: [
            b':' as c_char,
            b':' as c_char,
            b'1' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        name: [
            b'I' as c_char,
            b'P' as c_char,
            b'v' as c_char,
            b'6' as c_char,
            b' ' as c_char,
            b'T' as c_char,
            b'C' as c_char,
            b'P' as c_char,
            0,
            0,
        ],
    },
];

unsafe fn setup_netns() -> c_int {
    if !ASSERT_OK(unshare(CLONE_NEWNET), c"create netns".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK(system(c"ip link set dev lo up".as_ptr()), c"ip".as_ptr()) {
        return -1;
    }

    if !ASSERT_OK(
        write_sysctl(
            c"/proc/sys/net/ipv4/tcp_ecn".as_ptr(),
            c"1".as_ptr(),
        ),
        c"write_sysctl".as_ptr(),
    ) {
        return -1;
    }

    0
}

unsafe fn setup_tc(skel: *mut test_tcp_custom_syncookie) -> c_int {
    let mut qdisc_lo = bpf_tc_hook {
        sz: core::mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: BPF_TC_INGRESS,
    };
    let mut tc_attach = bpf_tc_opts {
        sz: core::mem::size_of::<bpf_tc_opts>(),
        prog_fd: bpf_program__fd((*skel).progs.tcp_custom_syncookie),
    };

    qdisc_lo.ifindex = if_nametoindex(c"lo".as_ptr()) as c_int;
    if !ASSERT_OK(
        bpf_tc_hook_create(&mut qdisc_lo),
        c"qdisc add dev lo clsact".as_ptr(),
    ) {
        return -1;
    }

    if !ASSERT_OK(
        bpf_tc_attach(&mut qdisc_lo, &mut tc_attach),
        c"filter add dev lo ingress".as_ptr(),
    ) {
        return -1;
    }

    0
}

unsafe fn transfer_message(sender: c_int, receiver: c_int) {
    let mut buf: [c_char; MSGLEN] = [0; MSGLEN];
    let mut ret: c_int;

    ret = send(sender, MSG.as_ptr() as *const c_void, MSGLEN, 0) as c_int;
    if !ASSERT_EQ(ret, MSGLEN as c_int, c"send".as_ptr()) {
        return;
    }

    memset(buf.as_mut_ptr() as *mut c_void, 0, core::mem::size_of_val(&buf));

    ret = recv(receiver, buf.as_mut_ptr() as *mut c_void, MSGLEN, 0) as c_int;
    if !ASSERT_EQ(ret, MSGLEN as c_int, c"recv".as_ptr()) {
        return;
    }

    ret = strncmp(buf.as_ptr(), MSG.as_ptr() as *const c_char, MSGLEN);
    if !ASSERT_EQ(ret, 0, c"strncmp".as_ptr()) {
        return;
    }
}

unsafe fn create_connection(test_case: *mut test_tcp_custom_syncookie_case) {
    let server: c_int;
    let client: c_int;
    let child: c_int;

    server = start_server((*test_case).family, (*test_case).type_, (*test_case).addr.as_ptr(), 0, 0);
    if !ASSERT_NEQ(server, -1, c"start_server".as_ptr()) {
        return;
    }

    client = connect_to_fd(server, 0);
    if !ASSERT_NEQ(client, -1, c"connect_to_fd".as_ptr()) {
        close(server);
        return;
    }

    child = accept(server, core::ptr::null_mut(), core::ptr::null_mut());
    if !ASSERT_NEQ(child, -1, c"accept".as_ptr()) {
        close(client);
        close(server);
        return;
    }

    transfer_message(client, child);
    transfer_message(child, client);

    close(child);
    close(client);
    close(server);
}

#[no_mangle]
pub unsafe extern "C" fn test_tcp_custom_syncookie() {
    let skel: *mut test_tcp_custom_syncookie;
    let mut i: usize;

    if setup_netns() != 0 {
        return;
    }

    skel = test_tcp_custom_syncookie__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) {
        return;
    }

    if setup_tc(skel) != 0 {
        test_tcp_custom_syncookie__destroy(skel);
        return;
    }

    i = 0;
    while i < test_cases.len() {
        if !test__start_subtest(test_cases[i].name.as_ptr()) {
            i += 1;
            continue;
        }

        (*(*skel).bss).handled_syn = false;
        (*(*skel).bss).handled_ack = false;

        create_connection(&raw mut test_cases[i]);

        ASSERT_EQ(
            (*(*skel).bss).handled_syn,
            true,
            c"SYN is not handled at tc.".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).handled_ack,
            true,
            c"ACK is not handled at tc".as_ptr(),
        );

        i += 1;
    }

    system(c"tc qdisc del dev lo clsact".as_ptr());

    test_tcp_custom_syncookie__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
