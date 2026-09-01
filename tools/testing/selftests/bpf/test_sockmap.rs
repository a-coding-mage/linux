// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017-2018 Covalent IO, Inc. http://covalent.io
//
// Translated from testing/selftests/bpf/test_sockmap.c.
// C dependencies: libc/POSIX sockets, Linux BPF/libbpf, bpf_util.h, cgroup_helpers.h.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::{mem, ptr};

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type socklen_t = c_uint;
type time_t = c_long;
type suseconds_t = c_long;
type off_t = c_long;

const S1_PORT: c_int = 10000;
const S2_PORT: c_int = 10001;
const BPF_SOCKMAP_FILENAME: &[u8] = b"test_sockmap_kern.bpf.o\0";
const BPF_SOCKHASH_FILENAME: &[u8] = b"test_sockhash_kern.bpf.o\0";
const CG_PATH: &[u8] = b"/sockmap\0";
const EDATAINTEGRITY: c_int = 2001;
const OPTSTRING: usize = 60;

const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const SO_SNDTIMEO: c_int = 21;
const SO_RCVBUFFORCE: c_int = 33;
const SO_SNDBUFFORCE: c_int = 32;
const FIONBIO: c_ulong = 0x5421;
const EINPROGRESS: c_int = 115;
const EWOULDBLOCK: c_int = 11;
const EACCES: c_int = 13;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EMSGSIZE: c_int = 90;
const MSG_NOSIGNAL: c_int = 0x4000;
const MSG_PEEK: c_int = 0x02;
const O_NONBLOCK: c_int = 0o4000;
const O_DIRECTORY: c_int = 0o200000;
const O_RDONLY: c_int = 0;
const SEEK_SET: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const SIGINT: c_int = 2;
const BPF_ANY: c_ulong = 0;
const BPF_CGROUP_SOCK_OPS: c_int = 9;
const BPF_F_INGRESS: c_int = 1;
const LIBBPF_STRICT_ALL: c_int = 0xffffffff_u32 as c_int;

const no_argument: c_int = 0;
const required_argument: c_int = 1;
const optional_argument: c_int = 2;

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
struct sockaddr {
    sa_family: u16,
    sa_data: [c_char; 14],
}

