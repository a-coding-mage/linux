// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */
// C dependencies translated as external declarations:
// <uapi/linux/if_link.h>, <test_progs.h>, <netinet/tcp.h>,
// <netinet/udp.h>, "network_helpers.h", "test_assign_reuse.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u16 = u16;
type __u64 = u64;
type socklen_t = u32;

const NS_TEST: &[u8] = b"assign_reuse\0";
const LOOPBACK: c_int = 1;
const PORT: __u16 = 4443;

const SOL_SOCKET: c_int = 1;
const SO_ATTACH_REUSEPORT_EBPF: c_int = 52;
const SO_COOKIE: c_int = 57;
const BPF_TC_INGRESS: c_uint = 1;
const BPF_TC_EGRESS: c_uint = 2;
const BPF_NOEXIST: u64 = 1;
const EEXIST: c_int = 17;
const ECONNREFUSED: c_int = 111;
const EAGAIN: c_int = 11;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;

#[repr(C)]
struct sockaddr_storage {
    ss_family: u16,
    __data: [u8; 126],
}

#[repr(C)]
struct bpf_tc_hook {
    sz: usize,
    ifindex: c_int,
    attach_point: c_uint,
    parent: c_uint,
}

#[repr(C)]
struct bpf_tc_opts {
    sz: usize,
    prog_fd: c_int,
    flags: c_uint,
    prog_id: c_uint,
    handle: c_uint,
    priority: c_uint,
}

#[repr(C)]
struct test_assign_reuse {
    rodata: *mut test_assign_reuse_rodata,
    bss: *mut test_assign_reuse_bss,
    progs: test_assign_reuse_progs,
    maps: test_assign_reuse_maps,
}

#[repr(C)]
struct test_assign_reuse_rodata {
    dest_port: __u16,
}

#[repr(C)]
struct test_assign_reuse_bss {
    sk_cookie_seen: __u64,
    reuseport_executed: c_int,
}

#[repr(C)]
struct test_assign_reuse_progs {
    tc_main: *mut c_void,
    reuse_accept: *mut c_void,
    reuse_drop: *mut c_void,
}

#[repr(C)]
struct test_assign_reuse_maps {
    sk_map: *mut c_void,
}

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

