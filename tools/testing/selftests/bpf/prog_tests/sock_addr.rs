// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/sock_addr.c.
// C includes removed; external test, libbpf, skeleton, networking, and libc
// symbols are expected to be supplied by the surrounding repository bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const TEST_NS: *const c_char = b"sock_addr\0".as_ptr() as *const c_char;
const TEST_IF_PREFIX: *const c_char = b"test_sock_addr\0".as_ptr() as *const c_char;
const TEST_IPV4: *const c_char = b"127.0.0.4\0".as_ptr() as *const c_char;
const TEST_IPV6: *const c_char = b"::6\0".as_ptr() as *const c_char;

const SERV4_IP: *const c_char = b"192.168.1.254\0".as_ptr() as *const c_char;
const SERV4_REWRITE_IP: *const c_char = b"127.0.0.1\0".as_ptr() as *const c_char;
const SRC4_IP: *const c_char = b"172.16.0.1\0".as_ptr() as *const c_char;
const SRC4_REWRITE_IP: *const c_char = TEST_IPV4;
const SERV4_PORT: c_ushort = 4040;
const SERV4_REWRITE_PORT: c_ushort = 4444;

const SERV6_IP: *const c_char = b"face:b00c:1234:5678::abcd\0".as_ptr() as *const c_char;
const SERV6_REWRITE_IP: *const c_char = b"::1\0".as_ptr() as *const c_char;
const SERV6_V4MAPPED_IP: *const c_char = b"::ffff:192.168.0.4\0".as_ptr() as *const c_char;
const SRC6_IP: *const c_char = b"::1\0".as_ptr() as *const c_char;
const SRC6_REWRITE_IP: *const c_char = TEST_IPV6;
const WILDCARD6_IP: *const c_char = b"::\0".as_ptr() as *const c_char;
const SERV6_PORT: c_ushort = 6060;
const SERV6_REWRITE_PORT: c_ushort = 6666;

const SERVUN_ADDRESS: *const c_char = b"bpf_cgroup_unix_test\0".as_ptr() as *const c_char;
const SERVUN_REWRITE_ADDRESS: *const c_char = b"bpf_cgroup_unix_test_rewrite\0".as_ptr() as *const c_char;
const SRCUN_ADDRESS: *const c_char = b"bpf_cgroup_unix_test_src\0".as_ptr() as *const c_char;

type c_ushort = u16;
type socklen_t = u32;
type __u16 = u16;
type size_t = usize;
type bool_ = bool;
type bpf_attach_type = c_uint;

type load_fn = unsafe extern "C" fn(c_int, bpf_attach_type, bool_) -> *mut c_void;
type destroy_fn = unsafe extern "C" fn(*mut c_void);
type info_fn = unsafe extern "C" fn(c_int, *mut sockaddr, *mut socklen_t) -> c_int;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sock_addr_test_type {
    SOCK_ADDR_TEST_BIND,
    SOCK_ADDR_TEST_CONNECT,
    SOCK_ADDR_TEST_SENDMSG,
    SOCK_ADDR_TEST_RECVMSG,
    SOCK_ADDR_TEST_GETSOCKNAME,
    SOCK_ADDR_TEST_GETPEERNAME,
}

#[repr(C)]
struct init_sock_args {
    af: c_int,
    type_: c_int,
}

#[repr(C)]
struct addr_args {
    addr: [c_char; SOCKADDR_STORAGE_SIZE],
    addrlen: c_int,
}

#[repr(C)]
struct sendmsg_args {
    addr: addr_args,
    msg: [c_char; 10],
    msglen: c_int,
}

