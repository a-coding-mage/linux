/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * C dependencies removed from executable Rust:
 * <sys/socket.h>, <linux/bitops.h>, <linux/kernel.h>, <linux/vm_sockets.h>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

/* All known vsock transports, see callers of vsock_core_register() */
/* KNOWN_TRANSPORTS(x):
 * x(LOOPBACK, "loopback")
 * x(VIRTIO, "virtio")
 * x(VHOST, "vhost")
 * x(VMCI, "vmci")
 * x(HYPERV, "hvs")
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum transport {
    TRANSPORT_COUNTER_BASE = 1,
    TRANSPORT_LOOPBACK = 1 << 0,
    TRANSPORT_VIRTIO = 1 << 1,
    TRANSPORT_VHOST = 1 << 2,
    TRANSPORT_VMCI = 1 << 3,
    TRANSPORT_HYPERV = 1 << 4,
    TRANSPORT_NUM = 5,
}

pub const transport_ksyms: [*const c_char; transport::TRANSPORT_NUM as usize] = [
    b" loopback_transport\0".as_ptr() as *const c_char,
    b" virtio_transport\0".as_ptr() as *const c_char,
    b" vhost_transport\0".as_ptr() as *const c_char,
    b" vmci_transport\0".as_ptr() as *const c_char,
    b" hvs_transport\0".as_ptr() as *const c_char,
];

/* C static_assert equivalents:
 * ARRAY_SIZE(transport_ksyms) == TRANSPORT_NUM
 * BITS_PER_TYPE(int) >= TRANSPORT_NUM
 */
const _: [(); transport::TRANSPORT_NUM as usize] =
    [(); transport_ksyms.len()];
const _: [(); 1] =
    [(); ((c_int::BITS as usize) >= transport::TRANSPORT_NUM as usize) as usize];

pub const TRANSPORTS_G2H: c_int = transport::TRANSPORT_VIRTIO as c_int
    | transport::TRANSPORT_VMCI as c_int
    | transport::TRANSPORT_HYPERV as c_int;
pub const TRANSPORTS_H2G: c_int =
    transport::TRANSPORT_VHOST as c_int | transport::TRANSPORT_VMCI as c_int;
pub const TRANSPORTS_LOCAL: c_int = transport::TRANSPORT_LOOPBACK as c_int;

/* Tests can either run as the client or the server */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum test_mode {
    TEST_MODE_UNSET,
    TEST_MODE_CLIENT,
    TEST_MODE_SERVER,
}

pub const DEFAULT_PEER_PORT: c_uint = 1234;

/* Test runner options */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_opts {
    pub mode: test_mode,
    pub peer_cid: c_uint,
    pub peer_port: c_uint,
}

/* A test case definition.  Test functions must print failures to stderr and
 * terminate with exit(EXIT_FAILURE).
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case {
    pub name: *const c_char, /* human-readable name */

    /* Called when test mode is TEST_MODE_CLIENT */
    pub run_client: Option<unsafe extern "C" fn(opts: *const test_opts)>,

    /* Called when test mode is TEST_MODE_SERVER */
    pub run_server: Option<unsafe extern "C" fn(opts: *const test_opts)>,

    pub skip: bool,
}

unsafe extern "C" {
    pub fn init_signals();
    pub fn parse_cid(str: *const c_char) -> c_uint;
    pub fn parse_port(str: *const c_char) -> c_uint;
    pub fn vsock_connect_fd(fd: c_int, cid: c_uint, port: c_uint) -> c_int;
    pub fn vsock_connect(cid: c_uint, port: c_uint, type_: c_int) -> c_int;
    pub fn vsock_accept(
        cid: c_uint,
        port: c_uint,
        clientaddrp: *mut libc::sockaddr_vm,
        type_: c_int,
    ) -> c_int;
    pub fn vsock_stream_connect(cid: c_uint, port: c_uint) -> c_int;
    pub fn vsock_bind_try(cid: c_uint, port: c_uint, type_: c_int) -> c_int;
    pub fn vsock_bind(cid: c_uint, port: c_uint, type_: c_int) -> c_int;
    pub fn vsock_bind_connect(
        cid: c_uint,
        port: c_uint,
        bind_port: c_uint,
        type_: c_int,
    ) -> c_int;
    pub fn vsock_seqpacket_connect(cid: c_uint, port: c_uint) -> c_int;
    pub fn vsock_stream_accept(
        cid: c_uint,
        port: c_uint,
        clientaddrp: *mut libc::sockaddr_vm,
    ) -> c_int;
    pub fn vsock_stream_listen(cid: c_uint, port: c_uint) -> c_int;
    pub fn vsock_seqpacket_accept(
        cid: c_uint,
        port: c_uint,
        clientaddrp: *mut libc::sockaddr_vm,
    ) -> c_int;
    pub fn vsock_wait_remote_close(fd: c_int);
    pub fn vsock_ioctl_int(fd: c_int, op: c_ulong, expected: c_int) -> bool;
    pub fn vsock_wait_sent(fd: c_int) -> bool;
    pub fn send_buf(
        fd: c_int,
        buf: *const c_void,
        len: usize,
        flags: c_int,
        expected_ret: isize,
    );
    pub fn recv_buf(
        fd: c_int,
        buf: *mut c_void,
        len: usize,
        flags: c_int,
        expected_ret: isize,
    );
    pub fn send_byte(fd: c_int, expected_ret: c_int, flags: c_int);
    pub fn recv_byte(fd: c_int, expected_ret: c_int, flags: c_int);
    pub fn run_tests(test_cases: *const test_case, opts: *const test_opts);
    pub fn list_tests(test_cases: *const test_case);
    pub fn skip_test(
        test_cases: *mut test_case,
        test_cases_len: usize,
        test_id_str: *const c_char,
    );
    pub fn pick_test(
        test_cases: *mut test_case,
        test_cases_len: usize,
        test_id_str: *const c_char,
    );
    pub fn hash_djb2(data: *const c_void, len: usize) -> c_ulong;
    pub fn iovec_bytes(iov: *const libc::iovec, iovnum: usize) -> usize;
    pub fn iovec_hash_djb2(iov: *const libc::iovec, iovnum: usize) -> c_ulong;
    pub fn alloc_test_iovec(test_iovec: *const libc::iovec, iovnum: c_int) -> *mut libc::iovec;
    pub fn free_test_iovec(
        test_iovec: *const libc::iovec,
        iovec: *mut libc::iovec,
        iovnum: c_int,
    );
    pub fn setsockopt_ull_check(
        fd: c_int,
        level: c_int,
        optname: c_int,
        val: c_ulonglong,
        errmsg: *const c_char,
    );
    pub fn setsockopt_int_check(
        fd: c_int,
        level: c_int,
        optname: c_int,
        val: c_int,
        errmsg: *const c_char,
    );
    pub fn setsockopt_timeval_check(
        fd: c_int,
        level: c_int,
        optname: c_int,
        val: libc::timeval,
        errmsg: *const c_char,
    );
    pub fn enable_so_zerocopy_check(fd: c_int);
    pub fn enable_so_linger(fd: c_int, timeout: c_int);
    pub fn get_transports() -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