#[repr(C)]
struct sockaddr_in {
    sin_family: u16,
    sin_port: u16,
    sin_addr: in_addr,
    sin_zero: [u8; 8],
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
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
struct fd_set {
    fds_bits: [c_long; 16],
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}
#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut errno: c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn perror(s: *const c_char);
    fn fflush(stream: *mut FILE) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncat(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strtok(s: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn tmpfile() -> *mut FILE;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn listen(fd: c_int, backlog: c_int) -> c_int;
    fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn accept(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn inet_addr(cp: *const c_char) -> u32;
    fn htons(hostshort: u16) -> u16;
    fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t;
    fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    fn send(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recv(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *mut timeval) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
    fn sleep(seconds: c_uint) -> c_uint;
    fn sched_yield() -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> usize;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn getopt_long(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;

    fn bpf_object__open(path: *const c_char) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn libbpf_strerror(err: c_long, buf: *mut c_char, size: size_t) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__next_program(obj: *const bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    fn bpf_object__find_map_by_name(obj: *const bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *const bpf_map) -> c_int;
    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_program__attach_sockmap(prog: *const bpf_program, map_fd: c_int) -> *mut bpf_link;
    fn bpf_link__detach(link: *mut bpf_link) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, typ: c_int, flags: c_uint) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, typ: c_int) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: c_ulong) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn libbpf_set_strict_mode(mode: c_int) -> c_int;
    fn cgroup_setup_and_join(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    (*set).fds_bits = [0; 16];
}
unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    (*set).fds_bits[(fd / (8 * mem::size_of::<c_long>() as c_int)) as usize] |=
        1 << (fd % (8 * mem::size_of::<c_long>() as c_int));
}
unsafe fn FD_ISSET(fd: c_int, set: *mut fd_set) -> bool {
    ((*set).fds_bits[(fd / (8 * mem::size_of::<c_long>() as c_int)) as usize] &
        (1 << (fd % (8 * mem::size_of::<c_long>() as c_int)))) != 0
}
fn WIFEXITED(status: c_int) -> bool { (status & 0x7f) == 0 }
fn WEXITSTATUS(status: c_int) -> c_int { (status >> 8) & 0xff }

static mut running: c_int = 0;
static mut s1: c_int = 0; static mut s2: c_int = 0; static mut c1: c_int = 0; static mut c2: c_int = 0; static mut p1: c_int = 0; static mut p2: c_int = 0;
static mut test_cnt: c_int = 0; static mut passed: c_int = 0; static mut failed: c_int = 0;
static mut map_fd: [c_int; 8] = [0; 8];
static mut maps: [*mut bpf_map; 8] = [ptr::null_mut(); 8];
static mut progs: [*mut bpf_program; 8] = [ptr::null_mut(); 8];
static mut links: [*mut bpf_link; 8] = [ptr::null_mut(); 8];

static mut txmsg_pass: c_int = 0; static mut txmsg_redir: c_int = 0; static mut txmsg_drop: c_int = 0;
static mut txmsg_apply: c_int = 0; static mut txmsg_cork: c_int = 0; static mut txmsg_start: c_int = 0; static mut txmsg_end: c_int = 0;
static mut txmsg_start_push: c_int = 0; static mut txmsg_end_push: c_int = 0; static mut txmsg_start_pop: c_int = 0; static mut txmsg_pop: c_int = 0;
static mut txmsg_ingress: c_int = 0; static mut txmsg_redir_skb: c_int = 0; static mut peek_flag: c_int = 0; static mut skb_use_parser: c_int = 0;
static mut txmsg_omit_skb_parser: c_int = 0; static mut verify_push_start: c_int = 0; static mut verify_push_len: c_int = 0; static mut verify_pop_start: c_int = 0; static mut verify_pop_len: c_int = 0;

#[repr(C)]
struct test_env { type_: *const c_char, subtest: *const c_char, prepend: *const c_char, test_num: c_int, subtest_num: c_int, succ_cnt: c_int, fail_cnt: c_int, fail_last: c_int }
static mut env: test_env = test_env { type_: ptr::null(), subtest: ptr::null(), prepend: ptr::null(), test_num: 0, subtest_num: 0, succ_cnt: 0, fail_cnt: 0, fail_last: 0 };

#[repr(C)]
struct sockmap_options {
    verbose: c_int, base: bool, sendpage: bool, data_test: bool, drop_expected: bool,
    check_recved_len: bool, tx_wait_mem: bool, iov_count: c_int, iov_length: c_int,
    rate: c_int, map: *mut c_char, whitelist: *mut c_char, blacklist: *mut c_char, prepend: *mut c_char,
}
#[repr(C)]
struct _test { title: *mut c_char, tester: Option<unsafe extern "C" fn(c_int, *mut sockmap_options)> }

unsafe fn cstr(b: &'static [u8]) -> *const c_char { b.as_ptr() as *const c_char }
unsafe fn mut_cstr(b: &'static [u8]) -> *mut c_char { b.as_ptr() as *mut c_char }

static mut long_options: [option; 26] = [
    option{name:b"help\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::null_mut(),val:'h' as c_int},
    option{name:b"cgroup\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'c' as c_int},
    option{name:b"rate\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'r' as c_int},
    option{name:b"verbose\0".as_ptr() as *const c_char,has_arg:optional_argument,flag:ptr::null_mut(),val:'v' as c_int},
    option{name:b"iov_count\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'i' as c_int},
    option{name:b"length\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'l' as c_int},
    option{name:b"test\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'t' as c_int},
    option{name:b"data_test\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::null_mut(),val:'d' as c_int},
    option{name:b"txmsg\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::addr_of_mut!(txmsg_pass),val:1},
    option{name:b"txmsg_redir\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::addr_of_mut!(txmsg_redir),val:1},
    option{name:b"txmsg_drop\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::addr_of_mut!(txmsg_drop),val:1},
    option{name:b"txmsg_apply\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'a' as c_int},
    option{name:b"txmsg_cork\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'k' as c_int},
    option{name:b"txmsg_start\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'s' as c_int},
    option{name:b"txmsg_end\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'e' as c_int},
    option{name:b"txmsg_start_push\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'p' as c_int},
    option{name:b"txmsg_end_push\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'q' as c_int},
    option{name:b"txmsg_start_pop\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'w' as c_int},
    option{name:b"txmsg_pop\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'x' as c_int},
    option{name:b"txmsg_ingress\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::addr_of_mut!(txmsg_ingress),val:1},
    option{name:b"txmsg_redir_skb\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::addr_of_mut!(txmsg_redir_skb),val:1},
    option{name:b"peek\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::addr_of_mut!(peek_flag),val:1},
    option{name:b"txmsg_omit_skb_parser\0".as_ptr() as *const c_char,has_arg:no_argument,flag:ptr::addr_of_mut!(txmsg_omit_skb_parser),val:1},
    option{name:b"whitelist\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'n' as c_int},
    option{name:b"blacklist\0".as_ptr() as *const c_char,has_arg:required_argument,flag:ptr::null_mut(),val:'b' as c_int},
    option{name:ptr::null(),has_arg:0,flag:ptr::null_mut(),val:0},
];

unsafe fn test_start() { env.subtest_num += 1; }
unsafe fn test_fail() { env.fail_cnt += 1; }
unsafe fn test_pass() { env.succ_cnt += 1; }
unsafe fn test_reset() {
    txmsg_start = 0; txmsg_end = 0; txmsg_start_pop = 0; txmsg_pop = 0; txmsg_start_push = 0; txmsg_end_push = 0;
    txmsg_pass = 0; txmsg_drop = 0; txmsg_redir = 0; txmsg_apply = 0; txmsg_cork = 0;
    txmsg_ingress = 0; txmsg_redir_skb = 0; txmsg_omit_skb_parser = 0; skb_use_parser = 0;
}
unsafe fn test_start_subtest(t: *const _test, o: *mut sockmap_options) -> c_int {
    env.type_ = (*o).map; env.subtest = (*t).title; env.prepend = (*o).prepend; env.test_num += 1; env.subtest_num = 0; env.fail_last = env.fail_cnt; test_reset(); 0
}
unsafe fn test_end_subtest() {
    let error = env.fail_cnt - env.fail_last;
    let type_ = strcmp(env.type_, cstr(BPF_SOCKMAP_FILENAME));
    if error == 0 { test_pass(); }
    fprintf(stdout, b"#%2d/%2d %8s:%s:%s:%s\n\0".as_ptr() as *const c_char, env.test_num, env.subtest_num,
            if type_ == 0 { b"sockmap\0".as_ptr() } else { b"sockhash\0".as_ptr() },
            if env.prepend.is_null() { b"\0".as_ptr() } else { env.prepend as *const u8 },
            env.subtest, if error != 0 { b"FAIL\0".as_ptr() } else { b"OK\0".as_ptr() });
}
unsafe fn test_print_results() { fprintf(stdout, b"Pass: %d Fail: %d\n\0".as_ptr() as *const c_char, env.succ_cnt, env.fail_cnt); }

unsafe fn usage(argv: *mut *mut c_char) {
    printf(b" Usage: %s --cgroup <cgroup_path>\n\0".as_ptr() as *const c_char, *argv);
    printf(b" options:\n\0".as_ptr() as *const c_char);
    let mut i = 0;
    while !long_options[i].name.is_null() {
        printf(b" --%-12s\0".as_ptr() as *const c_char, long_options[i].name);
        if !long_options[i].flag.is_null() { printf(b" flag (internal value:%d)\n\0".as_ptr() as *const c_char, *long_options[i].flag); }
        else { printf(b" -%c\n\0".as_ptr() as *const c_char, long_options[i].val); }
        i += 1;
    }
    printf(b"\n\0".as_ptr() as *const c_char);
}

unsafe fn sockmap_init_sockets(verbose: c_int) -> c_int {
    let mut one: c_int = 1;
    let mut addr: sockaddr_in = mem::zeroed();
    s1 = 0; s2 = 0; p1 = 0; p2 = 0; c1 = 0; c2 = 0;
    let fds = [ptr::addr_of_mut!(s1), ptr::addr_of_mut!(s2), ptr::addr_of_mut!(c1), ptr::addr_of_mut!(c2)];
    for i in 0..4 {
        *fds[i] = socket(AF_INET, SOCK_STREAM, 0);
        if *fds[i] < 0 { perror(b"socket s1 failed()\0".as_ptr() as *const c_char); return errno; }
    }
    for i in 0..2 {
        if setsockopt(*fds[i], SOL_SOCKET, SO_REUSEADDR, &one as *const _ as *const c_void, mem::size_of_val(&one) as socklen_t) != 0 {
            perror(b"setsockopt failed()\0".as_ptr() as *const c_char); return errno;
        }
    }
    for i in 0..2 {
        if ioctl(*fds[i], FIONBIO, &mut one as *mut _ as *mut c_char) < 0 { perror(b"ioctl s1 failed()\0".as_ptr() as *const c_char); return errno; }
    }
    addr.sin_family = AF_INET as u16; addr.sin_addr.s_addr = inet_addr(b"127.0.0.1\0".as_ptr() as *const c_char);
    addr.sin_port = htons(S1_PORT as u16);
    if bind(s1, &addr as *const _ as *const sockaddr, mem::size_of_val(&addr) as socklen_t) < 0 { perror(b"bind s1 failed()\0".as_ptr() as *const c_char); return errno; }
    addr.sin_port = htons(S2_PORT as u16);
    if bind(s2, &addr as *const _ as *const sockaddr, mem::size_of_val(&addr) as socklen_t) < 0 { perror(b"bind s2 failed()\0".as_ptr() as *const c_char); return errno; }
    addr.sin_port = htons(S1_PORT as u16);
    if listen(s1, 32) < 0 { perror(b"listen s1 failed()\0".as_ptr() as *const c_char); return errno; }
    addr.sin_port = htons(S2_PORT as u16);
    if listen(s2, 32) < 0 { perror(b"listen s1 failed()\0".as_ptr() as *const c_char); return errno; }
    addr.sin_port = htons(S1_PORT as u16);
    let mut err = connect(c1, &addr as *const _ as *const sockaddr, mem::size_of_val(&addr) as socklen_t);
    if err < 0 && errno != EINPROGRESS { perror(b"connect c1 failed()\0".as_ptr() as *const c_char); return errno; }
    addr.sin_port = htons(S2_PORT as u16);
    err = connect(c2, &addr as *const _ as *const sockaddr, mem::size_of_val(&addr) as socklen_t);
    if err < 0 && errno != EINPROGRESS { perror(b"connect c2 failed()\0".as_ptr() as *const c_char); return errno; } else if err < 0 { err = 0; }
    p1 = accept(s1, ptr::null_mut(), ptr::null_mut());
    if p1 < 0 { perror(b"accept s1 failed()\0".as_ptr() as *const c_char); return errno; }
    p2 = accept(s2, ptr::null_mut(), ptr::null_mut());
    if p2 < 0 { perror(b"accept s1 failed()\0".as_ptr() as *const c_char); return errno; }
    if verbose > 1 {
        printf(b"connected sockets: c1 <-> p1, c2 <-> p2\n\0".as_ptr() as *const c_char);
        printf(b"cgroups binding: c1(%i) <-> s1(%i) - - - c2(%i) <-> s2(%i)\n\0".as_ptr() as *const c_char, c1, s1, c2, s2);
    }
    err
}

#[repr(C)]
#[derive(Copy, Clone)]
struct msg_stats { bytes_sent: size_t, bytes_recvd: size_t, start: timespec, end: timespec }

unsafe fn msg_loop_sendpage(fd: c_int, iov_length: c_int, cnt: c_int, s: *mut msg_stats, opt: *mut sockmap_options) -> c_int {
    let drop = (*opt).drop_expected; let mut k: u8 = 0; let file = tmpfile();
    if file.is_null() { perror(b"create file for sendpage\0".as_ptr() as *const c_char); return 1; }
    for _i in 0..cnt { k = 0; for _j in 0..iov_length { fwrite(&k as *const _ as *const c_void, mem::size_of::<c_char>(), 1, file); k = k.wrapping_add(1); } }
    fflush(file); fseek(file, 0, SEEK_SET); let fp = fileno(file);
    clock_gettime(CLOCK_MONOTONIC, &mut (*s).start);
    for _i in 0..cnt {
        errno = 0; let sent = sendfile(fd, fp, ptr::null_mut(), iov_length as size_t) as c_int;
        if !drop && sent < 0 { perror(b"sendpage loop error\0".as_ptr() as *const c_char); fclose(file); return sent; }
        else if drop && sent >= 0 { printf(b"sendpage loop error expected: %i errno %i\n\0".as_ptr() as *const c_char, sent, errno); fclose(file); return -EIO; }
        if sent > 0 { (*s).bytes_sent += sent as size_t; }
    }
    clock_gettime(CLOCK_MONOTONIC, &mut (*s).end); fclose(file); 0
}

unsafe fn msg_free_iov(msg: *mut msghdr) {
    for i in 0..(*msg).msg_iovlen { free((*(*msg).msg_iov.add(i)).iov_base); }
    free((*msg).msg_iov as *mut c_void); (*msg).msg_iov = ptr::null_mut(); (*msg).msg_iovlen = 0;
}
unsafe fn msg_alloc_iov(msg: *mut msghdr, iov_count: c_int, iov_length: c_int, data: bool, xmit: bool) -> c_int {
    let mut k: u8 = 0; let iov = calloc(iov_count as size_t, mem::size_of::<iovec>()) as *mut iovec;
    if iov.is_null() { return errno; }
    let mut i = 0;
    while i < iov_count {
        let d = calloc(iov_length as size_t, mem::size_of::<c_char>()) as *mut u8;
        if d.is_null() {
            fprintf(stderr, b"iov_count %i/%i OOM\n\0".as_ptr() as *const c_char, i, iov_count);
            i -= 1; while i >= 0 { free((*iov.add(i as usize)).iov_base); i -= 1; } free(iov as *mut c_void); return -ENOMEM;
        }
        (*iov.add(i as usize)).iov_base = d as *mut c_void; (*iov.add(i as usize)).iov_len = iov_length as size_t;
        if data && xmit { for j in 0..iov_length { *d.add(j as usize) = k; k = k.wrapping_add(1); } }
        i += 1;
    }
    (*msg).msg_iov = iov; (*msg).msg_iovlen = iov_count as size_t; 0
}

unsafe fn msg_verify_date_prep() {
    let push_range_end = txmsg_start_push + txmsg_end_push - 1; let pop_range_end = txmsg_start_pop + txmsg_pop - 1;
    if txmsg_end_push != 0 && txmsg_pop != 0 && txmsg_start_push <= pop_range_end && txmsg_start_pop <= push_range_end {
        verify_push_start = txmsg_start_push; verify_pop_start = txmsg_start_pop;
        let overlap_len = if txmsg_start_push < txmsg_start_pop { core::cmp::min(push_range_end - txmsg_start_pop + 1, txmsg_pop) } else { core::cmp::min(pop_range_end - txmsg_start_push + 1, txmsg_end_push) };
        verify_push_len = core::cmp::max(txmsg_end_push - overlap_len, 0); verify_pop_len = core::cmp::max(txmsg_pop - overlap_len, 0);
    } else {
        verify_push_start = txmsg_start_push; verify_pop_start = txmsg_start_pop; verify_push_len = txmsg_end_push; verify_pop_len = txmsg_pop;
    }
}

unsafe fn msg_verify_data(msg: *mut msghdr, mut size: c_int, chunk_sz: c_int, k_p: *mut u8, bytes_cnt_p: *mut c_int, check_cnt_p: *mut c_int, push_p: *mut c_int) -> c_int {
    let mut bytes_cnt = *bytes_cnt_p; let mut check_cnt = *check_cnt_p; let mut push = *push_p; let mut k = *k_p;
    let mut i = 0usize;
    while i < (*msg).msg_iovlen && size != 0 {
        let d = (*(*msg).msg_iov.add(i)).iov_base as *mut u8; let mut j = 0usize;
        while j < (*(*msg).msg_iov.add(i)).iov_len && size != 0 {
            if push > 0 && check_cnt == verify_push_start + verify_push_len - push {
                let skipped = if j + push as usize >= (*(*msg).msg_iov.add(i)).iov_len { ((*(*msg).msg_iov.add(i)).iov_len - j) as c_int } else { push };
                push -= skipped; size -= skipped; j += (skipped - 1) as usize; check_cnt += skipped; j += 1; continue;
            }
            if verify_pop_len > 0 && check_cnt == verify_pop_start {
                bytes_cnt += verify_pop_len; check_cnt += verify_pop_len; k = k.wrapping_add(verify_pop_len as u8);
                if bytes_cnt == chunk_sz { k = 0; bytes_cnt = 0; check_cnt = 0; push = verify_push_len; }
                if push > 0 && check_cnt == verify_push_start + verify_push_len - push { continue; }
            }
            if *d.add(j) != k {
                fprintf(stderr, b"detected data corruption @iov[%i]:%i %02x != %02x, %02x ?= %02x\n\0".as_ptr() as *const c_char,
                        i as c_int, j as c_int, *d.add(j) as c_int, k as c_int, *d.add(j + 1) as c_int, k.wrapping_add(1) as c_int);
                return -EDATAINTEGRITY;
            }
            k = k.wrapping_add(1); bytes_cnt += 1; check_cnt += 1;
            if bytes_cnt == chunk_sz { k = 0; bytes_cnt = 0; check_cnt = 0; push = verify_push_len; }
            size -= 1; j += 1;
        }
        i += 1;
    }
    *k_p = k; *bytes_cnt_p = bytes_cnt; *check_cnt_p = check_cnt; *push_p = push; 0
}

unsafe fn msg_loop(fd: c_int, mut iov_count: c_int, iov_length: c_int, cnt: c_int, s: *mut msg_stats, tx: bool, opt: *mut sockmap_options) -> c_int {
    let mut msg: msghdr = mem::zeroed(); let mut msg_peek: msghdr = mem::zeroed(); let mut flags = MSG_NOSIGNAL;
    let drop = (*opt).drop_expected; let data = (*opt).data_test; let mut iov_alloc_length = iov_length;
    if !tx && (*opt).check_recved_len { iov_alloc_length *= 2; }
    let mut err = msg_alloc_iov(&mut msg, iov_count, iov_alloc_length, data, tx); if err != 0 { msg_free_iov(&mut msg); return errno; }
    if peek_flag != 0 { err = msg_alloc_iov(&mut msg_peek, iov_count, iov_length, data, tx); if err != 0 { msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; } }
    if tx {
        clock_gettime(CLOCK_MONOTONIC, &mut (*s).start);
        for _ in 0..cnt {
            errno = 0; let sent = sendmsg(fd, &msg, flags) as c_int;
            if !drop && sent < 0 {
                if (*opt).tx_wait_mem && errno == EACCES { errno = 0; msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
                perror(b"sendmsg loop error\0".as_ptr() as *const c_char); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno;
            } else if drop && sent >= 0 {
                fprintf(stderr, b"sendmsg loop error expected: %i errno %i\n\0".as_ptr() as *const c_char, sent, errno); errno = -EIO; msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno;
            }
            if sent > 0 { (*s).bytes_sent += sent as size_t; }
        }
        clock_gettime(CLOCK_MONOTONIC, &mut (*s).end);
    } else {
        let mut recvp = 0; let max_fd = fd; let mut timeout: timeval = mem::zeroed(); let mut k: u8 = 0; let mut bytes_cnt = 0; let mut check_cnt = 0; let mut push = 0; let mut w: fd_set = mem::zeroed();
        fcntl(fd, O_NONBLOCK);
        let mut total_bytes = (iov_length as f32) * (cnt as f32); if !(*opt).sendpage { total_bytes *= iov_count as f32; }
        let (txmsg_push_total, txmsg_pop_total) = if txmsg_apply != 0 { (txmsg_end_push as f32 * (total_bytes / txmsg_apply as f32), txmsg_pop as f32 * (total_bytes / txmsg_apply as f32)) } else { (txmsg_end_push as f32 * cnt as f32, txmsg_pop as f32 * cnt as f32) };
        total_bytes += txmsg_push_total; total_bytes -= txmsg_pop_total;
        if data { msg_verify_date_prep(); push = verify_push_len; }
        err = clock_gettime(CLOCK_MONOTONIC, &mut (*s).start); if err < 0 { perror(b"recv start time\0".as_ptr() as *const c_char); }
        while ((*s).bytes_recvd as f32) < total_bytes {
            if txmsg_cork != 0 { timeout.tv_sec = 0; timeout.tv_usec = 300000; } else { timeout.tv_sec = 3; timeout.tv_usec = 0; }
            FD_ZERO(&mut w); FD_SET(fd, &mut w);
            let slct = select(max_fd + 1, &mut w, ptr::null_mut(), ptr::null_mut(), &mut timeout);
            if slct == -1 { perror(b"select()\0".as_ptr() as *const c_char); clock_gettime(CLOCK_MONOTONIC, &mut (*s).end); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
            else if slct == 0 { if (*opt).verbose != 0 { fprintf(stderr, b"unexpected timeout: recved %zu/%f pop_total %f\n\0".as_ptr() as *const c_char, (*s).bytes_recvd, total_bytes as f64, txmsg_pop_total as f64); } errno = -EIO; clock_gettime(CLOCK_MONOTONIC, &mut (*s).end); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
            if (*opt).tx_wait_mem { FD_ZERO(&mut w); FD_SET(fd, &mut w); select(max_fd + 1, ptr::null_mut(), ptr::null_mut(), &mut w, &mut timeout); errno = 0; close(fd); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
            errno = 0;
            if peek_flag != 0 {
                flags |= MSG_PEEK; recvp = recvmsg(fd, &mut msg_peek, flags) as c_int;
                if recvp < 0 && errno != EWOULDBLOCK { clock_gettime(CLOCK_MONOTONIC, &mut (*s).end); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
                flags = 0;
            }
            let recv_ = recvmsg(fd, &mut msg, flags) as c_int;
            if recv_ < 0 && errno != EWOULDBLOCK { clock_gettime(CLOCK_MONOTONIC, &mut (*s).end); perror(b"recv failed()\0".as_ptr() as *const c_char); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
            if recv_ > 0 { (*s).bytes_recvd += recv_ as size_t; }
            if (*opt).check_recved_len && ((*s).bytes_recvd as f32) > total_bytes { errno = EMSGSIZE; fprintf(stderr, b"recv failed(), bytes_recvd:%zd, total_bytes:%f\n\0".as_ptr() as *const c_char, (*s).bytes_recvd, total_bytes as f64); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
            if data {
                let chunk_sz = if (*opt).sendpage { iov_length } else { iov_length * iov_count };
                errno = msg_verify_data(&mut msg, recv_, chunk_sz, &mut k, &mut bytes_cnt, &mut check_cnt, &mut push);
                if errno != 0 { perror(b"data verify msg failed\0".as_ptr() as *const c_char); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; }
                if recvp != 0 { errno = msg_verify_data(&mut msg_peek, recvp, chunk_sz, &mut k, &mut bytes_cnt, &mut check_cnt, &mut push); if errno != 0 { perror(b"data verify msg_peek failed\0".as_ptr() as *const c_char); msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); return errno; } }
            }
        }
        clock_gettime(CLOCK_MONOTONIC, &mut (*s).end);
    }
    msg_free_iov(&mut msg); msg_free_iov(&mut msg_peek); err
}

static mut giga: f32 = 1000000000.0;
unsafe fn sentBps(s: msg_stats) -> f32 { s.bytes_sent as f32 / (s.end.tv_sec - s.start.tv_sec) as f32 }
unsafe fn recvdBps(s: msg_stats) -> f32 { s.bytes_recvd as f32 / (s.end.tv_sec - s.start.tv_sec) as f32 }

unsafe fn sendmsg_test(opt: *mut sockmap_options) -> c_int {
    let mut sent_Bps = 0.0f32; let mut recvd_Bps = 0.0f32; let mut err = 0; let mut s: msg_stats = mem::zeroed();
    let mut iov_count = (*opt).iov_count; let iov_buf = (*opt).iov_length; let cnt = (*opt).rate; errno = 0;
    let rx_fd = if (*opt).base { p1 } else { p2 };
    if (*opt).tx_wait_mem {
        let timeout = timeval{tv_sec:3,tv_usec:0}; let rxtx_buf_len: c_int = 1024;
        err = setsockopt(c2, SOL_SOCKET, SO_SNDTIMEO, &timeout as *const _ as *const c_void, mem::size_of::<timeval>() as socklen_t);
        err |= setsockopt(c2, SOL_SOCKET, SO_SNDBUFFORCE, &rxtx_buf_len as *const _ as *const c_void, mem::size_of::<c_int>() as socklen_t);
        err |= setsockopt(p2, SOL_SOCKET, SO_RCVBUFFORCE, &rxtx_buf_len as *const _ as *const c_void, mem::size_of::<c_int>() as socklen_t);
        if err != 0 { perror(b"setsockopt failed()\0".as_ptr() as *const c_char); return errno; }
    }
    let rxpid = fork();
    if rxpid == 0 {
        if (*opt).drop_expected || iov_buf == 0 { _exit(0); }
        if (*opt).sendpage { iov_count = 1; }
        err = msg_loop(rx_fd, iov_count, iov_buf, cnt, &mut s, false, opt);
        if (*opt).verbose > 1 { fprintf(stderr, b"msg_loop_rx: iov_count %i iov_buf %i cnt %i err %i\n\0".as_ptr() as *const c_char, iov_count, iov_buf, cnt, err); }
        if s.end.tv_sec - s.start.tv_sec != 0 { sent_Bps = sentBps(s); recvd_Bps = recvdBps(s); }
        if (*opt).verbose > 1 { fprintf(stdout, b"rx_sendmsg: TX: %zuB %fB/s %fGB/s RX: %zuB %fB/s %fGB/s %s\n\0".as_ptr() as *const c_char, s.bytes_sent, sent_Bps as f64, (sent_Bps/giga) as f64, s.bytes_recvd, recvd_Bps as f64, (recvd_Bps/giga) as f64, if peek_flag != 0 { b"(peek_msg)\0".as_ptr() } else { b"\0".as_ptr() }); }
        if err != 0 && err != -EDATAINTEGRITY && txmsg_cork != 0 { err = 0; } exit(if err != 0 {1} else {0});
    } else if rxpid == -1 { perror(b"msg_loop_rx\0".as_ptr() as *const c_char); return errno; }
    if (*opt).tx_wait_mem { close(c2); }
    let txpid = fork();
    if txpid == 0 {
        err = if (*opt).sendpage { msg_loop_sendpage(c1, iov_buf, cnt, &mut s, opt) } else { msg_loop(c1, iov_count, iov_buf, cnt, &mut s, true, opt) };
        if err != 0 { fprintf(stderr, b"msg_loop_tx: iov_count %i iov_buf %i cnt %i err %i\n\0".as_ptr() as *const c_char, iov_count, iov_buf, cnt, err); }
        if s.end.tv_sec - s.start.tv_sec != 0 { sent_Bps = sentBps(s); recvd_Bps = recvdBps(s); }
        if (*opt).verbose > 1 { fprintf(stdout, b"tx_sendmsg: TX: %zuB %fB/s %f GB/s RX: %zuB %fB/s %fGB/s\n\0".as_ptr() as *const c_char, s.bytes_sent, sent_Bps as f64, (sent_Bps/giga) as f64, s.bytes_recvd, recvd_Bps as f64, (recvd_Bps/giga) as f64); }
        exit(if err != 0 {1} else {0});
    } else if txpid == -1 { perror(b"msg_loop_tx\0".as_ptr() as *const c_char); return errno; }
    let mut rx_status = 0; let mut tx_status = 0;
    assert!(waitpid(rxpid, &mut rx_status, 0) == rxpid); assert!(waitpid(txpid, &mut tx_status, 0) == txpid);
    if WIFEXITED(rx_status) { err = WEXITSTATUS(rx_status); if err != 0 { fprintf(stderr, b"rx thread exited with err %d.\n\0".as_ptr() as *const c_char, err); return err; } }
    if WIFEXITED(tx_status) { err = WEXITSTATUS(tx_status); if err != 0 { fprintf(stderr, b"tx thread exited with err %d.\n\0".as_ptr() as *const c_char, err); } }
    err
}

unsafe fn forever_ping_pong(rate: c_int, opt: *mut sockmap_options) -> c_int {
    let mut timeout = timeval{tv_sec:10,tv_usec:0}; let mut buf = [0i8; 1024];
    let mut sc = send(c1, buf.as_ptr() as *const c_void, buf.len(), 0) as c_int; if sc < 0 { perror(b"send failed()\0".as_ptr() as *const c_char); return sc; }
    while running != 0 {
        let mut w: fd_set = mem::zeroed(); FD_ZERO(&mut w); FD_SET(c1,&mut w); FD_SET(c2,&mut w); FD_SET(p1,&mut w); FD_SET(p2,&mut w);
        let max_fd = p2; let mut s = select(max_fd + 1, &mut w, ptr::null_mut(), ptr::null_mut(), &mut timeout);
        if s == -1 { perror(b"select()\0".as_ptr() as *const c_char); break; } else if s == 0 { fprintf(stderr, b"unexpected timeout\n\0".as_ptr() as *const c_char); break; }
        let mut i = 0; while i <= max_fd && s > 0 {
            if !FD_ISSET(i, &mut w) { i += 1; continue; } s -= 1;
            let rc = recv(i, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) as c_int;
            if rc < 0 && errno != EWOULDBLOCK { perror(b"recv failed()\0".as_ptr() as *const c_char); return rc; }
            if rc == 0 { close(i); break; }
            sc = send(i, buf.as_ptr() as *const c_void, rc as size_t, 0) as c_int; if sc < 0 { perror(b"send failed()\0".as_ptr() as *const c_char); return sc; }
            i += 1;
        }
        if rate != 0 { sleep(rate as c_uint); }
        if (*opt).verbose != 0 { printf(b".\0".as_ptr() as *const c_char); fflush(stdout); }
    }
    0
}

const SELFTESTS: c_int = 0; const PING_PONG: c_int = 1; const SENDMSG: c_int = 2; const BASE: c_int = 3; const BASE_SENDPAGE: c_int = 4; const SENDPAGE: c_int = 5;

unsafe fn run_options(options: *mut sockmap_options, cg_fd: c_int, test: c_int) -> c_int {
    let mut err = 0; let zero: c_int = 0;
    if test != BASE && test != BASE_SENDPAGE {
        if txmsg_omit_skb_parser == 0 {
            links[0] = bpf_program__attach_sockmap(progs[0], map_fd[0]);
            if links[0].is_null() { fprintf(stderr, b"ERROR: bpf_program__attach_sockmap (sockmap %i->%i): (%s)\n\0".as_ptr() as *const c_char, bpf_program__fd(progs[0]), map_fd[0], b"errno\0".as_ptr()); return -1; }
        }
        links[1] = bpf_program__attach_sockmap(progs[1], map_fd[0]); if links[1].is_null() { fprintf(stderr, b"ERROR: bpf_program__attach_sockmap (sockmap): (%s)\n\0".as_ptr() as *const c_char, b"errno\0".as_ptr()); return -1; }
        err = bpf_prog_attach(bpf_program__fd(progs[2]), cg_fd, BPF_CGROUP_SOCK_OPS, 0); if err != 0 { fprintf(stderr, b"ERROR: bpf_prog_attach (groups): %d\n\0".as_ptr() as *const c_char, err); return err; }
    }
    err = sockmap_init_sockets((*options).verbose); if err != 0 { fprintf(stderr, b"ERROR: test socket failed: %d\n\0".as_ptr() as *const c_char, err); }
    else {
        let tx_prog = if txmsg_pass != 0 { progs[3] } else if txmsg_redir != 0 { progs[4] } else if txmsg_apply != 0 { progs[5] } else if txmsg_cork != 0 { progs[6] } else if txmsg_drop != 0 { progs[7] } else { ptr::null_mut() };
        if !tx_prog.is_null() {
            links[4] = bpf_program__attach_sockmap(tx_prog, map_fd[1]); if links[4].is_null() { err = -1; }
            let mut i = 0; bpf_map_update_elem(map_fd[1], &i as *const _ as *const c_void, &c1 as *const _ as *const c_void, BPF_ANY);
            let mut redir_fd = if txmsg_redir != 0 { c2 } else { c1 }; bpf_map_update_elem(map_fd[2], &i as *const _ as *const c_void, &redir_fd as *const _ as *const c_void, BPF_ANY);
            if txmsg_apply != 0 { bpf_map_update_elem(map_fd[3], &i as *const _ as *const c_void, &txmsg_apply as *const _ as *const c_void, BPF_ANY); }
            if txmsg_cork != 0 { bpf_map_update_elem(map_fd[4], &i as *const _ as *const c_void, &txmsg_cork as *const _ as *const c_void, BPF_ANY); }
            if txmsg_start != 0 { bpf_map_update_elem(map_fd[5], &i as *const _ as *const c_void, &txmsg_start as *const _ as *const c_void, BPF_ANY); }
            if txmsg_end != 0 { i = 1; bpf_map_update_elem(map_fd[5], &i as *const _ as *const c_void, &txmsg_end as *const _ as *const c_void, BPF_ANY); }
            if txmsg_start_push != 0 { i = 2; bpf_map_update_elem(map_fd[5], &i as *const _ as *const c_void, &txmsg_start_push as *const _ as *const c_void, BPF_ANY); }
            if txmsg_end_push != 0 { i = 3; bpf_map_update_elem(map_fd[5], &i as *const _ as *const c_void, &txmsg_end_push as *const _ as *const c_void, BPF_ANY); }
            i = 4; bpf_map_update_elem(map_fd[5], &i as *const _ as *const c_void, &txmsg_start_pop as *const _ as *const c_void, BPF_ANY);
            i = 5; bpf_map_update_elem(map_fd[5], &i as *const _ as *const c_void, &txmsg_pop as *const _ as *const c_void, BPF_ANY);
            if txmsg_ingress != 0 {
                let ingress = BPF_F_INGRESS; i = 0; bpf_map_update_elem(map_fd[6], &i as *const _ as *const c_void, &ingress as *const _ as *const c_void, BPF_ANY);
                i = 1; bpf_map_update_elem(map_fd[1], &i as *const _ as *const c_void, &p1 as *const _ as *const c_void, BPF_ANY); bpf_map_update_elem(map_fd[2], &i as *const _ as *const c_void, &p1 as *const _ as *const c_void, BPF_ANY);
                i = 2; bpf_map_update_elem(map_fd[2], &i as *const _ as *const c_void, &p2 as *const _ as *const c_void, BPF_ANY);
            }
            if txmsg_redir_skb != 0 {
                let skb_fd = if test == SENDMSG || test == SENDPAGE { p2 } else { p1 }; let ingress = BPF_F_INGRESS; i = 0; bpf_map_update_elem(map_fd[7], &i as *const _ as *const c_void, &ingress as *const _ as *const c_void, BPF_ANY);
                i = 3; bpf_map_update_elem(map_fd[0], &i as *const _ as *const c_void, &skb_fd as *const _ as *const c_void, BPF_ANY);
            }
        }
        if skb_use_parser != 0 { let i = 2; bpf_map_update_elem(map_fd[7], &i as *const _ as *const c_void, &skb_use_parser as *const _ as *const c_void, BPF_ANY); }
        if txmsg_drop != 0 { (*options).drop_expected = true; }
        err = match test { PING_PONG => forever_ping_pong((*options).rate, options), SENDMSG => { (*options).base=false; (*options).sendpage=false; sendmsg_test(options) }, SENDPAGE => { (*options).base=false; (*options).sendpage=true; sendmsg_test(options) }, BASE => { (*options).base=true; (*options).sendpage=false; sendmsg_test(options) }, BASE_SENDPAGE => { (*options).base=true; (*options).sendpage=true; sendmsg_test(options) }, _ => { fprintf(stderr, b"unknown test\n\0".as_ptr() as *const c_char); err } };
    }
    bpf_prog_detach2(bpf_program__fd(progs[2]), cg_fd, BPF_CGROUP_SOCK_OPS);
    for i in 0..links.len() { if !links[i].is_null() { bpf_link__detach(links[i]); } }
    for i in 0..map_fd.len() {
        let mut key = 0; let mut next_key = 0; bpf_map_update_elem(map_fd[i], &key as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_ANY);
        while bpf_map_get_next_key(map_fd[i], &key as *const _ as *const c_void, &mut next_key as *mut _ as *mut c_void) == 0 {
            bpf_map_update_elem(map_fd[i], &key as *const _ as *const c_void, &zero as *const _ as *const c_void, BPF_ANY); key = next_key;
        }
    }
    close(s1); close(s2); close(p1); close(p2); close(c1); close(c2); err
}

unsafe fn test_to_str(test: c_int) -> *const c_char { match test { SENDMSG => b"sendmsg\0".as_ptr() as *const c_char, SENDPAGE => b"sendpage\0".as_ptr() as *const c_char, _ => b"unknown\0".as_ptr() as *const c_char } }
unsafe fn append_str(dst: *mut c_char, src: *const c_char, dst_cap: size_t) { let avail = dst_cap - strlen(dst); if avail <= 1 { return; } strncat(dst, src, avail - 1); }
unsafe fn test_options(options: *mut c_char) {
    let mut tstr = [0 as c_char; OPTSTRING]; memset(options as *mut c_void, 0, OPTSTRING);
    if txmsg_pass != 0 { append_str(options, b"pass,\0".as_ptr() as *const c_char, OPTSTRING); }
    if txmsg_redir != 0 { append_str(options, b"redir,\0".as_ptr() as *const c_char, OPTSTRING); }
    if txmsg_drop != 0 { append_str(options, b"drop,\0".as_ptr() as *const c_char, OPTSTRING); }
    if txmsg_apply != 0 { snprintf(tstr.as_mut_ptr(), OPTSTRING, b"apply %d,\0".as_ptr() as *const c_char, txmsg_apply); append_str(options, tstr.as_ptr(), OPTSTRING); }
    if txmsg_cork != 0 { snprintf(tstr.as_mut_ptr(), OPTSTRING, b"cork %d,\0".as_ptr() as *const c_char, txmsg_cork); append_str(options, tstr.as_ptr(), OPTSTRING); }
    if txmsg_start != 0 { snprintf(tstr.as_mut_ptr(), OPTSTRING, b"start %d,\0".as_ptr() as *const c_char, txmsg_start); append_str(options, tstr.as_ptr(), OPTSTRING); }
    if txmsg_end != 0 { snprintf(tstr.as_mut_ptr(), OPTSTRING, b"end %d,\0".as_ptr() as *const c_char, txmsg_end); append_str(options, tstr.as_ptr(), OPTSTRING); }
    if txmsg_start_pop != 0 { snprintf(tstr.as_mut_ptr(), OPTSTRING, b"pop (%d,%d),\0".as_ptr() as *const c_char, txmsg_start_pop, txmsg_start_pop + txmsg_pop); append_str(options, tstr.as_ptr(), OPTSTRING); }
    if txmsg_ingress != 0 { append_str(options, b"ingress,\0".as_ptr() as *const c_char, OPTSTRING); }
    if txmsg_redir_skb != 0 { append_str(options, b"redir_skb,\0".as_ptr() as *const c_char, OPTSTRING); }
    if peek_flag != 0 { append_str(options, b"peek,\0".as_ptr() as *const c_char, OPTSTRING); }
}

unsafe fn __test_exec(cgrp: c_int, test: c_int, opt: *mut sockmap_options) -> c_int {
    let options = calloc(OPTSTRING, mem::size_of::<c_char>()) as *mut c_char; if test == SENDPAGE { (*opt).sendpage = true; } else { (*opt).sendpage = false; }
    (*opt).drop_expected = txmsg_drop != 0; test_options(options);
    if (*opt).verbose != 0 { fprintf(stdout, b" [TEST %i]: (%i, %i, %i, %s, %s): \0".as_ptr() as *const c_char, test_cnt, (*opt).rate, (*opt).iov_count, (*opt).iov_length, test_to_str(test), options); fflush(stdout); }
    let err = run_options(opt, cgrp, test); if (*opt).verbose != 0 { fprintf(stdout, b" %s\n\0".as_ptr() as *const c_char, if err == 0 { b"PASS\0".as_ptr() } else { b"FAILED\0".as_ptr() }); }
    test_cnt += 1; if err == 0 { passed += 1; } else { failed += 1; } free(options as *mut c_void); err
}
unsafe fn test_exec(cgrp: c_int, opt: *mut sockmap_options) { let type_ = strcmp((*opt).map, cstr(BPF_SOCKMAP_FILENAME)); test_start(); let err = if type_ == 0 { __test_exec(cgrp, SENDMSG, opt) } else { __test_exec(cgrp, SENDPAGE, opt) }; if err != 0 { test_fail(); } }
unsafe fn test_send_one(opt:*mut sockmap_options,cgrp:c_int){(*opt).iov_length=1;(*opt).iov_count=1;(*opt).rate=1;test_exec(cgrp,opt);(*opt).iov_length=1;(*opt).iov_count=1024;(*opt).rate=1;test_exec(cgrp,opt);(*opt).iov_length=1024;(*opt).iov_count=1;(*opt).rate=1;test_exec(cgrp,opt);}
unsafe fn test_send_many(opt:*mut sockmap_options,cgrp:c_int){(*opt).iov_length=3;(*opt).iov_count=1;(*opt).rate=512;test_exec(cgrp,opt);(*opt).rate=100;(*opt).iov_count=1;(*opt).iov_length=5;test_exec(cgrp,opt);}
unsafe fn test_send_large(opt:*mut sockmap_options,cgrp:c_int){(*opt).iov_length=8192;(*opt).iov_count=32;(*opt).rate=2;test_exec(cgrp,opt);}
unsafe fn test_send(opt:*mut sockmap_options,cgrp:c_int){test_send_one(opt,cgrp);test_send_many(opt,cgrp);test_send_large(opt,cgrp);sched_yield();}

unsafe extern "C" fn test_txmsg_pass_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=1;test_send(opt,cgrp);}
unsafe extern "C" fn test_txmsg_redir_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_redir=1;test_send(opt,cgrp);}
unsafe extern "C" fn test_txmsg_redir_wait_sndmem_fn(cgrp:c_int,opt:*mut sockmap_options){(*opt).tx_wait_mem=true;txmsg_redir=1;test_send_large(opt,cgrp);txmsg_redir=1;txmsg_apply=4097;test_send_large(opt,cgrp);(*opt).tx_wait_mem=false;}
unsafe extern "C" fn test_txmsg_drop_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_drop=1;test_send(opt,cgrp);}
unsafe extern "C" fn test_txmsg_ingress_redir_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=0;txmsg_drop=0;txmsg_ingress=1;txmsg_redir=1;test_send(opt,cgrp);}
unsafe extern "C" fn test_txmsg_cork_hangs_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=1;txmsg_redir=0;txmsg_cork=4097;txmsg_apply=4097;test_send_large(opt,cgrp);txmsg_pass=0;txmsg_redir=1;txmsg_apply=0;txmsg_cork=4097;test_send_large(opt,cgrp);txmsg_pass=0;txmsg_redir=1;txmsg_apply=4097;txmsg_cork=4097;test_send_large(opt,cgrp);}
unsafe extern "C" fn test_txmsg_pull_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=1;txmsg_start=1;txmsg_end=2;test_send(opt,cgrp);txmsg_pass=1;txmsg_start=4096;txmsg_end=9182;test_send_large(opt,cgrp);txmsg_redir=1;txmsg_start=1;txmsg_end=2;test_send(opt,cgrp);txmsg_redir=0;txmsg_cork=512;txmsg_start=1;txmsg_end=2;test_send_many(opt,cgrp);txmsg_redir=1;txmsg_cork=512;txmsg_start=1;txmsg_end=2;test_send_many(opt,cgrp);}
unsafe extern "C" fn test_txmsg_pop_fn(cgrp:c_int,opt:*mut sockmap_options){let data=(*opt).data_test;txmsg_pass=1;txmsg_start_pop=1;txmsg_pop=2;test_send_many(opt,cgrp);txmsg_pass=1;txmsg_start_pop=4096;txmsg_pop=4096;test_send_large(opt,cgrp);txmsg_redir=1;txmsg_start_pop=1;txmsg_pop=2;test_send_many(opt,cgrp);(*opt).data_test=false;txmsg_redir=0;txmsg_cork=512;txmsg_start_pop=1;txmsg_pop=2;test_send_many(opt,cgrp);txmsg_redir=1;txmsg_cork=4;txmsg_start_pop=1;txmsg_pop=2;test_send_many(opt,cgrp);(*opt).data_test=data;}
unsafe extern "C" fn test_txmsg_push_fn(cgrp:c_int,opt:*mut sockmap_options){let data=(*opt).data_test;txmsg_pass=1;txmsg_start_push=1;txmsg_end_push=1;test_send(opt,cgrp);txmsg_pass=1;txmsg_start_push=4096;txmsg_end_push=4096;test_send_large(opt,cgrp);txmsg_redir=1;txmsg_start_push=1;txmsg_end_push=2;test_send_many(opt,cgrp);(*opt).data_test=false;txmsg_redir=0;txmsg_cork=512;txmsg_start_push=1;txmsg_end_push=2;test_send_many(opt,cgrp);(*opt).data_test=data;}
unsafe extern "C" fn test_txmsg_push_pop_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=1;txmsg_start_push=1;txmsg_end_push=10;txmsg_start_pop=5;txmsg_pop=4;test_send_large(opt,cgrp);txmsg_pass=1;txmsg_start_push=1;txmsg_end_push=10;txmsg_start_pop=5;txmsg_pop=16;test_send_large(opt,cgrp);txmsg_pass=1;txmsg_start_push=5;txmsg_end_push=4;txmsg_start_pop=1;txmsg_pop=10;test_send_large(opt,cgrp);txmsg_pass=1;txmsg_start_push=5;txmsg_end_push=16;txmsg_start_pop=1;txmsg_pop=10;test_send_large(opt,cgrp);txmsg_pass=1;txmsg_start_push=1;txmsg_end_push=10;txmsg_start_pop=16;txmsg_pop=4;test_send_large(opt,cgrp);txmsg_pass=1;txmsg_start_push=16;txmsg_end_push=10;txmsg_start_pop=5;txmsg_pop=4;test_send_large(opt,cgrp);}
unsafe extern "C" fn test_txmsg_apply_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=1;txmsg_redir=0;txmsg_ingress=0;txmsg_apply=1;txmsg_cork=0;test_send_one(opt,cgrp);txmsg_pass=0;txmsg_redir=1;txmsg_ingress=0;txmsg_apply=1;txmsg_cork=0;test_send_one(opt,cgrp);txmsg_pass=0;txmsg_redir=1;txmsg_ingress=1;txmsg_apply=1;txmsg_cork=0;test_send_one(opt,cgrp);txmsg_pass=1;txmsg_redir=0;txmsg_ingress=0;txmsg_apply=1024;txmsg_cork=0;test_send_large(opt,cgrp);txmsg_pass=0;txmsg_redir=1;txmsg_ingress=0;txmsg_apply=1024;txmsg_cork=0;test_send_large(opt,cgrp);txmsg_pass=0;txmsg_redir=1;txmsg_ingress=1;txmsg_apply=1024;txmsg_cork=0;test_send_large(opt,cgrp);}
unsafe extern "C" fn test_txmsg_cork_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=1;txmsg_redir=0;txmsg_apply=0;txmsg_cork=1;test_send(opt,cgrp);txmsg_pass=1;txmsg_redir=0;txmsg_apply=1;txmsg_cork=1;test_send(opt,cgrp);}
unsafe extern "C" fn test_txmsg_ingress_parser_fn(cgrp:c_int,opt:*mut sockmap_options){txmsg_pass=1;skb_use_parser=512;(*opt).iov_length=256;(*opt).iov_count=1;(*opt).rate=2;test_exec(cgrp,opt);}
unsafe extern "C" fn test_txmsg_ingress_parser2_fn(cgrp:c_int,opt:*mut sockmap_options){skb_use_parser=10;(*opt).iov_length=20;(*opt).iov_count=1;(*opt).rate=1;(*opt).check_recved_len=true;test_exec(cgrp,opt);(*opt).check_recved_len=false;}

static mut map_names: [*mut c_char; 8] = [
    b"sock_map\0".as_ptr() as *mut c_char,b"sock_map_txmsg\0".as_ptr() as *mut c_char,b"sock_map_redir\0".as_ptr() as *mut c_char,b"sock_apply_bytes\0".as_ptr() as *mut c_char,
    b"sock_cork_bytes\0".as_ptr() as *mut c_char,b"sock_bytes\0".as_ptr() as *mut c_char,b"sock_redir_flags\0".as_ptr() as *mut c_char,b"sock_skb_opts\0".as_ptr() as *mut c_char,
];

unsafe fn populate_progs(bpf_file:*mut c_char)->c_int{
    let obj=bpf_object__open(bpf_file); let err=libbpf_get_error(obj as *const c_void);
    if err!=0{let mut err_buf=[0 as c_char;256];libbpf_strerror(err,err_buf.as_mut_ptr(),err_buf.len());printf(b"Unable to load eBPF objects in file '%s' : %s\n\0".as_ptr() as *const c_char,bpf_file,err_buf.as_ptr());return -1;}
    bpf_object__load(obj); let mut i=0usize; let mut prog: *mut bpf_program = ptr::null_mut();
    loop { prog = bpf_object__next_program(obj, prog); if prog.is_null(){break;} progs[i]=prog; i+=1; }
    for j in 0..map_fd.len(){ maps[j]=bpf_object__find_map_by_name(obj,map_names[j]); map_fd[j]=bpf_map__fd(maps[j]); if map_fd[j]<0{fprintf(stderr,b"load_bpf_file: (%i)\n\0".as_ptr() as *const c_char,map_fd[j]);return -1;} }
    for j in 0..links.len(){links[j]=ptr::null_mut();} 0
}

static mut test: [_test; 14] = [
    _test{title:b"txmsg test passthrough\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_pass_fn)},
    _test{title:b"txmsg test redirect\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_redir_fn)},
    _test{title:b"txmsg test redirect wait send mem\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_redir_wait_sndmem_fn)},
    _test{title:b"txmsg test drop\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_drop_fn)},
    _test{title:b"txmsg test ingress redirect\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_ingress_redir_fn)},
    _test{title:b"txmsg test apply\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_apply_fn)},
    _test{title:b"txmsg test cork\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_cork_fn)},
    _test{title:b"txmsg test hanging corks\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_cork_hangs_fn)},
    _test{title:b"txmsg test push_data\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_push_fn)},
    _test{title:b"txmsg test pull-data\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_pull_fn)},
    _test{title:b"txmsg test pop-data\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_pop_fn)},
    _test{title:b"txmsg test push/pop data\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_push_pop_fn)},
    _test{title:b"txmsg test ingress parser\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_ingress_parser_fn)},
    _test{title:b"txmsg test ingress parser2\0".as_ptr() as *mut c_char,tester:Some(test_txmsg_ingress_parser2_fn)},
];

unsafe fn check_whitelist(t:*mut _test,opt:*mut sockmap_options)->c_int{ if (*opt).whitelist.is_null(){return 0;} let ptr=strdup((*opt).whitelist); if ptr.is_null(){return -ENOMEM;} let mut entry=strtok(ptr,b",\0".as_ptr() as *const c_char); while !entry.is_null(){ if ((!(*opt).prepend.is_null())&& !strstr((*opt).prepend,entry).is_null()) || !strstr((*opt).map,entry).is_null() || !strstr((*t).title,entry).is_null(){free(ptr as *mut c_void);return 0;} entry=strtok(ptr::null_mut(),b",\0".as_ptr() as *const c_char);} free(ptr as *mut c_void); -EINVAL }
unsafe fn check_blacklist(t:*mut _test,opt:*mut sockmap_options)->c_int{ if (*opt).blacklist.is_null(){return -EINVAL;} let ptr=strdup((*opt).blacklist); if ptr.is_null(){return -ENOMEM;} let mut entry=strtok(ptr,b",\0".as_ptr() as *const c_char); while !entry.is_null(){ if ((!(*opt).prepend.is_null())&& !strstr((*opt).prepend,entry).is_null()) || !strstr((*opt).map,entry).is_null() || !strstr((*t).title,entry).is_null(){free(ptr as *mut c_void);return 0;} entry=strtok(ptr::null_mut(),b",\0".as_ptr() as *const c_char);} free(ptr as *mut c_void); -EINVAL }

unsafe fn __test_selftests(cg_fd:c_int,opt:*mut sockmap_options)->c_int{ let err=populate_progs((*opt).map); if err<0{fprintf(stderr,b"ERROR: (%i) load bpf failed\n\0".as_ptr() as *const c_char,err);return err;} for i in 0..test.len(){ let mut t=_test{title:test[i].title,tester:test[i].tester}; if check_whitelist(&mut t,opt)!=0{continue;} if check_blacklist(&mut t,opt)==0{continue;} test_start_subtest(&t,opt); if let Some(f)=t.tester{f(cg_fd,opt);} test_end_subtest(); } err }
unsafe fn test_selftests_sockmap(cg_fd:c_int,opt:*mut sockmap_options){(*opt).map=mut_cstr(BPF_SOCKMAP_FILENAME);__test_selftests(cg_fd,opt);}
unsafe fn test_selftests_sockhash(cg_fd:c_int,opt:*mut sockmap_options){(*opt).map=mut_cstr(BPF_SOCKHASH_FILENAME);__test_selftests(cg_fd,opt);}
unsafe fn test_selftest(cg_fd:c_int,opt:*mut sockmap_options)->c_int{test_selftests_sockmap(cg_fd,opt);test_selftests_sockhash(cg_fd,opt);test_print_results();0}

#[no_mangle]
pub unsafe extern "C" fn main(argc:c_int, argv:*mut *mut c_char)->c_int{
    let mut iov_count=1; let mut length=1024; let mut rate=1; let mut options: sockmap_options = mem::zeroed();
    let mut longindex=0; let mut err; let mut cg_fd=0; let bpf_file=mut_cstr(BPF_SOCKMAP_FILENAME); let mut test_id=SELFTESTS; let mut cg_created=false;
    loop {
        let opt=getopt_long(argc,argv,b":dhv:c:r:i:l:t:p:q:n:b:\0".as_ptr() as *const c_char,long_options.as_ptr(),&mut longindex);
        if opt == -1 { break; }
        match opt {
            x if x == 's' as c_int => txmsg_start=atoi(optarg),
            x if x == 'e' as c_int => txmsg_end=atoi(optarg),
            x if x == 'p' as c_int => txmsg_start_push=atoi(optarg),
            x if x == 'q' as c_int => txmsg_end_push=atoi(optarg),
            x if x == 'w' as c_int => txmsg_start_pop=atoi(optarg),
            x if x == 'x' as c_int => txmsg_pop=atoi(optarg),
            x if x == 'a' as c_int => txmsg_apply=atoi(optarg),
            x if x == 'k' as c_int => txmsg_cork=atoi(optarg),
            x if x == 'c' as c_int => { cg_fd=open(optarg,O_DIRECTORY,O_RDONLY); if cg_fd<0{fprintf(stderr,b"ERROR: (%i) open cg path failed: %s\n\0".as_ptr() as *const c_char,cg_fd,optarg);return cg_fd;} },
            x if x == 'r' as c_int => rate=atoi(optarg),
            x if x == 'v' as c_int => { options.verbose=1; if !optarg.is_null(){options.verbose=atoi(optarg);} },
            x if x == 'i' as c_int => iov_count=atoi(optarg),
            x if x == 'l' as c_int => length=atoi(optarg),
            x if x == 'd' as c_int => options.data_test=true,
            x if x == 't' as c_int => {
                if strcmp(optarg,b"ping\0".as_ptr() as *const c_char)==0{test_id=PING_PONG;} else if strcmp(optarg,b"sendmsg\0".as_ptr() as *const c_char)==0{test_id=SENDMSG;} else if strcmp(optarg,b"base\0".as_ptr() as *const c_char)==0{test_id=BASE;} else if strcmp(optarg,b"base_sendpage\0".as_ptr() as *const c_char)==0{test_id=BASE_SENDPAGE;} else if strcmp(optarg,b"sendpage\0".as_ptr() as *const c_char)==0{test_id=SENDPAGE;} else {usage(argv);return -1;}
            },
            x if x == 'n' as c_int => { options.whitelist=strdup(optarg); if options.whitelist.is_null(){return -ENOMEM;} },
            x if x == 'b' as c_int => { options.blacklist=strdup(optarg); if options.blacklist.is_null(){return -ENOMEM;} },
            0 => {},
            _ => { usage(argv); return -1; }
        }
    }
    if cg_fd == 0 { cg_fd=cgroup_setup_and_join(cstr(CG_PATH)); if cg_fd<0{return cg_fd;} cg_created=true; }
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    if test_id == SELFTESTS { err=test_selftest(cg_fd,&mut options); }
    else {
        err=populate_progs(bpf_file); if err != 0 { fprintf(stderr,b"populate program: (%s)\n\0".as_ptr() as *const c_char,bpf_file); return 1; }
        running=1; signal(SIGINT,running_handler); options.iov_count=iov_count; options.iov_length=length; options.rate=rate; err=run_options(&mut options,cg_fd,test_id);
    }
    if !options.whitelist.is_null(){free(options.whitelist as *mut c_void);} if !options.blacklist.is_null(){free(options.blacklist as *mut c_void);}
    close(cg_fd); if cg_created{cleanup_cgroup_environment();} err
}

extern "C" fn running_handler(_a:c_int){ unsafe { running=0; } }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