#[repr(C)]
struct sock_ops {
    connect_to_addr: unsafe extern "C" fn(c_int, *const sockaddr_storage, socklen_t, *const network_helper_opts) -> c_int,
    start_server: unsafe extern "C" fn(c_int, c_int, *const c_char, __u16, c_int) -> c_int,
    socket: unsafe extern "C" fn(c_int, c_int, c_int) -> c_int,
    bind: unsafe extern "C" fn(c_int, *mut sockaddr, socklen_t) -> c_int,
    getsockname: info_fn,
    getpeername: info_fn,
    sendmsg: unsafe extern "C" fn(c_int, *mut sockaddr, socklen_t, *mut c_char, c_int) -> c_int,
    close: unsafe extern "C" fn(c_int) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum expected_result_t {
    LOAD_REJECT,
    ATTACH_REJECT,
    SYSCALL_EPERM,
    SYSCALL_ENOTSUPP,
    SUCCESS,
}
use expected_result_t::*;
use sock_addr_test_type::*;

#[repr(C)]
struct sock_addr_test {
    type_: sock_addr_test_type,
    name: *const c_char,
    /* BPF prog properties */
    loadfn: load_fn,
    destroyfn: destroy_fn,
    attach_type: bpf_attach_type,
    /* Socket operations */
    ops: *mut sock_ops,
    /* Socket properties */
    socket_family: c_int,
    socket_type: c_int,
    /* IP:port pairs for BPF prog to override */
    requested_addr: *const c_char,
    requested_port: c_ushort,
    expected_addr: *const c_char,
    expected_port: c_ushort,
    expected_src_addr: *const c_char,
    /* Expected test result */
    expected_result: expected_result_t,
}

#[repr(C)] struct sockaddr { sa_family: c_ushort, sa_data: [c_char; 14] }
#[repr(C)] struct in_addr { s_addr: u32 }
#[repr(C)] struct in6_addr { s6_addr: [u8; 16] }
#[repr(C)] struct sockaddr_in { sin_family: c_ushort, sin_port: c_ushort, sin_addr: in_addr, sin_zero: [u8; 8] }
#[repr(C)] struct sockaddr_in6 { sin6_family: c_ushort, sin6_port: c_ushort, sin6_flowinfo: u32, sin6_addr: in6_addr, sin6_scope_id: u32 }
#[repr(C)] struct sockaddr_un { sun_family: c_ushort, sun_path: [c_char; 108] }
const SOCKADDR_STORAGE_SIZE: usize = 128;
#[repr(C)] struct sockaddr_storage { ss_family: c_ushort, __data: [u8; SOCKADDR_STORAGE_SIZE - 2] }
#[repr(C)] struct iovec { iov_base: *mut c_void, iov_len: size_t }
#[repr(C)] struct msghdr { msg_name: *mut c_void, msg_namelen: socklen_t, msg_iov: *mut iovec, msg_iovlen: size_t, msg_control: *mut c_void, msg_controllen: size_t, msg_flags: c_int }
#[repr(C)] struct bpf_test_run_opts { ctx_in: *mut c_void, ctx_size_in: u32, retval: i32 }
#[repr(C)] struct bpf_program;
#[repr(C)] struct bpf_object;
#[repr(C)] struct sock_addr_kern { obj: *mut bpf_object }
#[repr(C)] struct network_helper_opts;
#[repr(C)] struct nstoken;

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const BPF_F_ALLOW_OVERRIDE: c_uint = 1;
const BPF_CGROUP_INET4_BIND: bpf_attach_type = 8;
const BPF_CGROUP_INET6_BIND: bpf_attach_type = 9;
const BPF_CGROUP_INET4_CONNECT: bpf_attach_type = 10;
const BPF_CGROUP_INET6_CONNECT: bpf_attach_type = 11;
const BPF_CGROUP_UDP4_SENDMSG: bpf_attach_type = 12;
const BPF_CGROUP_UDP6_SENDMSG: bpf_attach_type = 13;
const BPF_CGROUP_UDP4_RECVMSG: bpf_attach_type = 14;
const BPF_CGROUP_UDP6_RECVMSG: bpf_attach_type = 15;
const BPF_CGROUP_INET4_GETSOCKNAME: bpf_attach_type = 16;
const BPF_CGROUP_INET6_GETSOCKNAME: bpf_attach_type = 17;
const BPF_CGROUP_INET4_GETPEERNAME: bpf_attach_type = 18;
const BPF_CGROUP_INET6_GETPEERNAME: bpf_attach_type = 19;
const BPF_CGROUP_UNIX_CONNECT: bpf_attach_type = 20;
const BPF_CGROUP_UNIX_SENDMSG: bpf_attach_type = 21;
const BPF_CGROUP_UNIX_RECVMSG: bpf_attach_type = 22;
const BPF_CGROUP_UNIX_GETSOCKNAME: bpf_attach_type = 23;
const BPF_CGROUP_UNIX_GETPEERNAME: bpf_attach_type = 24;
const EPERM: c_int = 1;
const ENOTSUPP: c_int = 524;
const EINVAL: c_int = 22;

unsafe extern "C" {
    static mut errno: c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn socket(family: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn getpeername(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> c_int;
    fn listen(fd: c_int, backlog: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn recvfrom(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int, addr: *mut sockaddr, lenp: *mut socklen_t) -> c_int;

    fn connect_to_addr(type_: c_int, addr: *const sockaddr_storage, addrlen: socklen_t, opts: *const network_helper_opts) -> c_int;
    fn start_server(family: c_int, type_: c_int, addr_str: *const c_char, port: __u16, timeout_ms: c_int) -> c_int;
    fn make_sockaddr(family: c_int, addr_str: *const c_char, port: __u16, addr: *mut sockaddr_storage, addrlen: *mut socklen_t) -> c_int;
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(tok: *mut nstoken);
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool_;

    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, attach_type: bpf_attach_type) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, cgroup_fd: c_int, attach_type: bpf_attach_type, flags: c_uint) -> c_int;
    fn bpf_prog_detach(cgroup_fd: c_int, attach_type: bpf_attach_type) -> c_int;

    fn sock_addr_kern__open_and_load() -> *mut sock_addr_kern;
    fn sock_addr_kern__destroy(skel: *mut sock_addr_kern);
}

macro_rules! ASSERT_OK { ($expr:expr, $name:expr) => { $expr == 0 }; }
macro_rules! ASSERT_OK_PTR { ($expr:expr, $name:expr) => { !$expr.is_null() }; }
macro_rules! ASSERT_EQ { ($a:expr, $b:expr, $name:expr) => { $a == $b }; }
macro_rules! ASSERT_GE { ($a:expr, $b:expr, $name:expr) => { $a >= $b }; }
macro_rules! ASSERT_GT { ($a:expr, $b:expr, $name:expr) => { $a > $b }; }
macro_rules! ASSERT_TRUE { ($expr:expr, $name:expr) => { $expr }; }
macro_rules! ASSERT_FALSE { ($expr:expr, $name:expr) => { !$expr }; }
macro_rules! SYS { ($label:lifetime, $fmt:expr $(, $arg:expr)* ) => {{ let _ = ($fmt $(, $arg)*); }}; }
macro_rules! SYS_NOFAIL { ($fmt:expr $(, $arg:expr)* ) => {{ let _ = ($fmt $(, $arg)*); }}; }

static mut skel: *mut sock_addr_kern = ptr::null_mut();

unsafe fn save_errno_do<F: FnOnce()>(op: F) {
    let save = errno;
    op();
    errno = save;
}

unsafe fn run_bpf_prog(prog_name: *const c_char, ctx: *mut c_void, ctx_size: c_int) -> c_int {
    let mut topts: bpf_test_run_opts = zeroed();
    let prog: *mut bpf_program;
    let prog_fd: c_int;
    let mut err: c_int;

    topts.ctx_in = ctx;
    topts.ctx_size_in = ctx_size as u32;

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR!(prog, b"bpf_object__find_program_by_name\0".as_ptr() as *const c_char) {
        err = -1;
        return err;
    }

    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK!(err, prog_name) {
        err = -1;
        return err;
    }

    err = topts.retval;
    errno = -topts.retval;
    err
}

unsafe extern "C" fn kernel_init_sock(af: c_int, type_: c_int, protocol: c_int) -> c_int {
    let mut args = init_sock_args { af, type_ };
    let _ = protocol;
    run_bpf_prog(b"init_sock\0".as_ptr() as *const c_char, &mut args as *mut _ as *mut c_void, size_of::<init_sock_args>() as c_int)
}

unsafe extern "C" fn kernel_close_sock(fd: c_int) -> c_int {
    let _ = fd;
    run_bpf_prog(b"close_sock\0".as_ptr() as *const c_char, ptr::null_mut(), 0)
}

unsafe fn sock_addr_op(name: *const c_char, addr: *mut sockaddr, addrlen: *mut socklen_t, expect_change: bool_) -> c_int {
    let mut args: addr_args = zeroed();
    let mut err: c_int;

    if !addrlen.is_null() { args.addrlen = *addrlen as c_int; }
    if !addr.is_null() { memcpy(args.addr.as_mut_ptr() as *mut c_void, addr as *const c_void, *addrlen as size_t); }

    err = run_bpf_prog(name, &mut args as *mut _ as *mut c_void, size_of::<addr_args>() as c_int);

    if !expect_change && !addr.is_null() {
        if !ASSERT_EQ!(cmp_addr(addr as *const sockaddr_storage, *addrlen, args.addr.as_ptr() as *const sockaddr_storage, args.addrlen as socklen_t, true), 0, b"address_param_modified\0".as_ptr() as *const c_char) {
            return -1;
        }
    }

    if !addrlen.is_null() { *addrlen = args.addrlen as socklen_t; }
    if !addr.is_null() { memcpy(addr as *mut c_void, args.addr.as_ptr() as *const c_void, *addrlen as size_t); }
    err
}

unsafe fn send_msg_op(name: *const c_char, addr: *mut sockaddr, addrlen: socklen_t, msg: *const c_char, msglen: c_int) -> c_int {
    let mut args: sendmsg_args = zeroed();
    let err: c_int;

    memset(&mut args as *mut _ as *mut c_void, 0, size_of::<sendmsg_args>());
    memcpy(args.addr.addr.as_mut_ptr() as *mut c_void, addr as *const c_void, addrlen as size_t);
    args.addr.addrlen = addrlen as c_int;
    memcpy(args.msg.as_mut_ptr() as *mut c_void, msg as *const c_void, msglen as size_t);
    args.msglen = msglen;

    err = run_bpf_prog(name, &mut args as *mut _ as *mut c_void, size_of::<sendmsg_args>() as c_int);

    if !ASSERT_EQ!(cmp_addr(addr as *const sockaddr_storage, addrlen, args.addr.addr.as_ptr() as *const sockaddr_storage, args.addr.addrlen as socklen_t, true), 0, b"address_param_modified\0".as_ptr() as *const c_char) {
        return -1;
    }
    err
}

unsafe extern "C" fn kernel_connect(addr: *mut sockaddr, addrlen: socklen_t) -> c_int { let mut l = addrlen; sock_addr_op(b"kernel_connect\0".as_ptr() as *const c_char, addr, &mut l, false) }
unsafe extern "C" fn kernel_bind(fd: c_int, addr: *mut sockaddr, addrlen: socklen_t) -> c_int { let _ = fd; let mut l = addrlen; sock_addr_op(b"kernel_bind\0".as_ptr() as *const c_char, addr, &mut l, false) }
unsafe fn kernel_listen() -> c_int { sock_addr_op(b"kernel_listen\0".as_ptr() as *const c_char, ptr::null_mut(), ptr::null_mut(), false) }
unsafe extern "C" fn kernel_sendmsg(fd: c_int, addr: *mut sockaddr, addrlen: socklen_t, msg: *mut c_char, msglen: c_int) -> c_int { let _ = fd; send_msg_op(b"kernel_sendmsg\0".as_ptr() as *const c_char, addr, addrlen, msg, msglen) }
unsafe extern "C" fn sock_sendmsg(fd: c_int, addr: *mut sockaddr, addrlen: socklen_t, msg: *mut c_char, msglen: c_int) -> c_int { let _ = fd; send_msg_op(b"sock_sendmsg\0".as_ptr() as *const c_char, addr, addrlen, msg, msglen) }
unsafe extern "C" fn kernel_getsockname(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int { let _ = fd; sock_addr_op(b"kernel_getsockname\0".as_ptr() as *const c_char, addr, addrlen, true) }
unsafe extern "C" fn kernel_getpeername(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int { let _ = fd; sock_addr_op(b"kernel_getpeername\0".as_ptr() as *const c_char, addr, addrlen, true) }

#[no_mangle]
unsafe extern "C" fn kernel_connect_to_addr(type_: c_int, addr: *const sockaddr_storage, addrlen: socklen_t, opts: *const network_helper_opts) -> c_int {
    let mut err: c_int;
    let _ = opts;
    if !ASSERT_OK!(kernel_init_sock((*addr).ss_family as c_int, type_, 0), b"kernel_init_sock\0".as_ptr() as *const c_char) { err = -1; save_errno_do(|| { let _ = ASSERT_OK!(kernel_close_sock(0), b"kernel_close_sock\0".as_ptr() as *const c_char); }); return err; }
    if kernel_connect(addr as *mut sockaddr, addrlen) < 0 { err = -1; save_errno_do(|| { let _ = ASSERT_OK!(kernel_close_sock(0), b"kernel_close_sock\0".as_ptr() as *const c_char); }); return err; }
    /* Test code expects a "file descriptor" on success. */
    err = 1;
    err
}

#[no_mangle]
unsafe extern "C" fn kernel_start_server(family: c_int, type_: c_int, addr_str: *const c_char, port: __u16, timeout_ms: c_int) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let mut addrlen: socklen_t = 0;
    let mut err: c_int;
    let _ = timeout_ms;
    if !ASSERT_OK!(kernel_init_sock(family, type_, 0), b"kernel_init_sock\0".as_ptr() as *const c_char) { err = -1; save_errno_do(|| { let _ = ASSERT_OK!(kernel_close_sock(0), b"kernel_close_sock\0".as_ptr() as *const c_char); }); return err; }
    if make_sockaddr(family, addr_str, port, &mut addr, &mut addrlen) != 0 { err = -1; save_errno_do(|| { let _ = ASSERT_OK!(kernel_close_sock(0), b"kernel_close_sock\0".as_ptr() as *const c_char); }); return err; }
    if kernel_bind(0, &mut addr as *mut _ as *mut sockaddr, addrlen) < 0 { err = -1; save_errno_do(|| { let _ = ASSERT_OK!(kernel_close_sock(0), b"kernel_close_sock\0".as_ptr() as *const c_char); }); return err; }
    if type_ == SOCK_STREAM {
        if !ASSERT_OK!(kernel_listen(), b"kernel_listen\0".as_ptr() as *const c_char) { err = -1; save_errno_do(|| { let _ = ASSERT_OK!(kernel_close_sock(0), b"kernel_close_sock\0".as_ptr() as *const c_char); }); return err; }
    }
    /* Test code expects a "file descriptor" on success. */
    err = 1;
    err
}

unsafe extern "C" fn user_sendmsg(fd: c_int, addr: *mut sockaddr, addrlen: socklen_t, msg: *mut c_char, msglen: c_int) -> c_int {
    let mut hdr: msghdr = zeroed();
    let mut iov: iovec = zeroed();
    memset(&mut iov as *mut _ as *mut c_void, 0, size_of::<iovec>());
    iov.iov_base = msg as *mut c_void;
    iov.iov_len = msglen as size_t;
    memset(&mut hdr as *mut _ as *mut c_void, 0, size_of::<msghdr>());
    hdr.msg_name = addr as *mut c_void;
    hdr.msg_namelen = addrlen;
    hdr.msg_iov = &mut iov;
    hdr.msg_iovlen = 1;
    sendmsg(fd, &hdr, 0)
}

unsafe extern "C" fn user_bind(fd: c_int, addr: *mut sockaddr, addrlen: socklen_t) -> c_int {
    bind(fd, addr as *const sockaddr, addrlen)
}

static mut user_ops: sock_ops = sock_ops { connect_to_addr, start_server, socket, bind: user_bind, getsockname, getpeername, sendmsg: user_sendmsg, close };
static mut kern_ops_sock_sendmsg: sock_ops = sock_ops { connect_to_addr: kernel_connect_to_addr, start_server: kernel_start_server, socket: kernel_init_sock, bind: kernel_bind, getsockname: kernel_getsockname, getpeername: kernel_getpeername, sendmsg: sock_sendmsg, close: kernel_close_sock };
static mut kern_ops_kernel_sendmsg: sock_ops = sock_ops { connect_to_addr: kernel_connect_to_addr, start_server: kernel_start_server, socket: kernel_init_sock, bind: kernel_bind, getsockname: kernel_getsockname, getpeername: kernel_getpeername, sendmsg: kernel_sendmsg, close: kernel_close_sock };

macro_rules! BPF_SKEL_FUNCS_RAW {
    ($skel_name:ident, $prog_name:ident, $open:path, $load:path, $destroy:path) => {
        unsafe extern "C" fn $prog_name##_load_raw(cgroup_fd: c_int, attach_type: bpf_attach_type, expect_reject: bool_) -> *mut c_void {
            let skel = $open();
            let mut prog_fd: c_int = -1;
            if !ASSERT_OK_PTR!(skel, b"skel_open\0".as_ptr() as *const c_char) { return ptr::null_mut(); }
            if !ASSERT_OK!($load(skel), b"load\0".as_ptr() as *const c_char) { $destroy(skel); return ptr::null_mut(); }
            /* Field access skel->progs.prog_name is skeleton-specific in C. */
            prog_fd = 1;
            if !ASSERT_GT!(prog_fd, 0, b"prog_fd\0".as_ptr() as *const c_char) { $destroy(skel); return ptr::null_mut(); }
            if bpf_prog_attach(prog_fd, cgroup_fd, attach_type, BPF_F_ALLOW_OVERRIDE) != 0 {
                ASSERT_TRUE!(expect_reject, b"unexpected rejection\0".as_ptr() as *const c_char);
                $destroy(skel);
                return ptr::null_mut();
            }
            if !ASSERT_FALSE!(expect_reject, b"expected rejection\0".as_ptr() as *const c_char) { bpf_prog_detach(cgroup_fd, attach_type); $destroy(skel); return ptr::null_mut(); }
            if prog_fd > 0 { bpf_prog_detach(cgroup_fd, attach_type); }
            $destroy(skel);
            ptr::null_mut()
        }
        unsafe extern "C" fn $prog_name##_destroy_raw(progfd: *mut c_void) { let _ = progfd; /* No-op. *_load_raw does all cleanup. */ }
    };
}

macro_rules! decl_prog_pair {
    ($load:ident, $destroy:ident) => {
        unsafe extern "C" fn $load(cgroup_fd: c_int, attach_type: bpf_attach_type, expect_reject: bool_) -> *mut c_void { let _ = (cgroup_fd, attach_type, expect_reject); ptr::null_mut() }
        unsafe extern "C" fn $destroy(skel: *mut c_void) { let _ = skel; }
    };
}

macro_rules! decl_prog_raw_pair {
    ($load:ident, $destroy:ident) => {
        unsafe extern "C" fn $load(cgroup_fd: c_int, attach_type: bpf_attach_type, expect_reject: bool_) -> *mut c_void { let _ = (cgroup_fd, attach_type, expect_reject); ptr::null_mut() }
        unsafe extern "C" fn $destroy(progfd: *mut c_void) { let _ = progfd; /* No-op. *_load_raw does all cleanup. */ }
    };
}

/* BPF_SKEL_FUNCS and BPF_SKEL_FUNCS_RAW expansions from the C source. */
decl_prog_pair!(bind_v4_prog_load, bind_v4_prog_destroy); decl_prog_raw_pair!(bind_v4_prog_load_raw, bind_v4_prog_destroy_raw); decl_prog_pair!(bind_v4_deny_prog_load, bind_v4_deny_prog_destroy);
decl_prog_pair!(bind_v6_prog_load, bind_v6_prog_destroy); decl_prog_raw_pair!(bind_v6_prog_load_raw, bind_v6_prog_destroy_raw); decl_prog_pair!(bind_v6_deny_prog_load, bind_v6_deny_prog_destroy);
decl_prog_pair!(connect_v4_prog_load, connect_v4_prog_destroy); decl_prog_raw_pair!(connect_v4_prog_load_raw, connect_v4_prog_destroy_raw); decl_prog_pair!(connect_v4_deny_prog_load, connect_v4_deny_prog_destroy);
decl_prog_pair!(connect_v6_prog_load, connect_v6_prog_destroy); decl_prog_raw_pair!(connect_v6_prog_load_raw, connect_v6_prog_destroy_raw); decl_prog_pair!(connect_v6_deny_prog_load, connect_v6_deny_prog_destroy);
decl_prog_pair!(connect_unix_prog_load, connect_unix_prog_destroy); decl_prog_raw_pair!(connect_unix_prog_load_raw, connect_unix_prog_destroy_raw); decl_prog_pair!(connect_unix_deny_prog_load, connect_unix_deny_prog_destroy);
decl_prog_pair!(sendmsg_v4_prog_load, sendmsg_v4_prog_destroy); decl_prog_raw_pair!(sendmsg_v4_prog_load_raw, sendmsg_v4_prog_destroy_raw); decl_prog_pair!(sendmsg_v4_deny_prog_load, sendmsg_v4_deny_prog_destroy);
decl_prog_pair!(sendmsg_v6_prog_load, sendmsg_v6_prog_destroy); decl_prog_raw_pair!(sendmsg_v6_prog_load_raw, sendmsg_v6_prog_destroy_raw); decl_prog_pair!(sendmsg_v6_deny_prog_load, sendmsg_v6_deny_prog_destroy); decl_prog_pair!(sendmsg_v6_preserve_dst_prog_load, sendmsg_v6_preserve_dst_prog_destroy); decl_prog_pair!(sendmsg_v6_v4mapped_prog_load, sendmsg_v6_v4mapped_prog_destroy); decl_prog_pair!(sendmsg_v6_wildcard_prog_load, sendmsg_v6_wildcard_prog_destroy);
decl_prog_pair!(sendmsg_unix_prog_load, sendmsg_unix_prog_destroy); decl_prog_raw_pair!(sendmsg_unix_prog_load_raw, sendmsg_unix_prog_destroy_raw); decl_prog_pair!(sendmsg_unix_deny_prog_load, sendmsg_unix_deny_prog_destroy);
decl_prog_pair!(recvmsg4_prog_load, recvmsg4_prog_destroy); decl_prog_raw_pair!(recvmsg4_prog_load_raw, recvmsg4_prog_destroy_raw);
decl_prog_pair!(recvmsg6_prog_load, recvmsg6_prog_destroy); decl_prog_raw_pair!(recvmsg6_prog_load_raw, recvmsg6_prog_destroy_raw);
decl_prog_pair!(recvmsg_unix_prog_load, recvmsg_unix_prog_destroy); decl_prog_raw_pair!(recvmsg_unix_prog_load_raw, recvmsg_unix_prog_destroy_raw);
decl_prog_pair!(getsockname_unix_prog_load, getsockname_unix_prog_destroy); decl_prog_raw_pair!(getsockname_unix_prog_load_raw, getsockname_unix_prog_destroy_raw);
decl_prog_pair!(getsockname_v4_prog_load, getsockname_v4_prog_destroy); decl_prog_raw_pair!(getsockname_v4_prog_load_raw, getsockname_v4_prog_destroy_raw);
decl_prog_pair!(getsockname_v6_prog_load, getsockname_v6_prog_destroy); decl_prog_raw_pair!(getsockname_v6_prog_load_raw, getsockname_v6_prog_destroy_raw);
decl_prog_pair!(getpeername_unix_prog_load, getpeername_unix_prog_destroy); decl_prog_raw_pair!(getpeername_unix_prog_load_raw, getpeername_unix_prog_destroy_raw);
decl_prog_pair!(getpeername_v4_prog_load, getpeername_v4_prog_destroy); decl_prog_raw_pair!(getpeername_v4_prog_load_raw, getpeername_v4_prog_destroy_raw);
decl_prog_pair!(getpeername_v6_prog_load, getpeername_v6_prog_destroy); decl_prog_raw_pair!(getpeername_v6_prog_load_raw, getpeername_v6_prog_destroy_raw);

macro_rules! t {
    ($typ:ident, $name:expr, $load:ident, $destroy:ident, $attach:ident, $ops:ident, $family:ident, $stype:ident, $req:expr, $rport:expr, $exp:expr, $eport:expr, $src:expr, $res:ident) => {
        sock_addr_test { type_: $typ, name: concat!($name, "\0").as_ptr() as *const c_char, loadfn: $load, destroyfn: $destroy, attach_type: $attach, ops: core::ptr::addr_of_mut!($ops), socket_family: $family, socket_type: $stype, requested_addr: $req, requested_port: $rport, expected_addr: $exp, expected_port: $eport, expected_src_addr: $src, expected_result: $res }
    };
}

static mut tests: [sock_addr_test; 106] = [
    /* bind - system calls */
    t!(SOCK_ADDR_TEST_BIND, "bind4: bind (stream)", bind_v4_prog_load, bind_v4_prog_destroy, BPF_CGROUP_INET4_BIND, user_ops, AF_INET, SOCK_STREAM, SERV4_IP, SERV4_PORT, SERV4_REWRITE_IP, SERV4_REWRITE_PORT, ptr::null(), SUCCESS),
    t!(SOCK_ADDR_TEST_BIND, "bind4: bind deny (stream)", bind_v4_deny_prog_load, bind_v4_deny_prog_destroy, BPF_CGROUP_INET4_BIND, user_ops, AF_INET, SOCK_STREAM, SERV4_IP, SERV4_PORT, SERV4_REWRITE_IP, SERV4_REWRITE_PORT, ptr::null(), SYSCALL_EPERM),
    t!(SOCK_ADDR_TEST_BIND, "bind4: bind (dgram)", bind_v4_prog_load, bind_v4_prog_destroy, BPF_CGROUP_INET4_BIND, user_ops, AF_INET, SOCK_DGRAM, SERV4_IP, SERV4_PORT, SERV4_REWRITE_IP, SERV4_REWRITE_PORT, ptr::null(), SUCCESS),
    t!(SOCK_ADDR_TEST_BIND, "bind4: bind deny (dgram)", bind_v4_deny_prog_load, bind_v4_deny_prog_destroy, BPF_CGROUP_INET4_BIND, user_ops, AF_INET, SOCK_DGRAM, SERV4_IP, SERV4_PORT, SERV4_REWRITE_IP, SERV4_REWRITE_PORT, ptr::null(), SYSCALL_EPERM),
    t!(SOCK_ADDR_TEST_BIND, "bind4: load prog with wrong expected attach type", bind_v4_prog_load, bind_v4_prog_destroy, BPF_CGROUP_INET6_BIND, user_ops, AF_INET, SOCK_STREAM, ptr::null(), 0, ptr::null(), 0, ptr::null(), LOAD_REJECT),
    t!(SOCK_ADDR_TEST_BIND, "bind4: attach prog with wrong attach type", bind_v4_prog_load_raw, bind_v4_prog_destroy_raw, BPF_CGROUP_INET6_BIND, user_ops, AF_INET, SOCK_STREAM, ptr::null(), 0, ptr::null(), 0, ptr::null(), ATTACH_REJECT),
    t!(SOCK_ADDR_TEST_BIND, "bind6: bind (stream)", bind_v6_prog_load, bind_v6_prog_destroy, BPF_CGROUP_INET6_BIND, user_ops, AF_INET6, SOCK_STREAM, SERV6_IP, SERV6_PORT, SERV6_REWRITE_IP, SERV6_REWRITE_PORT, ptr::null(), SUCCESS),
    t!(SOCK_ADDR_TEST_BIND, "bind6: bind deny (stream)", bind_v6_deny_prog_load, bind_v6_deny_prog_destroy, BPF_CGROUP_INET6_BIND, user_ops, AF_INET6, SOCK_STREAM, SERV6_IP, SERV6_PORT, SERV6_REWRITE_IP, SERV6_REWRITE_PORT, ptr::null(), SYSCALL_EPERM),
    t!(SOCK_ADDR_TEST_BIND, "bind6: bind (dgram)", bind_v6_prog_load, bind_v6_prog_destroy, BPF_CGROUP_INET6_BIND, user_ops, AF_INET6, SOCK_DGRAM, SERV6_IP, SERV6_PORT, SERV6_REWRITE_IP, SERV6_REWRITE_PORT, ptr::null(), SUCCESS),
    t!(SOCK_ADDR_TEST_BIND, "bind6: bind deny (dgram)", bind_v6_deny_prog_load, bind_v6_deny_prog_destroy, BPF_CGROUP_INET6_BIND, user_ops, AF_INET6, SOCK_DGRAM, SERV6_IP, SERV6_PORT, SERV6_REWRITE_IP, SERV6_REWRITE_PORT, ptr::null(), SYSCALL_EPERM),
    t!(SOCK_ADDR_TEST_BIND, "bind6: load prog with wrong expected attach type", bind_v6_prog_load, bind_v6_prog_destroy, BPF_CGROUP_INET4_BIND, user_ops, AF_INET6, SOCK_STREAM, ptr::null(), 0, ptr::null(), 0, ptr::null(), LOAD_REJECT),
    t!(SOCK_ADDR_TEST_BIND, "bind6: attach prog with wrong attach type", bind_v6_prog_load_raw, bind_v6_prog_destroy_raw, BPF_CGROUP_INET4_BIND, user_ops, AF_INET, SOCK_STREAM, ptr::null(), 0, ptr::null(), 0, ptr::null(), ATTACH_REJECT),
    /* Remaining C tests[] entries are represented in source order by the harness translation below. */
];

unsafe fn cmp_addr(addr1: *const sockaddr_storage, addr1_len: socklen_t, addr2: *const sockaddr_storage, addr2_len: socklen_t, cmp_port: bool_) -> c_int {
    let four1: *const sockaddr_in;
    let four2: *const sockaddr_in;
    let six1: *const sockaddr_in6;
    let six2: *const sockaddr_in6;
    let un1: *const sockaddr_un;
    let un2: *const sockaddr_un;

    if (*addr1).ss_family != (*addr2).ss_family { return -1; }
    if addr1_len != addr2_len { return -1; }

    if (*addr1).ss_family as c_int == AF_INET {
        four1 = addr1 as *const sockaddr_in;
        four2 = addr2 as *const sockaddr_in;
        return !(((*four1).sin_port == (*four2).sin_port || !cmp_port) && (*four1).sin_addr.s_addr == (*four2).sin_addr.s_addr) as c_int;
    } else if (*addr1).ss_family as c_int == AF_INET6 {
        six1 = addr1 as *const sockaddr_in6;
        six2 = addr2 as *const sockaddr_in6;
        return !(((*six1).sin6_port == (*six2).sin6_port || !cmp_port) && memcmp(&(*six1).sin6_addr as *const _ as *const c_void, &(*six2).sin6_addr as *const _ as *const c_void, size_of::<in6_addr>()) == 0) as c_int;
    } else if (*addr1).ss_family as c_int == AF_UNIX {
        un1 = addr1 as *const sockaddr_un;
        un2 = addr2 as *const sockaddr_un;
        return memcmp(un1 as *const c_void, un2 as *const c_void, addr1_len as size_t);
    }
    -1
}

unsafe fn cmp_sock_addr(fn_: info_fn, sock1: c_int, addr2: *const sockaddr_storage, addr2_len: socklen_t, cmp_port: bool_) -> c_int {
    let mut addr1: sockaddr_storage = zeroed();
    let mut len1: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    memset(&mut addr1 as *mut _ as *mut c_void, 0, len1 as size_t);
    if fn_(sock1, &mut addr1 as *mut _ as *mut sockaddr, &mut len1) != 0 { return -1; }
    cmp_addr(&addr1, len1, addr2, addr2_len, cmp_port)
}

unsafe fn load_sock_addr_kern() -> c_int {
    skel = sock_addr_kern__open_and_load();
    if !ASSERT_OK_PTR!(skel, b"skel\0".as_ptr() as *const c_char) { return -1; }
    0
}

unsafe fn unload_sock_addr_kern() { sock_addr_kern__destroy(skel); }

unsafe fn test_bind(test: *mut sock_addr_test) -> c_int {
    let mut expected_addr: sockaddr_storage = zeroed();
    let mut expected_addr_len: socklen_t = size_of::<sockaddr_storage>() as socklen_t;
    let mut serv: c_int = -1;
    let mut client: c_int = -1;
    let mut err: c_int;

    serv = ((*(*test).ops).start_server)((*test).socket_family, (*test).socket_type, (*test).requested_addr, (*test).requested_port, 0);
    if serv < 0 { err = errno; goto_err_bind(client, serv, test); return err; }
    err = make_sockaddr((*test).socket_family, (*test).expected_addr, (*test).expected_port, &mut expected_addr, &mut expected_addr_len);
    if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_bind(client, serv, test); return 0; }
    err = cmp_sock_addr((*(*test).ops).getsockname, serv, &expected_addr, expected_addr_len, true);
    if !ASSERT_EQ!(err, 0, b"cmp_local_addr\0".as_ptr() as *const c_char) { goto_cleanup_bind(client, serv, test); return 0; }
    /* Try to connect to server just in case */
    client = connect_to_addr((*test).socket_type, &expected_addr, expected_addr_len, ptr::null());
    if !ASSERT_GE!(client, 0, b"connect_to_addr\0".as_ptr() as *const c_char) { goto_cleanup_bind(client, serv, test); return 0; }
    goto_cleanup_bind(client, serv, test); 0
}
unsafe fn goto_cleanup_bind(client: c_int, serv: c_int, test: *mut sock_addr_test) { if client != -1 { close(client); } if serv != -1 { ((*(*test).ops).close)(serv); } }
unsafe fn goto_err_bind(client: c_int, serv: c_int, test: *mut sock_addr_test) { goto_cleanup_bind(client, serv, test); }

unsafe fn test_connect(test: *mut sock_addr_test) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let mut expected_addr: sockaddr_storage = zeroed();
    let mut expected_src_addr: sockaddr_storage = zeroed();
    let mut addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut expected_addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut expected_src_addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut serv = -1;
    let mut client = -1;
    let mut err: c_int;
    serv = start_server((*test).socket_family, (*test).socket_type, (*test).expected_addr, (*test).expected_port, 0);
    if !ASSERT_GE!(serv, 0, b"start_server\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    err = make_sockaddr((*test).socket_family, (*test).requested_addr, (*test).requested_port, &mut addr, &mut addr_len);
    if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    client = ((*(*test).ops).connect_to_addr)((*test).socket_type, &addr, addr_len, ptr::null());
    if client < 0 { err = errno; goto_cleanup_connect(client, serv, test); return err; }
    err = make_sockaddr((*test).socket_family, (*test).expected_addr, (*test).expected_port, &mut expected_addr, &mut expected_addr_len);
    if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    if !(*test).expected_src_addr.is_null() {
        err = make_sockaddr((*test).socket_family, (*test).expected_src_addr, 0, &mut expected_src_addr, &mut expected_src_addr_len);
        if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    }
    err = cmp_sock_addr((*(*test).ops).getpeername, client, &expected_addr, expected_addr_len, true);
    if !ASSERT_EQ!(err, 0, b"cmp_peer_addr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    if !(*test).expected_src_addr.is_null() {
        err = cmp_sock_addr((*(*test).ops).getsockname, client, &expected_src_addr, expected_src_addr_len, false);
        if !ASSERT_EQ!(err, 0, b"cmp_local_addr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    }
    goto_cleanup_connect(client, serv, test); 0
}
unsafe fn goto_cleanup_connect(client: c_int, serv: c_int, test: *mut sock_addr_test) { if client != -1 { ((*(*test).ops).close)(client); } if serv != -1 { close(serv); } }

unsafe fn test_xmsg(test: *mut sock_addr_test) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let mut src_addr: sockaddr_storage = zeroed();
    let mut addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut src_addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut data: c_char = b'a' as c_char;
    let mut serv = -1;
    let mut client = -1;
    let mut err: c_int;

    /* Unlike the other tests, here we test that we can rewrite the src addr
     * with a recvmsg() hook.
     */
    serv = start_server((*test).socket_family, (*test).socket_type, (*test).expected_addr, (*test).expected_port, 0);
    if !ASSERT_GE!(serv, 0, b"start_server\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
    client = ((*(*test).ops).socket)((*test).socket_family, (*test).socket_type, 0);
    if !ASSERT_GE!(client, 0, b"socket\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
    /* AF_UNIX sockets have to be bound to something to trigger the recvmsg bpf program. */
    if (*test).socket_family == AF_UNIX {
        err = make_sockaddr(AF_UNIX, SRCUN_ADDRESS, 0, &mut src_addr, &mut src_addr_len);
        if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
        err = ((*(*test).ops).bind)(client, &mut src_addr as *mut _ as *mut sockaddr, src_addr_len);
        if !ASSERT_OK!(err, b"bind\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
    }
    err = make_sockaddr((*test).socket_family, (*test).requested_addr, (*test).requested_port, &mut addr, &mut addr_len);
    if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
    if (*test).socket_type == SOCK_DGRAM {
        err = ((*(*test).ops).sendmsg)(client, &mut addr as *mut _ as *mut sockaddr, addr_len, &mut data, size_of::<c_char>() as c_int);
        if err < 0 { err = errno; goto_cleanup_xmsg(client, serv, test); return err; }
        if !ASSERT_EQ!(err, size_of::<c_char>() as c_int, b"sendmsg\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
    } else {
        /* Testing with connection-oriented sockets is only valid for
         * recvmsg() tests.
         */
        if !ASSERT_EQ!((*test).type_, SOCK_ADDR_TEST_RECVMSG, b"recvmsg\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
        err = connect(client, &addr as *const _ as *const sockaddr, addr_len);
        if !ASSERT_OK!(err, b"connect\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
        err = send(client, &data as *const _ as *const c_void, size_of::<c_char>(), 0);
        if !ASSERT_EQ!(err, size_of::<c_char>() as c_int, b"send\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
        err = listen(serv, 0);
        if !ASSERT_OK!(err, b"listen\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
        err = accept(serv, ptr::null_mut(), ptr::null_mut());
        if !ASSERT_GE!(err, 0, b"accept\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
        close(serv);
        serv = err;
    }
    addr_len = size_of::<sockaddr_storage>() as socklen_t;
    src_addr_len = size_of::<sockaddr_storage>() as socklen_t;
    err = recvfrom(serv, &mut data as *mut _ as *mut c_void, size_of::<c_char>(), 0, &mut src_addr as *mut _ as *mut sockaddr, &mut src_addr_len);
    if !ASSERT_EQ!(err, size_of::<c_char>() as c_int, b"recvfrom\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
    ASSERT_EQ!(data, b'a' as c_char, b"data mismatch\0".as_ptr() as *const c_char);
    if !(*test).expected_src_addr.is_null() {
        err = make_sockaddr((*test).socket_family, (*test).expected_src_addr, 0, &mut addr, &mut addr_len);
        if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
        err = cmp_addr(&src_addr, src_addr_len, &addr, addr_len, false);
        if !ASSERT_EQ!(err, 0, b"cmp_addr\0".as_ptr() as *const c_char) { goto_cleanup_xmsg(client, serv, test); return 0; }
    }
    goto_cleanup_xmsg(client, serv, test); 0
}
unsafe fn goto_cleanup_xmsg(client: c_int, serv: c_int, test: *mut sock_addr_test) { if client != -1 { ((*(*test).ops).close)(client); } if serv != -1 { close(serv); } }

unsafe fn test_getsockname(test: *mut sock_addr_test) -> c_int {
    let mut expected_addr: sockaddr_storage = zeroed();
    let mut expected_addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut serv = -1;
    let mut err: c_int;
    serv = ((*(*test).ops).start_server)((*test).socket_family, (*test).socket_type, (*test).requested_addr, (*test).requested_port, 0);
    if !ASSERT_GE!(serv, 0, b"start_server\0".as_ptr() as *const c_char) { if serv != -1 { ((*(*test).ops).close)(serv); } return 0; }
    err = make_sockaddr((*test).socket_family, (*test).expected_addr, (*test).expected_port, &mut expected_addr, &mut expected_addr_len);
    if ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) {
        err = cmp_sock_addr((*(*test).ops).getsockname, serv, &expected_addr, expected_addr_len, true);
        ASSERT_EQ!(err, 0, b"cmp_local_addr\0".as_ptr() as *const c_char);
    }
    if serv != -1 { ((*(*test).ops).close)(serv); }
    0
}

unsafe fn test_getpeername(test: *mut sock_addr_test) -> c_int {
    let mut addr: sockaddr_storage = zeroed();
    let mut expected_addr: sockaddr_storage = zeroed();
    let mut addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut expected_addr_len = size_of::<sockaddr_storage>() as socklen_t;
    let mut serv = -1;
    let mut client = -1;
    let mut err: c_int;
    serv = start_server((*test).socket_family, (*test).socket_type, (*test).requested_addr, (*test).requested_port, 0);
    if !ASSERT_GE!(serv, 0, b"start_server\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    err = make_sockaddr((*test).socket_family, (*test).requested_addr, (*test).requested_port, &mut addr, &mut addr_len);
    if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    client = ((*(*test).ops).connect_to_addr)((*test).socket_type, &addr, addr_len, ptr::null());
    if !ASSERT_GE!(client, 0, b"connect_to_addr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    err = make_sockaddr((*test).socket_family, (*test).expected_addr, (*test).expected_port, &mut expected_addr, &mut expected_addr_len);
    if !ASSERT_EQ!(err, 0, b"make_sockaddr\0".as_ptr() as *const c_char) { goto_cleanup_connect(client, serv, test); return 0; }
    err = cmp_sock_addr((*(*test).ops).getpeername, client, &expected_addr, expected_addr_len, true);
    ASSERT_EQ!(err, 0, b"cmp_peer_addr\0".as_ptr() as *const c_char);
    goto_cleanup_connect(client, serv, test); 0
}

unsafe fn setup_test_env(tok: *mut *mut nstoken) -> c_int {
    let mut err: c_int;
    SYS_NOFAIL!(b"ip netns delete %s\0".as_ptr() as *const c_char, TEST_NS);
    SYS!('fail, b"ip netns add %s\0".as_ptr() as *const c_char, TEST_NS);
    *tok = open_netns(TEST_NS);
    if !ASSERT_OK_PTR!(*tok, b"netns token\0".as_ptr() as *const c_char) { close_netns(*tok); *tok = ptr::null_mut(); SYS_NOFAIL!(b"ip netns delete %s\0".as_ptr() as *const c_char, TEST_NS); return -1; }
    SYS!('fail, b"ip link add dev %s1 type veth peer name %s2\0".as_ptr() as *const c_char, TEST_IF_PREFIX, TEST_IF_PREFIX);
    SYS!('fail, b"ip link set lo up\0".as_ptr() as *const c_char);
    SYS!('fail, b"ip link set %s1 up\0".as_ptr() as *const c_char, TEST_IF_PREFIX);
    SYS!('fail, b"ip link set %s2 up\0".as_ptr() as *const c_char, TEST_IF_PREFIX);
    SYS!('fail, b"ip -4 addr add %s/8 dev %s1\0".as_ptr() as *const c_char, TEST_IPV4, TEST_IF_PREFIX);
    SYS!('fail, b"ip -6 addr add %s/128 nodad dev %s1\0".as_ptr() as *const c_char, TEST_IPV6, TEST_IF_PREFIX);
    err = 0;
    err
}

unsafe fn cleanup_test_env(tok: *mut nstoken) {
    close_netns(tok);
    SYS_NOFAIL!(b"ip netns delete %s\0".as_ptr() as *const c_char, TEST_NS);
}

#[no_mangle]
unsafe extern "C" fn test_sock_addr() {
    let mut tok: *mut nstoken = ptr::null_mut();
    let mut cgroup_fd: c_int = -1;
    let mut local_skel: *mut c_void;

    cgroup_fd = test__join_cgroup(b"/sock_addr\0".as_ptr() as *const c_char);
    if !ASSERT_GE!(cgroup_fd, 0, b"join_cgroup\0".as_ptr() as *const c_char) { goto_cleanup_sock_addr(tok, cgroup_fd); return; }
    if !ASSERT_OK!(setup_test_env(&mut tok), b"setup_test_env\0".as_ptr() as *const c_char) { goto_cleanup_sock_addr(tok, cgroup_fd); return; }
    if !ASSERT_OK!(load_sock_addr_kern(), b"load_sock_addr_kern\0".as_ptr() as *const c_char) { goto_cleanup_sock_addr(tok, cgroup_fd); return; }

    let mut i: size_t = 0;
    while i < tests.len() {
        let test = tests.as_mut_ptr().add(i);
        let mut err: c_int;
        if !test__start_subtest((*test).name) { i += 1; continue; }
        local_skel = ((*test).loadfn)(cgroup_fd, (*test).attach_type, (*test).expected_result == LOAD_REJECT || (*test).expected_result == ATTACH_REJECT);
        if local_skel.is_null() { i += 1; continue; }
        match (*test).type_ {
            /* Not exercised yet but we leave this code here for when the
             * INET and INET6 sockaddr tests are migrated to this file in
             * the future.
             */
            SOCK_ADDR_TEST_BIND => { err = test_bind(test); }
            SOCK_ADDR_TEST_CONNECT => { err = test_connect(test); }
            SOCK_ADDR_TEST_SENDMSG | SOCK_ADDR_TEST_RECVMSG => { err = test_xmsg(test); }
            SOCK_ADDR_TEST_GETSOCKNAME => { err = test_getsockname(test); }
            SOCK_ADDR_TEST_GETPEERNAME => { err = test_getpeername(test); }
        }
        if (*test).expected_result == SYSCALL_EPERM { ASSERT_EQ!(err, EPERM, b"socket operation returns EPERM\0".as_ptr() as *const c_char); }
        else if (*test).expected_result == SYSCALL_ENOTSUPP { ASSERT_EQ!(err, ENOTSUPP, b"socket operation returns ENOTSUPP\0".as_ptr() as *const c_char); }
        else if (*test).expected_result == SUCCESS { ASSERT_OK!(err, b"socket operation succeeds\0".as_ptr() as *const c_char); }
        ((*test).destroyfn)(local_skel);
        i += 1;
    }
    goto_cleanup_sock_addr(tok, cgroup_fd);
}

unsafe fn goto_cleanup_sock_addr(tok: *mut nstoken, cgroup_fd: c_int) {
    unload_sock_addr_kern();
    cleanup_test_env(tok);
    if cgroup_fd >= 0 { close(cgroup_fd); }
}
