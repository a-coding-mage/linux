// SPDX-License-Identifier: GPL-2.0-only
/*
 * vsock_test - vsock.ko test suite
 *
 * Copyright (C) 2017 Red Hat, Inc.
 *
 * Author: Stefan Hajnoczi <stefanha@redhat.com>
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;
type time_t = i64;
type pid_t = c_int;
type pthread_t = c_ulong;
type sig_atomic_t = c_int;
type __sighandler_t = Option<unsafe extern "C" fn(c_int)>;
type FILE = c_void;

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_vm {
    svm_family: u16,
    svm_reserved1: u16,
    svm_port: c_uint,
    svm_cid: c_uint,
    svm_zero: [u8; 4],
}

#[repr(C)]
union sockaddr_any {
    sa: sockaddr,
    svm: sockaddr_vm,
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: time_t,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: time_t,
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct msghdr {
    msg_name: *mut c_void,
    msg_namelen: socklen_t,
    msg_iov: *mut iovec,
    msg_iovlen: size_t,
    msg_control: *mut c_void,
    msg_controllen: size_t,
    msg_flags: c_int,
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct sigaction {
    sa_handler: __sighandler_t,
    sa_mask: [c_ulong; 16],
    sa_flags: c_int,
    sa_restorer: *mut c_void,
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct test_opts {
    mode: c_int,
    peer_cid: c_uint,
    peer_port: c_uint,
}

#[repr(C)]
struct test_case {
    name: *const c_char,
    run_client: Option<unsafe fn(*const test_opts)>,
    run_server: Option<unsafe fn(*const test_opts)>,
    skip: bool,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn accept(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn shutdown(fd: c_int, how: c_int) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, len: size_t) -> ssize_t;
    fn getsockopt(fd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut socklen_t) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getpid() -> pid_t;
    fn signal(sig: c_int, handler: __sighandler_t) -> __sighandler_t;
    fn sigaction(sig: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_setcanceltype(ty: c_int, oldtype: *mut c_int) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn getpagesize() -> c_int;
    fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, off: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
    fn exit(status: c_int) -> !;
}

unsafe extern "C" {
    fn timeout_begin(t: c_int);
    fn timeout_check(s: *const c_char);
    fn timeout_end();
    fn timeout_usleep(usec: c_int);
    fn control_expectln(s: *const c_char);
    fn control_writeln(s: *const c_char);
    fn control_writeulong(v: c_ulong);
    fn control_readulong() -> c_ulong;
    fn control_init(host: *const c_char, port: *const c_char, server: bool);
    fn control_cleanup();
    fn vsock_bind(cid: c_uint, port: c_uint, ty: c_int) -> c_int;
    fn vsock_bind_try(cid: c_uint, port: c_uint, ty: c_int) -> c_int;
    fn vsock_stream_connect(cid: c_uint, port: c_uint) -> c_int;
    fn vsock_stream_accept(cid: c_uint, port: c_uint, peeraddr: *mut sockaddr_vm) -> c_int;
    fn vsock_stream_listen(cid: c_uint, port: c_uint) -> c_int;
    fn vsock_seqpacket_connect(cid: c_uint, port: c_uint) -> c_int;
    fn vsock_seqpacket_accept(cid: c_uint, port: c_uint, peeraddr: *mut sockaddr_vm) -> c_int;
    fn vsock_accept(cid: c_uint, port: c_uint, peeraddr: *mut sockaddr_vm, ty: c_int) -> c_int;
    fn vsock_connect(cid: c_uint, port: c_uint, ty: c_int) -> c_int;
    fn vsock_bind_connect(cid: c_uint, port: c_uint, bind_port: c_uint, ty: c_int) -> c_int;
    fn vsock_connect_fd(fd: c_int, cid: c_uint, port: c_uint) -> c_int;
    fn vsock_wait_remote_close(fd: c_int);
    fn vsock_wait_sent(fd: c_int) -> bool;
    fn vsock_ioctl_int(fd: c_int, request: c_ulong, expected: c_int) -> bool;
    fn send_byte(fd: c_int, expected_ret: c_int, flags: c_int);
    fn recv_byte(fd: c_int, expected_ret: c_int, flags: c_int);
    fn send_buf(fd: c_int, buf: *const c_void, len: size_t, flags: c_int, expected_ret: ssize_t);
    fn recv_buf(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int, expected_ret: ssize_t);
    fn hash_djb2(buf: *const c_void, len: size_t) -> c_ulong;
    fn setsockopt_ull_check(fd: c_int, level: c_int, optname: c_int, val: c_ulonglong, msg: *const c_char);
    fn setsockopt_int_check(fd: c_int, level: c_int, optname: c_int, val: c_int, msg: *const c_char);
    fn setsockopt_timeval_check(fd: c_int, level: c_int, optname: c_int, val: timeval, msg: *const c_char);
    fn enable_so_zerocopy_check(fd: c_int);
    fn enable_so_linger(fd: c_int, timeout: c_int);
    fn get_transports() -> c_int;
    fn init_signals();
    fn parse_cid(s: *const c_char) -> c_uint;
    fn parse_port(s: *const c_char) -> c_uint;
    fn list_tests(t: *mut test_case);
    fn skip_test(t: *mut test_case, n: size_t, id: *const c_char);
    fn pick_test(t: *mut test_case, n: size_t, id: *const c_char);
    fn run_tests(t: *mut test_case, opts: *const test_opts);
    fn test_stream_msgzcopy_client(opts: *const test_opts);
    fn test_stream_msgzcopy_server(opts: *const test_opts);
    fn test_seqpacket_msgzcopy_client(opts: *const test_opts);
    fn test_seqpacket_msgzcopy_server(opts: *const test_opts);
    fn test_stream_msgzcopy_empty_errq_client(opts: *const test_opts);
    fn test_stream_msgzcopy_empty_errq_server(opts: *const test_opts);
    fn test_stream_msgzcopy_mangle_client(opts: *const test_opts);
    fn test_stream_msgzcopy_mangle_server(opts: *const test_opts);
}

const CONTROL_CONTINUE: c_ulong = 1;
const CONTROL_DONE: c_ulong = 0;
const MULTICONN_NFDS: usize = 100;
const MSG_PEEK_BUF_LEN: usize = 64;
const SOCK_BUF_SIZE: usize = 2 * 1024 * 1024;
const SOCK_BUF_SIZE_SMALL: usize = 64 * 1024;
const MAX_MSG_PAGES: usize = 4;
const MESSAGE_TRUNC_SZ: usize = 32;
const RCVTIMEO_TIMEOUT_SEC: time_t = 1;
const READ_OVERHEAD_NSEC: time_t = 250000000;
const BUF_PATTERN_1: c_int = b'a' as c_int;
const BUF_PATTERN_2: c_int = b'b' as c_int;
const RCVLOWAT_BUF_SIZE: usize = 128;
const INV_BUF_TEST_DATA_LEN: usize = 512;
const HELLO_STR: &[u8] = b"HELLO\0";
const WORLD_STR: &[u8] = b"WORLD\0";
const HELLOWORLD_STR: &[u8] = b"HELLOWORLD\0";
const SEND_SLEEP_USEC: c_int = 10 * 1000;
const MSG_BUF_IOCTL_LEN: usize = 64;
const RCVLOWAT_CREDIT_UPD_BUF_SIZE: usize = 1024 * 128;
/* This define is the same as in 'include/linux/virtio_vsock.h':
 * it is used to decide when to send credit update message during
 * reading from rx queue of a socket. Value and its usage in
 * kernel is important for this test.
 */
const VIRTIO_VSOCK_MAX_PKT_BUF_SIZE: usize = 1024 * 64;
const ACCEPTQ_LEAK_RACE_TIMEOUT: time_t = 2;
const MAX_PAGE_ORDER: usize = 10;
const PAGE_SIZE: usize = 4096;
const MAX_PORT_RETRIES: usize = 24;
const TRANSPORT_CHANGE_TIMEOUT: time_t = 2;
const LINGER_TIMEOUT: c_int = TIMEOUT / 2;
const COLLAPSE_PKT_MIN: usize = 129;
const COLLAPSE_PKT_MAX: usize = 512;
const COLLAPSE_TOTAL: usize = 2 * 1024 * 1024;

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const AF_VSOCK: c_int = 40;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const SOCK_SEQPACKET: c_int = 5;
const SOCK_NONBLOCK: c_int = 0o0004000;
const SOL_SOCKET: c_int = 1;
const SO_RCVTIMEO: c_int = 20;
const SO_RCVLOWAT: c_int = 18;
const SO_VM_SOCKETS_BUFFER_SIZE: c_int = 0;
const SO_VM_SOCKETS_BUFFER_MAX_SIZE: c_int = 2;
const VMADDR_CID_ANY: c_uint = 0xFFFF_FFFF;
const VMADDR_CID_HYPERVISOR: c_int = 0;
const VMADDR_CID_HOST: c_int = 2;
const VMADDR_PORT_ANY: c_uint = 0xFFFF_FFFF;
const DEFAULT_PEER_PORT: c_uint = 1234;
const TIMEOUT: c_int = 30;
const TEST_MODE_UNSET: c_int = 0;
const TEST_MODE_CLIENT: c_int = 1;
const TEST_MODE_SERVER: c_int = 2;
const MSG_PEEK: c_int = 0x02;
const MSG_DONTWAIT: c_int = 0x40;
const MSG_EOR: c_int = 0x80;
const MSG_TRUNC: c_int = 0x20;
const MSG_NOSIGNAL: c_int = 0x4000;
const MSG_ZEROCOPY: c_int = 0x4000000;
const EAGAIN: c_int = 11;
const EWOULDBLOCK: c_int = EAGAIN;
const EINTR: c_int = 4;
const ECONNRESET: c_int = 104;
const EPIPE: c_int = 32;
const EFAULT: c_int = 14;
const EMSGSIZE: c_int = 90;
const ENOMEM: c_int = 12;
const EADDRNOTAVAIL: c_int = 99;
const ENODEV: c_int = 19;
const NSEC_PER_SEC: time_t = 1_000_000_000;
const CLOCK_REALTIME: c_int = 0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const POLLIN: c_short = 0x0001;
const POLLERR: c_short = 0x0008;
const POLLHUP: c_short = 0x0010;
const POLLRDNORM: c_short = 0x0040;
const POLLRDHUP: c_short = 0x2000;
const SHUT_RD: c_int = 0;
const SHUT_WR: c_int = 1;
const SIGPIPE: c_int = 13;
const SIGUSR1: c_int = 10;
const SIG_ERR: __sighandler_t = None;
const PTHREAD_CANCEL_ASYNCHRONOUS: c_int = 1;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const O_NONBLOCK: c_int = 0o0004000;
const SIOCINQ: c_ulong = 0x541B;
const TRANSPORT_VIRTIO: c_int = 1;
const TRANSPORTS_G2H: c_int = 0x2;
const required_argument: c_int = 1;
const no_argument: c_int = 0;

