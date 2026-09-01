// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/*
 * C dependencies translated as external Rust dependencies:
 * <sys/types.h>, <bpf/bpf.h>, <bpf/libbpf.h>, "test_progs.h",
 * "network_helpers.h", "test_sk_storage_trace_itself.skel.h",
 * and "test_sk_storage_tracing.skel.h".
 */

use core::ffi::{c_char, c_int, c_void};

const LO_ADDR6: &[u8] = b"::1\0";
const TEST_COMM: &[u8] = b"test_progs\0";

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;
const SHUT_WR: c_int = 1;
const BPF_TCP_LISTEN: u32 = 10;
const BPF_TCP_FIN_WAIT2: u32 = 5;
const BPF_TCP_LAST_ACK: u32 = 9;

type __u32 = u32;
type pid_t = c_int;

#[repr(C)]
struct sk_stg {
    pid: __u32,
    last_notclose_state: __u32,
    comm: [c_char; 16],
}

#[repr(C)]
struct test_sk_storage_tracing {
    maps: test_sk_storage_tracing__maps,
    bss: *mut test_sk_storage_tracing__bss,
}

#[repr(C)]
struct test_sk_storage_tracing__maps {
    sk_stg_map: *mut bpf_map,
    del_sk_stg_map: *mut bpf_map,
}

#[repr(C)]
struct test_sk_storage_tracing__bss {
    task_comm: [c_char; 16],
}

enum bpf_map {}
enum test_sk_storage_trace_itself {}

static mut skel: *mut test_sk_storage_tracing = core::ptr::null_mut();
static mut duration: __u32 = 0;
static mut my_pid: pid_t = 0;

extern "C" {
    static mut errno: c_int;

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;

    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, addrlen: *mut c_void) -> c_int;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> pid_t;

    fn test_sk_storage_trace_itself__open_and_load() -> *mut test_sk_storage_trace_itself;
    fn test_sk_storage_trace_itself__destroy(skel: *mut test_sk_storage_trace_itself);
    fn test_sk_storage_tracing__open_and_load() -> *mut test_sk_storage_tracing;
    fn test_sk_storage_tracing__attach(skel: *mut test_sk_storage_tracing) -> c_int;
    fn test_sk_storage_tracing__destroy(skel: *mut test_sk_storage_tracing);

    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u32, expected: __u32, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_NULL(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
}

unsafe fn check_sk_stg(sk_fd: c_int, expected_state: __u32) -> c_int {
    let mut sk_stg: sk_stg = core::mem::zeroed();
    let mut err: c_int;

    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.sk_stg_map),
        &sk_fd as *const c_int as *const c_void,
        &mut sk_stg as *mut sk_stg as *mut c_void,
    );
    if !ASSERT_OK(err, b"map_lookup(sk_stg_map)\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_EQ(
        sk_stg.last_notclose_state,
        expected_state,
        b"last_notclose_state\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    if !ASSERT_EQ(sk_stg.pid, my_pid as __u32, b"pid\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_STREQ(
        sk_stg.comm.as_ptr(),
        (*(*skel).bss).task_comm.as_ptr(),
        b"task_comm\0".as_ptr() as *const c_char,
    ) {
        return -1;
    }

    0
}

unsafe fn do_test() {
    let mut listen_fd: c_int = -1;
    let mut passive_fd: c_int = -1;
    let mut active_fd: c_int = -1;
    let mut value: c_int = 1;
    let mut err: c_int;
    let mut abyte: c_char = 0;

    listen_fd = start_server(AF_INET6, SOCK_STREAM, LO_ADDR6.as_ptr() as *const c_char, 0, 0);
    if CHECK(
        listen_fd == -1,
        b"start_server\0".as_ptr() as *const c_char,
        b"listen_fd:%d errno:%d\n\0".as_ptr() as *const c_char,
        listen_fd,
        errno,
    ) {
        return;
    }

    active_fd = connect_to_fd(listen_fd, 0);
    if CHECK(
        active_fd == -1,
        b"connect_to_fd\0".as_ptr() as *const c_char,
        b"active_fd:%d errno:%d\n\0".as_ptr() as *const c_char,
        active_fd,
        errno,
    ) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    err = bpf_map_update_elem(
        bpf_map__fd((*skel).maps.del_sk_stg_map),
        &active_fd as *const c_int as *const c_void,
        &value as *const c_int as *const c_void,
        0,
    );
    if !ASSERT_OK(err, b"map_update(del_sk_stg_map)\0".as_ptr() as *const c_char) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    passive_fd = accept(listen_fd, core::ptr::null_mut(), core::ptr::null_mut());
    if CHECK(
        passive_fd == -1,
        b"accept\0".as_ptr() as *const c_char,
        b"passive_fd:%d errno:%d\n\0".as_ptr() as *const c_char,
        passive_fd,
        errno,
    ) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    shutdown(active_fd, SHUT_WR);
    err = read(
        passive_fd,
        &mut abyte as *mut c_char as *mut c_void,
        1,
    ) as c_int;
    if !ASSERT_OK(err, b"read(passive_fd)\0".as_ptr() as *const c_char) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    shutdown(passive_fd, SHUT_WR);
    err = read(active_fd, &mut abyte as *mut c_char as *mut c_void, 1) as c_int;
    if !ASSERT_OK(err, b"read(active_fd)\0".as_ptr() as *const c_char) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.del_sk_stg_map),
        &active_fd as *const c_int as *const c_void,
        &mut value as *mut c_int as *mut c_void,
    );
    if !ASSERT_ERR(err, b"map_lookup(del_sk_stg_map)\0".as_ptr() as *const c_char) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    err = check_sk_stg(listen_fd, BPF_TCP_LISTEN);
    if !ASSERT_OK(err, b"listen_fd sk_stg\0".as_ptr() as *const c_char) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    err = check_sk_stg(active_fd, BPF_TCP_FIN_WAIT2);
    if !ASSERT_OK(err, b"active_fd sk_stg\0".as_ptr() as *const c_char) {
        goto_out(active_fd, passive_fd, listen_fd);
        return;
    }

    err = check_sk_stg(passive_fd, BPF_TCP_LAST_ACK);
    ASSERT_OK(err, b"passive_fd sk_stg\0".as_ptr() as *const c_char);

    goto_out(active_fd, passive_fd, listen_fd);
}

unsafe fn goto_out(active_fd: c_int, passive_fd: c_int, listen_fd: c_int) {
    if active_fd != -1 {
        close(active_fd);
    }
    if passive_fd != -1 {
        close(passive_fd);
    }
    if listen_fd != -1 {
        close(listen_fd);
    }
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_sk_storage_tracing() {
    let mut skel_itself: *mut test_sk_storage_trace_itself;
    let mut err: c_int;

    my_pid = getpid();

    skel_itself = test_sk_storage_trace_itself__open_and_load();

    if !ASSERT_NULL(
        skel_itself as *const c_void,
        b"test_sk_storage_trace_itself\0".as_ptr() as *const c_char,
    ) {
        test_sk_storage_trace_itself__destroy(skel_itself);
        return;
    }

    skel = test_sk_storage_tracing__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"test_sk_storage_tracing\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    err = test_sk_storage_tracing__attach(skel);
    if !ASSERT_OK(
        err,
        b"test_sk_storage_tracing__attach\0".as_ptr() as *const c_char,
    ) {
        test_sk_storage_tracing__destroy(skel);
        return;
    }

    do_test();

    test_sk_storage_tracing__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
