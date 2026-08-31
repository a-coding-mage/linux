// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause

/*
 * BPF-based flow shaping
 *
 * The test brings up two veth in two isolated namespaces, attach some flow
 * shaping program onto it, and ensures that a manual speedtest maximum
 * value matches the rate set in the BPF shapers.
 */

// C dependencies translated as external symbols:
// <asm-generic/socket.h>, <stdio.h>, <unistd.h>, <fcntl.h>, <math.h>,
// <sys/time.h>, <sys/socket.h>, <bpf/libbpf.h>, <pthread.h>,
// "test_progs.h", "network_helpers.h", "test_tc_edt.skel.h"

use core::ffi::{c_char, c_double, c_int, c_void};

const SERVER_NS: &[u8] = b"tc-edt-server-ns\0";
const CLIENT_NS: &[u8] = b"tc-edt-client-ns\0";
const IP4_ADDR_VETH1: &[u8] = b"192.168.1.1\0";
const IP4_ADDR_VETH2: &[u8] = b"192.168.1.2\0";
const IP4_ADDR_VETH2_HEX: u32 = 0xC0A80102;

const TIMEOUT_MS: c_int = 2000;
const TEST_PORT: c_int = 9000;
const TARGET_RATE_MBPS: c_double = 5.0;
const TX_BYTES_COUNT: c_int = 1 * 1000 * 1000;
const RATE_ERROR_PERCENT: c_double = 2.0;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;

#[repr(C)]
struct connection {
    server_listen_fd: c_int,
    server_conn_fd: c_int,
    client_conn_fd: c_int,
}

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
struct test_tc_edt {
    progs: *mut test_tc_edt_progs,
    bss: *mut test_tc_edt_bss,
}

#[repr(C)]
struct test_tc_edt_progs {
    tc_prog: *mut bpf_program,
}

#[repr(C)]
struct test_tc_edt_bss {
    target_rate: c_double,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

extern "C" {
    fn make_netns(name: *const c_char) -> c_int;
    fn remove_netns(name: *const c_char);
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn system(cmd: *const c_char) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn tc_prog_attach(ifname: *const c_char, ifindex: c_int, prog_fd: c_int) -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_int) -> c_int;
    fn get_time_ns() -> u64;
    fn send_recv_data(server_fd: c_int, client_fd: c_int, bytes: c_int) -> c_int;
    fn fabs(x: c_double) -> c_double;
    fn test_tc_edt__open_and_load() -> *mut test_tc_edt;
    fn test_tc_edt__destroy(skel: *mut test_tc_edt);

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_LE(left: c_double, right: c_double, name: *const c_char) -> bool;
}

unsafe fn sys(cmd: *const c_char) -> c_int {
    system(cmd)
}

unsafe fn setup(skel: *mut test_tc_edt) -> c_int {
    let mut nstoken_client: *mut nstoken;
    let mut nstoken_server: *mut nstoken;
    let ret: c_int;

    if !ASSERT_OK(make_netns(CLIENT_NS.as_ptr() as *const c_char), b"create client ns\0".as_ptr() as *const c_char) {
        return -1;
    }
    if !ASSERT_OK(make_netns(SERVER_NS.as_ptr() as *const c_char), b"create server ns\0".as_ptr() as *const c_char) {
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }

    nstoken_client = open_netns(CLIENT_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken_client as *const c_void, b"open client ns\0".as_ptr() as *const c_char) {
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    if sys(b"ip link add veth1 type veth peer name veth2 netns tc-edt-server-ns\0".as_ptr() as *const c_char) != 0 {
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    if sys(b"ip -4 addr add 192.168.1.1/24 dev veth1\0".as_ptr() as *const c_char) != 0 {
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    if sys(b"ip link set veth1 up\0".as_ptr() as *const c_char) != 0 {
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }

    nstoken_server = open_netns(SERVER_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken_server as *const c_void, b"enter server ns\0".as_ptr() as *const c_char) {
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    if sys(b"ip -4 addr add 192.168.1.2/24 dev veth2\0".as_ptr() as *const c_char) != 0 {
        close_netns(nstoken_server);
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    if sys(b"ip link set veth2 up\0".as_ptr() as *const c_char) != 0 {
        close_netns(nstoken_server);
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    if sys(b"tc qdisc add dev veth2 root fq\0".as_ptr() as *const c_char) != 0 {
        close_netns(nstoken_server);
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    ret = tc_prog_attach(
        b"veth2\0".as_ptr() as *const c_char,
        -1,
        bpf_program__fd((*(*skel).progs).tc_prog),
    );
    if !ASSERT_OK(ret, b"attach bpf prog\0".as_ptr() as *const c_char) {
        close_netns(nstoken_server);
        close_netns(nstoken_client);
        remove_netns(SERVER_NS.as_ptr() as *const c_char);
        remove_netns(CLIENT_NS.as_ptr() as *const c_char);
        return -1;
    }
    (*(*skel).bss).target_rate = TARGET_RATE_MBPS * 1000.0 * 1000.0;
    close_netns(nstoken_server);
    close_netns(nstoken_client);

    0
}

unsafe fn cleanup() {
    remove_netns(CLIENT_NS.as_ptr() as *const c_char);
    remove_netns(SERVER_NS.as_ptr() as *const c_char);
}

unsafe fn run_test() {
    let server_fd: c_int;
    let client_fd: c_int;
    let err: c_int;
    let rate_mbps: c_double;
    let rate_error: c_double;
    let mut nstoken: *mut nstoken;
    let ts_start: u64;
    let ts_end: u64;

    nstoken = open_netns(SERVER_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"open server ns\0".as_ptr() as *const c_char) {
        return;
    }
    server_fd = start_server(
        AF_INET,
        SOCK_STREAM,
        IP4_ADDR_VETH2.as_ptr() as *const c_char,
        TEST_PORT,
        TIMEOUT_MS,
    );
    if !ASSERT_OK_FD(server_fd, b"start server\0".as_ptr() as *const c_char) {
        return;
    }

    close_netns(nstoken);
    nstoken = open_netns(CLIENT_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, b"open client ns\0".as_ptr() as *const c_char) {
        return;
    }
    client_fd = connect_to_fd(server_fd, 0);
    if !ASSERT_OK_FD(client_fd, b"connect client\0".as_ptr() as *const c_char) {
        return;
    }

    ts_start = get_time_ns();
    err = send_recv_data(server_fd, client_fd, TX_BYTES_COUNT);
    ts_end = get_time_ns();
    close_netns(nstoken);
    ASSERT_OK(err, b"send_recv_data\0".as_ptr() as *const c_char);

    rate_mbps = (TX_BYTES_COUNT as c_double) / (((ts_end - ts_start) as c_double) / 1000.0);
    rate_error = fabs((rate_mbps - TARGET_RATE_MBPS) * 100.0 / TARGET_RATE_MBPS);

    ASSERT_LE(
        rate_error,
        RATE_ERROR_PERCENT,
        b"rate error is lower than threshold\0".as_ptr() as *const c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn test_tc_edt() {
    let skel: *mut test_tc_edt;

    skel = test_tc_edt__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel open and load\0".as_ptr() as *const c_char) {
        return;
    }

    if !ASSERT_OK(setup(skel), b"global setup\0".as_ptr() as *const c_char) {
        return;
    }

    run_test();

    cleanup();
    test_tc_edt__destroy(skel);
}
