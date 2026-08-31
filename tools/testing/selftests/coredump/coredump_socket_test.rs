// SPDX-License-Identifier: GPL-2.0

// Translated from coredump_socket_test.c.
// C include dependencies:
// <sys/stat.h>, <sys/epoll.h>, <sys/socket.h>, <sys/un.h>, "coredump_test.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type pid_t = c_int;
type ssize_t = isize;
type size_t = usize;

const ESRCH: c_int = 3;
const ENOENT: c_int = 2;
const ENOSPC: c_int = 28;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = 0o2000000;
const SIGTERM: c_int = 15;
const SIGKILL: c_int = 9;
const SIGSEGV: c_int = 11;
const SIGABRT: c_int = 6;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const SEGV_MAPERR: c_int = 1;
const SI_TKILL: c_int = -6;

extern "C" {
    static mut stderr: *mut FILE;
    static mut errno: c_int;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn feof(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn unlink(pathname: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn accept4(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t, flags: c_int) -> c_int;
    fn creat(pathname: *const c_char, mode: c_uint) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn _exit(status: c_int) -> !;
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn pause() -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn abort() -> !;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;

    fn create_detached_tmpfs() -> c_int;
    fn set_core_pattern(pattern: *const c_char) -> bool;
    fn create_and_listen_unix_socket(path: *const c_char) -> c_int;
    fn write_nointr(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read_nointr(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn get_peer_pidfd(fd: c_int) -> c_int;
    fn get_pidfd_info(pidfd: c_int, info: *mut pidfd_info) -> bool;
    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;
    fn sys_pidfd_send_signal(pidfd: c_int, sig: c_int, info: *mut c_void, flags: c_uint) -> c_int;
    fn wait_and_check_coredump_server(pid: pid_t, metadata: *mut c_void, self_: *mut coredump) -> ();
    fn crashing_child() -> !;
    fn open_coredump_tmpfile(fd_tmpfs_detached: c_int) -> c_int;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct sockaddr {
    sa_family: c_uint,
    sa_data: [c_char; 14],
}

type socklen_t = c_uint;

#[repr(C)]
struct sockaddr_un {
    sun_family: c_uint,
    sun_path: [c_char; 108],
}

#[repr(C)]
struct stat {
    _opaque: [u8; 0],
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
struct coredump {
    pid_coredump_server: pid_t,
    fd_tmpfs_detached: c_int,
    original_core_pattern: [c_char; 4096],
}

const PIDFD_INFO_COREDUMP: u64 = 1 << 0;
const PIDFD_COREDUMPED: u64 = 1 << 0;
const PIDFD_INFO_COREDUMP_SIGNAL: u64 = 1 << 1;
const PIDFD_INFO_COREDUMP_CODE: u64 = 1 << 2;

macro_rules! ASSERT_NE { ($left:expr, $right:expr) => { assert_ne!($left, $right) }; }
macro_rules! ASSERT_TRUE { ($expr:expr) => { assert!($expr) }; }
macro_rules! ASSERT_FALSE { ($expr:expr) => { assert!(!$expr) }; }
macro_rules! ASSERT_LT { ($left:expr, $right:expr) => { assert!($left < $right) }; }
macro_rules! ASSERT_GE { ($left:expr, $right:expr) => { assert!($left >= $right) }; }
macro_rules! ASSERT_GT { ($left:expr, $right:expr) => { assert!($left > $right) }; }
macro_rules! ASSERT_EQ { ($left:expr, $right:expr) => { assert_eq!($left, $right) }; }
macro_rules! EXPECT_EQ { ($left:expr, $right:expr) => { assert_eq!($left, $right) }; }

fn WIFSIGNALED(status: c_int) -> bool {
    ((status & 0x7f) + 1) >> 1 > 0
}

fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

fn WCOREDUMP(status: c_int) -> bool {
    (status & 0x80) != 0
}

unsafe fn coredump_fixture_setup(self_: *mut coredump) {
    let mut file: *mut FILE;
    let mut ret: c_int;

    (*self_).pid_coredump_server = -ESRCH;
    (*self_).fd_tmpfs_detached = -1;
    file = fopen(c"/proc/sys/kernel/core_pattern".as_ptr(), c"r".as_ptr());
    ASSERT_NE!(core::ptr::null_mut::<FILE>(), file);

    ret = fread(
        (*self_).original_core_pattern.as_mut_ptr() as *mut c_void,
        1,
        core::mem::size_of_val(&(*self_).original_core_pattern),
        file,
    ) as c_int;
    ASSERT_TRUE!(ret != 0 || feof(file) != 0);
    ASSERT_LT!(ret as usize, core::mem::size_of_val(&(*self_).original_core_pattern));

    (*self_).original_core_pattern[ret as usize] = b'\0' as c_char;
    (*self_).fd_tmpfs_detached = create_detached_tmpfs();
    ASSERT_GE!((*self_).fd_tmpfs_detached, 0);

    ret = fclose(file);
    ASSERT_EQ!(0, ret);
}

unsafe fn coredump_fixture_teardown(self_: *mut coredump) {
    let reason: *const c_char;
    let mut file: *mut FILE;
    let mut ret: c_int;
    let mut status: c_int = 0;

    if (*self_).pid_coredump_server > 0 {
        kill((*self_).pid_coredump_server, SIGTERM);
        waitpid((*self_).pid_coredump_server, &mut status, 0);
    }
    unlink(c"/tmp/coredump.file".as_ptr());
    unlink(c"/tmp/coredump.socket".as_ptr());

    file = fopen(c"/proc/sys/kernel/core_pattern".as_ptr(), c"w".as_ptr());
    if file.is_null() {
        reason = c"Unable to open core_pattern".as_ptr();
        goto_fail(reason);
        return;
    }

    ret = fprintf(file, c"%s".as_ptr(), (*self_).original_core_pattern.as_ptr());
    if ret < 0 {
        reason = c"Unable to write to core_pattern".as_ptr();
        goto_fail(reason);
        return;
    }

    ret = fclose(file);
    if ret != 0 {
        reason = c"Unable to close core_pattern".as_ptr();
        goto_fail(reason);
        return;
    }

    if (*self_).fd_tmpfs_detached >= 0 {
        ret = close((*self_).fd_tmpfs_detached);
        if ret < 0 {
            reason = c"Unable to close detached tmpfs".as_ptr();
            goto_fail(reason);
            return;
        }
        (*self_).fd_tmpfs_detached = -1;
    }
}

unsafe fn goto_fail(reason: *const c_char) {
    /* This should never happen */
    fprintf(stderr, c"Failed to cleanup coredump test: %s\n".as_ptr(), reason);
}

unsafe fn coredump_socket(self_: *mut coredump, _metadata: *mut c_void) {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pid_coredump_server: pid_t;
    let mut st: stat = core::mem::zeroed();
    let mut info: pidfd_info = core::mem::zeroed();
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(c"@/tmp/coredump.socket".as_ptr()));

    ret = socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut fd_server: c_int = -1;
        let mut fd_coredump: c_int = -1;
        let mut fd_peer_pidfd: c_int = -1;
        let mut fd_core_file: c_int = -1;
        let mut exit_code: c_int = EXIT_FAILURE;

        close(ipc_sockets[0]);

        fd_server = create_and_listen_unix_socket(c"/tmp/coredump.socket".as_ptr());
        if fd_server < 0 {
            fprintf(stderr, c"socket test: create_and_listen_unix_socket failed: %m\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if write_nointr(ipc_sockets[1], c"1".as_ptr() as *const c_void, 1) < 0 {
            fprintf(stderr, c"socket test: write_nointr to ipc socket failed: %m\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        close(ipc_sockets[1]);

        fd_coredump = accept4(fd_server, core::ptr::null_mut(), core::ptr::null_mut(), SOCK_CLOEXEC);
        if fd_coredump < 0 {
            fprintf(stderr, c"socket test: accept4 failed: %m\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        fd_peer_pidfd = get_peer_pidfd(fd_coredump);
        if fd_peer_pidfd < 0 {
            fprintf(stderr, c"socket test: get_peer_pidfd failed\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if !get_pidfd_info(fd_peer_pidfd, &mut info) {
            fprintf(stderr, c"socket test: get_pidfd_info failed\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if (info.mask & PIDFD_INFO_COREDUMP) == 0 {
            fprintf(stderr, c"socket test: PIDFD_INFO_COREDUMP not set in mask\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if (info.coredump_mask & PIDFD_COREDUMPED) == 0 {
            fprintf(stderr, c"socket test: PIDFD_COREDUMPED not set in coredump_mask\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        fd_core_file = creat(c"/tmp/coredump.file".as_ptr(), 0o644);
        if fd_core_file < 0 {
            fprintf(stderr, c"socket test: creat coredump file failed: %m\n".as_ptr());
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        loop {
            let mut buffer: [c_char; 4096] = [0; 4096];
            let bytes_read: ssize_t;
            let bytes_write: ssize_t;

            bytes_read = read(fd_coredump, buffer.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buffer));
            if bytes_read < 0 {
                fprintf(stderr, c"socket test: read from coredump socket failed: %m\n".as_ptr());
                goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
            }

            if bytes_read == 0 {
                break;
            }

            bytes_write = write(fd_core_file, buffer.as_ptr() as *const c_void, bytes_read as size_t);
            if bytes_read != bytes_write {
                if bytes_write < 0 && errno == ENOSPC {
                    continue;
                }
                fprintf(stderr, c"socket test: write to core file failed (read=%zd, write=%zd): %m\n".as_ptr(), bytes_read, bytes_write);
                goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
            }
        }

        exit_code = EXIT_SUCCESS;
        fprintf(stderr, c"socket test: completed successfully\n".as_ptr());
        goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;

    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        crashing_child();
    }

    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);

    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    ASSERT_TRUE!(WCOREDUMP(status));

    ASSERT_TRUE!(get_pidfd_info(pidfd, &mut info));
    ASSERT_GT!(info.mask & PIDFD_INFO_COREDUMP, 0);
    ASSERT_GT!(info.coredump_mask & PIDFD_COREDUMPED, 0);

    wait_and_check_coredump_server(pid_coredump_server, _metadata, self_);

    ASSERT_EQ!(stat(c"/tmp/coredump.file".as_ptr(), &mut st), 0);
    ASSERT_GT!(st.st_size, 0);
}

unsafe fn goto_socket_out(fd_core_file: c_int, fd_peer_pidfd: c_int, fd_coredump: c_int, fd_server: c_int, exit_code: c_int) -> ! {
    if fd_core_file >= 0 {
        close(fd_core_file);
    }
    if fd_peer_pidfd >= 0 {
        close(fd_peer_pidfd);
    }
    if fd_coredump >= 0 {
        close(fd_coredump);
    }
    if fd_server >= 0 {
        close(fd_server);
    }
    _exit(exit_code);
}

unsafe fn coredump_socket_detect_userspace_client(self_: *mut coredump, _metadata: *mut c_void) {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pid_coredump_server: pid_t;
    let mut st: stat = core::mem::zeroed();
    let mut info: pidfd_info = pidfd_info {
        mask: PIDFD_INFO_COREDUMP,
        coredump_mask: 0,
        coredump_signal: 0,
        coredump_code: 0,
    };
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(c"@/tmp/coredump.socket".as_ptr()));

    ret = socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut fd_server: c_int = -1;
        let mut fd_coredump: c_int = -1;
        let mut fd_peer_pidfd: c_int = -1;
        let mut exit_code: c_int = EXIT_FAILURE;

        close(ipc_sockets[0]);

        fd_server = create_and_listen_unix_socket(c"/tmp/coredump.socket".as_ptr());
        if fd_server < 0 {
            fprintf(stderr, c"socket_detect_userspace_client: create_and_listen_unix_socket failed: %m\n".as_ptr());
            goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if write_nointr(ipc_sockets[1], c"1".as_ptr() as *const c_void, 1) < 0 {
            fprintf(stderr, c"socket_detect_userspace_client: write_nointr to ipc socket failed: %m\n".as_ptr());
            goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        close(ipc_sockets[1]);

        fd_coredump = accept4(fd_server, core::ptr::null_mut(), core::ptr::null_mut(), SOCK_CLOEXEC);
        if fd_coredump < 0 {
            fprintf(stderr, c"socket_detect_userspace_client: accept4 failed: %m\n".as_ptr());
            goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        fd_peer_pidfd = get_peer_pidfd(fd_coredump);
        if fd_peer_pidfd < 0 {
            fprintf(stderr, c"socket_detect_userspace_client: get_peer_pidfd failed\n".as_ptr());
            goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if !get_pidfd_info(fd_peer_pidfd, &mut info) {
            fprintf(stderr, c"socket_detect_userspace_client: get_pidfd_info failed\n".as_ptr());
            goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if (info.mask & PIDFD_INFO_COREDUMP) == 0 {
            fprintf(stderr, c"socket_detect_userspace_client: PIDFD_INFO_COREDUMP not set in mask\n".as_ptr());
            goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if (info.coredump_mask & PIDFD_COREDUMPED) != 0 {
            fprintf(stderr, c"socket_detect_userspace_client: PIDFD_COREDUMPED incorrectly set (should be userspace client)\n".as_ptr());
            goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        exit_code = EXIT_SUCCESS;
        fprintf(stderr, c"socket_detect_userspace_client: completed successfully\n".as_ptr());
        goto_detect_out(fd_peer_pidfd, fd_coredump, fd_server, exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;

    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        let fd_socket: c_int;
        let ret: ssize_t;
        let mut coredump_sk: sockaddr_un = core::mem::zeroed();
        coredump_sk.sun_family = AF_UNIX as c_uint;
        copy_cstr_to_sun_path(&mut coredump_sk, c"/tmp/coredump.socket".as_ptr());
        let coredump_sk_len: size_t =
            core::mem::offset_of!(sockaddr_un, sun_path) + core::mem::size_of_val(c"/tmp/coredump.socket".to_bytes_with_nul());

        fd_socket = socket(AF_UNIX, SOCK_STREAM, 0);
        if fd_socket < 0 {
            fprintf(stderr, c"socket_detect_userspace_client (client): socket failed: %m\n".as_ptr());
            _exit(EXIT_FAILURE);
        }

        ret = connect(fd_socket, &coredump_sk as *const _ as *const sockaddr, coredump_sk_len as socklen_t) as ssize_t;
        if ret < 0 {
            fprintf(stderr, c"socket_detect_userspace_client (client): connect failed: %m\n".as_ptr());
            _exit(EXIT_FAILURE);
        }

        close(fd_socket);
        pause();
        fprintf(stderr, c"socket_detect_userspace_client (client): completed successfully\n".as_ptr());
        _exit(EXIT_SUCCESS);
    }

    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_TRUE!(get_pidfd_info(pidfd, &mut info));
    ASSERT_GT!(info.mask & PIDFD_INFO_COREDUMP, 0);
    ASSERT_EQ!(info.coredump_mask & PIDFD_COREDUMPED, 0);

    wait_and_check_coredump_server(pid_coredump_server, _metadata, self_);

    ASSERT_EQ!(sys_pidfd_send_signal(pidfd, SIGKILL, core::ptr::null_mut(), 0), 0);
    ASSERT_EQ!(close(pidfd), 0);

    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    ASSERT_EQ!(WTERMSIG(status), SIGKILL);

    ASSERT_NE!(stat(c"/tmp/coredump.file".as_ptr(), &mut st), 0);
    ASSERT_EQ!(errno, ENOENT);
}

unsafe fn goto_detect_out(fd_peer_pidfd: c_int, fd_coredump: c_int, fd_server: c_int, exit_code: c_int) -> ! {
    if fd_peer_pidfd >= 0 {
        close(fd_peer_pidfd);
    }
    if fd_coredump >= 0 {
        close(fd_coredump);
    }
    if fd_server >= 0 {
        close(fd_server);
    }
    _exit(exit_code);
}

unsafe fn copy_cstr_to_sun_path(dst: &mut sockaddr_un, src: *const c_char) {
    let mut i = 0usize;
    while i < dst.sun_path.len() {
        let ch = *src.add(i);
        dst.sun_path[i] = ch;
        if ch == 0 {
            break;
        }
        i += 1;
    }
}

unsafe fn coredump_socket_enoent(_self: *mut coredump, _metadata: *mut c_void) {
    let mut pidfd: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;

    ASSERT_TRUE!(set_core_pattern(c"@/tmp/coredump.socket".as_ptr()));

    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        crashing_child();
    }

    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);

    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    ASSERT_FALSE!(WCOREDUMP(status));
}

unsafe fn coredump_socket_no_listener(self_: *mut coredump, _metadata: *mut c_void) {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pid_coredump_server: pid_t;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;
    let mut coredump_sk: sockaddr_un = core::mem::zeroed();
    coredump_sk.sun_family = AF_UNIX as c_uint;
    copy_cstr_to_sun_path(&mut coredump_sk, c"/tmp/coredump.socket".as_ptr());
    let coredump_sk_len: size_t = core::mem::offset_of!(sockaddr_un, sun_path)
        + core::mem::size_of_val(c"/tmp/coredump.socket".to_bytes_with_nul());

    ASSERT_TRUE!(set_core_pattern(c"@/tmp/coredump.socket".as_ptr()));

    ret = socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut fd_server: c_int = -1;
        let mut exit_code: c_int = EXIT_FAILURE;

        close(ipc_sockets[0]);

        fd_server = socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);
        if fd_server < 0 {
            fprintf(stderr, c"socket_no_listener: socket failed: %m\n".as_ptr());
            goto_no_listener_out(fd_server, ipc_sockets[1], exit_code);
        }

        ret = bind(fd_server, &coredump_sk as *const _ as *const sockaddr, coredump_sk_len as socklen_t);
        if ret < 0 {
            fprintf(stderr, c"socket_no_listener: bind failed: %m\n".as_ptr());
            goto_no_listener_out(fd_server, ipc_sockets[1], exit_code);
        }

        if write_nointr(ipc_sockets[1], c"1".as_ptr() as *const c_void, 1) < 0 {
            fprintf(stderr, c"socket_no_listener: write_nointr to ipc socket failed: %m\n".as_ptr());
            goto_no_listener_out(fd_server, ipc_sockets[1], exit_code);
        }

        exit_code = EXIT_SUCCESS;
        fprintf(stderr, c"socket_no_listener: completed successfully\n".as_ptr());
        goto_no_listener_out(fd_server, ipc_sockets[1], exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;

    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        crashing_child();
    }

    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);

    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    ASSERT_FALSE!(WCOREDUMP(status));

    wait_and_check_coredump_server(pid_coredump_server, _metadata, self_);
}

unsafe fn goto_no_listener_out(fd_server: c_int, ipc_socket: c_int, exit_code: c_int) -> ! {
    if fd_server >= 0 {
        close(fd_server);
    }
    close(ipc_socket);
    _exit(exit_code);
}

/*
 * Test: PIDFD_INFO_COREDUMP_SIGNAL via simple socket coredump
 *
 * Verify that when using simple socket-based coredump (@ pattern),
 * the coredump_signal field is correctly exposed as SIGSEGV.
 * Also check that the coredump_code field is correctly exposed
 * as SEGV_MAPERR.
 */
unsafe fn coredump_socket_coredump_signal_sigsegv(self_: *mut coredump, _metadata: *mut c_void) {
    coredump_socket_coredump_signal_common(
        self_,
        _metadata,
        c"socket_coredump_signal_sigsegv".as_ptr(),
        SIGSEGV,
        SEGV_MAPERR,
        true,
    );
}

/*
 * Test: PIDFD_INFO_COREDUMP_SIGNAL via simple socket coredump with SIGABRT
 *
 * Verify that when using simple socket-based coredump (@ pattern),
 * the coredump_signal field is correctly exposed as SIGABRT.
 * Also check that the coredump_code field is correctly exposed
 * as SI_TKILL.
 */
unsafe fn coredump_socket_coredump_signal_sigabrt(self_: *mut coredump, _metadata: *mut c_void) {
    coredump_socket_coredump_signal_common(
        self_,
        _metadata,
        c"socket_coredump_signal_sigabrt".as_ptr(),
        SIGABRT,
        SI_TKILL,
        false,
    );
}

unsafe fn coredump_socket_coredump_signal_common(
    self_: *mut coredump,
    _metadata: *mut c_void,
    name: *const c_char,
    expected_signal: c_int,
    expected_code: c_int,
    use_crashing_child: bool,
) {
    let mut pidfd: c_int;
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut pid: pid_t;
    let mut pid_coredump_server: pid_t;
    let mut info: pidfd_info = core::mem::zeroed();
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;

    ASSERT_TRUE!(set_core_pattern(c"@/tmp/coredump.socket".as_ptr()));

    ret = socketpair(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_EQ!(ret, 0);

    pid_coredump_server = fork();
    ASSERT_GE!(pid_coredump_server, 0);
    if pid_coredump_server == 0 {
        let mut fd_server: c_int = -1;
        let mut fd_coredump: c_int = -1;
        let mut fd_peer_pidfd: c_int = -1;
        let mut fd_core_file: c_int = -1;
        let mut exit_code: c_int = EXIT_FAILURE;

        close(ipc_sockets[0]);

        fd_server = create_and_listen_unix_socket(c"/tmp/coredump.socket".as_ptr());
        if fd_server < 0 {
            fprintf(stderr, c"%s: create_and_listen_unix_socket failed: %m\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if write_nointr(ipc_sockets[1], c"1".as_ptr() as *const c_void, 1) < 0 {
            fprintf(stderr, c"%s: write_nointr to ipc socket failed: %m\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        close(ipc_sockets[1]);

        fd_coredump = accept4(fd_server, core::ptr::null_mut(), core::ptr::null_mut(), SOCK_CLOEXEC);
        if fd_coredump < 0 {
            fprintf(stderr, c"%s: accept4 failed: %m\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        fd_peer_pidfd = get_peer_pidfd(fd_coredump);
        if fd_peer_pidfd < 0 {
            fprintf(stderr, c"%s: get_peer_pidfd failed\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if !get_pidfd_info(fd_peer_pidfd, &mut info) {
            fprintf(stderr, c"%s: get_pidfd_info failed\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if (info.mask & PIDFD_INFO_COREDUMP) == 0 {
            fprintf(stderr, c"%s: PIDFD_INFO_COREDUMP not set in mask\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if (info.coredump_mask & PIDFD_COREDUMPED) == 0 {
            fprintf(stderr, c"%s: PIDFD_COREDUMPED not set in coredump_mask\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        /* Verify coredump_signal is available and correct */
        if (info.mask & PIDFD_INFO_COREDUMP_SIGNAL) == 0 {
            fprintf(stderr, c"%s: PIDFD_INFO_COREDUMP_SIGNAL not set in mask\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if info.coredump_signal != expected_signal {
            fprintf(stderr, c"%s: coredump_signal=%d, expected signal=%d\n".as_ptr(), name, info.coredump_signal, expected_signal);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        /* Verify coredump_code is available and correct */
        if (info.mask & PIDFD_INFO_COREDUMP_CODE) == 0 {
            fprintf(stderr, c"%s: PIDFD_INFO_COREDUMP_CODE not set in mask\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        if info.coredump_code != expected_code {
            fprintf(stderr, c"%s: coredump_code=%d, expected code=%d\n".as_ptr(), name, info.coredump_code, expected_code);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        fd_core_file = open_coredump_tmpfile((*self_).fd_tmpfs_detached);
        if fd_core_file < 0 {
            fprintf(stderr, c"%s: open_coredump_tmpfile failed: %m\n".as_ptr(), name);
            goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
        }

        loop {
            let mut buffer: [c_char; 4096] = [0; 4096];
            let bytes_read: ssize_t;
            let bytes_write: ssize_t;

            bytes_read = read(fd_coredump, buffer.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&buffer));
            if bytes_read < 0 {
                fprintf(stderr, c"%s: read from coredump socket failed: %m\n".as_ptr(), name);
                goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
            }

            if bytes_read == 0 {
                break;
            }

            bytes_write = write(fd_core_file, buffer.as_ptr() as *const c_void, bytes_read as size_t);
            if bytes_read != bytes_write {
                fprintf(stderr, c"%s: write to core file failed (read=%zd, write=%zd): %m\n".as_ptr(), name, bytes_read, bytes_write);
                goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
            }
        }

        exit_code = EXIT_SUCCESS;
        fprintf(stderr, c"%s: completed successfully\n".as_ptr(), name);
        goto_socket_out(fd_core_file, fd_peer_pidfd, fd_coredump, fd_server, exit_code);
    }
    (*self_).pid_coredump_server = pid_coredump_server;

    EXPECT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut _ as *mut c_void, 1), 1);
    EXPECT_EQ!(close(ipc_sockets[0]), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        if use_crashing_child {
            crashing_child();
        } else {
            abort();
        }
    }

    pidfd = sys_pidfd_open(pid, 0);
    ASSERT_GE!(pidfd, 0);

    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFSIGNALED(status));
    ASSERT_EQ!(WTERMSIG(status), expected_signal);
    ASSERT_TRUE!(WCOREDUMP(status));

    ASSERT_TRUE!(get_pidfd_info(pidfd, &mut info));
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP) != 0);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP_SIGNAL) != 0);
    ASSERT_EQ!(info.coredump_signal, expected_signal);
    ASSERT_TRUE!((info.mask & PIDFD_INFO_COREDUMP_CODE) != 0);
    ASSERT_EQ!(info.coredump_code, expected_code);

    wait_and_check_coredump_server(pid_coredump_server, _metadata, self_);
}

unsafe fn coredump_socket_invalid_paths(_self: *mut coredump, _metadata: *mut c_void) {
    ASSERT_FALSE!(set_core_pattern(c"@ /tmp/coredump.socket".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@/tmp/../coredump.socket".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@../coredump.socket".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@/tmp/coredump.socket/..".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@..".as_ptr()));

    ASSERT_FALSE!(set_core_pattern(c"@@ /tmp/coredump.socket".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@@/tmp/../coredump.socket".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@@../coredump.socket".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@@/tmp/coredump.socket/..".as_ptr()));
    ASSERT_FALSE!(set_core_pattern(c"@@..".as_ptr()));

    ASSERT_FALSE!(set_core_pattern(c"@@@/tmp/coredump.socket".as_ptr()));
}

fn main() {}
