// SPDX-License-Identifier: GPL-2.0

// C source dependencies:
// errno.h, fcntl.h, limits.h, linux/types.h, poll.h, sched.h, signal.h,
// stdio.h, stdlib.h, string.h, syscall.h, sys/prctl.h, sys/wait.h,
// unistd.h, sys/socket.h, linux/kcmp.h, sys/stat.h, "pidfd.h",
// "kselftest_harness.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const EBADF: c_int = 9;
const ENOTDIR: c_int = 20;
const AF_LOCAL: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = 0o2000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const SIGKILL: c_int = 9;
const WEXITED: c_int = 0x00000004;
const WNOWAIT: c_int = 0x01000000;
const P_PID: c_int = 1;
const AT_EMPTY_PATH: c_int = 0x1000;
const AT_FDCWD: c_int = -100;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o0004000;
const O_NDELAY: c_int = O_NONBLOCK;
const O_EXCL: c_int = 0o0000200;
const O_CREAT: c_int = 0o0000100;
const O_NOCTTY: c_int = 0o0000400;
const O_TRUNC: c_int = 0o0001000;
const O_APPEND: c_int = 0o0002000;
const O_SYNC: c_int = 0o4010000;
const O_DSYNC: c_int = 0o10000;
const O_DIRECT: c_int = 0o40000;
const O_DIRECTORY: c_int = 0o200000;
const O_NOFOLLOW: c_int = 0o400000;
const O_NOATIME: c_int = 0o1000000;
const O_PATH: c_int = 0o10000000;
const O_TMPFILE: c_int = 0o20200000;
const FASYNC: c_int = 0o20000;
const F_GETFL: c_int = 3;
const PIDFD_THREAD: c_int = O_EXCL;
const MAX_HANDLE_SZ: c_uint = 128;
const FD_PIDFS_ROOT: c_int = -10002;

const AT_HANDLE_CONNECTABLE: c_int = 0x002;
const AT_HANDLE_FID: c_int = 0x200;

#[repr(C)]
struct file_handle {
    handle_bytes: c_uint,
    handle_type: c_int,
    f_handle: [u8; 0],
}

#[repr(C)]
struct stat {
    st_dev: u64,
    st_ino: u64,
    _rest: [u8; 0],
}

#[repr(C)]
struct file_handle_fixture {
    pid: pid_t,
    pidfd: c_int,

    child_pid1: pid_t,
    child_pidfd1: c_int,

    child_pid2: pid_t,
    child_pidfd2: c_int,

    child_pid3: pid_t,
    child_pidfd3: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn getpid() -> pid_t;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pause() -> c_int;
    fn _exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn name_to_handle_at(
        dirfd: c_int,
        pathname: *const c_char,
        handle: *mut file_handle,
        mount_id: *mut c_int,
        flags: c_int,
    ) -> c_int;
    fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int;

    fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> c_int;
    fn sys_pidfd_send_signal(pidfd: c_int, sig: c_int, info: *mut c_void, flags: c_uint) -> c_int;
    fn sys_waitid(idtype: c_int, id: pid_t, infop: *mut c_void, options: c_int) -> c_int;
    fn create_child(pidfd: *mut c_int, flags: c_int) -> pid_t;
    fn write_nointr(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read_nointr(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn wait_for_pid(pid: pid_t) -> c_int;
}

unsafe fn file_handle_setup(self_: *mut file_handle_fixture) {
    let mut ret: c_int;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;

    (*self_).pid = getpid();
    (*self_).pidfd = sys_pidfd_open((*self_).pid, 0);
    ASSERT_GE!((*self_).pidfd, 0);

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    (*self_).child_pid1 = create_child(&mut (*self_).child_pidfd1, CLONE_NEWUSER);
    EXPECT_GE!((*self_).child_pid1, 0);

    if (*self_).child_pid1 == 0 {
        close(ipc_sockets[0]);

        if write_nointr(ipc_sockets[1], c"1".as_ptr() as *const c_void, 1) < 0 {
            _exit(EXIT_FAILURE);
        }

        close(ipc_sockets[1]);

        pause();
        _exit(EXIT_SUCCESS);
    }

    close(ipc_sockets[1]);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut c_char as *mut c_void, 1), 1);
    close(ipc_sockets[0]);

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    (*self_).child_pid2 = create_child(&mut (*self_).child_pidfd2, CLONE_NEWUSER | CLONE_NEWPID);
    EXPECT_GE!((*self_).child_pid2, 0);