unsafe extern "C" {
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
    fn getsockname(socket: c_int, address: *mut c_void, address_len: *mut socklen_t) -> c_int;
    fn send(socket: c_int, buffer: *const c_void, length: usize, flags: c_int) -> isize;
    fn sendto(
        socket: c_int,
        message: *const c_void,
        length: usize,
        flags: c_int,
        dest_addr: *const c_void,
        dest_len: socklen_t,
    ) -> isize;
    fn recv(socket: c_int, buffer: *mut c_void, length: usize, flags: c_int) -> isize;
    fn accept(socket: c_int, address: *mut c_void, address_len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn free_fds(fds: *mut c_int, cnt: c_int);
    fn errno_location() -> *mut c_int;

    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn start_reuseport_server(
        family: c_int,
        sotype: c_int,
        addr_str: *const c_char,
        port: __u16,
        timeout_ms: c_int,
        reuseport: c_int,
    ) -> *mut c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(tok: *mut nstoken);

    fn test_assign_reuse__open() -> *mut test_assign_reuse;
    fn test_assign_reuse__load(skel: *mut test_assign_reuse) -> c_int;
    fn test_assign_reuse__destroy(skel: *mut test_assign_reuse);
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_detach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_GT<T>(left: T, right: T, name: *const c_char) -> bool;
    fn ASSERT_GE<T>(left: T, right: T, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(left: T, right: T, name: *const c_char) -> bool;
    fn ASSERT_NEQ<T>(left: *mut T, right: *mut T, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    // SYS/SYS_NOFAIL are C test macros; preserved as external dependency hooks.
    fn SYS(label: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn SYS_NOFAIL(fmt: *const c_char, ...) -> c_int;
}

unsafe fn errno() -> c_int {
    *errno_location()
}

unsafe fn attach_reuseport(sock_fd: c_int, prog_fd: c_int) -> c_int {
    setsockopt(
        sock_fd,
        SOL_SOCKET,
        SO_ATTACH_REUSEPORT_EBPF,
        &prog_fd as *const c_int as *const c_void,
        core::mem::size_of_val(&prog_fd) as socklen_t,
    )
}

unsafe fn cookie(fd: c_int) -> __u64 {
    let mut cookie: __u64 = 0;
    let mut cookie_len: socklen_t = core::mem::size_of_val(&cookie) as socklen_t;
    let ret: c_int;

    ret = getsockopt(
        fd,
        SOL_SOCKET,
        SO_COOKIE,
        &mut cookie as *mut __u64 as *mut c_void,
        &mut cookie_len,
    );
    ASSERT_OK(ret, b"cookie\0".as_ptr() as *const c_char);
    ASSERT_GT(cookie, 0u64, b"cookie_invalid\0".as_ptr() as *const c_char);

    cookie
}

unsafe fn echo_test_udp(fd_sv: c_int) -> c_int {
    let mut addr: sockaddr_storage = core::mem::zeroed();
    let mut len: socklen_t = core::mem::size_of_val(&addr) as socklen_t;
    let mut buff = [0 as c_char; 1];
    let mut fd_cl: c_int = -1;
    let ret: c_int;

    fd_cl = connect_to_fd(fd_sv, 100);
    ASSERT_GT(fd_cl, 0, b"create_client\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        getsockname(fd_cl, &mut addr as *mut sockaddr_storage as *mut c_void, &mut len),
        0,
        b"getsockname\0".as_ptr() as *const c_char,
    );

    ASSERT_EQ(
        send(fd_cl, buff.as_ptr() as *const c_void, core::mem::size_of_val(&buff), 0),
        1isize,
        b"send_client\0".as_ptr() as *const c_char,
    );

    ret = recv(fd_sv, buff.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buff), 0) as c_int;
    if ret < 0 {
        close(fd_cl);
        return errno();
    }

    ASSERT_EQ(ret, 1, b"recv_server\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        sendto(
            fd_sv,
            buff.as_ptr() as *const c_void,
            core::mem::size_of_val(&buff),
            0,
            &addr as *const sockaddr_storage as *const c_void,
            len,
        ),
        1isize,
        b"send_server\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        recv(fd_cl, buff.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buff), 0),
        1isize,
        b"recv_client\0".as_ptr() as *const c_char,
    );
    close(fd_cl);
    0
}

unsafe fn echo_test_tcp(fd_sv: c_int) -> c_int {
    let mut buff = [0 as c_char; 1];
    let mut fd_cl: c_int = -1;
    let mut fd_sv_cl: c_int = -1;

    fd_cl = connect_to_fd(fd_sv, 100);
    if fd_cl < 0 {
        return errno();
    }

    fd_sv_cl = accept(fd_sv, core::ptr::null_mut(), core::ptr::null_mut());
    ASSERT_GE(fd_sv_cl, 0, b"accept_fd\0".as_ptr() as *const c_char);

    ASSERT_EQ(
        send(fd_cl, buff.as_ptr() as *const c_void, core::mem::size_of_val(&buff), 0),
        1isize,
        b"send_client\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        recv(fd_sv_cl, buff.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buff), 0),
        1isize,
        b"recv_server\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        send(fd_sv_cl, buff.as_ptr() as *const c_void, core::mem::size_of_val(&buff), 0),
        1isize,
        b"send_server\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        recv(fd_cl, buff.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buff), 0),
        1isize,
        b"recv_client\0".as_ptr() as *const c_char,
    );
    close(fd_sv_cl);
    close(fd_cl);
    0
}

#[no_mangle]
pub unsafe extern "C" fn run_assign_reuse(
    family: c_int,
    sotype: c_int,
    ip: *const c_char,
    port: __u16,
) {
    let mut tc_hook = bpf_tc_hook {
        sz: core::mem::size_of::<bpf_tc_hook>(),
        ifindex: LOOPBACK,
        attach_point: BPF_TC_INGRESS,
        parent: 0,
    };
    let mut tc_opts = bpf_tc_opts {
        sz: core::mem::size_of::<bpf_tc_opts>(),
        prog_fd: 0,
        flags: 0,
        prog_id: 0,
        handle: 1,
        priority: 1,
    };
    let mut hook_created = false;
    let mut tc_attached = false;
    let mut ret: c_int;
    let fd_tc: c_int;
    let fd_accept: c_int;
    let fd_drop: c_int;
    let fd_map: c_int;
    let mut fd_sv: *mut c_int = core::ptr::null_mut();
    let mut fd_val: __u64;
    let mut skel: *mut test_assign_reuse;
    let zero: c_int = 0;

    skel = test_assign_reuse__open();
    'cleanup: loop {
        if !ASSERT_OK_PTR(skel, b"skel_open\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        (*(*skel).rodata).dest_port = port;

        ret = test_assign_reuse__load(skel);
        if !ASSERT_OK(ret, b"skel_load\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        ASSERT_EQ(
            (*(*skel).bss).sk_cookie_seen,
            0u64,
            b"cookie_init\0".as_ptr() as *const c_char,
        );

        fd_tc = bpf_program__fd((*skel).progs.tc_main);
        fd_accept = bpf_program__fd((*skel).progs.reuse_accept);
        fd_drop = bpf_program__fd((*skel).progs.reuse_drop);
        fd_map = bpf_map__fd((*skel).maps.sk_map);

        fd_sv = start_reuseport_server(family, sotype, ip, port, 100, 1);
        if !ASSERT_NEQ(
            fd_sv,
            core::ptr::null_mut(),
            b"start_reuseport_server\0".as_ptr() as *const c_char,
        ) {
            break 'cleanup;
        }

        ret = attach_reuseport(*fd_sv, fd_drop);
        if !ASSERT_OK(ret, b"attach_reuseport\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        fd_val = *fd_sv as __u64;
        ret = bpf_map_update_elem(
            fd_map,
            &zero as *const c_int as *const c_void,
            &fd_val as *const __u64 as *const c_void,
            BPF_NOEXIST,
        );
        if !ASSERT_OK(ret, b"bpf_sk_map\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        ret = bpf_tc_hook_create(&mut tc_hook);
        if ret == 0 {
            hook_created = true;
        }
        ret = if ret == -EEXIST { 0 } else { ret };
        if !ASSERT_OK(ret, b"bpf_tc_hook_create\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        tc_opts.prog_fd = fd_tc;
        ret = bpf_tc_attach(&mut tc_hook, &mut tc_opts);
        if !ASSERT_OK(ret, b"bpf_tc_attach\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }
        tc_attached = true;

        if sotype == SOCK_STREAM {
            ASSERT_EQ(
                echo_test_tcp(*fd_sv),
                ECONNREFUSED,
                b"drop_tcp\0".as_ptr() as *const c_char,
            );
        } else {
            ASSERT_EQ(
                echo_test_udp(*fd_sv),
                EAGAIN,
                b"drop_udp\0".as_ptr() as *const c_char,
            );
        }
        ASSERT_EQ(
            (*(*skel).bss).reuseport_executed,
            1,
            b"program executed once\0".as_ptr() as *const c_char,
        );

        (*(*skel).bss).sk_cookie_seen = 0;
        (*(*skel).bss).reuseport_executed = 0;
        ASSERT_OK(
            attach_reuseport(*fd_sv, fd_accept),
            b"attach_reuseport(accept)\0".as_ptr() as *const c_char,
        );

        if sotype == SOCK_STREAM {
            ASSERT_EQ(echo_test_tcp(*fd_sv), 0, b"echo_tcp\0".as_ptr() as *const c_char);
        } else {
            ASSERT_EQ(echo_test_udp(*fd_sv), 0, b"echo_udp\0".as_ptr() as *const c_char);
        }

        ASSERT_EQ(
            (*(*skel).bss).sk_cookie_seen,
            cookie(*fd_sv),
            b"cookie_mismatch\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*(*skel).bss).reuseport_executed,
            1,
            b"program executed once\0".as_ptr() as *const c_char,
        );
        break 'cleanup;
    }

    if tc_attached {
        tc_opts.flags = 0;
        tc_opts.prog_fd = 0;
        tc_opts.prog_id = 0;
        ret = bpf_tc_detach(&mut tc_hook, &mut tc_opts);
        ASSERT_OK(ret, b"bpf_tc_detach\0".as_ptr() as *const c_char);
    }
    if hook_created {
        tc_hook.attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
        bpf_tc_hook_destroy(&mut tc_hook);
    }
    test_assign_reuse__destroy(skel);
    free_fds(fd_sv, 1);
}

#[no_mangle]
pub unsafe extern "C" fn test_assign_reuse() {
    let mut tok: *mut nstoken = core::ptr::null_mut();

    // SYS(out, "ip netns add %s", NS_TEST);
    if SYS(
        b"out\0".as_ptr() as *const c_char,
        b"ip netns add %s\0".as_ptr() as *const c_char,
        NS_TEST.as_ptr() as *const c_char,
    ) != 0
    {
        return;
    }

    'cleanup: loop {
        // SYS(cleanup, "ip -net %s link set dev lo up", NS_TEST);
        if SYS(
            b"cleanup\0".as_ptr() as *const c_char,
            b"ip -net %s link set dev lo up\0".as_ptr() as *const c_char,
            NS_TEST.as_ptr() as *const c_char,
        ) != 0
        {
            break 'cleanup;
        }

        tok = open_netns(NS_TEST.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(tok, b"netns token\0".as_ptr() as *const c_char) {
            return;
        }

        if test__start_subtest(b"tcpv4\0".as_ptr() as *const c_char) {
            run_assign_reuse(
                AF_INET,
                SOCK_STREAM,
                b"127.0.0.1\0".as_ptr() as *const c_char,
                PORT,
            );
        }
        if test__start_subtest(b"tcpv6\0".as_ptr() as *const c_char) {
            run_assign_reuse(
                AF_INET6,
                SOCK_STREAM,
                b"::1\0".as_ptr() as *const c_char,
                PORT,
            );
        }
        if test__start_subtest(b"udpv4\0".as_ptr() as *const c_char) {
            run_assign_reuse(
                AF_INET,
                SOCK_DGRAM,
                b"127.0.0.1\0".as_ptr() as *const c_char,
                PORT,
            );
        }
        if test__start_subtest(b"udpv6\0".as_ptr() as *const c_char) {
            run_assign_reuse(
                AF_INET6,
                SOCK_DGRAM,
                b"::1\0".as_ptr() as *const c_char,
                PORT,
            );
        }
        break 'cleanup;
    }

    close_netns(tok);
    SYS_NOFAIL(
        b"ip netns delete %s\0".as_ptr() as *const c_char,
        NS_TEST.as_ptr() as *const c_char,
    );
}