fn cmsg_space(length: usize) -> usize {
    (length + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

unsafe fn die_perror(s: *const c_char) -> ! {
    perror(s);
    exit(EXIT_FAILURE);
}

unsafe fn test_stream_connection_reset(opts: *const test_opts) {
    let mut addr = sockaddr_any { svm: sockaddr_vm { svm_family: AF_VSOCK as u16, svm_reserved1: 0, svm_port: (*opts).peer_port, svm_cid: (*opts).peer_cid, svm_zero: [0; 4] } };
    let fd = socket(AF_VSOCK, SOCK_STREAM, 0);
    let mut ret: c_int;
    timeout_begin(TIMEOUT);
    loop {
        ret = connect(fd, &addr.sa, size_of::<sockaddr_vm>() as socklen_t);
        timeout_check(c"connect".as_ptr());
        if !(ret < 0 && errno == EINTR) { break; }
    }
    timeout_end();
    if ret != -1 {
        fprintf(stderr, c"expected connect(2) failure, got %d\n".as_ptr(), ret);
        exit(EXIT_FAILURE);
    }
    if errno != ECONNRESET {
        fprintf(stderr, c"unexpected connect(2) errno %d\n".as_ptr(), errno);
        exit(EXIT_FAILURE);
    }
    close(fd);
}

unsafe fn test_stream_bind_only_client(opts: *const test_opts) {
    let mut addr = sockaddr_any { svm: sockaddr_vm { svm_family: AF_VSOCK as u16, svm_reserved1: 0, svm_port: (*opts).peer_port, svm_cid: (*opts).peer_cid, svm_zero: [0; 4] } };
    control_expectln(c"BIND".as_ptr());
    let fd = socket(AF_VSOCK, SOCK_STREAM, 0);
    let mut ret: c_int;
    timeout_begin(TIMEOUT);
    loop {
        ret = connect(fd, &addr.sa, size_of::<sockaddr_vm>() as socklen_t);
        timeout_check(c"connect".as_ptr());
        if !(ret < 0 && errno == EINTR) { break; }
    }
    timeout_end();
    if ret != -1 {
        fprintf(stderr, c"expected connect(2) failure, got %d\n".as_ptr(), ret);
        exit(EXIT_FAILURE);
    }
    if errno != ECONNRESET {
        fprintf(stderr, c"unexpected connect(2) errno %d\n".as_ptr(), errno);
        exit(EXIT_FAILURE);
    }
    control_writeln(c"DONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_bind_only_server(opts: *const test_opts) {
    let fd = vsock_bind(VMADDR_CID_ANY, (*opts).peer_port, SOCK_STREAM);
    control_writeln(c"BIND".as_ptr());
    control_expectln(c"DONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_client_close_client(opts: *const test_opts) {
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    send_byte(fd, 1, 0);
    close(fd);
}

unsafe fn test_stream_client_close_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    /* Wait for the remote to close the connection, before check
     * -EPIPE error on send.
     */
    vsock_wait_remote_close(fd);
    send_byte(fd, -EPIPE, 0);
    recv_byte(fd, 1, 0);
    recv_byte(fd, 0, 0);
    close(fd);
}

unsafe fn test_stream_server_close_client(opts: *const test_opts) {
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    /* Wait for the remote to close the connection, before check
     * -EPIPE error on send.
     */
    vsock_wait_remote_close(fd);
    send_byte(fd, -EPIPE, 0);
    recv_byte(fd, 1, 0);
    recv_byte(fd, 0, 0);
    close(fd);
}

unsafe fn test_stream_server_close_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    send_byte(fd, 1, 0);
    close(fd);
}

unsafe fn test_stream_multiconn_client(opts: *const test_opts) {
    let mut fds = [0 as c_int; MULTICONN_NFDS];
    for i in 0..MULTICONN_NFDS {
        fds[i] = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
        if fds[i] < 0 { die_perror(c"connect".as_ptr()); }
    }
    for i in 0..MULTICONN_NFDS {
        if i % 2 != 0 { recv_byte(fds[i], 1, 0); } else { send_byte(fds[i], 1, 0); }
    }
    for i in 0..MULTICONN_NFDS { close(fds[i]); }
}

unsafe fn test_stream_multiconn_server(opts: *const test_opts) {
    let mut fds = [0 as c_int; MULTICONN_NFDS];
    for i in 0..MULTICONN_NFDS {
        fds[i] = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
        if fds[i] < 0 { die_perror(c"accept".as_ptr()); }
    }
    for i in 0..MULTICONN_NFDS {
        if i % 2 != 0 { send_byte(fds[i], 1, 0); } else { recv_byte(fds[i], 1, 0); }
    }
    for i in 0..MULTICONN_NFDS { close(fds[i]); }
}

unsafe fn test_msg_peek_client(opts: *const test_opts, seqpacket: bool) {
    let mut buf = [0u8; MSG_PEEK_BUF_LEN];
    let fd = if seqpacket { vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port) } else { vsock_stream_connect((*opts).peer_cid, (*opts).peer_port) };
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    for i in 0..buf.len() { buf[i] = (rand() & 0xFF) as u8; }
    control_expectln(c"SRVREADY".as_ptr());
    send_buf(fd, buf.as_ptr() as *const c_void, buf.len(), 0, buf.len() as ssize_t);
    close(fd);
}

unsafe fn test_msg_peek_server(opts: *const test_opts, seqpacket: bool) {
    let mut buf_half = [0u8; MSG_PEEK_BUF_LEN / 2];
    let mut buf_normal = [0u8; MSG_PEEK_BUF_LEN];
    let mut buf_peek = [0u8; MSG_PEEK_BUF_LEN];
    let fd = if seqpacket { vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut()) } else { vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut()) };
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    /* Peek from empty socket. */
    recv_buf(fd, buf_peek.as_mut_ptr() as *mut c_void, buf_peek.len(), MSG_PEEK | MSG_DONTWAIT, -EAGAIN as ssize_t);
    control_writeln(c"SRVREADY".as_ptr());
    /* Peek part of data. */
    recv_buf(fd, buf_half.as_mut_ptr() as *mut c_void, buf_half.len(), MSG_PEEK, buf_half.len() as ssize_t);
    /* Peek whole data. */
    recv_buf(fd, buf_peek.as_mut_ptr() as *mut c_void, buf_peek.len(), MSG_PEEK, buf_peek.len() as ssize_t);
    /* Compare partial and full peek. */
    if memcmp(buf_half.as_ptr() as *const c_void, buf_peek.as_ptr() as *const c_void, buf_half.len()) != 0 {
        fprintf(stderr, c"Partial peek data mismatch\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    if seqpacket {
        /* This type of socket supports MSG_TRUNC flag,
         * so check it with MSG_PEEK. We must get length
         * of the message.
         */
        recv_buf(fd, buf_half.as_mut_ptr() as *mut c_void, buf_half.len(), MSG_PEEK | MSG_TRUNC, buf_peek.len() as ssize_t);
    }
    recv_buf(fd, buf_normal.as_mut_ptr() as *mut c_void, buf_normal.len(), 0, buf_normal.len() as ssize_t);
    /* Compare full peek and normal read. */
    if memcmp(buf_peek.as_ptr() as *const c_void, buf_normal.as_ptr() as *const c_void, buf_peek.len()) != 0 {
        fprintf(stderr, c"Full peek data mismatch\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    close(fd);
}

unsafe fn test_stream_msg_peek_client(opts: *const test_opts) { test_msg_peek_client(opts, false); }
unsafe fn test_stream_msg_peek_server(opts: *const test_opts) { test_msg_peek_server(opts, false); }

unsafe fn test_stream_peek_after_recv_server(opts: *const test_opts) {
    let mut buf_normal = [0u8; MSG_PEEK_BUF_LEN];
    let mut buf_peek = [0u8; MSG_PEEK_BUF_LEN];
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    control_writeln(c"SRVREADY".as_ptr());
    /* Partial recv to advance offset within the skb */
    recv_buf(fd, buf_normal.as_mut_ptr() as *mut c_void, 1, 0, 1);
    /* Peek with a buffer larger than the remaining data */
    recv_buf(fd, buf_peek.as_mut_ptr() as *mut c_void, buf_peek.len(), MSG_PEEK, (buf_peek.len() - 1) as ssize_t);
    /* Consume the remaining data */
    recv_buf(fd, buf_normal.as_mut_ptr() as *mut c_void, buf_normal.len() - 1, 0, (buf_normal.len() - 1) as ssize_t);
    /* Compare full peek and normal read. */
    if memcmp(buf_peek.as_ptr() as *const c_void, buf_normal.as_ptr() as *const c_void, buf_peek.len() - 1) != 0 {
        fprintf(stderr, c"Full peek data mismatch\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    close(fd);
}

unsafe fn test_seqpacket_msg_bounds_client(opts: *const test_opts) {
    let fd = vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    let sock_buf_size = SOCK_BUF_SIZE as c_ulonglong;
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_MAX_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_MAX_SIZE)".as_ptr());
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_SIZE)".as_ptr());
    /* Wait, until receiver sets buffer size. */
    control_expectln(c"SRVREADY".as_ptr());
    let mut curr_hash: c_ulong = 0;
    let page_size = getpagesize() as usize;
    let max_msg_size = MAX_MSG_PAGES * page_size;
    let msg_count = SOCK_BUF_SIZE / max_msg_size;
    for i in 0..msg_count {
        let buf_size = if i & 1 != 0 { page_size + (rand() as usize % (max_msg_size - page_size)) } else { 1 + (rand() as usize % page_size) };
        let buf = malloc(buf_size);
        if buf.is_null() { die_perror(c"malloc".as_ptr()); }
        memset(buf, rand() & 0xff, buf_size);
        let flags: c_int;
        if i == msg_count / 2 || (rand() & 1) != 0 {
            flags = MSG_EOR;
            curr_hash = curr_hash.wrapping_add(1);
        } else {
            flags = 0;
        }
        send_buf(fd, buf, buf_size, flags, buf_size as ssize_t);
        /*
         * Hash sum is computed at both client and server in
         * the same way:
         * H += hash('message data')
         * Such hash "controls" both data integrity and message
         * bounds. After data exchange, both sums are compared
         * using control socket, and if message bounds wasn't
         * broken - two values must be equal.
         */
        curr_hash = curr_hash.wrapping_add(hash_djb2(buf, buf_size));
        free(buf);
    }
    control_writeln(c"SENDDONE".as_ptr());
    control_writeulong(curr_hash);
    close(fd);
}

unsafe fn test_seqpacket_msg_bounds_server(opts: *const test_opts) {
    let fd = vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    let sock_buf_size = SOCK_BUF_SIZE as c_ulonglong;
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_MAX_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_MAX_SIZE)".as_ptr());
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_SIZE)".as_ptr());
    /* Ready to receive data. */
    control_writeln(c"SRVREADY".as_ptr());
    /* Wait, until peer sends whole data. */
    control_expectln(c"SENDDONE".as_ptr());
    let mut msg: msghdr = zeroed();
    let mut iov: iovec = zeroed();
    iov.iov_len = MAX_MSG_PAGES * getpagesize() as usize;
    iov.iov_base = malloc(iov.iov_len);
    if iov.iov_base.is_null() { die_perror(c"malloc".as_ptr()); }
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    let mut curr_hash: c_ulong = 0;
    loop {
        let recv_size = recvmsg(fd, &mut msg, 0);
        if recv_size == 0 { break; }
        if recv_size < 0 { die_perror(c"recvmsg".as_ptr()); }
        if msg.msg_flags & MSG_EOR != 0 { curr_hash = curr_hash.wrapping_add(1); }
        curr_hash = curr_hash.wrapping_add(hash_djb2((*msg.msg_iov).iov_base, recv_size as usize));
    }
    free(iov.iov_base);
    close(fd);
    let remote_hash = control_readulong();
    if curr_hash != remote_hash {
        fprintf(stderr, c"Message bounds broken\n".as_ptr());
        exit(EXIT_FAILURE);
    }
}