    if (*self_).child_pid2 == 0 {
        close(ipc_sockets[0]);

        if write_nointr(ipc_sockets[1], c"1".as_ptr() as *const c_void, 1) < 0 {
            _exit(EXIT_FAILURE);
        }

        close(ipc_sockets[1]);

        pause();
        _exit(EXIT_SUCCESS);
    }

    close(ipc_sockets[1]);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut c_char as *mut c_void, 1), 1);
    close(ipc_sockets[0]);

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    EXPECT_EQ!(ret, 0);

    (*self_).child_pid3 = create_child(&mut (*self_).child_pidfd3, CLONE_NEWUSER | CLONE_NEWPID);
    EXPECT_GE!((*self_).child_pid3, 0);

    if (*self_).child_pid3 == 0 {
        close(ipc_sockets[0]);

        if write_nointr(ipc_sockets[1], c"1".as_ptr() as *const c_void, 1) < 0 {
            _exit(EXIT_FAILURE);
        }

        close(ipc_sockets[1]);

        pause();
        _exit(EXIT_SUCCESS);
    }

    close(ipc_sockets[1]);
    ASSERT_EQ!(read_nointr(ipc_sockets[0], &mut c as *mut c_char as *mut c_void, 1), 1);
    close(ipc_sockets[0]);
}

unsafe fn file_handle_teardown(self_: *mut file_handle_fixture) {
    EXPECT_EQ!(close((*self_).pidfd), 0);

    EXPECT_EQ!(sys_pidfd_send_signal((*self_).child_pidfd1, SIGKILL, ptr::null_mut(), 0), 0);
    if (*self_).child_pidfd1 >= 0 {
        EXPECT_EQ!(0, close((*self_).child_pidfd1));
    }

    EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid1, ptr::null_mut(), WEXITED), 0);

    EXPECT_EQ!(sys_pidfd_send_signal((*self_).child_pidfd2, SIGKILL, ptr::null_mut(), 0), 0);
    if (*self_).child_pidfd2 >= 0 {
        EXPECT_EQ!(0, close((*self_).child_pidfd2));
    }

    EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid2, ptr::null_mut(), WEXITED), 0);

    if (*self_).child_pidfd3 >= 0 {
        EXPECT_EQ!(sys_pidfd_send_signal((*self_).child_pidfd3, SIGKILL, ptr::null_mut(), 0), 0);
        EXPECT_EQ!(0, close((*self_).child_pidfd3));
        EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid3, ptr::null_mut(), WEXITED), 0);
    }
}

unsafe fn alloc_file_handle() -> *mut file_handle {
    let fh = malloc(mem::size_of::<file_handle>() + MAX_HANDLE_SZ as usize) as *mut file_handle;
    ASSERT_NE!(fh, ptr::null_mut());
    memset(fh as *mut c_void, 0, mem::size_of::<file_handle>() + MAX_HANDLE_SZ as usize);
    (*fh).handle_bytes = MAX_HANDLE_SZ;
    fh
}

/*
 * Test that we can decode a pidfs file handle in the same pid
 * namespace.
 */
unsafe fn file_handle_same_pidns(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;
    let mut pidfd: c_int = -EBADF;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).child_pidfd1, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    ASSERT_EQ!(fstat((*self_).child_pidfd1, &mut st1), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, O_CLOEXEC);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, O_NONBLOCK);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    free(fh as *mut c_void);
}

/*
 * Test that we can decode a pidfs file handle from a child pid
 * namespace.
 */
unsafe fn file_handle_child_pidns(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;
    let mut pidfd: c_int = -EBADF;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).child_pidfd2, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    ASSERT_EQ!(fstat((*self_).child_pidfd2, &mut st1), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, O_CLOEXEC);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, O_NONBLOCK);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    free(fh as *mut c_void);
}

/*
 * Test that we fail to decode a pidfs file handle from an ancestor
 * child pid namespace.
 */
