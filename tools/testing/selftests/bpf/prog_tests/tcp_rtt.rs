// SPDX-License-Identifier: GPL-2.0
// C dependencies: test_progs.h, cgroup_helpers.h, network_helpers.h, tcp_rtt.skel.h

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;
type socklen_t = c_uint;

const SOL_TCP: c_int = 6;
const TCP_INFO: c_int = 11;
const BPF_CGROUP_SOCK_OPS: c_int = 15;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;

#[repr(C)]
struct tcp_rtt_storage {
    invoked: __u32,
    dsack_dups: __u32,
    delivered: __u32,
    delivered_ce: __u32,
    icsk_retransmits: __u32,

    mrtt_us: __u32, /* args[0] */
    srtt: __u32,    /* args[1] */
}

#[repr(C)]
struct tcp_info {
    tcpi_unacked: __u32,
}

#[repr(C)]
struct tcp_rtt {
    maps: tcp_rtt_maps,
    progs: tcp_rtt_progs,
}

#[repr(C)]
struct tcp_rtt_maps {
    socket_storage_map: *mut c_void,
}

#[repr(C)]
struct tcp_rtt_progs {
    _sockops: *mut c_void,
}

unsafe extern "C" {
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn getsockopt(
        fd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut socklen_t,
    ) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn log_err(fmt: *const c_char, ...);
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_int, flags: c_uint) -> c_int;
    fn tcp_rtt__open_and_load() -> *mut tcp_rtt;
    fn tcp_rtt__destroy(obj: *mut tcp_rtt);
    fn connect_to_fd(server_fd: c_int, timeout_ms: c_uint) -> c_int;
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_void,
        port: c_int,
        timeout_ms: c_uint,
    ) -> c_int;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ASSERT_EQ {
    ($actual:expr, $expected:expr, $msg:literal) => {
        ($actual) == ($expected)
    };
}

macro_rules! ASSERT_GE {
    ($actual:expr, $expected:expr, $msg:literal) => {
        ($actual) >= ($expected)
    };
}

macro_rules! ASSERT_OK {
    ($actual:expr, $msg:literal) => {
        ($actual) == 0
    };
}

macro_rules! ASSERT_OK_PTR {
    ($actual:expr, $msg:literal) => {
        !($actual).is_null()
    };
}

unsafe fn send_byte(fd: c_int) {
    let b: c_char = 0x55;

    ASSERT_EQ!(
        unsafe { write(fd, &b as *const c_char as *const c_void, core::mem::size_of_val(&b)) },
        1,
        "send single byte"
    );
}

unsafe fn wait_for_ack(fd: c_int, retries: c_int) -> c_int {
    let mut info: tcp_info = unsafe { core::mem::zeroed() };
    let mut optlen: socklen_t;
    let mut i: c_int;
    let mut err: c_int;

    i = 0;
    while i < retries {
        optlen = core::mem::size_of_val(&info) as socklen_t;
        err = unsafe {
            getsockopt(
                fd,
                SOL_TCP,
                TCP_INFO,
                &mut info as *mut tcp_info as *mut c_void,
                &mut optlen,
            )
        };
        if err < 0 {
            unsafe { log_err(c_str!("Failed to lookup TCP stats")) };
            return err;
        }

        if info.tcpi_unacked == 0 {
            return 0;
        }

        unsafe { usleep(10) };
        i += 1;
    }

    unsafe { log_err(c_str!("Did not receive ACK")) };
    -1
}

