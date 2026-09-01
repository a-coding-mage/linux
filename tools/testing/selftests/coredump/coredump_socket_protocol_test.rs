// SPDX-License-Identifier: GPL-2.0

// Translated from coredump_socket_protocol_test.c.
// C include dependencies: sys/stat.h, sys/epoll.h, sys/socket.h, sys/un.h,
// and "coredump_test.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::zeroed;
use core::ptr::{null, null_mut};

type pid_t = c_int;
type ssize_t = isize;
type size_t = usize;
type FILE = c_void;

const NUM_CRASHING_COREDUMPS: usize = 5;

extern "C" {
    static mut stderr: *mut FILE;
    static mut errno: c_int;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn feof(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn close(fd: c_int) -> c_int;
    fn accept4(fd: c_int, addr: *mut c_void, len: *mut c_void, flags: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn creat(path: *const c_char, mode: c_uint) -> c_int;
    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn unlink(path: *const c_char) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn abort() -> !;
    fn _exit(status: c_int) -> !;

    fn set_core_pattern(pattern: *const c_char) -> bool;
    fn create_detached_tmpfs() -> c_int;
    fn create_and_listen_unix_socket(path: *const c_char) -> c_int;
    fn write_nointr(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read_nointr(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn get_peer_pidfd(fd: c_int) -> c_int;
    fn get_pidfd_info(fd: c_int, info: *mut pidfd_info) -> bool;
    fn read_coredump_req(fd: c_int, req: *mut coredump_req) -> bool;
    fn check_coredump_req(req: *const coredump_req, size: u64, flags: u64) -> bool;
    fn send_coredump_ack(fd: c_int, req: *const coredump_req, flags: u64, size: u64) -> bool;
    fn read_marker(fd: c_int, marker: u64) -> bool;
    fn wait_and_check_coredump_server(pid: pid_t, metadata: *mut c_void, self_: *mut coredump) -> ();
    fn crashing_child() -> !;
    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;
    fn open_coredump_tmpfile(fd_tmpfs_detached: c_int) -> c_int;
    fn process_coredump_worker(fd_coredump: c_int, fd_peer_pidfd: c_int, fd_core_file: c_int) -> !;
}

#[repr(C)]
struct stat {
    _private: [u8; 0],
    st_size: c_long,
}

#[repr(C)]
struct pidfd_info {
    mask: u64,
    coredump_mask: u64,
    coredump_signal: c_int,
    coredump_code: c_int,
}

#[repr(C)]
struct coredump_req {
    _private: [u8; 0],
}

#[repr(C)]
struct coredump {
    pid_coredump_server: pid_t,
    fd_tmpfs_detached: c_int,
    original_core_pattern: [c_char; 4096],
}

const ESRCH: c_int = 3;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const SIGTERM: c_int = 15;
const SIGSEGV: c_int = 11;
const SIGABRT: c_int = 6;
const SI_TKILL: c_int = -6;
const SEGV_MAPERR: c_int = 1;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = 0o2000000;
const EAGAIN: c_int = 11;
const EWOULDBLOCK: c_int = EAGAIN;
const ENOSPC: c_int = 28;
const PAGE_SIZE: u64 = 4096;
const PIDFD_GET_INFO: c_ulong = 0;
const PIDFD_INFO_EXIT: u64 = 0;
const PIDFD_INFO_COREDUMP: u64 = 0;
const PIDFD_INFO_COREDUMP_SIGNAL: u64 = 0;
const PIDFD_INFO_COREDUMP_CODE: u64 = 0;
const PIDFD_COREDUMPED: u64 = 0;
const COREDUMP_ACK_SIZE_VER0: u64 = 0;
const COREDUMP_KERNEL: u64 = 0;
const COREDUMP_USERSPACE: u64 = 0;
const COREDUMP_REJECT: u64 = 0;
const COREDUMP_WAIT: u64 = 0;
const COREDUMP_MARK_REQACK: u64 = 0;
const COREDUMP_MARK_CONFLICTING: u64 = 0;
const COREDUMP_MARK_UNSUPPORTED: u64 = 0;
const COREDUMP_MARK_MINSIZE: u64 = 0;
const COREDUMP_MARK_MAXSIZE: u64 = 0;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! ASSERT_TRUE { ($e:expr) => { assert!($e); }; }
macro_rules! ASSERT_FALSE { ($e:expr) => { assert!(!$e); }; }
macro_rules! ASSERT_EQ { ($a:expr, $b:expr) => { assert_eq!($a, $b); }; }
macro_rules! ASSERT_NE { ($a:expr, $b:expr) => { assert_ne!($a, $b); }; }
macro_rules! ASSERT_LT { ($a:expr, $b:expr) => { assert!($a < $b); }; }
macro_rules! ASSERT_GE { ($a:expr, $b:expr) => { assert!($a >= $b); }; }
macro_rules! ASSERT_GT { ($a:expr, $b:expr) => { assert!($a > $b); }; }
macro_rules! EXPECT_EQ { ($a:expr, $b:expr) => { assert_eq!($a, $b); }; }

unsafe fn WIFSIGNALED(status: c_int) -> bool { (status & 0x7f) != 0 && (status & 0x7f) != 0x7f }
unsafe fn WCOREDUMP(status: c_int) -> bool { (status & 0x80) != 0 }
unsafe fn WTERMSIG(status: c_int) -> c_int { status & 0x7f }
unsafe fn WIFEXITED(status: c_int) -> bool { (status & 0x7f) == 0 }
unsafe fn WEXITSTATUS(status: c_int) -> c_int { (status >> 8) & 0xff }

unsafe fn coredump_setup(self_: *mut coredump) {
    let mut ret: c_int;
    (*self_).pid_coredump_server = -ESRCH;
    (*self_).fd_tmpfs_detached = -1;
    let file = fopen(cstr!("/proc/sys/kernel/core_pattern"), cstr!("r"));
    ASSERT_NE!(null_mut::<FILE>(), file);

    ret = fread((*self_).original_core_pattern.as_mut_ptr() as *mut c_void, 1, (*self_).original_core_pattern.len(), file) as c_int;
    ASSERT_TRUE!(ret != 0 || feof(file) != 0);
    ASSERT_LT!(ret as usize, (*self_).original_core_pattern.len());

    (*self_).original_core_pattern[ret as usize] = 0;
    (*self_).fd_tmpfs_detached = create_detached_tmpfs();
    ASSERT_GE!((*self_).fd_tmpfs_detached, 0);

    ret = fclose(file);
    ASSERT_EQ!(0, ret);
}

unsafe fn coredump_teardown(self_: *mut coredump) {
    let mut reason: *const c_char;
    let mut ret: c_int;
    let mut status: c_int = 0;

    if (*self_).pid_coredump_server > 0 {
        kill((*self_).pid_coredump_server, SIGTERM);
        waitpid((*self_).pid_coredump_server, &mut status, 0);
    }
    unlink(cstr!("/tmp/coredump.file"));
    unlink(cstr!("/tmp/coredump.socket"));

    let file = fopen(cstr!("/proc/sys/kernel/core_pattern"), cstr!("w"));
    if file.is_null() {
        reason = cstr!("Unable to open core_pattern");
        fprintf(stderr, cstr!("Failed to cleanup coredump test: %s\n"), reason);
        return;
    }

    ret = fprintf(file, cstr!("%s"), (*self_).original_core_pattern.as_ptr());
    if ret < 0 {
        reason = cstr!("Unable to write to core_pattern");
        fprintf(stderr, cstr!("Failed to cleanup coredump test: %s\n"), reason);
        return;
    }

    ret = fclose(file);
    if ret != 0 {
        reason = cstr!("Unable to close core_pattern");
        fprintf(stderr, cstr!("Failed to cleanup coredump test: %s\n"), reason);
        return;
    }

    if (*self_).fd_tmpfs_detached >= 0 {
        ret = close((*self_).fd_tmpfs_detached);
        if ret < 0 {
            reason = cstr!("Unable to close detached tmpfs");
            fprintf(stderr, cstr!("Failed to cleanup coredump test: %s\n"), reason);
            return;
        }
        (*self_).fd_tmpfs_detached = -1;
    }
}

unsafe fn run_socket_request(
    self_: *mut coredump,
    metadata: *mut c_void,
    name: *const c_char,
    ack_flags: u64,
    ack_size: u64,
    expected_marker: u64,
    expect_core_data: bool,
    expect_wcore: bool,
    write_file: bool,
) {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pid_coredump_server: pid_t;
    let mut info: pidfd_info = zeroed();
    let mut ipc_sockets = [0 as c_int; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(cstr!("@@/tmp/coredump.socket")));
    ret = socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut req: coredump_req = zeroed();
        let mut fd_server = -1;
        let mut fd_coredump = -1;
        let mut fd_core_file = -1;
        let mut fd_peer_pidfd = -1;
        let mut exit_code = EXIT_FAILURE;

        'out: loop {
            close(ipc_sockets[0]);
            fd_server = create_and_listen_unix_socket(cstr!("/tmp/coredump.socket"));
            if fd_server < 0 { fprintf(stderr, cstr!("%s: create_and_listen_unix_socket failed: %m\n"), name); break 'out; }
            if write_nointr(ipc_sockets[1], cstr!("1") as *const c_void, 1) < 0 { fprintf(stderr, cstr!("%s: write_nointr to ipc socket failed: %m\n"), name); break 'out; }
            close(ipc_sockets[1]);
            fd_coredump = accept4(fd_server, null_mut(), null_mut(), SOCK_CLOEXEC);
            if fd_coredump < 0 { fprintf(stderr, cstr!("%s: accept4 failed: %m\n"), name); break 'out; }
            fd_peer_pidfd = get_peer_pidfd(fd_coredump);
            if fd_peer_pidfd < 0 { fprintf(stderr, cstr!("%s: get_peer_pidfd failed\n"), name); break 'out; }
            if !get_pidfd_info(fd_peer_pidfd, &mut info) { fprintf(stderr, cstr!("%s: get_pidfd_info failed\n"), name); break 'out; }
            if (info.mask & PIDFD_INFO_COREDUMP) == 0 { fprintf(stderr, cstr!("%s: PIDFD_INFO_COREDUMP not set in mask\n"), name); break 'out; }
            if (info.coredump_mask & PIDFD_COREDUMPED) == 0 { fprintf(stderr, cstr!("%s: PIDFD_COREDUMPED not set in coredump_mask\n"), name); break 'out; }
            if write_file {
                fd_core_file = creat(cstr!("/tmp/coredump.file"), 0o644);
                if fd_core_file < 0 { fprintf(stderr, cstr!("%s: creat coredump file failed: %m\n"), name); break 'out; }
            }
            if !read_coredump_req(fd_coredump, &mut req) { fprintf(stderr, cstr!("%s: read_coredump_req failed\n"), name); break 'out; }
            if !check_coredump_req(&req, COREDUMP_ACK_SIZE_VER0, COREDUMP_KERNEL | COREDUMP_USERSPACE | COREDUMP_REJECT | COREDUMP_WAIT) {
                fprintf(stderr, cstr!("%s: check_coredump_req failed\n"), name);
                break 'out;
            }
            if !send_coredump_ack(fd_coredump, &req, ack_flags, ack_size) { fprintf(stderr, cstr!("%s: send_coredump_ack failed\n"), name); break 'out; }
            if !read_marker(fd_coredump, expected_marker) { fprintf(stderr, cstr!("%s: read_marker failed\n"), name); break 'out; }
            if expect_core_data || !write_file {
                loop {
                    let mut buffer = [0u8; 4096];
                    let bytes_read = read(fd_coredump, buffer.as_mut_ptr() as *mut c_void, buffer.len());
                    if !expect_core_data && bytes_read > 0 { fprintf(stderr, cstr!("%s: unexpected data received (expected no coredump data)\n"), name); break 'out; }
                    if bytes_read < 0 { fprintf(stderr, cstr!("%s: read from coredump socket failed: %m\n"), name); break 'out; }
                    if bytes_read == 0 { break; }
                    if write_file {
                        let bytes_write = write(fd_core_file, buffer.as_ptr() as *const c_void, bytes_read as size_t);
                        if bytes_read != bytes_write {
                            if bytes_write < 0 && errno == ENOSPC { continue; }
                            fprintf(stderr, cstr!("%s: write to core file failed (read=%zd, write=%zd): %m\n"), name, bytes_read, bytes_write);
                            break 'out;
                        }
                    }
                }
            }
            exit_code = EXIT_SUCCESS;
            fprintf(stderr, cstr!("%s: completed successfully\n"), name);
            break 'out;
        }
        if fd_core_file >= 0 { close(fd_core_file); }
        if fd_peer_pidfd >= 0 { close(fd_peer_pidfd); }
        if fd_coredump >= 0 { close(fd_coredump); }
        if fd_server >= 0 { close(fd_server); }
        _exit(exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;

    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 { crashing_child(); }

    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    if expect_wcore { ASSERT_TRUE!(WCOREDUMP(status)); } else { ASSERT_FALSE!(WCOREDUMP(status)); }

    ASSERT_TRUE!(get_pidfd_info(pidfd, &mut info));
    ASSERT_GT!(info.mask & PIDFD_INFO_COREDUMP, 0);
    ASSERT_GT!(info.coredump_mask & PIDFD_COREDUMPED, 0);
    wait_and_check_coredump_server(pid_coredump_server, metadata, self_);
}

unsafe fn socket_request_kernel(self_: *mut coredump, metadata: *mut c_void) {
    let mut st: stat = zeroed();
    run_socket_request(self_, metadata, cstr!("socket_request_kernel"), COREDUMP_KERNEL | COREDUMP_WAIT, 0, COREDUMP_MARK_REQACK, true, true, true);
    ASSERT_EQ!(stat(cstr!("/tmp/coredump.file"), &mut st), 0);
    ASSERT_GT!(st.st_size, 0);
    system(cstr!("file /tmp/coredump.file"));
}

unsafe fn socket_request_userspace(self_: *mut coredump, metadata: *mut c_void) {
    run_socket_request(self_, metadata, cstr!("socket_request_userspace"), COREDUMP_USERSPACE | COREDUMP_WAIT, 0, COREDUMP_MARK_REQACK, false, true, false);
}

unsafe fn socket_request_reject(self_: *mut coredump, metadata: *mut c_void) {
    run_socket_request(self_, metadata, cstr!("socket_request_reject"), COREDUMP_REJECT | COREDUMP_WAIT, 0, COREDUMP_MARK_REQACK, false, false, false);
}

unsafe fn socket_request_invalid_flag_combination(self_: *mut coredump, metadata: *mut c_void) {
    run_socket_request(self_, metadata, cstr!("socket_request_invalid_flag_combination"), COREDUMP_KERNEL | COREDUMP_REJECT | COREDUMP_WAIT, 0, COREDUMP_MARK_CONFLICTING, false, false, false);
}

unsafe fn socket_request_unknown_flag(self_: *mut coredump, metadata: *mut c_void) {
    run_socket_request(self_, metadata, cstr!("socket_request_unknown_flag"), 1u64 << 63, 0, COREDUMP_MARK_UNSUPPORTED, false, false, false);
}

unsafe fn socket_request_invalid_size_small(self_: *mut coredump, metadata: *mut c_void) {
    run_socket_request(self_, metadata, cstr!("socket_request_invalid_size_small"), COREDUMP_REJECT | COREDUMP_WAIT, COREDUMP_ACK_SIZE_VER0 / 2, COREDUMP_MARK_MINSIZE, false, false, false);
}

unsafe fn socket_request_invalid_size_large(self_: *mut coredump, metadata: *mut c_void) {
    run_socket_request(self_, metadata, cstr!("socket_request_invalid_size_large"), COREDUMP_REJECT | COREDUMP_WAIT, COREDUMP_ACK_SIZE_VER0 + PAGE_SIZE, COREDUMP_MARK_MAXSIZE, false, false, false);
}

/*
 * Test: PIDFD_INFO_COREDUMP_SIGNAL via socket coredump with SIGSEGV
 *
 * Verify that when using socket-based coredump protocol,
 * the coredump_signal field is correctly exposed as SIGSEGV.
 * Also check that the coredump_code field is correctly exposed
 * as SEGV_MAPERR.
 */
unsafe fn socket_coredump_signal_sigsegv(self_: *mut coredump, metadata: *mut c_void) {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pid_coredump_server: pid_t;
    let mut info: pidfd_info = zeroed();
    let mut ipc_sockets = [0 as c_int; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(cstr!("@@/tmp/coredump.socket")));
    ret = socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);
    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut req: coredump_req = zeroed();
        let mut fd_server = -1;
        let mut fd_coredump = -1;
        let mut fd_peer_pidfd = -1;
        let mut exit_code = EXIT_FAILURE;
        'out: loop {
            close(ipc_sockets[0]);
            fd_server = create_and_listen_unix_socket(cstr!("/tmp/coredump.socket"));
            if fd_server < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: create_and_listen_unix_socket failed: %m\n")); break 'out; }
            if write_nointr(ipc_sockets[1], cstr!("1") as *const c_void, 1) < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: write_nointr to ipc socket failed: %m\n")); break 'out; }
            close(ipc_sockets[1]);
            fd_coredump = accept4(fd_server, null_mut(), null_mut(), SOCK_CLOEXEC);
            if fd_coredump < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: accept4 failed: %m\n")); break 'out; }
            fd_peer_pidfd = get_peer_pidfd(fd_coredump);
            if fd_peer_pidfd < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: get_peer_pidfd failed\n")); break 'out; }
            if !get_pidfd_info(fd_peer_pidfd, &mut info) { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: get_pidfd_info failed\n")); break 'out; }
            if (info.mask & PIDFD_INFO_COREDUMP) == 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: PIDFD_INFO_COREDUMP not set in mask\n")); break 'out; }
            if (info.coredump_mask & PIDFD_COREDUMPED) == 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: PIDFD_COREDUMPED not set in coredump_mask\n")); break 'out; }
            /* Verify coredump_signal is available and correct */
            if (info.mask & PIDFD_INFO_COREDUMP_SIGNAL) == 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: PIDFD_INFO_COREDUMP_SIGNAL not set in mask\n")); break 'out; }
            if info.coredump_signal != SIGSEGV { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: coredump_signal=%d, expected SIGSEGV=%d\n"), info.coredump_signal, SIGSEGV); break 'out; }
            /* Verify coredump_code is available and correct */
            if (info.mask & PIDFD_INFO_COREDUMP_CODE) == 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: PIDFD_INFO_COREDUMP_CODE not set in mask\n")); break 'out; }
            if info.coredump_code != SEGV_MAPERR { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: coredump_code=%d, expected SEGV_MAPERR=%d\n"), info.coredump_code, SEGV_MAPERR); break 'out; }
            if !read_coredump_req(fd_coredump, &mut req) { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: read_coredump_req failed\n")); break 'out; }
            if !send_coredump_ack(fd_coredump, &req, COREDUMP_REJECT | COREDUMP_WAIT, 0) { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: send_coredump_ack failed\n")); break 'out; }
            if !read_marker(fd_coredump, COREDUMP_MARK_REQACK) { fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: read_marker COREDUMP_MARK_REQACK failed\n")); break 'out; }
            exit_code = EXIT_SUCCESS;
            fprintf(stderr, cstr!("socket_coredump_signal_sigsegv: completed successfully\n"));
            break 'out;
        }
        if fd_peer_pidfd >= 0 { close(fd_peer_pidfd); }
        if fd_coredump >= 0 { close(fd_coredump); }
        if fd_server >= 0 { close(fd_server); }
        _exit(exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;
    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 { crashing_child(); }
    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    ASSERT_EQ!(WTERMSIG(status), SIGSEGV);
    ASSERT_TRUE!(get_pidfd_info(pidfd, &mut info));
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP_SIGNAL) != 0);
    ASSERT_EQ!(info.coredump_signal, SIGSEGV);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP_CODE) != 0);
    ASSERT_EQ!(info.coredump_code, SEGV_MAPERR);
    wait_and_check_coredump_server(pid_coredump_server, metadata, self_);
}

/*
 * Test: PIDFD_INFO_COREDUMP_SIGNAL via socket coredump with SIGABRT
 *
 * Verify that when using socket-based coredump protocol,
 * the coredump_signal field is correctly exposed as SIGABRT.
 * Also check that the coredump_code field is correctly exposed
 * as SI_TKILL.
 */
unsafe fn socket_coredump_signal_sigabrt(self_: *mut coredump, metadata: *mut c_void) {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pid_coredump_server: pid_t;
    let mut info: pidfd_info = zeroed();
    let mut ipc_sockets = [0 as c_int; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(cstr!("@@/tmp/coredump.socket")));
    ret = socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);
    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut req: coredump_req = zeroed();
        let mut fd_server = -1;
        let mut fd_coredump = -1;
        let mut fd_peer_pidfd = -1;
        let mut exit_code = EXIT_FAILURE;
        'out: loop {
            close(ipc_sockets[0]);
            fd_server = create_and_listen_unix_socket(cstr!("/tmp/coredump.socket"));
            if fd_server < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: create_and_listen_unix_socket failed: %m\n")); break 'out; }
            if write_nointr(ipc_sockets[1], cstr!("1") as *const c_void, 1) < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: write_nointr to ipc socket failed: %m\n")); break 'out; }
            close(ipc_sockets[1]);
            fd_coredump = accept4(fd_server, null_mut(), null_mut(), SOCK_CLOEXEC);
            if fd_coredump < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: accept4 failed: %m\n")); break 'out; }
            fd_peer_pidfd = get_peer_pidfd(fd_coredump);
            if fd_peer_pidfd < 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: get_peer_pidfd failed\n")); break 'out; }
            if !get_pidfd_info(fd_peer_pidfd, &mut info) { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: get_pidfd_info failed\n")); break 'out; }
            if (info.mask & PIDFD_INFO_COREDUMP) == 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: PIDFD_INFO_COREDUMP not set in mask\n")); break 'out; }
            if (info.coredump_mask & PIDFD_COREDUMPED) == 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: PIDFD_COREDUMPED not set in coredump_mask\n")); break 'out; }
            /* Verify coredump_signal is available and correct */
            if (info.mask & PIDFD_INFO_COREDUMP_SIGNAL) == 0 { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: PIDFD_INFO_COREDUMP_SIGNAL not set in mask\n")); break 'out; }
            if info.coredump_signal != SIGABRT { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: coredump_signal=%d, expected SIGABRT=%d\n"), info.coredump_signal, SIGABRT); break 'out; }
            if info.coredump_code != SI_TKILL { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: coredump_code=%d, expected SI_TKILL=%d\n"), info.coredump_code, SI_TKILL); break 'out; }
            if !read_coredump_req(fd_coredump, &mut req) { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: read_coredump_req failed\n")); break 'out; }
            if !send_coredump_ack(fd_coredump, &req, COREDUMP_REJECT | COREDUMP_WAIT, 0) { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: send_coredump_ack failed\n")); break 'out; }
            if !read_marker(fd_coredump, COREDUMP_MARK_REQACK) { fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: read_marker COREDUMP_MARK_REQACK failed\n")); break 'out; }
            exit_code = EXIT_SUCCESS;
            fprintf(stderr, cstr!("socket_coredump_signal_sigabrt: completed successfully\n"));
            break 'out;
        }
        if fd_peer_pidfd >= 0 { close(fd_peer_pidfd); }
        if fd_coredump >= 0 { close(fd_coredump); }
        if fd_server >= 0 { close(fd_server); }
        _exit(exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;
    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 { abort(); }
    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    ASSERT_EQ!(WTERMSIG(status), SIGABRT);
    ASSERT_TRUE!(get_pidfd_info(pidfd, &mut info));
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP_SIGNAL) != 0);
    ASSERT_EQ!(info.coredump_signal, SIGABRT);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP_CODE) != 0);
    ASSERT_EQ!(info.coredump_code, SI_TKILL);
    wait_and_check_coredump_server(pid_coredump_server, metadata, self_);
}

unsafe fn socket_multiple_crashing_coredumps(self_: *mut coredump, metadata: *mut c_void) {
    let mut pidfd = [0 as c_int; NUM_CRASHING_COREDUMPS];
    let mut status = [0 as c_int; NUM_CRASHING_COREDUMPS];
    let mut pid = [0 as pid_t; NUM_CRASHING_COREDUMPS];
    let mut pid_coredump_server: pid_t;
    let mut info: pidfd_info = zeroed();
    let mut ipc_sockets = [0 as c_int; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(cstr!("@@/tmp/coredump.socket")));
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);
    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut fd_server = -1;
        let mut fd_coredump = -1;
        let mut fd_peer_pidfd = -1;
        let mut fd_core_file = -1;
        let mut exit_code = EXIT_FAILURE;
        let mut req: coredump_req = zeroed();
        'out: loop {
            close(ipc_sockets[0]);
            fd_server = create_and_listen_unix_socket(cstr!("/tmp/coredump.socket"));
            if fd_server < 0 { fprintf(stderr, cstr!("Failed to create and listen on unix socket\n")); break 'out; }
            if write_nointr(ipc_sockets[1], cstr!("1") as *const c_void, 1) < 0 { fprintf(stderr, cstr!("Failed to notify parent via ipc socket\n")); break 'out; }
            close(ipc_sockets[1]);
            for i in 0..NUM_CRASHING_COREDUMPS {
                fd_coredump = accept4(fd_server, null_mut(), null_mut(), SOCK_CLOEXEC);
                if fd_coredump < 0 { fprintf(stderr, cstr!("accept4 failed: %m\n")); break 'out; }
                fd_peer_pidfd = get_peer_pidfd(fd_coredump);
                if fd_peer_pidfd < 0 { fprintf(stderr, cstr!("get_peer_pidfd failed for fd %d: %m\n"), fd_coredump); break 'out; }
                if !get_pidfd_info(fd_peer_pidfd, &mut info) { fprintf(stderr, cstr!("get_pidfd_info failed for fd %d\n"), fd_peer_pidfd); break 'out; }
                if (info.mask & PIDFD_INFO_COREDUMP) == 0 { fprintf(stderr, cstr!("pidfd info missing PIDFD_INFO_COREDUMP for fd %d\n"), fd_peer_pidfd); break 'out; }
                if (info.coredump_mask & PIDFD_COREDUMPED) == 0 { fprintf(stderr, cstr!("pidfd info missing PIDFD_COREDUMPED for fd %d\n"), fd_peer_pidfd); break 'out; }
                if !read_coredump_req(fd_coredump, &mut req) { fprintf(stderr, cstr!("read_coredump_req failed for fd %d\n"), fd_coredump); break 'out; }
                if !check_coredump_req(&req, COREDUMP_ACK_SIZE_VER0, COREDUMP_KERNEL | COREDUMP_USERSPACE | COREDUMP_REJECT | COREDUMP_WAIT) { fprintf(stderr, cstr!("check_coredump_req failed for fd %d\n"), fd_coredump); break 'out; }
                if !send_coredump_ack(fd_coredump, &req, COREDUMP_KERNEL | COREDUMP_WAIT, 0) { fprintf(stderr, cstr!("send_coredump_ack failed for fd %d\n"), fd_coredump); break 'out; }
                if !read_marker(fd_coredump, COREDUMP_MARK_REQACK) { fprintf(stderr, cstr!("read_marker failed for fd %d\n"), fd_coredump); break 'out; }
                fd_core_file = open_coredump_tmpfile((*self_).fd_tmpfs_detached);
                if fd_core_file < 0 { fprintf(stderr, cstr!("%m - open_coredump_tmpfile failed for fd %d\n"), fd_coredump); break 'out; }
                loop {
                    let mut buffer = [0u8; 4096];
                    let bytes_read = read(fd_coredump, buffer.as_mut_ptr() as *mut c_void, buffer.len());
                    if bytes_read < 0 { fprintf(stderr, cstr!("read failed for fd %d: %m\n"), fd_coredump); break 'out; }
                    if bytes_read == 0 { break; }
                    let bytes_write = write(fd_core_file, buffer.as_ptr() as *const c_void, bytes_read as size_t);
                    if bytes_read != bytes_write {
                        if bytes_write < 0 && errno == ENOSPC { continue; }
                        fprintf(stderr, cstr!("write failed for fd %d: %m\n"), fd_core_file);
                        break 'out;
                    }
                }
                close(fd_core_file);
                close(fd_peer_pidfd);
                close(fd_coredump);
                fd_peer_pidfd = -1;
                fd_coredump = -1;
            }
            exit_code = EXIT_SUCCESS;
            break 'out;
        }
        if fd_core_file >= 0 { close(fd_core_file); }
        if fd_peer_pidfd >= 0 { close(fd_peer_pidfd); }
        if fd_coredump >= 0 { close(fd_coredump); }
        if fd_server >= 0 { close(fd_server); }
        _exit(exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;
    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);
    for i in 0..NUM_CRASHING_COREDUMPS {
        pid[i] = fork();
        ASSERT_GE!(pid[i], 0);
        if pid[i] == 0 { crashing_child(); }
        pidfd[i] = sys_pidfd_open(pid[i], 0);
        ASSERT_GE!(pidfd[i], 0);
    }
    for i in 0..NUM_CRASHING_COREDUMPS {
        waitpid(pid[i], &mut status[i], 0);
        ASSERT_TRUE!(WIFSIGNALED(status[i]));
        ASSERT_TRUE!(WCOREDUMP(status[i]));
    }
    for i in 0..NUM_CRASHING_COREDUMPS {
        info.mask = PIDFD_INFO_EXIT | PIDFD_INFO_COREDUMP;
        ASSERT_EQ!(ioctl(pidfd[i], PIDFD_GET_INFO, &mut info), 0);
        ASSERT_GT!(info.mask & PIDFD_INFO_COREDUMP, 0);
        ASSERT_GT!(info.coredump_mask & PIDFD_COREDUMPED, 0);
    }
    wait_and_check_coredump_server(pid_coredump_server, metadata, self_);
}

unsafe fn socket_multiple_crashing_coredumps_epoll_workers(self_: *mut coredump, metadata: *mut c_void) {
    let mut pidfd = [0 as c_int; NUM_CRASHING_COREDUMPS];
    let mut status = [0 as c_int; NUM_CRASHING_COREDUMPS];
    let mut pid = [0 as pid_t; NUM_CRASHING_COREDUMPS];
    let mut worker_pids = [0 as pid_t; NUM_CRASHING_COREDUMPS];
    let mut pid_coredump_server: pid_t;
    let mut info: pidfd_info = zeroed();
    let mut ipc_sockets = [0 as c_int; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(cstr!("@@/tmp/coredump.socket")));
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr()), 0);
    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut fd_server = -1;
        let mut exit_code = EXIT_FAILURE;
        let mut n_conns: usize = 0;
        'out: loop {
            close(ipc_sockets[0]);
            fd_server = create_and_listen_unix_socket(cstr!("/tmp/coredump.socket"));
            if fd_server < 0 { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: create_and_listen_unix_socket failed: %m\n")); break 'out; }
            if write_nointr(ipc_sockets[1], cstr!("1") as *const c_void, 1) < 0 { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: write_nointr to ipc socket failed: %m\n")); break 'out; }
            close(ipc_sockets[1]);
            while n_conns < NUM_CRASHING_COREDUMPS {
                let mut fd_coredump = -1;
                let mut fd_peer_pidfd = -1;
                let mut fd_core_file = -1;
                let mut req: coredump_req = zeroed();
                fd_coredump = accept4(fd_server, null_mut(), null_mut(), SOCK_CLOEXEC);
                if fd_coredump < 0 {
                    if errno == EAGAIN || errno == EWOULDBLOCK { continue; }
                    fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: accept4 failed: %m\n"));
                    break 'out;
                }
                fd_peer_pidfd = get_peer_pidfd(fd_coredump);
                if fd_peer_pidfd < 0 { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: get_peer_pidfd failed\n")); break 'out; }
                if !get_pidfd_info(fd_peer_pidfd, &mut info) { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: get_pidfd_info failed\n")); break 'out; }
                if (info.mask & PIDFD_INFO_COREDUMP) == 0 || (info.coredump_mask & PIDFD_COREDUMPED) == 0 { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: missing PIDFD_INFO_COREDUMP or PIDFD_COREDUMPED\n")); break 'out; }
                if !read_coredump_req(fd_coredump, &mut req) { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: read_coredump_req failed\n")); break 'out; }
                if !check_coredump_req(&req, COREDUMP_ACK_SIZE_VER0, COREDUMP_KERNEL | COREDUMP_USERSPACE | COREDUMP_REJECT | COREDUMP_WAIT) { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: check_coredump_req failed\n")); break 'out; }
                if !send_coredump_ack(fd_coredump, &req, COREDUMP_KERNEL | COREDUMP_WAIT, 0) { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: send_coredump_ack failed\n")); break 'out; }
                if !read_marker(fd_coredump, COREDUMP_MARK_REQACK) { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: read_marker failed\n")); break 'out; }
                fd_core_file = open_coredump_tmpfile((*self_).fd_tmpfs_detached);
                if fd_core_file < 0 { fprintf(stderr, cstr!("socket_multiple_crashing_coredumps_epoll_workers: open_coredump_tmpfile failed: %m\n")); break 'out; }
                let worker: pid_t = fork();
                if worker == 0 {
                    close(fd_server);
                    process_coredump_worker(fd_coredump, fd_peer_pidfd, fd_core_file);
                }
                worker_pids[n_conns] = worker;
                if fd_coredump >= 0 { close(fd_coredump); }
                if fd_peer_pidfd >= 0 { close(fd_peer_pidfd); }
                if fd_core_file >= 0 { close(fd_core_file); }
                n_conns += 1;
            }
            exit_code = EXIT_SUCCESS;
            break 'out;
        }
        if fd_server >= 0 { close(fd_server); }

        // Reap all worker processes
        for i in 0..n_conns {
            let mut wstatus: c_int = 0;
            if waitpid(worker_pids[i], &mut wstatus, 0) < 0 {
                fprintf(stderr, cstr!("Failed to wait for worker %d: %m\n"), worker_pids[i]);
            } else if WIFEXITED(wstatus) && WEXITSTATUS(wstatus) != EXIT_SUCCESS {
                fprintf(stderr, cstr!("Worker %d exited with error code %d\n"), worker_pids[i], WEXITSTATUS(wstatus));
                exit_code = EXIT_FAILURE;
            }
        }
        _exit(exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;
    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);
    for i in 0..NUM_CRASHING_COREDUMPS {
        pid[i] = fork();
        ASSERT_GE!(pid[i], 0);
        if pid[i] == 0 { crashing_child(); }
        pidfd[i] = sys_pidfd_open(pid[i], 0);
        ASSERT_GE!(pidfd[i], 0);
    }
    for i in 0..NUM_CRASHING_COREDUMPS {
        ASSERT_GE!(waitpid(pid[i], &mut status[i], 0), 0);
        ASSERT_TRUE!(WIFSIGNALED(status[i]));
        ASSERT_TRUE!(WCOREDUMP(status[i]));
    }
    for i in 0..NUM_CRASHING_COREDUMPS {
        info.mask = PIDFD_INFO_EXIT | PIDFD_INFO_COREDUMP;
        ASSERT_EQ!(ioctl(pidfd[i], PIDFD_GET_INFO, &mut info), 0);
        ASSERT_GT!(info.mask & PIDFD_INFO_COREDUMP, 0);
        ASSERT_GT!(info.coredump_mask & PIDFD_COREDUMPED, 0);
    }
    wait_and_check_coredump_server(pid_coredump_server, metadata, self_);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