unsafe fn file_handle_foreign_pidns(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;
    let mut pid: pid_t;

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).pidfd, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    ASSERT_EQ!(setns((*self_).child_pidfd2, CLONE_NEWUSER | CLONE_NEWPID), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
        if pidfd >= 0 {
            TH_LOG!("Managed to open pidfd outside of the caller's pid namespace hierarchy");
            _exit(1);
        }
        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);

    free(fh as *mut c_void);
}

/*
 * Test that we can decode a pidfs file handle of a process that has
 * exited but not been reaped.
 */
unsafe fn pid_has_exited(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let mut pidfd: c_int;
    let mut child_pidfd3: c_int;
    let fh: *mut file_handle;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).child_pidfd3, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    ASSERT_EQ!(fstat((*self_).child_pidfd3, &mut st1), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    child_pidfd3 = (*self_).child_pidfd3;
    (*self_).child_pidfd3 = -EBADF;
    EXPECT_EQ!(sys_pidfd_send_signal(child_pidfd3, SIGKILL, ptr::null_mut(), 0), 0);
    EXPECT_EQ!(close(child_pidfd3), 0);
    EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid3, ptr::null_mut(), WEXITED | WNOWAIT), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_GE!(pidfd, 0);

    EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid3, ptr::null_mut(), WEXITED), 0);
}

/*
 * Test that we fail to decode a pidfs file handle of a process that has
 * already been reaped.
 */
unsafe fn pid_has_been_reaped(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let mut pidfd: c_int;
    let mut child_pidfd3: c_int;
    let fh: *mut file_handle;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).child_pidfd3, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    ASSERT_EQ!(fstat((*self_).child_pidfd3, &mut st1), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    child_pidfd3 = (*self_).child_pidfd3;
    (*self_).child_pidfd3 = -EBADF;
    EXPECT_EQ!(sys_pidfd_send_signal(child_pidfd3, SIGKILL, ptr::null_mut(), 0), 0);
    EXPECT_EQ!(close(child_pidfd3), 0);
    EXPECT_EQ!(sys_waitid(P_PID, (*self_).child_pid3, ptr::null_mut(), WEXITED), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_LT!(pidfd, 0);
}

/*
 * Test valid flags to open a pidfd file handle. Note, that
 * PIDFD_NONBLOCK is defined as O_NONBLOCK and O_NONBLOCK is an alias to
 * O_NDELAY. Also note that PIDFD_THREAD is an alias for O_EXCL.
 */
unsafe fn open_by_handle_at_valid_flags(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;
    let mut pidfd: c_int = -EBADF;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).child_pidfd2, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    ASSERT_EQ!(fstat((*self_).child_pidfd2, &mut st1), 0);

    pidfd = open_by_handle_at(
        (*self_).pidfd,
        fh,
        O_RDONLY | O_WRONLY | O_RDWR | O_NONBLOCK | O_NDELAY | O_CLOEXEC | O_EXCL,
    );
    ASSERT_GE!(pidfd, 0);
    ASSERT_NE!(fcntl(pidfd, F_GETFL) & PIDFD_THREAD, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);
}

#[repr(C)]
struct invalid_pidfs_file_handle_flags {
    oflag: c_int,
    oflag_name: *const c_char,
}

/*
 * Test that invalid flags passed to open a pidfd file handle are
 * rejected.
 */
unsafe fn open_by_handle_at_invalid_flags(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;
    let mut pidfd: c_int = -EBADF;
    static invalid_pidfs_file_handle_flags_: [invalid_pidfs_file_handle_flags; 14] = [
        invalid_pidfs_file_handle_flags { oflag: FASYNC, oflag_name: c"FASYNC".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_CREAT, oflag_name: c"O_CREAT".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_NOCTTY, oflag_name: c"O_NOCTTY".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_CREAT, oflag_name: c"O_CREAT".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_TRUNC, oflag_name: c"O_TRUNC".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_APPEND, oflag_name: c"O_APPEND".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_SYNC, oflag_name: c"O_SYNC".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_DSYNC, oflag_name: c"O_DSYNC".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_DIRECT, oflag_name: c"O_DIRECT".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_DIRECTORY, oflag_name: c"O_DIRECTORY".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_NOFOLLOW, oflag_name: c"O_NOFOLLOW".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_NOATIME, oflag_name: c"O_NOATIME".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_PATH, oflag_name: c"O_PATH".as_ptr() },
        invalid_pidfs_file_handle_flags { oflag: O_TMPFILE, oflag_name: c"O_TMPFILE".as_ptr() },
        /*
         * O_LARGEFILE is added implicitly by
         * open_by_handle_at() so pidfs simply masks it off.
         */
    ];

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).child_pidfd2, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    for i in 0..invalid_pidfs_file_handle_flags_.len() {
        pidfd = open_by_handle_at((*self_).pidfd, fh, invalid_pidfs_file_handle_flags_[i].oflag);
        ASSERT_LT!(pidfd, 0, {
            TH_LOG!(
                "open_by_handle_at() succeeded with invalid flags: %s",
                invalid_pidfs_file_handle_flags_[i].oflag_name
            );
        });
    }
}