unsafe fn test_seqpacket_msg_trunc_client(opts: *const test_opts) {
    let mut buf = [0i8; MESSAGE_TRUNC_SZ];
    let fd = vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    send_buf(fd, buf.as_mut_ptr() as *const c_void, buf.len(), 0, buf.len() as ssize_t);
    control_writeln(c"SENDDONE".as_ptr());
    close(fd);
}

unsafe fn test_seqpacket_msg_trunc_server(opts: *const test_opts) {
    let mut buf = [0i8; MESSAGE_TRUNC_SZ / 2];
    let mut msg: msghdr = zeroed();
    let mut iov: iovec = zeroed();
    let fd = vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    control_expectln(c"SENDDONE".as_ptr());
    iov.iov_base = buf.as_mut_ptr() as *mut c_void;
    iov.iov_len = buf.len();
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    let ret = recvmsg(fd, &mut msg, MSG_TRUNC);
    if ret != MESSAGE_TRUNC_SZ as ssize_t {
        printf(c"%zi\n".as_ptr(), ret);
        die_perror(c"MSG_TRUNC doesn't work".as_ptr());
    }
    if msg.msg_flags & MSG_TRUNC == 0 {
        fprintf(stderr, c"MSG_TRUNC expected\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    close(fd);
}

unsafe fn current_nsec() -> time_t {
    let mut ts: timespec = zeroed();
    if clock_gettime(CLOCK_REALTIME, &mut ts) != 0 {
        die_perror(c"clock_gettime(3) failed".as_ptr());
    }
    ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec
}

unsafe fn test_seqpacket_timeout_client(opts: *const test_opts) {
    let fd = vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    let tv = timeval { tv_sec: RCVTIMEO_TIMEOUT_SEC, tv_usec: 0 };
    setsockopt_timeval_check(fd, SOL_SOCKET, SO_RCVTIMEO, tv, c"setsockopt(SO_RCVTIMEO)".as_ptr());
    let read_enter_ns = current_nsec();
    let mut dummy: c_char = 0;
    if read(fd, &mut dummy as *mut _ as *mut c_void, size_of::<c_char>()) != -1 {
        fprintf(stderr, c"expected 'dummy' read(2) failure\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    if errno != EAGAIN { die_perror(c"EAGAIN expected".as_ptr()); }
    let read_overhead_ns = current_nsec() - read_enter_ns - NSEC_PER_SEC * RCVTIMEO_TIMEOUT_SEC;
    if read_overhead_ns > READ_OVERHEAD_NSEC {
        fprintf(stderr, c"too much time in read(2), %lu > %i ns\n".as_ptr(), read_overhead_ns as c_ulong, READ_OVERHEAD_NSEC as c_int);
        exit(EXIT_FAILURE);
    }
    control_writeln(c"WAITDONE".as_ptr());
    close(fd);
}

unsafe fn test_seqpacket_timeout_server(opts: *const test_opts) {
    let fd = vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    control_expectln(c"WAITDONE".as_ptr());
    close(fd);
}

unsafe fn test_seqpacket_bigmsg_client(opts: *const test_opts) {
    let mut sock_buf_size: c_ulonglong = 0;
    let mut len = size_of::<c_ulonglong>() as socklen_t;
    let fd = vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    if getsockopt(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, &mut sock_buf_size as *mut _ as *mut c_void, &mut len) != 0 {
        die_perror(c"getsockopt".as_ptr());
    }
    sock_buf_size += 1;
    /* size_t can be < unsigned long long */
    let buf_size = sock_buf_size as size_t;
    if buf_size as c_ulonglong != sock_buf_size {
        fprintf(stderr, c"Returned BUFFER_SIZE too large\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    let data = malloc(buf_size);
    if data.is_null() { die_perror(c"malloc".as_ptr()); }
    send_buf(fd, data, buf_size, 0, -EMSGSIZE as ssize_t);
    control_writeln(c"CLISENT".as_ptr());
    free(data);
    close(fd);
}

unsafe fn test_seqpacket_bigmsg_server(opts: *const test_opts) {
    let fd = vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    control_expectln(c"CLISENT".as_ptr());
    close(fd);
}

unsafe fn test_seqpacket_invalid_rec_buffer_client(opts: *const test_opts) {
    let buf_size = getpagesize() * 3;
    let fd = vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    let buf1 = malloc(buf_size as usize) as *mut u8;
    if buf1.is_null() { die_perror(c"'malloc()' for 'buf1'".as_ptr()); }
    let buf2 = malloc(buf_size as usize) as *mut u8;
    if buf2.is_null() { die_perror(c"'malloc()' for 'buf2'".as_ptr()); }
    memset(buf1 as *mut c_void, BUF_PATTERN_1, buf_size as usize);
    memset(buf2 as *mut c_void, BUF_PATTERN_2, buf_size as usize);
    send_buf(fd, buf1 as *const c_void, buf_size as usize, 0, buf_size as ssize_t);
    send_buf(fd, buf2 as *const c_void, buf_size as usize, 0, buf_size as ssize_t);
    close(fd);
}

unsafe fn test_seqpacket_invalid_rec_buffer_server(opts: *const test_opts) {
    let page_size = getpagesize();
    let buf_size = page_size * 3;
    let prot = PROT_READ | PROT_WRITE;
    let flags = MAP_PRIVATE | MAP_ANONYMOUS;
    let fd = vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    /* Setup first buffer. */
    let broken_buf = mmap(null_mut(), buf_size as usize, prot, flags, -1, 0) as *mut u8;
    if broken_buf as *mut c_void == MAP_FAILED { die_perror(c"mmap for 'broken_buf'".as_ptr()); }
    /* Unmap "hole" in buffer. */
    if munmap(broken_buf.add(page_size as usize) as *mut c_void, page_size as usize) != 0 { die_perror(c"'broken_buf' setup".as_ptr()); }
    let valid_buf = mmap(null_mut(), buf_size as usize, prot, flags, -1, 0) as *mut u8;
    if valid_buf as *mut c_void == MAP_FAILED { die_perror(c"mmap for 'valid_buf'".as_ptr()); }
    /* Try to fill buffer with unmapped middle. */
    let mut res = read(fd, broken_buf as *mut c_void, buf_size as usize);
    if res != -1 {
        fprintf(stderr, c"expected 'broken_buf' read(2) failure, got %zi\n".as_ptr(), res);
        exit(EXIT_FAILURE);
    }
    if errno != EFAULT { die_perror(c"unexpected errno of 'broken_buf'".as_ptr()); }
    /* Try to fill valid buffer. */
    res = read(fd, valid_buf as *mut c_void, buf_size as usize);
    if res < 0 { die_perror(c"unexpected 'valid_buf' read(2) failure".as_ptr()); }
    if res != buf_size as ssize_t {
        fprintf(stderr, c"invalid 'valid_buf' read(2), expected %i, got %zi\n".as_ptr(), buf_size, res);
        exit(EXIT_FAILURE);
    }
    for i in 0..buf_size {
        if *valid_buf.add(i as usize) != BUF_PATTERN_2 as u8 {
            fprintf(stderr, c"invalid pattern for 'valid_buf' at %i, expected %hhX, got %hhX\n".as_ptr(), i, BUF_PATTERN_2, *valid_buf.add(i as usize) as c_int);
            exit(EXIT_FAILURE);
        }
    }
    /* Unmap buffers. */
    munmap(broken_buf as *mut c_void, page_size as usize);
    munmap(broken_buf.add((page_size * 2) as usize) as *mut c_void, page_size as usize);
    munmap(valid_buf as *mut c_void, buf_size as usize);
    close(fd);
}

unsafe fn test_stream_poll_rcvlowat_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    /* Send 1 byte. */
    send_byte(fd, 1, 0);
    control_writeln(c"SRVSENT".as_ptr());
    /* Wait until client is ready to receive rest of data. */
    control_expectln(c"CLNSENT".as_ptr());
    for _ in 0..(RCVLOWAT_BUF_SIZE - 1) { send_byte(fd, 1, 0); }
    /* Keep socket in active state. */
    control_expectln(c"POLLDONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_poll_rcvlowat_client(opts: *const test_opts) {
    let lowat_val = RCVLOWAT_BUF_SIZE as c_int;
    let mut buf = [0i8; RCVLOWAT_BUF_SIZE];
    let mut fds: pollfd = zeroed();
    let poll_flags = POLLIN | POLLRDNORM;
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    setsockopt_int_check(fd, SOL_SOCKET, SO_RCVLOWAT, lowat_val, c"setsockopt(SO_RCVLOWAT)".as_ptr());
    control_expectln(c"SRVSENT".as_ptr());
    /* At this point, server sent 1 byte. */
    fds.fd = fd;
    fds.events = poll_flags;
    /* Try to wait for 1 sec. */
    if poll(&mut fds, 1, 1000) < 0 { die_perror(c"poll".as_ptr()); }
    /* poll() must return nothing. */
    if fds.revents != 0 {
        fprintf(stderr, c"Unexpected poll result %hx\n".as_ptr(), fds.revents as c_int);
        exit(EXIT_FAILURE);
    }
    /* Tell server to send rest of data. */
    control_writeln(c"CLNSENT".as_ptr());
    /* Poll for data. */
    if poll(&mut fds, 1, 10000) < 0 { die_perror(c"poll".as_ptr()); }
    /* Only these two bits are expected. */
    if fds.revents != poll_flags {
        fprintf(stderr, c"Unexpected poll result %hx\n".as_ptr(), fds.revents as c_int);
        exit(EXIT_FAILURE);
    }
    /* Use MSG_DONTWAIT, if call is going to wait, EAGAIN
     * will be returned.
     */
    recv_buf(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), MSG_DONTWAIT, RCVLOWAT_BUF_SIZE as ssize_t);
    control_writeln(c"POLLDONE".as_ptr());
    close(fd);
}

unsafe fn test_inv_buf_client(opts: *const test_opts, stream: bool) {
    let mut data = [0u8; INV_BUF_TEST_DATA_LEN];
    let fd = if stream { vsock_stream_connect((*opts).peer_cid, (*opts).peer_port) } else { vsock_seqpacket_connect((*opts).peer_cid, (*opts).peer_port) };
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    control_expectln(c"SENDDONE".as_ptr());
    /* Use invalid buffer here. */
    recv_buf(fd, null_mut(), data.len(), 0, -EFAULT as ssize_t);
    let expected_ret = if stream {
        /* For SOCK_STREAM we must continue reading. */
        data.len() as ssize_t
    } else {
        /* For SOCK_SEQPACKET socket's queue must be empty. */
        -EAGAIN as ssize_t
    };
    recv_buf(fd, data.as_mut_ptr() as *mut c_void, data.len(), MSG_DONTWAIT, expected_ret);
    control_writeln(c"DONE".as_ptr());
    close(fd);
}

unsafe fn test_inv_buf_server(opts: *const test_opts, stream: bool) {
    let data = [0u8; INV_BUF_TEST_DATA_LEN];
    let fd = if stream { vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut()) } else { vsock_seqpacket_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut()) };
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    send_buf(fd, data.as_ptr() as *const c_void, data.len(), 0, data.len() as ssize_t);
    control_writeln(c"SENDDONE".as_ptr());
    control_expectln(c"DONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_inv_buf_client(opts: *const test_opts) { test_inv_buf_client(opts, true); }
unsafe fn test_stream_inv_buf_server(opts: *const test_opts) { test_inv_buf_server(opts, true); }
unsafe fn test_seqpacket_inv_buf_client(opts: *const test_opts) { test_inv_buf_client(opts, false); }
unsafe fn test_seqpacket_inv_buf_server(opts: *const test_opts) { test_inv_buf_server(opts, false); }

unsafe fn test_stream_virtio_skb_merge_client(opts: *const test_opts) {
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    /* Send first skbuff. */
    send_buf(fd, HELLO_STR.as_ptr() as *const c_void, strlen(HELLO_STR.as_ptr() as *const c_char), 0, strlen(HELLO_STR.as_ptr() as *const c_char) as ssize_t);
    control_writeln(c"SEND0".as_ptr());
    /* Peer reads part of first skbuff. */
    control_expectln(c"REPLY0".as_ptr());
    /* Send second skbuff, it will be appended to the first. */
    send_buf(fd, WORLD_STR.as_ptr() as *const c_void, strlen(WORLD_STR.as_ptr() as *const c_char), 0, strlen(WORLD_STR.as_ptr() as *const c_char) as ssize_t);
    control_writeln(c"SEND1".as_ptr());
    /* Peer reads merged skbuff packet. */
    control_expectln(c"REPLY1".as_ptr());
    close(fd);
}

unsafe fn test_stream_virtio_skb_merge_server(opts: *const test_opts) {
    let mut readn: size_t = 0;
    let mut buf = [0u8; 64];
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    control_expectln(c"SEND0".as_ptr());
    /* Read skbuff partially. */
    let mut to_read: size_t = 2;
    recv_buf(fd, buf.as_mut_ptr().add(readn) as *mut c_void, to_read, 0, to_read as ssize_t);
    readn += to_read;
    control_writeln(c"REPLY0".as_ptr());
    control_expectln(c"SEND1".as_ptr());
    /* Read the rest of both buffers */
    to_read = strlen(HELLOWORLD_STR.as_ptr() as *const c_char) - readn;
    recv_buf(fd, buf.as_mut_ptr().add(readn) as *mut c_void, to_read, 0, to_read as ssize_t);
    readn += to_read;
    /* No more bytes should be there */
    to_read = buf.len() - readn;
    recv_buf(fd, buf.as_mut_ptr().add(readn) as *mut c_void, to_read, MSG_DONTWAIT, -EAGAIN as ssize_t);
    if memcmp(buf.as_ptr() as *const c_void, HELLOWORLD_STR.as_ptr() as *const c_void, strlen(HELLOWORLD_STR.as_ptr() as *const c_char)) != 0 {
        fprintf(stderr, c"pattern mismatch\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    control_writeln(c"REPLY1".as_ptr());
    close(fd);
}

unsafe fn test_seqpacket_msg_peek_client(opts: *const test_opts) { test_msg_peek_client(opts, true); }
unsafe fn test_seqpacket_msg_peek_server(opts: *const test_opts) { test_msg_peek_server(opts, true); }

static mut have_sigpipe: sig_atomic_t = 0;

unsafe extern "C" fn sigpipe(_signo: c_int) {
    have_sigpipe = 1;
}

unsafe fn test_stream_check_sigpipe(fd: c_int) {
    have_sigpipe = 0;
    /* When the other peer calls shutdown(SHUT_RD), there is a chance that
     * the send() call could occur before the message carrying the close
     * information arrives over the transport. In such cases, the send()
     * might still succeed. To avoid this race, let's retry the send() call
     * a few times, ensuring the test is more reliable.
     */
    timeout_begin(TIMEOUT);
    loop {
        let res = send(fd, c"A".as_ptr() as *const c_void, 1, 0);
        if res == -1 && errno != EINTR { break; }
        /* Sleep a little before trying again to avoid flooding the
         * other peer and filling its receive buffer, causing
         * false-negative.
         */
        timeout_usleep(SEND_SLEEP_USEC);
        timeout_check(c"send".as_ptr());
    }
    timeout_end();
    if errno != EPIPE {
        fprintf(stderr, c"unexpected send(2) errno %d\n".as_ptr(), errno);
        exit(EXIT_FAILURE);
    }
    if have_sigpipe == 0 {
        fprintf(stderr, c"SIGPIPE expected\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    have_sigpipe = 0;
    timeout_begin(TIMEOUT);
    loop {
        let res = send(fd, c"A".as_ptr() as *const c_void, 1, MSG_NOSIGNAL);
        if res == -1 && errno != EINTR { break; }
        timeout_usleep(SEND_SLEEP_USEC);
        timeout_check(c"send".as_ptr());
    }
    timeout_end();
    if errno != EPIPE {
        fprintf(stderr, c"unexpected send(2) errno %d\n".as_ptr(), errno);
        exit(EXIT_FAILURE);
    }
    if have_sigpipe != 0 {
        fprintf(stderr, c"SIGPIPE not expected\n".as_ptr());
        exit(EXIT_FAILURE);
    }
}

unsafe fn test_stream_shutwr_client(opts: *const test_opts) {
    let act = sigaction { sa_handler: Some(sigpipe), sa_mask: [0; 16], sa_flags: 0, sa_restorer: null_mut() };
    sigaction(SIGPIPE, &act, null_mut());
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    if shutdown(fd, SHUT_WR) != 0 { die_perror(c"shutdown".as_ptr()); }
    test_stream_check_sigpipe(fd);
    control_writeln(c"CLIENTDONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_shutwr_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    control_expectln(c"CLIENTDONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_shutrd_client(opts: *const test_opts) {
    let act = sigaction { sa_handler: Some(sigpipe), sa_mask: [0; 16], sa_flags: 0, sa_restorer: null_mut() };
    sigaction(SIGPIPE, &act, null_mut());
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    control_expectln(c"SHUTRDDONE".as_ptr());
    test_stream_check_sigpipe(fd);
    control_writeln(c"CLIENTDONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_shutrd_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    if shutdown(fd, SHUT_RD) != 0 { die_perror(c"shutdown".as_ptr()); }
    control_writeln(c"SHUTRDDONE".as_ptr());
    control_expectln(c"CLIENTDONE".as_ptr());
    close(fd);
}

unsafe fn test_double_bind_connect_server(opts: *const test_opts) {
    let listen_fd = vsock_stream_listen(VMADDR_CID_ANY, (*opts).peer_port);
    for _ in 0..2 {
        let mut sa_client: sockaddr_vm = zeroed();
        let mut socklen_client = size_of::<sockaddr_vm>() as socklen_t;
        control_writeln(c"LISTENING".as_ptr());
        let mut client_fd: c_int;
        timeout_begin(TIMEOUT);
        loop {
            client_fd = accept(listen_fd, &mut sa_client as *mut _ as *mut sockaddr, &mut socklen_client);
            timeout_check(c"accept".as_ptr());
            if !(client_fd < 0 && errno == EINTR) { break; }
        }
        timeout_end();
        if client_fd < 0 { die_perror(c"accept".as_ptr()); }
        /* Waiting for remote peer to close connection */
        vsock_wait_remote_close(client_fd);
    }
    close(listen_fd);
}

unsafe fn test_double_bind_connect_client(opts: *const test_opts) {
    for _ in 0..2 {
        /* Wait until server is ready to accept a new connection */
        control_expectln(c"LISTENING".as_ptr());
        /* We use 'peer_port + 1' as "some" port for the 'bind()'
         * call. It is safe for overflow, but must be considered,
         * when running multiple test applications simultaneously
         * where 'peer-port' argument differs by 1.
         */
        let client_fd = vsock_bind_connect((*opts).peer_cid, (*opts).peer_port, (*opts).peer_port.wrapping_add(1), SOCK_STREAM);
        close(client_fd);
    }
}

unsafe fn test_unsent_bytes_server(opts: *const test_opts, ty: c_int) {
    let mut buf = [0u8; MSG_BUF_IOCTL_LEN];
    let client_fd = vsock_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut(), ty);
    if client_fd < 0 { die_perror(c"accept".as_ptr()); }
    recv_buf(client_fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0, buf.len() as ssize_t);
    control_writeln(c"RECEIVED".as_ptr());
    close(client_fd);
}

unsafe fn test_unsent_bytes_client(opts: *const test_opts, ty: c_int) {
    let mut buf = [0u8; MSG_BUF_IOCTL_LEN];
    let fd = vsock_connect((*opts).peer_cid, (*opts).peer_port, ty);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    for i in 0..buf.len() { buf[i] = (rand() & 0xFF) as u8; }
    send_buf(fd, buf.as_ptr() as *const c_void, buf.len(), 0, buf.len() as ssize_t);
    control_expectln(c"RECEIVED".as_ptr());
    /* SIOCOUTQ isn't guaranteed to instantly track sent data. Even though
     * the "RECEIVED" message means that the other side has received the
     * data, there can be a delay in our kernel before updating the "unsent
     * bytes" counter. vsock_wait_sent() will repeat SIOCOUTQ until it
     * returns 0.
     */
    if !vsock_wait_sent(fd) {
        fprintf(stderr, c"Test skipped, SIOCOUTQ not supported.\n".as_ptr());
    }
    close(fd);
}

unsafe fn test_unread_bytes_server(opts: *const test_opts, ty: c_int) {
    let mut buf = [0u8; MSG_BUF_IOCTL_LEN];
    let client_fd = vsock_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut(), ty);
    if client_fd < 0 { die_perror(c"accept".as_ptr()); }
    for i in 0..buf.len() { buf[i] = (rand() & 0xFF) as u8; }
    send_buf(client_fd, buf.as_ptr() as *const c_void, buf.len(), 0, buf.len() as ssize_t);
    control_writeln(c"SENT".as_ptr());
    close(client_fd);
}

unsafe fn test_unread_bytes_client(opts: *const test_opts, ty: c_int) {
    let mut buf = [0u8; MSG_BUF_IOCTL_LEN];
    let fd = vsock_connect((*opts).peer_cid, (*opts).peer_port, ty);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    control_expectln(c"SENT".as_ptr());
    /* The data has arrived but has not been read. The expected is
     * MSG_BUF_IOCTL_LEN.
     */
    if !vsock_ioctl_int(fd, SIOCINQ, MSG_BUF_IOCTL_LEN as c_int) {
        fprintf(stderr, c"Test skipped, SIOCINQ not supported.\n".as_ptr());
        close(fd);
        return;
    }
    recv_buf(fd, buf.as_mut_ptr() as *mut c_void, buf.len(), 0, buf.len() as ssize_t);
    /* All data has been consumed, so the expected is 0. */
    vsock_ioctl_int(fd, SIOCINQ, 0);
    close(fd);
}

unsafe fn test_stream_unsent_bytes_client(opts: *const test_opts) { test_unsent_bytes_client(opts, SOCK_STREAM); }
unsafe fn test_stream_unsent_bytes_server(opts: *const test_opts) { test_unsent_bytes_server(opts, SOCK_STREAM); }
unsafe fn test_seqpacket_unsent_bytes_client(opts: *const test_opts) { test_unsent_bytes_client(opts, SOCK_SEQPACKET); }
unsafe fn test_seqpacket_unsent_bytes_server(opts: *const test_opts) { test_unsent_bytes_server(opts, SOCK_SEQPACKET); }
unsafe fn test_stream_unread_bytes_client(opts: *const test_opts) { test_unread_bytes_client(opts, SOCK_STREAM); }
unsafe fn test_stream_unread_bytes_server(opts: *const test_opts) { test_unread_bytes_server(opts, SOCK_STREAM); }
unsafe fn test_seqpacket_unread_bytes_client(opts: *const test_opts) { test_unread_bytes_client(opts, SOCK_SEQPACKET); }
unsafe fn test_seqpacket_unread_bytes_server(opts: *const test_opts) { test_unread_bytes_server(opts, SOCK_SEQPACKET); }

unsafe fn test_stream_rcvlowat_def_cred_upd_client(opts: *const test_opts) {
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    /* Send 1 byte more than peer's buffer size. */
    let buf_size = RCVLOWAT_CREDIT_UPD_BUF_SIZE + 1;
    let buf = malloc(buf_size);
    if buf.is_null() { die_perror(c"malloc".as_ptr()); }
    /* Wait until peer sets needed buffer size. */
    recv_byte(fd, 1, 0);
    if send(fd, buf, buf_size, 0) != buf_size as ssize_t {
        die_perror(c"send failed".as_ptr());
    }
    free(buf);
    close(fd);
}

unsafe fn test_stream_credit_update_test(opts: *const test_opts, low_rx_bytes_test: bool) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    let buf_size = RCVLOWAT_CREDIT_UPD_BUF_SIZE;
    /* size_t can be < unsigned long long */
    let sock_buf_size = buf_size as c_ulonglong;
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_SIZE)".as_ptr());
    let mut recv_buf_size: c_int = 0;
    if low_rx_bytes_test {
        /* Set new SO_RCVLOWAT here. This enables sending credit
         * update when number of bytes if our rx queue become <
         * SO_RCVLOWAT value.
         */
        recv_buf_size = 1 + VIRTIO_VSOCK_MAX_PKT_BUF_SIZE as c_int;
        setsockopt_int_check(fd, SOL_SOCKET, SO_RCVLOWAT, recv_buf_size, c"setsockopt(SO_RCVLOWAT)".as_ptr());
    }
    /* Send one dummy byte here, because 'setsockopt()' above also
     * sends special packet which tells sender to update our buffer
     * size. This 'send_byte()' will serialize such packet with data
     * reads in a loop below. Sender starts transmission only when
     * it receives this single byte.
     */
    send_byte(fd, 1, 0);
    let buf = malloc(buf_size);
    if buf.is_null() { die_perror(c"malloc".as_ptr()); }
    /* Wait until there will be 128KB of data in rx queue. */
    recv_buf(fd, buf, buf_size, MSG_PEEK, buf_size as ssize_t);
    /* There is 128KB of data in the socket's rx queue, dequeue first
     * 64KB, credit update is sent if 'low_rx_bytes_test' == true.
     * Otherwise, credit update is sent in 'if (!low_rx_bytes_test)'.
     */
    recv_buf_size = VIRTIO_VSOCK_MAX_PKT_BUF_SIZE as c_int;
    recv_buf(fd, buf, recv_buf_size as usize, 0, recv_buf_size as ssize_t);
    if !low_rx_bytes_test {
        recv_buf_size += 1;
        /* Updating SO_RCVLOWAT will send credit update. */
        setsockopt_int_check(fd, SOL_SOCKET, SO_RCVLOWAT, recv_buf_size, c"setsockopt(SO_RCVLOWAT)".as_ptr());
    }
    let mut fds = pollfd { fd, events: POLLIN | POLLRDNORM | POLLERR | POLLRDHUP | POLLHUP, revents: 0 };
    /* This 'poll()' will return once we receive last byte
     * sent by client.
     */
    if poll(&mut fds, 1, -1) < 0 { die_perror(c"poll".as_ptr()); }
    if fds.revents & POLLERR != 0 {
        fprintf(stderr, c"'poll()' error\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    if fds.revents & (POLLIN | POLLRDNORM) != 0 {
        recv_buf(fd, buf, recv_buf_size as usize, MSG_DONTWAIT, recv_buf_size as ssize_t);
    } else {
        /* These flags must be set, as there is at
         * least 64KB of data ready to read.
         */
        fprintf(stderr, c"POLLIN | POLLRDNORM expected\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    free(buf);
    close(fd);
}

unsafe fn test_stream_cred_upd_on_low_rx_bytes(opts: *const test_opts) { test_stream_credit_update_test(opts, true); }
unsafe fn test_stream_cred_upd_on_set_rcvlowat(opts: *const test_opts) { test_stream_credit_update_test(opts, false); }

/* The goal of test leak_acceptq is to stress the race between connect() and
 * close(listener). Implementation of client/server loops boils down to:
 *
 * client                server
 * ------                ------
 * write(CONTINUE)
 *                       expect(CONTINUE)
 *                       listen()
 *                       write(LISTENING)
 * expect(LISTENING)
 * connect()             close()
 */
unsafe fn test_stream_leak_acceptq_client(opts: *const test_opts) {
    let tout = current_nsec() + ACCEPTQ_LEAK_RACE_TIMEOUT * NSEC_PER_SEC;
    loop {
        control_writeulong(CONTROL_CONTINUE);
        let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
        if fd >= 0 { close(fd); }
        if current_nsec() >= tout { break; }
    }
    control_writeulong(CONTROL_DONE);
}

/* Test for a memory leak. User is expected to run kmemleak scan, see README. */
unsafe fn test_stream_leak_acceptq_server(opts: *const test_opts) {
    while control_readulong() == CONTROL_CONTINUE {
        let fd = vsock_stream_listen(VMADDR_CID_ANY, (*opts).peer_port);
        control_writeln(c"LISTENING".as_ptr());
        close(fd);
    }
}

/* Test for a memory leak. User is expected to run kmemleak scan, see README. */
unsafe fn test_stream_msgzcopy_leak_errq_client(opts: *const test_opts) {
    let mut fds: pollfd = zeroed();
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    enable_so_zerocopy_check(fd);
    send_byte(fd, 1, MSG_ZEROCOPY);
    fds.fd = fd;
    fds.events = 0;
    if poll(&mut fds, 1, -1) < 0 { die_perror(c"poll".as_ptr()); }
    close(fd);
}

unsafe fn test_stream_msgzcopy_leak_errq_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    recv_byte(fd, 1, 0);
    vsock_wait_remote_close(fd);
    close(fd);
}

/* Test msgzcopy_leak_zcskb is meant to exercise sendmsg() error handling path,
 * that might leak an skb. The idea is to fail virtio_transport_init_zcopy_skb()
 * by hitting net.core.optmem_max limit in sock_omalloc(), specifically
 *
 *   vsock_connectible_sendmsg
 *     virtio_transport_stream_enqueue
 *       virtio_transport_send_pkt_info
 *         virtio_transport_init_zcopy_skb
 *         . msg_zerocopy_realloc
 *         .   msg_zerocopy_alloc
 *         .     sock_omalloc
 *         .       sk_omem_alloc + size > sysctl_optmem_max
 *         return -ENOMEM
 *
 * We abuse the implementation detail of net/socket.c:____sys_sendmsg().
 * sk_omem_alloc can be precisely bumped by sock_kmalloc(), as it is used to
 * fetch user-provided control data.
 *
 * While this approach works for now, it relies on assumptions regarding the
 * implementation and configuration (for example, order of net.core.optmem_max
 * can not exceed MAX_PAGE_ORDER), which may not hold in the future. A more
 * resilient testing could be implemented by leveraging the Fault injection
 * framework (CONFIG_FAULT_INJECTION), e.g.
 *
 *   client# echo N > /sys/kernel/debug/failslab/ignore-gfp-wait
 *   client# echo 0 > /sys/kernel/debug/failslab/verbose
 *
 *   void client(const struct test_opts *opts)
 *   {
 *       char buf[16];
 *       int f, s, i;
 *
 *       f = open("/proc/self/fail-nth", O_WRONLY);
 *
 *       for (i = 1; i < 32; i++) {
 *           control_writeulong(CONTROL_CONTINUE);
 *
 *           s = vsock_stream_connect(opts->peer_cid, opts->peer_port);
 *           enable_so_zerocopy_check(s);
 *
 *           sprintf(buf, "%d", i);
 *           write(f, buf, strlen(buf));
 *
 *           send(s, &(char){ 0 }, 1, MSG_ZEROCOPY);
 *
 *           write(f, "0", 1);
 *           close(s);
 *       }
 *
 *       control_writeulong(CONTROL_DONE);
 *       close(f);
 *   }
 *
 *   void server(const struct test_opts *opts)
 *   {
 *       int fd;
 *
 *       while (control_readulong() == CONTROL_CONTINUE) {
 *           fd = vsock_stream_accept(VMADDR_CID_ANY, opts->peer_port, NULL);
 *           vsock_wait_remote_close(fd);
 *           close(fd);
 *       }
 *   }
 *
 * Refer to Documentation/fault-injection/fault-injection.rst.
 */
/* Test for a memory leak. User is expected to run kmemleak scan, see README. */
unsafe fn test_stream_msgzcopy_leak_zcskb_client(opts: *const test_opts) {
    let f = fopen(c"/proc/sys/net/core/optmem_max".as_ptr(), c"r".as_ptr());
    if f.is_null() { die_perror(c"fopen(optmem_max)".as_ptr()); }
    let mut optmem_max: size_t = 0;
    if fscanf(f, c"%zu".as_ptr(), &mut optmem_max as *mut size_t) != 1 {
        fprintf(stderr, c"fscanf(optmem_max) failed\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    fclose(f);
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    enable_so_zerocopy_check(fd);
    let ctl_len = optmem_max - 1;
    if ctl_len > (PAGE_SIZE << MAX_PAGE_ORDER) {
        fprintf(stderr, c"Try with net.core.optmem_max = 100000\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    let chunk_size = cmsg_space(ctl_len);
    let chunk = malloc(chunk_size) as *mut c_char;
    if chunk.is_null() { die_perror(c"malloc".as_ptr()); }
    memset(chunk as *mut c_void, 0, chunk_size);
    let mut byte: c_char = 0;
    let mut iov = iovec { iov_base: &mut byte as *mut _ as *mut c_void, iov_len: 1 };
    let mut msg: msghdr = zeroed();
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1;
    msg.msg_control = chunk as *mut c_void;
    msg.msg_controllen = ctl_len;
    errno = 0;
    let res = sendmsg(fd, &msg, MSG_ZEROCOPY);
    if res >= 0 || errno != ENOMEM {
        fprintf(stderr, c"Expected ENOMEM, got errno=%d res=%d\n".as_ptr(), errno, res as c_int);
        exit(EXIT_FAILURE);
    }
    close(fd);
}

unsafe fn test_stream_msgzcopy_leak_zcskb_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    vsock_wait_remote_close(fd);
    close(fd);
}

unsafe fn test_stream_transport_uaf(cid: c_int) -> bool {
    let mut sockets = [0 as c_int; MAX_PORT_RETRIES];
    let mut addr: sockaddr_vm = zeroed();
    /* Probe for a transport by attempting a local CID bind. Unavailable
     * transport (or more specifically: an unsupported transport/CID
     * combination) results in EADDRNOTAVAIL, other errnos are fatal.
     */
    let mut fd = vsock_bind_try(cid as c_uint, VMADDR_PORT_ANY, SOCK_STREAM);
    if fd < 0 {
        if errno != EADDRNOTAVAIL { die_perror(c"Unexpected bind() errno".as_ptr()); }
        return false;
    }
    let mut alen = size_of::<sockaddr_vm>() as socklen_t;
    if getsockname(fd, &mut addr as *mut _ as *mut sockaddr, &mut alen) != 0 {
        die_perror(c"getsockname".as_ptr());
    }
    /* Drain the autobind pool; see __vsock_bind_connectible(). */
    let mut i = 0usize;
    while i < MAX_PORT_RETRIES {
        addr.svm_port = addr.svm_port.wrapping_add(1);
        sockets[i] = vsock_bind(cid as c_uint, addr.svm_port, SOCK_STREAM);
        i += 1;
    }
    close(fd);
    /* Setting SOCK_NONBLOCK makes connect() return soon after
     * (re-)assigning the transport. We are not connecting to anything
     * anyway, so there is no point entering the main loop in
     * vsock_connect(); waiting for timeout, checking for signals, etc.
     */
    fd = socket(AF_VSOCK, SOCK_STREAM | SOCK_NONBLOCK, 0);
    if fd < 0 { die_perror(c"socket".as_ptr()); }
    /* Assign transport, while failing to autobind. Autobind pool was
     * drained, so EADDRNOTAVAIL coming from __vsock_bind_connectible() is
     * expected.
     *
     * One exception is ENODEV which is thrown by vsock_assign_transport(),
     * i.e. before vsock_auto_bind(), when the only transport loaded is
     * vhost.
     */
    if connect(fd, &addr as *const _ as *const sockaddr, alen) == 0 {
        fprintf(stderr, c"Unexpected connect() success\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    let ret: bool;
    if errno == ENODEV && cid == VMADDR_CID_HOST {
        ret = false;
    } else {
        if errno != EADDRNOTAVAIL { die_perror(c"Unexpected connect() errno".as_ptr()); }
        /* Reassign transport, triggering old transport release and
         * (potentially) unbinding of an unbound socket.
         *
         * Vulnerable system may crash now.
         */
        let mut c = VMADDR_CID_HYPERVISOR;
        while c <= VMADDR_CID_HOST + 1 {
            if c != cid {
                addr.svm_cid = c as c_uint;
                let _ = connect(fd, &addr as *const _ as *const sockaddr, alen);
            }
            c += 1;
        }
        ret = true;
    }
    close(fd);
    while i > 0 {
        i -= 1;
        close(sockets[i]);
    }
    ret
}

/* Test attempts to trigger a transport release for an unbound socket. This can
 * lead to a reference count mishandling.
 */
unsafe fn test_stream_transport_uaf_client(_opts: *const test_opts) {
    let mut tested = false;
    let mut cid = VMADDR_CID_HYPERVISOR;
    while cid <= VMADDR_CID_HOST + 1 {
        tested |= test_stream_transport_uaf(cid);
        cid += 1;
    }
    let tr = get_transports();
    if tr == 0 {
        fprintf(stderr, c"No transports detected\n".as_ptr());
    } else if tr == TRANSPORT_VIRTIO {
        fprintf(stderr, c"Setup unsupported: sole virtio transport\n".as_ptr());
    } else if !tested {
        fprintf(stderr, c"No transports tested\n".as_ptr());
    }
}

unsafe fn test_stream_connect_retry_client(opts: *const test_opts) {
    let fd = socket(AF_VSOCK, SOCK_STREAM, 0);
    if fd < 0 { die_perror(c"socket".as_ptr()); }
    if vsock_connect_fd(fd, (*opts).peer_cid, (*opts).peer_port) == 0 {
        fprintf(stderr, c"Unexpected connect() #1 success\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    control_writeln(c"LISTEN".as_ptr());
    control_expectln(c"LISTENING".as_ptr());
    if vsock_connect_fd(fd, (*opts).peer_cid, (*opts).peer_port) != 0 {
        die_perror(c"connect() #2".as_ptr());
    }
    close(fd);
}

unsafe fn test_stream_connect_retry_server(opts: *const test_opts) {
    control_expectln(c"LISTEN".as_ptr());
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    vsock_wait_remote_close(fd);
    close(fd);
}

unsafe extern "C" fn test_stream_transport_change_thread(vargp: *mut c_void) -> *mut c_void {
    let pid = vargp as *mut pid_t;
    let ret = pthread_setcanceltype(PTHREAD_CANCEL_ASYNCHRONOUS, null_mut());
    if ret != 0 {
        fprintf(stderr, c"pthread_setcanceltype: %d\n".as_ptr(), ret);
        exit(EXIT_FAILURE);
    }
    loop {
        if kill(*pid, SIGUSR1) < 0 {
            die_perror(c"kill".as_ptr());
        }
    }
}

unsafe extern "C" fn test_transport_change_signal_handler(_signal: c_int) {
    /* We need a custom handler for SIGUSR1 as the default one terminates the process. */
}

unsafe fn test_stream_transport_change_client(opts: *const test_opts) {
    let tr = get_transports();
    /* Print a warning if there is a G2H transport loaded.
     * This is on a best effort basis because VMCI can be either G2H and H2G, and there is
     * no easy way to understand it.
     * The bug we are testing only appears when G2H transports are not loaded.
     * This is because `vsock_assign_transport`, when using CID 0, assigns a G2H transport
     * to vsk->transport. If none is available it is set to NULL, causing the null-ptr-deref.
     */
    if tr & TRANSPORTS_G2H != 0 {
        fprintf(stderr, c"G2H Transport detected. This test will not fail.\n".as_ptr());
    }
    let old_handler = signal(SIGUSR1, Some(test_transport_change_signal_handler));
    if old_handler == SIG_ERR { die_perror(c"signal".as_ptr()); }
    let mut pid = getpid();
    let mut thread_id: pthread_t = 0;
    let mut ret = pthread_create(&mut thread_id, null(), test_stream_transport_change_thread, &mut pid as *mut _ as *mut c_void);
    if ret != 0 {
        fprintf(stderr, c"pthread_create: %d\n".as_ptr(), ret);
        exit(EXIT_FAILURE);
    }
    control_expectln(c"LISTENING".as_ptr());
    let tout = current_nsec() + TRANSPORT_CHANGE_TIMEOUT * NSEC_PER_SEC;
    while current_nsec() < tout {
        let mut sa = sockaddr_vm { svm_family: AF_VSOCK as u16, svm_reserved1: 0, svm_cid: (*opts).peer_cid, svm_port: (*opts).peer_port, svm_zero: [0; 4] };
        let mut send_control = false;
        let s = socket(AF_VSOCK, SOCK_STREAM, 0);
        if s < 0 { die_perror(c"socket".as_ptr()); }
        /* Although setting SO_LINGER does not affect the original test
         * for null-ptr-deref, it may trigger a lockdep warning.
         */
        enable_so_linger(s, 1);
        ret = connect(s, &sa as *const _ as *const sockaddr, size_of::<sockaddr_vm>() as socklen_t);
        /* The connect can fail due to signals coming from the thread,
         * or because the receiver connection queue is full.
         * Ignoring also the latter case because there is no way
         * of synchronizing client's connect and server's accept when
         * connect(s) are constantly being interrupted by signals.
         */
        if ret == -1 && errno != EINTR && errno != ECONNRESET { die_perror(c"connect".as_ptr()); }
        /* Notify the server if the connect() is successful or the
         * receiver connection queue is full, so it will do accept()
         * to drain it.
         */
        if ret == 0 || errno == ECONNRESET { send_control = true; }
        /* Set CID to 0 cause a transport change. */
        sa.svm_cid = 0;
        /* There is a case where this will not fail:
         * if the previous connect() is interrupted while the
         * connection request is already sent, this second
         * connect() will wait for the response.
         */
        ret = connect(s, &sa as *const _ as *const sockaddr, size_of::<sockaddr_vm>() as socklen_t);
        if ret == 0 || errno == ECONNRESET { send_control = true; }
        close(s);
        if send_control { control_writeulong(CONTROL_CONTINUE); }
    }
    control_writeulong(CONTROL_DONE);
    ret = pthread_cancel(thread_id);
    if ret != 0 {
        fprintf(stderr, c"pthread_cancel: %d\n".as_ptr(), ret);
        exit(EXIT_FAILURE);
    }
    ret = pthread_join(thread_id, null_mut());
    if ret != 0 {
        fprintf(stderr, c"pthread_join: %d\n".as_ptr(), ret);
        exit(EXIT_FAILURE);
    }
    if signal(SIGUSR1, old_handler) == SIG_ERR {
        die_perror(c"signal".as_ptr());
    }
}

unsafe fn test_stream_transport_change_server(opts: *const test_opts) {
    let s = vsock_stream_listen(VMADDR_CID_ANY, (*opts).peer_port);
    /* Set the socket to be nonblocking because connects that have been interrupted
     * (EINTR) can fill the receiver's accept queue anyway, leading to connect failure.
     * As of today (6.15) in such situation there is no way to understand, from the
     * client side, if the connection has been queued in the server or not.
     */
    if fcntl(s, F_SETFL, fcntl(s, F_GETFL, 0) | O_NONBLOCK) < 0 {
        die_perror(c"fcntl".as_ptr());
    }
    control_writeln(c"LISTENING".as_ptr());
    while control_readulong() == CONTROL_CONTINUE {
        /* Must accept the connection, otherwise the `listen`
         * queue will fill up and new connections will fail.
         * There can be more than one queued connection,
         * clear them all.
         */
        loop {
            let client = accept(s, null_mut(), null_mut());
            if client < 0 {
                if errno == EAGAIN { break; }
                die_perror(c"accept".as_ptr());
            }
            close(client);
        }
    }
    close(s);
}

unsafe fn test_stream_linger_client(opts: *const test_opts) {
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    enable_so_linger(fd, 1);
    close(fd);
}

unsafe fn test_stream_linger_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    vsock_wait_remote_close(fd);
    close(fd);
}

unsafe fn test_stream_nolinger_client(opts: *const test_opts) {
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    enable_so_linger(fd, LINGER_TIMEOUT);
    send_byte(fd, 1, 0); /* Left unread to expose incorrect behaviour. */
    let waited = vsock_wait_sent(fd);
    let mut ns = current_nsec();
    close(fd);
    ns = current_nsec() - ns;
    if !waited {
        fprintf(stderr, c"Test skipped, SIOCOUTQ not supported.\n".as_ptr());
    } else if (ns + NSEC_PER_SEC - 1) / NSEC_PER_SEC >= LINGER_TIMEOUT as time_t {
        fprintf(stderr, c"Unexpected lingering\n".as_ptr());
        exit(EXIT_FAILURE);
    }
    control_writeln(c"DONE".as_ptr());
}

unsafe fn test_stream_nolinger_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    control_expectln(c"DONE".as_ptr());
    close(fd);
}

unsafe fn test_stream_accepted_setsockopt_client(opts: *const test_opts) {
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    close(fd);
}

unsafe fn test_stream_accepted_setsockopt_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    enable_so_zerocopy_check(fd);
    close(fd);
}

unsafe fn test_stream_tx_credit_bounds_client(opts: *const test_opts) {
    let mut total: size_t = 0;
    let mut buf = [0i8; 4096];
    memset(buf.as_mut_ptr() as *mut c_void, b'A' as c_int, buf.len());
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    let sock_buf_size = SOCK_BUF_SIZE_SMALL as c_ulonglong;
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_MAX_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_MAX_SIZE)".as_ptr());
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_SIZE)".as_ptr());
    if fcntl(fd, F_SETFL, fcntl(fd, F_GETFL, 0) | O_NONBLOCK) < 0 {
        die_perror(c"fcntl(F_SETFL)".as_ptr());
    }
    control_expectln(c"SRVREADY".as_ptr());
    loop {
        let sent = send(fd, buf.as_ptr() as *const c_void, buf.len(), 0);
        if sent == 0 {
            fprintf(stderr, c"unexpected EOF while sending bytes\n".as_ptr());
            exit(EXIT_FAILURE);
        }
        if sent < 0 {
            if errno == EINTR { continue; }
            if errno == EAGAIN || errno == EWOULDBLOCK { break; }
            die_perror(c"send".as_ptr());
        }
        total += sent as usize;
    }
    control_writeln(c"CLIDONE".as_ptr());
    close(fd);
    /* We should not be able to send more bytes than the value set as
     * local buffer size.
     */
    if total as c_ulonglong > sock_buf_size {
        fprintf(stderr, c"TX credit too large: queued %zu bytes (expected <= %llu)\n".as_ptr(), total, sock_buf_size);
        exit(EXIT_FAILURE);
    }
}

unsafe fn test_stream_tx_credit_bounds_server(opts: *const test_opts) {
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    let sock_buf_size = SOCK_BUF_SIZE as c_ulonglong;
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_MAX_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_MAX_SIZE)".as_ptr());
    setsockopt_ull_check(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE, sock_buf_size, c"setsockopt(SO_VM_SOCKETS_BUFFER_SIZE)".as_ptr());
    control_writeln(c"SRVREADY".as_ptr());
    control_expectln(c"CLIDONE".as_ptr());
    close(fd);
}

/* Test that many small packets don't cause a connection reset under pressure
 * and that data integrity is preserved.  Packet sizes vary randomly between
 * 129 and 512 bytes, above GOOD_COPY_LEN (128) to bypass in-place coalescing
 * in recv_enqueue, forcing each one into its own skb.  Without receive queue
 * collapsing, the per-skb overhead eventually exceeds buf_alloc and the
 * connection is reset.
 */
unsafe fn test_stream_collapse_client(opts: *const test_opts) {
    let data = malloc(COLLAPSE_TOTAL) as *mut u8;
    if data.is_null() { die_perror(c"malloc".as_ptr()); }
    for i in 0..COLLAPSE_TOTAL { *data.add(i) = (rand() & 0xff) as u8; }
    let fd = vsock_stream_connect((*opts).peer_cid, (*opts).peer_port);
    if fd < 0 { die_perror(c"connect".as_ptr()); }
    let mut offset: size_t = 0;
    while offset < COLLAPSE_TOTAL {
        let mut pkt_size = COLLAPSE_PKT_MIN + rand() as usize % (COLLAPSE_PKT_MAX - COLLAPSE_PKT_MIN + 1);
        pkt_size = core::cmp::min(pkt_size, COLLAPSE_TOTAL - offset);
        send_buf(fd, data.add(offset) as *const c_void, pkt_size, 0, pkt_size as ssize_t);
        offset += pkt_size;
    }
    let hash = hash_djb2(data as *const c_void, COLLAPSE_TOTAL);
    control_writeulong(hash);
    free(data as *mut c_void);
    close(fd);
}

unsafe fn test_stream_collapse_server(opts: *const test_opts) {
    let data = malloc(COLLAPSE_TOTAL) as *mut u8;
    if data.is_null() { die_perror(c"malloc".as_ptr()); }
    let fd = vsock_stream_accept(VMADDR_CID_ANY, (*opts).peer_port, null_mut());
    if fd < 0 { die_perror(c"accept".as_ptr()); }
    recv_buf(fd, data as *mut c_void, COLLAPSE_TOTAL, 0, COLLAPSE_TOTAL as ssize_t);
    let hash = hash_djb2(data as *const c_void, COLLAPSE_TOTAL);
    let remote_hash = control_readulong();
    if hash != remote_hash {
        fprintf(stderr, c"hash mismatch: local %lu remote %lu\n".as_ptr(), hash, remote_hash);
        exit(EXIT_FAILURE);
    }
    free(data as *mut c_void);
    close(fd);
}

static mut test_cases: [test_case; 43] = [
    test_case { name: c"SOCK_STREAM connection reset".as_ptr(), run_client: Some(test_stream_connection_reset), run_server: None, skip: false },
    test_case { name: c"SOCK_STREAM bind only".as_ptr(), run_client: Some(test_stream_bind_only_client), run_server: Some(test_stream_bind_only_server), skip: false },
    test_case { name: c"SOCK_STREAM client close".as_ptr(), run_client: Some(test_stream_client_close_client), run_server: Some(test_stream_client_close_server), skip: false },
    test_case { name: c"SOCK_STREAM server close".as_ptr(), run_client: Some(test_stream_server_close_client), run_server: Some(test_stream_server_close_server), skip: false },
    test_case { name: c"SOCK_STREAM multiple connections".as_ptr(), run_client: Some(test_stream_multiconn_client), run_server: Some(test_stream_multiconn_server), skip: false },
    test_case { name: c"SOCK_STREAM MSG_PEEK".as_ptr(), run_client: Some(test_stream_msg_peek_client), run_server: Some(test_stream_msg_peek_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET msg bounds".as_ptr(), run_client: Some(test_seqpacket_msg_bounds_client), run_server: Some(test_seqpacket_msg_bounds_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET MSG_TRUNC flag".as_ptr(), run_client: Some(test_seqpacket_msg_trunc_client), run_server: Some(test_seqpacket_msg_trunc_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET timeout".as_ptr(), run_client: Some(test_seqpacket_timeout_client), run_server: Some(test_seqpacket_timeout_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET invalid receive buffer".as_ptr(), run_client: Some(test_seqpacket_invalid_rec_buffer_client), run_server: Some(test_seqpacket_invalid_rec_buffer_server), skip: false },
    test_case { name: c"SOCK_STREAM poll() + SO_RCVLOWAT".as_ptr(), run_client: Some(test_stream_poll_rcvlowat_client), run_server: Some(test_stream_poll_rcvlowat_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET big message".as_ptr(), run_client: Some(test_seqpacket_bigmsg_client), run_server: Some(test_seqpacket_bigmsg_server), skip: false },
    test_case { name: c"SOCK_STREAM test invalid buffer".as_ptr(), run_client: Some(test_stream_inv_buf_client), run_server: Some(test_stream_inv_buf_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET test invalid buffer".as_ptr(), run_client: Some(test_seqpacket_inv_buf_client), run_server: Some(test_seqpacket_inv_buf_server), skip: false },
    test_case { name: c"SOCK_STREAM virtio skb merge".as_ptr(), run_client: Some(test_stream_virtio_skb_merge_client), run_server: Some(test_stream_virtio_skb_merge_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET MSG_PEEK".as_ptr(), run_client: Some(test_seqpacket_msg_peek_client), run_server: Some(test_seqpacket_msg_peek_server), skip: false },
    test_case { name: c"SOCK_STREAM SHUT_WR".as_ptr(), run_client: Some(test_stream_shutwr_client), run_server: Some(test_stream_shutwr_server), skip: false },
    test_case { name: c"SOCK_STREAM SHUT_RD".as_ptr(), run_client: Some(test_stream_shutrd_client), run_server: Some(test_stream_shutrd_server), skip: false },
    test_case { name: c"SOCK_STREAM MSG_ZEROCOPY".as_ptr(), run_client: Some(test_stream_msgzcopy_client), run_server: Some(test_stream_msgzcopy_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET MSG_ZEROCOPY".as_ptr(), run_client: Some(test_seqpacket_msgzcopy_client), run_server: Some(test_seqpacket_msgzcopy_server), skip: false },
    test_case { name: c"SOCK_STREAM MSG_ZEROCOPY empty MSG_ERRQUEUE".as_ptr(), run_client: Some(test_stream_msgzcopy_empty_errq_client), run_server: Some(test_stream_msgzcopy_empty_errq_server), skip: false },
    test_case { name: c"SOCK_STREAM double bind connect".as_ptr(), run_client: Some(test_double_bind_connect_client), run_server: Some(test_double_bind_connect_server), skip: false },
    test_case { name: c"SOCK_STREAM virtio credit update + SO_RCVLOWAT".as_ptr(), run_client: Some(test_stream_rcvlowat_def_cred_upd_client), run_server: Some(test_stream_cred_upd_on_set_rcvlowat), skip: false },
    test_case { name: c"SOCK_STREAM virtio credit update + low rx_bytes".as_ptr(), run_client: Some(test_stream_rcvlowat_def_cred_upd_client), run_server: Some(test_stream_cred_upd_on_low_rx_bytes), skip: false },
    test_case { name: c"SOCK_STREAM ioctl(SIOCOUTQ) 0 unsent bytes".as_ptr(), run_client: Some(test_stream_unsent_bytes_client), run_server: Some(test_stream_unsent_bytes_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET ioctl(SIOCOUTQ) 0 unsent bytes".as_ptr(), run_client: Some(test_seqpacket_unsent_bytes_client), run_server: Some(test_seqpacket_unsent_bytes_server), skip: false },
    test_case { name: c"SOCK_STREAM leak accept queue".as_ptr(), run_client: Some(test_stream_leak_acceptq_client), run_server: Some(test_stream_leak_acceptq_server), skip: false },
    test_case { name: c"SOCK_STREAM MSG_ZEROCOPY leak MSG_ERRQUEUE".as_ptr(), run_client: Some(test_stream_msgzcopy_leak_errq_client), run_server: Some(test_stream_msgzcopy_leak_errq_server), skip: false },
    test_case { name: c"SOCK_STREAM MSG_ZEROCOPY leak completion skb".as_ptr(), run_client: Some(test_stream_msgzcopy_leak_zcskb_client), run_server: Some(test_stream_msgzcopy_leak_zcskb_server), skip: false },
    test_case { name: c"SOCK_STREAM transport release use-after-free".as_ptr(), run_client: Some(test_stream_transport_uaf_client), run_server: None, skip: false },
    test_case { name: c"SOCK_STREAM retry failed connect()".as_ptr(), run_client: Some(test_stream_connect_retry_client), run_server: Some(test_stream_connect_retry_server), skip: false },
    test_case { name: c"SOCK_STREAM SO_LINGER null-ptr-deref".as_ptr(), run_client: Some(test_stream_linger_client), run_server: Some(test_stream_linger_server), skip: false },
    test_case { name: c"SOCK_STREAM SO_LINGER close() on unread".as_ptr(), run_client: Some(test_stream_nolinger_client), run_server: Some(test_stream_nolinger_server), skip: false },
    test_case { name: c"SOCK_STREAM transport change null-ptr-deref, lockdep warn".as_ptr(), run_client: Some(test_stream_transport_change_client), run_server: Some(test_stream_transport_change_server), skip: false },
    test_case { name: c"SOCK_STREAM ioctl(SIOCINQ) functionality".as_ptr(), run_client: Some(test_stream_unread_bytes_client), run_server: Some(test_stream_unread_bytes_server), skip: false },
    test_case { name: c"SOCK_SEQPACKET ioctl(SIOCINQ) functionality".as_ptr(), run_client: Some(test_seqpacket_unread_bytes_client), run_server: Some(test_seqpacket_unread_bytes_server), skip: false },
    test_case { name: c"SOCK_STREAM accept()ed socket custom setsockopt()".as_ptr(), run_client: Some(test_stream_accepted_setsockopt_client), run_server: Some(test_stream_accepted_setsockopt_server), skip: false },
    test_case { name: c"SOCK_STREAM virtio MSG_ZEROCOPY coalescence corruption".as_ptr(), run_client: Some(test_stream_msgzcopy_mangle_client), run_server: Some(test_stream_msgzcopy_mangle_server), skip: false },
    test_case { name: c"SOCK_STREAM TX credit bounds".as_ptr(), run_client: Some(test_stream_tx_credit_bounds_client), run_server: Some(test_stream_tx_credit_bounds_server), skip: false },
    test_case { name: c"SOCK_STREAM MSG_PEEK after partial recv".as_ptr(), run_client: Some(test_stream_msg_peek_client), run_server: Some(test_stream_peek_after_recv_server), skip: false },
    test_case { name: c"SOCK_STREAM small packets backpressure".as_ptr(), run_client: Some(test_stream_collapse_client), run_server: Some(test_stream_collapse_server), skip: false },
    test_case { name: null(), run_client: None, run_server: None, skip: false },
    test_case { name: null(), run_client: None, run_server: None, skip: false },
];

static optstring: &[u8] = b"\0";
static longopts: [option; 10] = [
    option { name: c"control-host".as_ptr(), has_arg: required_argument, flag: null_mut(), val: b'H' as c_int },
    option { name: c"control-port".as_ptr(), has_arg: required_argument, flag: null_mut(), val: b'P' as c_int },
    option { name: c"mode".as_ptr(), has_arg: required_argument, flag: null_mut(), val: b'm' as c_int },
    option { name: c"peer-cid".as_ptr(), has_arg: required_argument, flag: null_mut(), val: b'p' as c_int },
    option { name: c"peer-port".as_ptr(), has_arg: required_argument, flag: null_mut(), val: b'q' as c_int },
    option { name: c"list".as_ptr(), has_arg: no_argument, flag: null_mut(), val: b'l' as c_int },
    option { name: c"skip".as_ptr(), has_arg: required_argument, flag: null_mut(), val: b's' as c_int },
    option { name: c"pick".as_ptr(), has_arg: required_argument, flag: null_mut(), val: b't' as c_int },
    option { name: c"help".as_ptr(), has_arg: no_argument, flag: null_mut(), val: b'?' as c_int },
    option { name: null(), has_arg: 0, flag: null_mut(), val: 0 },
];

unsafe fn usage() -> ! {
    fprintf(stderr, c"Usage: vsock_test [--help] [--control-host=<host>] --control-port=<port> --mode=client|server --peer-cid=<cid> [--peer-port=<port>] [--list] [--skip=<test_id>]\n\n  Server: vsock_test --control-port=1234 --mode=server --peer-cid=3\n  Client: vsock_test --control-host=192.168.0.1 --control-port=1234 --mode=client --peer-cid=2\n\nRun vsock.ko tests.  Must be launched in both guest\nand host.  One side must use --mode=client and\nthe other side must use --mode=server.\n\nA TCP control socket connection is used to coordinate tests\nbetween the client and the server.  The server requires a\nlisten address and the client requires an address to\nconnect to.\n\nThe CID of the other side must be given with --peer-cid=<cid>.\nDuring the test, two AF_VSOCK ports will be used: the port\nspecified with --peer-port=<port> (or the default port)\nand the next one.\n\nOptions:\n  --help                 This help message\n  --control-host <host>  Server IP address to connect to\n  --control-port <port>  Server port to listen on/connect to\n  --mode client|server   Server or client mode\n  --peer-cid <cid>       CID of the other side\n  --peer-port <port>     AF_VSOCK port used for the test [default: %d]\n  --list                 List of tests that will be executed\n  --pick <test_id>       Test ID to execute selectively;\n                         use multiple --pick options to select more tests\n  --skip <test_id>       Test ID to skip;\n                         use multiple --skip options to skip more tests\n".as_ptr(), DEFAULT_PEER_PORT);
    exit(EXIT_FAILURE);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut control_host: *const c_char = null();
    let mut control_port: *const c_char = null();
    let mut opts = test_opts {
        mode: TEST_MODE_UNSET,
        peer_cid: VMADDR_CID_ANY,
        peer_port: DEFAULT_PEER_PORT,
    };
    srand(time(null_mut()) as c_uint);
    init_signals();
    loop {
        let opt = getopt_long(argc, argv, optstring.as_ptr() as *const c_char, longopts.as_ptr(), null_mut());
        if opt == -1 { break; }
        match opt {
            x if x == b'H' as c_int => control_host = optarg,
            x if x == b'm' as c_int => {
                if strcmp(optarg, c"client".as_ptr()) == 0 {
                    opts.mode = TEST_MODE_CLIENT;
                } else if strcmp(optarg, c"server".as_ptr()) == 0 {
                    opts.mode = TEST_MODE_SERVER;
                } else {
                    fprintf(stderr, c"--mode must be \"client\" or \"server\"\n".as_ptr());
                    return EXIT_FAILURE;
                }
            }
            x if x == b'p' as c_int => opts.peer_cid = parse_cid(optarg),
            x if x == b'q' as c_int => opts.peer_port = parse_port(optarg),
            x if x == b'P' as c_int => control_port = optarg,
            x if x == b'l' as c_int => list_tests(test_cases.as_mut_ptr()),
            x if x == b's' as c_int => skip_test(test_cases.as_mut_ptr(), test_cases.len() - 1, optarg),
            x if x == b't' as c_int => pick_test(test_cases.as_mut_ptr(), test_cases.len() - 1, optarg),
            _ => usage(),
        }
    }
    if control_port.is_null() { usage(); }
    if opts.mode == TEST_MODE_UNSET { usage(); }
    if opts.peer_cid == VMADDR_CID_ANY { usage(); }
    if control_host.is_null() {
        if opts.mode != TEST_MODE_SERVER { usage(); }
        control_host = c"0.0.0.0".as_ptr();
    }
    control_init(control_host, control_port, opts.mode == TEST_MODE_SERVER);
    run_tests(test_cases.as_mut_ptr(), &opts);
    control_cleanup();
    EXIT_SUCCESS
}