unsafe fn verify_sk(
    map_fd: c_int,
    client_fd: c_int,
    msg: *const c_char,
    invoked: __u32,
    dsack_dups: __u32,
    delivered: __u32,
    delivered_ce: __u32,
    icsk_retransmits: __u32,
) -> c_int {
    let mut err: c_int = 0;
    let mut val: tcp_rtt_storage = unsafe { core::mem::zeroed() };

    if !ASSERT_GE!(
        unsafe {
            bpf_map_lookup_elem(
                map_fd,
                &client_fd as *const c_int as *const c_void,
                &mut val as *mut tcp_rtt_storage as *mut c_void,
            )
        },
        0,
        "read socket storage"
    ) {
        return -1;
    }

    if val.invoked != invoked {
        unsafe {
            log_err(
                c_str!("%s: unexpected bpf_tcp_sock.invoked %d != %d"),
                msg,
                val.invoked,
                invoked,
            )
        };
        err += 1;
    }

    if val.dsack_dups != dsack_dups {
        unsafe {
            log_err(
                c_str!("%s: unexpected bpf_tcp_sock.dsack_dups %d != %d"),
                msg,
                val.dsack_dups,
                dsack_dups,
            )
        };
        err += 1;
    }

    if val.delivered != delivered {
        unsafe {
            log_err(
                c_str!("%s: unexpected bpf_tcp_sock.delivered %d != %d"),
                msg,
                val.delivered,
                delivered,
            )
        };
        err += 1;
    }

    if val.delivered_ce != delivered_ce {
        unsafe {
            log_err(
                c_str!("%s: unexpected bpf_tcp_sock.delivered_ce %d != %d"),
                msg,
                val.delivered_ce,
                delivered_ce,
            )
        };
        err += 1;
    }

    if val.icsk_retransmits != icsk_retransmits {
        unsafe {
            log_err(
                c_str!("%s: unexpected bpf_tcp_sock.icsk_retransmits %d != %d"),
                msg,
                val.icsk_retransmits,
                icsk_retransmits,
            )
        };
        err += 1;
    }

    /* Precise values of mrtt and srtt are unavailable, just make sure they are nonzero */
    if val.mrtt_us == 0 {
        unsafe {
            log_err(
                c_str!("%s: unexpected bpf_tcp_sock.args[0] (mrtt_us) %u == 0"),
                msg,
                val.mrtt_us,
            )
        };
        err += 1;
    }

    if val.srtt == 0 {
        unsafe {
            log_err(
                c_str!("%s: unexpected bpf_tcp_sock.args[1] (srtt) %u == 0"),
                msg,
                val.srtt,
            )
        };
        err += 1;
    }

    err
}

unsafe fn run_test(cgroup_fd: c_int, server_fd: c_int) -> c_int {
    let mut skel: *mut tcp_rtt;
    let client_fd: c_int;
    let prog_fd: c_int;
    let map_fd: c_int;
    let mut err: c_int;

    skel = unsafe { tcp_rtt__open_and_load() };
    if !ASSERT_OK_PTR!(skel, "skel_open_load") {
        return -1;
    }

    map_fd = unsafe { bpf_map__fd((*skel).maps.socket_storage_map) };
    prog_fd = unsafe { bpf_program__fd((*skel).progs._sockops) };

    err = unsafe { bpf_prog_attach(prog_fd, cgroup_fd, BPF_CGROUP_SOCK_OPS, 0) };
    if err != 0 {
        unsafe { log_err(c_str!("Failed to attach BPF program")) };
        unsafe { tcp_rtt__destroy(skel) };
        return err;
    }

    client_fd = unsafe { connect_to_fd(server_fd, 0) };
    if client_fd < 0 {
        err = -1;
        unsafe { tcp_rtt__destroy(skel) };
        return err;
    }

    err += unsafe {
        verify_sk(
            map_fd,
            client_fd,
            c_str!("syn-ack"),
            /*invoked=*/ 1,
            /*dsack_dups=*/ 0,
            /*delivered=*/ 1,
            /*delivered_ce=*/ 0,
            /*icsk_retransmits=*/ 0,
        )
    };

    unsafe { send_byte(client_fd) };
    if unsafe { wait_for_ack(client_fd, 100) } < 0 {
        err = -1;
        unsafe { close(client_fd) };
        unsafe { tcp_rtt__destroy(skel) };
        return err;
    }

    err += unsafe {
        verify_sk(
            map_fd,
            client_fd,
            c_str!("first payload byte"),
            /*invoked=*/ 2,
            /*dsack_dups=*/ 0,
            /*delivered=*/ 2,
            /*delivered_ce=*/ 0,
            /*icsk_retransmits=*/ 0,
        )
    };

    unsafe { close(client_fd) };

    unsafe { tcp_rtt__destroy(skel) };
    err
}

#[no_mangle]
pub unsafe extern "C" fn test_tcp_rtt() {
    let server_fd: c_int;
    let cgroup_fd: c_int;

    cgroup_fd = unsafe { test__join_cgroup(c_str!("/tcp_rtt")) };
    if !ASSERT_GE!(cgroup_fd, 0, "join_cgroup /tcp_rtt") {
        return;
    }

    server_fd = unsafe { start_server(AF_INET, SOCK_STREAM, core::ptr::null(), 0, 0) };
    if !ASSERT_GE!(server_fd, 0, "start_server") {
        unsafe { close(cgroup_fd) };
        return;
    }

    ASSERT_OK!(unsafe { run_test(cgroup_fd, server_fd) }, "run_test");

    unsafe { close(server_fd) };

    unsafe { close(cgroup_fd) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