/* Test that lookup fails. */
unsafe fn lookup_must_fail(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;

    fh = alloc_file_handle();

    ASSERT_NE!(
        name_to_handle_at(
            (*self_).child_pidfd2,
            c"lookup-is-not-possible-with-pidfs".as_ptr(),
            fh,
            &mut mnt_id,
            AT_EMPTY_PATH,
        ),
        0
    );
    ASSERT_EQ!(errno, ENOTDIR);
    ASSERT_NE!(
        name_to_handle_at(
            (*self_).child_pidfd2,
            c"lookup-is-not-possible-with-pidfs".as_ptr(),
            fh,
            &mut mnt_id,
            0,
        ),
        0
    );
    ASSERT_EQ!(errno, ENOTDIR);
}

/*
 * Test that AT_HANDLE_CONNECTABLE is rejected. Connectable file handles
 * don't make sense for pidfs. Note that currently AT_HANDLE_CONNECTABLE
 * is rejected because it is incompatible with AT_EMPTY_PATH which is
 * required with pidfds as we don't support lookup.
 */
unsafe fn invalid_name_to_handle_at_flags(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;

    fh = alloc_file_handle();

    ASSERT_NE!(
        name_to_handle_at(
            (*self_).child_pidfd2,
            c"".as_ptr(),
            fh,
            &mut mnt_id,
            AT_EMPTY_PATH | AT_HANDLE_CONNECTABLE,
        ),
        0
    );
}

/*
 * Test that a request with AT_HANDLE_FID always leads to decodable file
 * handle as pidfs always provides export operations.
 */
unsafe fn valid_name_to_handle_at_flags(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let mut pidfd: c_int;
    let fh: *mut file_handle;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    fh = alloc_file_handle();

    ASSERT_EQ!(
        name_to_handle_at(
            (*self_).child_pidfd2,
            c"".as_ptr(),
            fh,
            &mut mnt_id,
            AT_EMPTY_PATH | AT_HANDLE_FID,
        ),
        0
    );

    ASSERT_EQ!(fstat((*self_).child_pidfd2, &mut st1), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);
}

/*
 * That we decode a file handle without having to pass a pidfd.
 */
unsafe fn decode_purely_based_on_file_handle(self_: *mut file_handle_fixture) {
    let mut mnt_id: c_int = 0;
    let fh: *mut file_handle;
    let mut pidfd: c_int = -EBADF;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    fh = alloc_file_handle();

    ASSERT_EQ!(name_to_handle_at((*self_).child_pidfd1, c"".as_ptr(), fh, &mut mnt_id, AT_EMPTY_PATH), 0);

    ASSERT_EQ!(fstat((*self_).child_pidfd1, &mut st1), 0);

    pidfd = open_by_handle_at(FD_PIDFS_ROOT, fh, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at(FD_PIDFS_ROOT, fh, O_CLOEXEC);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at(FD_PIDFS_ROOT, fh, O_NONBLOCK);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at((*self_).pidfd, fh, 0);
    ASSERT_GE!(pidfd, 0);

    ASSERT_EQ!(fstat(pidfd, &mut st2), 0);
    ASSERT_TRUE!(st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino);

    ASSERT_EQ!(close(pidfd), 0);

    pidfd = open_by_handle_at(-EBADF, fh, 0);
    ASSERT_LT!(pidfd, 0);

    pidfd = open_by_handle_at(AT_FDCWD, fh, 0);
    ASSERT_LT!(pidfd, 0);

    free(fh as *mut c_void);
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
