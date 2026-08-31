// SPDX-License-Identifier: GPL-2.0
// Rust translation of testing/selftests/namespaces/ns_active_ref_test.c.
// C include dependencies intentionally remain as external libc/kernel symbols:
// errno.h, fcntl.h, sched.h, stdio.h, stdlib.h, string.h, linux/nsfs.h,
// sys/mount.h, sys/socket.h, sys/stat.h, sys/types.h, sys/wait.h,
// sys/syscall.h, unistd.h, pthread.h, kselftest_harness.h,
// filesystems/utils.h, and wrappers.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type pthread_t = c_ulong;
type __u64 = u64;

const FD_NSFS_ROOT: c_int = -10003; /* Root of the nsfs filesystem */
const FILEID_NSFS: c_int = 0xf1;
const MAX_HANDLE_SZ: usize = 128;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const AT_EMPTY_PATH: c_int = 0x1000;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const ESTALE: c_int = 116;
const SIGKILL: c_int = 9;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWIPC: c_int = 0x08000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const CLONE_NEWNET: c_int = 0x40000000;
const MS_BIND: c_ulong = 4096;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;
const NS_GET_USERNS: c_ulong = 0xb701;
const NS_GET_ID: c_ulong = 0xb705;

#[repr(C)]
struct file_handle {
    handle_bytes: c_uint,
    handle_type: c_int,
    f_handle: [u8; 0],
}

#[repr(C)]
struct nsfs_file_handle {
    ns_id: __u64,
    ns_type: __u64,
    ns_inum: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    __unused: [c_long; 6],
}

#[repr(C)]
struct ns_id_req {
    size: c_uint,
    spare: c_uint,
    ns_id: __u64,
    ns_type: __u64,
    spare2: __u64,
    user_ns_id: __u64,
}

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn unshare(flags: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn usleep(usec: c_uint) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn name_to_handle_at(
        dirfd: c_int,
        pathname: *const c_char,
        handle: *mut file_handle,
        mount_id: *mut c_int,
        flags: c_int,
    ) -> c_int;
    fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn getuid() -> c_uint;
    fn getgid() -> c_uint;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_cancel(thread: pthread_t) -> c_int;
    fn setup_userns() -> c_int;
    fn sys_listns(req: *mut ns_id_req, ns_ids: *mut __u64, nr_ns_ids: c_uint, flags: c_uint) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ASSERT_EQ { ($a:expr, $b:expr) => { assert_eq!($a, $b); }; }
macro_rules! ASSERT_NE { ($a:expr, $b:expr) => { assert_ne!($a, $b); }; }
macro_rules! ASSERT_GE { ($a:expr, $b:expr) => { assert!($a >= $b); }; }
macro_rules! ASSERT_GT { ($a:expr, $b:expr) => { assert!($a > $b); }; }
macro_rules! ASSERT_LT { ($a:expr, $b:expr) => { assert!($a < $b); }; }
macro_rules! ASSERT_TRUE { ($a:expr) => { assert!($a); }; }
macro_rules! ASSERT_FALSE { ($a:expr) => { assert!(!$a); }; }
macro_rules! TH_LOG { ($($arg:tt)*) => {{ }}; }
macro_rules! SKIP {
    (return, $msg:expr) => {{ return; }};
    ($cleanup:expr, $msg:expr) => {{ $cleanup }};
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn fh_from_buf(buf: *mut c_char) -> *mut file_handle {
    buf as *mut file_handle
}

unsafe fn nsfs_fh(handle: *mut file_handle) -> *mut nsfs_file_handle {
    (*handle).f_handle.as_mut_ptr() as *mut nsfs_file_handle
}

unsafe fn init_nsfs_handle(buf: *mut c_char, ns_id: __u64) -> *mut file_handle {
    let handle = fh_from_buf(buf);
    (*handle).handle_bytes = size_of::<nsfs_file_handle>() as c_uint;
    (*handle).handle_type = FILEID_NSFS;
    let fh = nsfs_fh(handle);
    (*fh).ns_id = ns_id;
    (*fh).ns_type = 0;
    (*fh).ns_inum = 0;
    handle
}

/*
 * Test that initial namespaces can be reopened via file handle.
 * Initial namespaces should have active ref count of 1 from boot.
 */
unsafe fn init_ns_always_active() {
    let handle = malloc(size_of::<file_handle>() + MAX_HANDLE_SZ) as *mut file_handle;
    let mut mount_id: c_int = 0;
    let mut ret: c_int;
    let mut fd1: c_int;
    let fd2: c_int;
    let mut st1: stat = zeroed();
    let mut st2: stat = zeroed();

    ASSERT_NE!(handle, null_mut());
    fd1 = open(cstr!("/proc/1/ns/net"), O_RDONLY);
    ASSERT_GE!(fd1, 0);
    (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
    ret = name_to_handle_at(fd1, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
    if ret < 0 && errno() == EOPNOTSUPP {
        free(handle as *mut c_void);
        close(fd1);
        return;
    }
    ASSERT_EQ!(ret, 0);
    close(fd1);
    fd2 = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if fd2 < 0 && (errno() == EINVAL || errno() == EOPNOTSUPP) {
        free(handle as *mut c_void);
        return;
    }
    ASSERT_GE!(fd2, 0);
    fd1 = open(cstr!("/proc/1/ns/net"), O_RDONLY);
    ASSERT_GE!(fd1, 0);
    ASSERT_EQ!(fstat(fd1, &mut st1), 0);
    ASSERT_EQ!(fstat(fd2, &mut st2), 0);
    ASSERT_EQ!(st1.st_ino, st2.st_ino);
    close(fd1);
    close(fd2);
    free(handle as *mut c_void);
}

/*
 * Test namespace lifecycle: create a namespace in a child process,
 * get a file handle while it's active, then try to reopen after
 * the process exits (namespace becomes inactive).
 */
unsafe fn ns_inactive_after_exit() {
    let mut handle: *mut file_handle;
    let mut mount_id: c_int = 0;
    let mut ret: ssize_t;
    let mut fd: c_int;
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];

    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        let uret = unshare(CLONE_NEWNET);
        if uret < 0 {
            close(pipefd[1]);
            exit(1);
        }
        fd = open(cstr!("/proc/self/ns/net"), O_RDONLY);
        if fd < 0 {
            close(pipefd[1]);
            exit(1);
        }
        handle = fh_from_buf(buf.as_mut_ptr());
        (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
        let nret = name_to_handle_at(fd, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
        close(fd);
        if nret < 0 {
            close(pipefd[1]);
            exit(1);
        }
        write(pipefd[1], buf.as_ptr() as *const c_void, size_of::<file_handle>() + (*handle).handle_bytes as usize);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    ret = read(pipefd[0], buf.as_mut_ptr() as *mut c_void, buf.len());
    close(pipefd[0]);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    ASSERT_GT!(ret, 0);
    handle = fh_from_buf(buf.as_mut_ptr());
    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(fd, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
}

/*
 * Test that a namespace remains active while a process is using it,
 * even after the creating process exits.
 */
unsafe fn ns_active_with_multiple_processes() {
    let mut handle: *mut file_handle;
    let mut mount_id: c_int = 0;
    let mut ret: ssize_t;
    let mut fd: c_int;
    let mut pipefd = [0 as c_int; 2];
    let mut syncpipe = [0 as c_int; 2];
    let pid1: pid_t;
    let pid2: pid_t;
    let mut status: c_int = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut sync_byte: c_char = 0;

    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    ASSERT_EQ!(pipe(syncpipe.as_mut_ptr()), 0);
    pid1 = fork();
    ASSERT_GE!(pid1, 0);
    if pid1 == 0 {
        close(pipefd[0]);
        close(syncpipe[1]);
        if unshare(CLONE_NEWNET) < 0 {
            close(pipefd[1]);
            close(syncpipe[0]);
            exit(1);
        }
        fd = open(cstr!("/proc/self/ns/net"), O_RDONLY);
        if fd < 0 {
            close(pipefd[1]);
            close(syncpipe[0]);
            exit(1);
        }
        handle = fh_from_buf(buf.as_mut_ptr());
        (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
        let nret = name_to_handle_at(fd, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
        close(fd);
        if nret < 0 {
            close(pipefd[1]);
            close(syncpipe[0]);
            exit(1);
        }
        write(pipefd[1], buf.as_ptr() as *const c_void, size_of::<file_handle>() + (*handle).handle_bytes as usize);
        close(pipefd[1]);
        read(syncpipe[0], &mut sync_byte as *mut _ as *mut c_void, 1);
        close(syncpipe[0]);
        exit(0);
    }

    close(pipefd[1]);
    ret = read(pipefd[0], buf.as_mut_ptr() as *mut c_void, buf.len());
    close(pipefd[0]);
    ASSERT_GT!(ret, 0);
    handle = fh_from_buf(buf.as_mut_ptr());
    pid2 = fork();
    ASSERT_GE!(pid2, 0);
    if pid2 == 0 {
        close(syncpipe[0]);
        close(syncpipe[1]);
        fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
        if fd < 0 {
            exit(1);
        }
        let sret = setns(fd, CLONE_NEWNET);
        close(fd);
        if sret < 0 {
            exit(1);
        }
        sleep(1);
        exit(0);
    }
    usleep(100000);
    close(syncpipe[0]);
    sync_byte = b'X' as c_char;
    write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1);
    close(syncpipe[1]);
    waitpid(pid1, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_GE!(fd, 0);
    close(fd);
    waitpid(pid2, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
}

unsafe fn child_get_handle_for_ns(pipefd: *mut c_int, ns_path: *const c_char, clone_flag: c_int) {
    let mut mount_id: c_int = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    close(*pipefd.add(0));
    if unshare(clone_flag) < 0 {
        close(*pipefd.add(1));
        exit(1);
    }
    let fd = open(ns_path, O_RDONLY);
    if fd < 0 {
        close(*pipefd.add(1));
        exit(1);
    }
    let handle = fh_from_buf(buf.as_mut_ptr());
    (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
    let ret = name_to_handle_at(fd, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
    close(fd);
    if ret < 0 {
        close(*pipefd.add(1));
        exit(1);
    }
    write(*pipefd.add(1), buf.as_ptr() as *const c_void, size_of::<file_handle>() + (*handle).handle_bytes as usize);
    close(*pipefd.add(1));
    exit(0);
}

/*
 * Test user namespace active ref tracking via credential lifecycle
 */
unsafe fn userns_active_ref_lifecycle() {
    let mut handle: *mut file_handle;
    let mut mount_id: c_int = 0;
    let mut ret: ssize_t;
    let mut fd: c_int;
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];

    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if unshare(CLONE_NEWUSER) < 0 {
            close(pipefd[1]);
            exit(1);
        }
        let uid_map_fd = open(cstr!("/proc/self/uid_map"), O_WRONLY);
        let gid_map_fd = open(cstr!("/proc/self/gid_map"), O_WRONLY);
        let setgroups_fd = open(cstr!("/proc/self/setgroups"), O_WRONLY);
        if uid_map_fd >= 0 && gid_map_fd >= 0 && setgroups_fd >= 0 {
            write(setgroups_fd, cstr!("deny") as *const c_void, 4);
            close(setgroups_fd);
            let mut mapping = [0 as c_char; 64];
            snprintf(mapping.as_mut_ptr(), mapping.len(), cstr!("0 %d 1"), getuid());
            write(uid_map_fd, mapping.as_ptr() as *const c_void, strlen(mapping.as_ptr()));
            close(uid_map_fd);
            snprintf(mapping.as_mut_ptr(), mapping.len(), cstr!("0 %d 1"), getgid());
            write(gid_map_fd, mapping.as_ptr() as *const c_void, strlen(mapping.as_ptr()));
            close(gid_map_fd);
        }
        fd = open(cstr!("/proc/self/ns/user"), O_RDONLY);
        if fd < 0 {
            close(pipefd[1]);
            exit(1);
        }
        handle = fh_from_buf(buf.as_mut_ptr());
        (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
        let nret = name_to_handle_at(fd, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
        close(fd);
        if nret < 0 {
            close(pipefd[1]);
            exit(1);
        }
        write(pipefd[1], buf.as_ptr() as *const c_void, size_of::<file_handle>() + (*handle).handle_bytes as usize);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    ret = read(pipefd[0], buf.as_mut_ptr() as *mut c_void, buf.len());
    close(pipefd[0]);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    ASSERT_GT!(ret, 0);
    handle = fh_from_buf(buf.as_mut_ptr());
    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(fd, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
}

/*
 * Test PID namespace active ref tracking
 */
unsafe fn pidns_active_ref_lifecycle() {
    let mut handle: *mut file_handle;
    let mut mount_id: c_int = 0;
    let mut ret: ssize_t;
    let mut fd: c_int;
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];

    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if unshare(CLONE_NEWPID) < 0 {
            close(pipefd[1]);
            exit(1);
        }
        let child = fork();
        if child < 0 {
            close(pipefd[1]);
            exit(1);
        }
        if child == 0 {
            fd = open(cstr!("/proc/self/ns/pid"), O_RDONLY);
            if fd < 0 {
                exit(1);
            }
            handle = fh_from_buf(buf.as_mut_ptr());
            (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
            let nret = name_to_handle_at(fd, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
            close(fd);
            if nret < 0 {
                exit(1);
            }
            write(pipefd[1], buf.as_ptr() as *const c_void, size_of::<file_handle>() + (*handle).handle_bytes as usize);
            close(pipefd[1]);
            exit(0);
        }
        waitpid(child, null_mut(), 0);
        exit(0);
    }
    close(pipefd[1]);
    ret = read(pipefd[0], buf.as_mut_ptr() as *mut c_void, buf.len());
    close(pipefd[0]);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    ASSERT_GT!(ret, 0);
    handle = fh_from_buf(buf.as_mut_ptr());
    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(fd, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
}

/*
 * Test that an open file descriptor keeps a namespace active.
 * Even after the creating process exits, the namespace should remain
 * active as long as an fd is held open.
 */
unsafe fn ns_fd_keeps_active() {
    let mut handle: *mut file_handle;
    let mut mount_id: c_int = 0;
    let mut ret: ssize_t;
    let mut nsfd: c_int;
    let mut pipe_child_ready = [0 as c_int; 2];
    let mut pipe_parent_ready = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut sync_byte: c_char = 0;
    let mut proc_path = [0 as c_char; 64];

    ASSERT_EQ!(pipe(pipe_child_ready.as_mut_ptr()), 0);
    ASSERT_EQ!(pipe(pipe_parent_ready.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipe_child_ready[0]);
        close(pipe_parent_ready[1]);
        if unshare(CLONE_NEWNET) < 0 {
            close(pipe_child_ready[1]);
            close(pipe_parent_ready[0]);
            exit(1);
        }
        nsfd = open(cstr!("/proc/self/ns/net"), O_RDONLY);
        if nsfd < 0 {
            close(pipe_child_ready[1]);
            close(pipe_parent_ready[0]);
            exit(1);
        }
        handle = fh_from_buf(buf.as_mut_ptr());
        (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
        let nret = name_to_handle_at(nsfd, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
        close(nsfd);
        if nret < 0 {
            close(pipe_child_ready[1]);
            close(pipe_parent_ready[0]);
            exit(1);
        }
        ret = write(pipe_child_ready[1], buf.as_ptr() as *const c_void, size_of::<file_handle>() + (*handle).handle_bytes as usize);
        close(pipe_child_ready[1]);
        ret = read(pipe_parent_ready[0], &mut sync_byte as *mut _ as *mut c_void, 1);
        close(pipe_parent_ready[0]);
        exit(0);
    }
    close(pipe_child_ready[1]);
    close(pipe_parent_ready[0]);
    ret = read(pipe_child_ready[0], buf.as_mut_ptr() as *mut c_void, buf.len());
    close(pipe_child_ready[0]);
    ASSERT_GT!(ret, 0);
    handle = fh_from_buf(buf.as_mut_ptr());
    snprintf(proc_path.as_mut_ptr(), proc_path.len(), cstr!("/proc/%d/ns/net"), pid);
    nsfd = open(proc_path.as_ptr(), O_RDONLY);
    if nsfd < 0 {
        close(pipe_parent_ready[1]);
        kill(pid, SIGKILL);
        waitpid(pid, null_mut(), 0);
        return;
    }
    sync_byte = b'G' as c_char;
    write(pipe_parent_ready[1], &sync_byte as *const _ as *const c_void, 1);
    close(pipe_parent_ready[1]);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let mut fd2 = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_GE!(fd2, 0);
    let mut st1: stat = zeroed();
    let mut st2: stat = zeroed();
    ASSERT_EQ!(fstat(nsfd, &mut st1), 0);
    ASSERT_EQ!(fstat(fd2, &mut st2), 0);
    ASSERT_EQ!(st1.st_ino, st2.st_ino);
    close(fd2);
    close(nsfd);
    fd2 = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(fd2, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
}

unsafe fn read_u64_or_skip(fd: c_int, pid: pid_t, value: *mut __u64, msg: &str) -> bool {
    let ret = read(fd, value as *mut c_void, size_of::<__u64>());
    if ret != size_of::<__u64>() as ssize_t {
        close(fd);
        waitpid(pid, null_mut(), 0);
        return false;
    }
    true
}

unsafe fn child_write_ns_id(pipe_write: c_int, path: *const c_char, id: *mut __u64) -> bool {
    let fd = open(path, O_RDONLY);
    if fd < 0 {
        close(pipe_write);
        return false;
    }
    if ioctl(fd, NS_GET_ID, id) < 0 {
        close(fd);
        close(pipe_write);
        return false;
    }
    close(fd);
    true
}

/*
 * Test hierarchical active reference propagation.
 * When a child namespace is active, its owning user namespace should also
 * be active automatically due to hierarchical active reference propagation.
 * This ensures parents are always reachable when children are active.
 */
unsafe fn ns_parent_always_reachable() {
    let mut parent_handle: *mut file_handle;
    let mut child_handle: *mut file_handle;
    let mut ret: ssize_t;
    let mut child_nsfd: c_int;
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut parent_id: __u64 = 0;
    let mut child_id: __u64 = 0;
    let mut parent_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut child_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];

    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if setup_userns() < 0 {
            close(pipefd[1]);
            exit(1);
        }
        if !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut parent_id) {
            exit(1);
        }
        if setup_userns() < 0 {
            close(pipefd[1]);
            exit(1);
        }
        if !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut child_id) {
            exit(1);
        }
        write(pipefd[1], &parent_id as *const _ as *const c_void, size_of::<__u64>());
        write(pipefd[1], &child_id as *const _ as *const c_void, size_of::<__u64>());
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    ret = read(pipefd[0], &mut parent_id as *mut _ as *mut c_void, size_of::<__u64>());
    if ret != size_of::<__u64>() as ssize_t {
        close(pipefd[0]);
        waitpid(pid, null_mut(), 0);
        return;
    }
    ret = read(pipefd[0], &mut child_id as *mut _ as *mut c_void, size_of::<__u64>());
    close(pipefd[0]);
    if ret != size_of::<__u64>() as ssize_t {
        waitpid(pid, null_mut(), 0);
        return;
    }
    parent_handle = init_nsfs_handle(parent_buf.as_mut_ptr(), parent_id);
    child_handle = init_nsfs_handle(child_buf.as_mut_ptr(), child_id);
    child_nsfd = open_by_handle_at(FD_NSFS_ROOT, child_handle, O_RDONLY);
    if child_nsfd < 0 {
        waitpid(pid, null_mut(), 0);
        return;
    }
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let mut parent_fd = open_by_handle_at(FD_NSFS_ROOT, parent_handle, O_RDONLY);
    ASSERT_GE!(parent_fd, 0);
    let parent_fd2 = ioctl(child_nsfd, NS_GET_USERNS);
    if parent_fd2 < 0 {
        close(parent_fd);
        close(child_nsfd);
        return;
    }
    let mut st1: stat = zeroed();
    let mut st2: stat = zeroed();
    ASSERT_EQ!(fstat(parent_fd, &mut st1), 0);
    ASSERT_EQ!(fstat(parent_fd2, &mut st2), 0);
    ASSERT_EQ!(st1.st_ino, st2.st_ino);
    close(child_nsfd);
    let parent_fd3 = open_by_handle_at(FD_NSFS_ROOT, parent_handle, O_RDONLY);
    ASSERT_GE!(parent_fd3, 0);
    close(parent_fd3);
    close(parent_fd);
    close(parent_fd2);
    parent_fd = open_by_handle_at(FD_NSFS_ROOT, parent_handle, O_RDONLY);
    ASSERT_LT!(parent_fd, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
}

/*
 * Test that bind mounts keep namespaces in the tree even when inactive
 */
unsafe fn ns_bind_mount_keeps_in_tree() {
    let mut handle: *mut file_handle;
    let mut mount_id: c_int = 0;
    let mut ret: ssize_t;
    let mut fd: c_int;
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut tmpfile = *b"/tmp/ns-test-XXXXXX\0";
    let tmpfd = mkstemp(tmpfile.as_mut_ptr() as *mut c_char);
    if tmpfd < 0 {
        return;
    }
    close(tmpfd);
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if unshare(CLONE_NEWNS) < 0 {
            close(pipefd[1]);
            unlink(tmpfile.as_ptr() as *const c_char);
            exit(1);
        }
        if mount(null(), cstr!("/"), null(), MS_PRIVATE | MS_REC, null()) < 0 {
            close(pipefd[1]);
            unlink(tmpfile.as_ptr() as *const c_char);
            exit(1);
        }
        if unshare(CLONE_NEWNET) < 0 {
            close(pipefd[1]);
            unlink(tmpfile.as_ptr() as *const c_char);
            exit(1);
        }
        if mount(cstr!("/proc/self/ns/net"), tmpfile.as_ptr() as *const c_char, null(), MS_BIND, null()) < 0 {
            close(pipefd[1]);
            unlink(tmpfile.as_ptr() as *const c_char);
            exit(1);
        }
        fd = open(cstr!("/proc/self/ns/net"), O_RDONLY);
        if fd < 0 {
            umount(tmpfile.as_ptr() as *const c_char);
            close(pipefd[1]);
            unlink(tmpfile.as_ptr() as *const c_char);
            exit(1);
        }
        handle = fh_from_buf(buf.as_mut_ptr());
        (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
        let nret = name_to_handle_at(fd, cstr!(""), handle, &mut mount_id, AT_EMPTY_PATH);
        close(fd);
        if nret < 0 {
            umount(tmpfile.as_ptr() as *const c_char);
            close(pipefd[1]);
            unlink(tmpfile.as_ptr() as *const c_char);
            exit(1);
        }
        write(pipefd[1], buf.as_ptr() as *const c_void, size_of::<file_handle>() + (*handle).handle_bytes as usize);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    ret = read(pipefd[0], buf.as_mut_ptr() as *mut c_void, buf.len());
    close(pipefd[0]);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    ASSERT_GT!(ret, 0);
    handle = fh_from_buf(buf.as_mut_ptr());
    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(fd, 0);
    if errno() != ENOENT && errno() != ESTALE {
        TH_LOG!("Unexpected error");
    }
    umount(tmpfile.as_ptr() as *const c_char);
    unlink(tmpfile.as_ptr() as *const c_char);
}

unsafe fn read_three_ids_or_skip(pipefd0: c_int, pid: pid_t, a: *mut __u64, b: *mut __u64, c: *mut __u64) -> bool {
    let mut ret = read(pipefd0, a as *mut c_void, size_of::<__u64>());
    if ret != size_of::<__u64>() as ssize_t {
        close(pipefd0);
        waitpid(pid, null_mut(), 0);
        return false;
    }
    ret = read(pipefd0, b as *mut c_void, size_of::<__u64>());
    if ret != size_of::<__u64>() as ssize_t {
        close(pipefd0);
        waitpid(pid, null_mut(), 0);
        return false;
    }
    ret = read(pipefd0, c as *mut c_void, size_of::<__u64>());
    close(pipefd0);
    if ret != size_of::<__u64>() as ssize_t {
        waitpid(pid, null_mut(), 0);
        return false;
    }
    true
}

unsafe fn read_two_ids_or_skip(pipefd0: c_int, pid: pid_t, a: *mut __u64, b: *mut __u64) -> bool {
    let mut ret = read(pipefd0, a as *mut c_void, size_of::<__u64>());
    if ret != size_of::<__u64>() as ssize_t {
        close(pipefd0);
        waitpid(pid, null_mut(), 0);
        return false;
    }
    ret = read(pipefd0, b as *mut c_void, size_of::<__u64>());
    close(pipefd0);
    if ret != size_of::<__u64>() as ssize_t {
        waitpid(pid, null_mut(), 0);
        return false;
    }
    true
}

unsafe fn write_three_ids(pipe_write: c_int, a: *const __u64, b: *const __u64, c: *const __u64) {
    write(pipe_write, a as *const c_void, size_of::<__u64>());
    write(pipe_write, b as *const c_void, size_of::<__u64>());
    write(pipe_write, c as *const c_void, size_of::<__u64>());
}

unsafe fn write_two_ids(pipe_write: c_int, a: *const __u64, b: *const __u64) {
    write(pipe_write, a as *const c_void, size_of::<__u64>());
    write(pipe_write, b as *const c_void, size_of::<__u64>());
}

/*
 * Test multi-level hierarchy (3+ levels deep).
 * Grandparent -> Parent -> Child
 * When child is active, both parent AND grandparent should be active.
 */
unsafe fn ns_multilevel_hierarchy() {
    let mut gp_handle: *mut file_handle;
    let mut p_handle: *mut file_handle;
    let mut c_handle: *mut file_handle;
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut gp_id: __u64 = 0;
    let mut p_id: __u64 = 0;
    let mut c_id: __u64 = 0;
    let mut gp_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut p_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut c_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut gp_id) {
            close(pipefd[1]);
            exit(1);
        }
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut p_id) {
            close(pipefd[1]);
            exit(1);
        }
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut c_id) {
            close(pipefd[1]);
            exit(1);
        }
        write_three_ids(pipefd[1], &gp_id, &p_id, &c_id);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    if !read_three_ids_or_skip(pipefd[0], pid, &mut gp_id, &mut p_id, &mut c_id) {
        return;
    }
    gp_handle = init_nsfs_handle(gp_buf.as_mut_ptr(), gp_id);
    p_handle = init_nsfs_handle(p_buf.as_mut_ptr(), p_id);
    c_handle = init_nsfs_handle(c_buf.as_mut_ptr(), c_id);
    let c_fd = open_by_handle_at(FD_NSFS_ROOT, c_handle, O_RDONLY);
    if c_fd < 0 {
        waitpid(pid, null_mut(), 0);
        return;
    }
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let p_fd = open_by_handle_at(FD_NSFS_ROOT, p_handle, O_RDONLY);
    ASSERT_GE!(p_fd, 0);
    let gp_fd = open_by_handle_at(FD_NSFS_ROOT, gp_handle, O_RDONLY);
    ASSERT_GE!(gp_fd, 0);
    close(c_fd);
    close(p_fd);
    close(gp_fd);
}

/*
 * Test multiple children sharing same parent.
 * Parent should stay active as long as ANY child is active.
 */
unsafe fn ns_multiple_children_same_parent() {
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let (mut p_id, mut c1_id, mut c2_id) = (0_u64, 0_u64, 0_u64);
    let mut p_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut c1_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut c2_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut p_id) {
            close(pipefd[1]);
            exit(1);
        }
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut c1_id) {
            close(pipefd[1]);
            exit(1);
        }
        /* We can't actually do this easily, so let's create a sibling namespace
         * by creating a network namespace instead */
        if unshare(CLONE_NEWNET) < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/net"), &mut c2_id) {
            close(pipefd[1]);
            exit(1);
        }
        write_three_ids(pipefd[1], &p_id, &c1_id, &c2_id);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    if !read_three_ids_or_skip(pipefd[0], pid, &mut p_id, &mut c1_id, &mut c2_id) {
        return;
    }
    let p_handle = init_nsfs_handle(p_buf.as_mut_ptr(), p_id);
    let c1_handle = init_nsfs_handle(c1_buf.as_mut_ptr(), c1_id);
    let c2_handle = init_nsfs_handle(c2_buf.as_mut_ptr(), c2_id);
    let mut c1_fd = open_by_handle_at(FD_NSFS_ROOT, c1_handle, O_RDONLY);
    let mut c2_fd = open_by_handle_at(FD_NSFS_ROOT, c2_handle, O_RDONLY);
    if c1_fd < 0 || c2_fd < 0 {
        if c1_fd >= 0 { close(c1_fd); }
        if c2_fd >= 0 { close(c2_fd); }
        waitpid(pid, null_mut(), 0);
        return;
    }
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let mut p_fd = open_by_handle_at(FD_NSFS_ROOT, p_handle, O_RDONLY);
    ASSERT_GE!(p_fd, 0);
    close(p_fd);
    close(c1_fd);
    p_fd = open_by_handle_at(FD_NSFS_ROOT, p_handle, O_RDONLY);
    ASSERT_GE!(p_fd, 0);
    close(p_fd);
    close(c2_fd);
    p_fd = open_by_handle_at(FD_NSFS_ROOT, p_handle, O_RDONLY);
    ASSERT_LT!(p_fd, 0);
}

unsafe fn three_namespace_type_test(
    make_second_user: bool,
    first_child_flag: c_int,
    first_child_path: *const c_char,
    second_child_flag: c_int,
    second_child_path: *const c_char,
) {
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let (mut u_id, mut n_id, mut ut_id) = (0_u64, 0_u64, 0_u64);
    let mut u_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut n_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut ut_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut u_id) {
            close(pipefd[1]);
            exit(1);
        }
        if first_child_flag != 0 && unshare(first_child_flag) < 0 {
            close(pipefd[1]);
            exit(1);
        }
        if !child_write_ns_id(pipefd[1], first_child_path, &mut n_id) {
            exit(1);
        }
        if second_child_flag != 0 && unshare(second_child_flag) < 0 {
            close(pipefd[1]);
            exit(1);
        }
        if !child_write_ns_id(pipefd[1], second_child_path, &mut ut_id) {
            exit(1);
        }
        write_three_ids(pipefd[1], &u_id, &n_id, &ut_id);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    if !read_three_ids_or_skip(pipefd[0], pid, &mut u_id, &mut n_id, &mut ut_id) {
        return;
    }
    let u_handle = init_nsfs_handle(u_buf.as_mut_ptr(), u_id);
    let n_handle = init_nsfs_handle(n_buf.as_mut_ptr(), n_id);
    let ut_handle = init_nsfs_handle(ut_buf.as_mut_ptr(), ut_id);
    let mut n_fd = open_by_handle_at(FD_NSFS_ROOT, n_handle, O_RDONLY);
    let mut ut_fd = open_by_handle_at(FD_NSFS_ROOT, ut_handle, O_RDONLY);
    if n_fd < 0 || ut_fd < 0 {
        if n_fd >= 0 { close(n_fd); }
        if ut_fd >= 0 { close(ut_fd); }
        waitpid(pid, null_mut(), 0);
        return;
    }
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let mut u_fd = open_by_handle_at(FD_NSFS_ROOT, u_handle, O_RDONLY);
    ASSERT_GE!(u_fd, 0);
    close(u_fd);
    close(n_fd);
    u_fd = open_by_handle_at(FD_NSFS_ROOT, u_handle, O_RDONLY);
    ASSERT_GE!(u_fd, 0);
    close(u_fd);
    close(ut_fd);
    u_fd = open_by_handle_at(FD_NSFS_ROOT, u_handle, O_RDONLY);
    ASSERT_LT!(u_fd, 0);
}

/*
 * Test that different namespace types with same owner all contribute
 * active references to the owning user namespace.
 */
unsafe fn ns_different_types_same_owner() {
    three_namespace_type_test(false, CLONE_NEWNET, cstr!("/proc/self/ns/net"), CLONE_NEWUTS, cstr!("/proc/self/ns/uts"));
}

/*
 * Test hierarchical propagation with deep namespace hierarchy.
 * Create: init_user_ns -> user_A -> user_B -> net_ns
 * When net_ns is active, both user_A and user_B should be active.
 * This verifies the conditional recursion in __ns_ref_active_put() works.
 */
unsafe fn ns_deep_hierarchy_propagation() {
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let (mut ua_id, mut ub_id, mut net_id) = (0_u64, 0_u64, 0_u64);
    let mut ua_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut ub_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut net_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut ua_id) {
            close(pipefd[1]);
            exit(1);
        }
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut ub_id) {
            close(pipefd[1]);
            exit(1);
        }
        if unshare(CLONE_NEWNET) < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/net"), &mut net_id) {
            close(pipefd[1]);
            exit(1);
        }
        write_three_ids(pipefd[1], &ua_id, &ub_id, &net_id);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    if !read_three_ids_or_skip(pipefd[0], pid, &mut ua_id, &mut ub_id, &mut net_id) {
        return;
    }
    let ua_handle = init_nsfs_handle(ua_buf.as_mut_ptr(), ua_id);
    let ub_handle = init_nsfs_handle(ub_buf.as_mut_ptr(), ub_id);
    let net_handle = init_nsfs_handle(net_buf.as_mut_ptr(), net_id);
    let net_fd = open_by_handle_at(FD_NSFS_ROOT, net_handle, O_RDONLY);
    if net_fd < 0 {
        waitpid(pid, null_mut(), 0);
        return;
    }
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let mut ub_fd = open_by_handle_at(FD_NSFS_ROOT, ub_handle, O_RDONLY);
    ASSERT_GE!(ub_fd, 0);
    let mut ua_fd = open_by_handle_at(FD_NSFS_ROOT, ua_handle, O_RDONLY);
    ASSERT_GE!(ua_fd, 0);
    close(net_fd);
    let ub_fd2 = open_by_handle_at(FD_NSFS_ROOT, ub_handle, O_RDONLY);
    ASSERT_GE!(ub_fd2, 0);
    close(ub_fd2);
    close(ub_fd);
    let ua_fd2 = open_by_handle_at(FD_NSFS_ROOT, ua_handle, O_RDONLY);
    ASSERT_GE!(ua_fd2, 0);
    close(ua_fd2);
    close(ua_fd);
    ua_fd = open_by_handle_at(FD_NSFS_ROOT, ua_handle, O_RDONLY);
    ASSERT_LT!(ua_fd, 0);
}

/*
 * Test that parent stays active as long as ANY child is active.
 * Create parent user namespace with two child net namespaces.
 * Parent should remain active until BOTH children are inactive.
 */
unsafe fn ns_parent_multiple_children_refcount() {
    let mut pipefd = [0 as c_int; 2];
    let mut syncpipe = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let (mut p_id, mut n1_id, mut n2_id) = (0_u64, 0_u64, 0_u64);
    let mut p_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut n1_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut n2_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut sync_byte: c_char = 0;
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    ASSERT_EQ!(pipe(syncpipe.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        close(syncpipe[1]);
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut p_id) {
            close(pipefd[1]);
            exit(1);
        }
        if unshare(CLONE_NEWNET) < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/net"), &mut n1_id) {
            close(pipefd[1]);
            close(syncpipe[0]);
            exit(1);
        }
        let n1_fd = open(cstr!("/proc/self/ns/net"), O_RDONLY);
        if unshare(CLONE_NEWNET) < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/net"), &mut n2_id) {
            if n1_fd >= 0 { close(n1_fd); }
            close(pipefd[1]);
            close(syncpipe[0]);
            exit(1);
        }
        write_three_ids(pipefd[1], &p_id, &n1_id, &n2_id);
        close(pipefd[1]);
        read(syncpipe[0], &mut sync_byte as *mut _ as *mut c_void, 1);
        close(syncpipe[0]);
        exit(0);
    }
    close(pipefd[1]);
    close(syncpipe[0]);
    if !read_three_ids_or_skip(pipefd[0], pid, &mut p_id, &mut n1_id, &mut n2_id) {
        return;
    }
    let parent_handle = init_nsfs_handle(p_buf.as_mut_ptr(), p_id);
    let net1_handle = init_nsfs_handle(n1_buf.as_mut_ptr(), n1_id);
    let net2_handle = init_nsfs_handle(n2_buf.as_mut_ptr(), n2_id);
    let n1_fd = open_by_handle_at(FD_NSFS_ROOT, net1_handle, O_RDONLY);
    let n2_fd = open_by_handle_at(FD_NSFS_ROOT, net2_handle, O_RDONLY);
    if n1_fd < 0 || n2_fd < 0 {
        if n1_fd >= 0 { close(n1_fd); }
        if n2_fd >= 0 { close(n2_fd); }
        sync_byte = b'G' as c_char;
        write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1);
        close(syncpipe[1]);
        waitpid(pid, null_mut(), 0);
        return;
    }
    sync_byte = b'G' as c_char;
    write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1);
    close(syncpipe[1]);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let mut p_fd = open_by_handle_at(FD_NSFS_ROOT, parent_handle, O_RDONLY);
    ASSERT_GE!(p_fd, 0);
    close(p_fd);
    close(n1_fd);
    p_fd = open_by_handle_at(FD_NSFS_ROOT, parent_handle, O_RDONLY);
    ASSERT_GE!(p_fd, 0);
    close(p_fd);
    close(n2_fd);
    p_fd = open_by_handle_at(FD_NSFS_ROOT, parent_handle, O_RDONLY);
    ASSERT_LT!(p_fd, 0);
}

/*
 * Test that user namespace as a child also propagates correctly.
 * Create user_A -> user_B, verify when user_B is active that user_A
 * is also active. This is different from non-user namespace children.
 */
unsafe fn ns_userns_child_propagation() {
    let mut pipefd = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let (mut ua_id, mut ub_id) = (0_u64, 0_u64);
    let mut ua_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut ub_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut ua_id) {
            close(pipefd[1]);
            exit(1);
        }
        if setup_userns() < 0 || !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut ub_id) {
            close(pipefd[1]);
            exit(1);
        }
        write_two_ids(pipefd[1], &ua_id, &ub_id);
        close(pipefd[1]);
        exit(0);
    }
    close(pipefd[1]);
    if !read_two_ids_or_skip(pipefd[0], pid, &mut ua_id, &mut ub_id) {
        return;
    }
    let ua_handle = init_nsfs_handle(ua_buf.as_mut_ptr(), ua_id);
    let ub_handle = init_nsfs_handle(ub_buf.as_mut_ptr(), ub_id);
    let ub_fd = open_by_handle_at(FD_NSFS_ROOT, ub_handle, O_RDONLY);
    if ub_fd < 0 {
        waitpid(pid, null_mut(), 0);
        return;
    }
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    ASSERT_EQ!(WEXITSTATUS(status), 0);
    let mut ua_fd = open_by_handle_at(FD_NSFS_ROOT, ua_handle, O_RDONLY);
    ASSERT_GE!(ua_fd, 0);
    close(ub_fd);
    let ua_fd2 = open_by_handle_at(FD_NSFS_ROOT, ua_handle, O_RDONLY);
    ASSERT_GE!(ua_fd2, 0);
    close(ua_fd2);
    close(ua_fd);
    ua_fd = open_by_handle_at(FD_NSFS_ROOT, ua_handle, O_RDONLY);
    ASSERT_LT!(ua_fd, 0);
}

/*
 * Test different namespace types (net, uts, ipc) all contributing
 * active references to the same owning user namespace.
 */
unsafe fn ns_mixed_types_same_owner() {
    three_namespace_type_test(false, CLONE_NEWNET, cstr!("/proc/self/ns/net"), CLONE_NEWUTS, cstr!("/proc/self/ns/uts"));
}

/* Thread test helpers and structures */
#[repr(C)]
struct thread_ns_info {
    ns_id: __u64,
    pipefd: c_int,
    syncfd_read: c_int,
    syncfd_write: c_int,
    exit_code: c_int,
}

unsafe extern "C" fn thread_create_namespace(arg: *mut c_void) -> *mut c_void {
    let info = arg as *mut thread_ns_info;
    let ret: c_int;
    if unshare(CLONE_NEWNET) < 0 {
        (*info).exit_code = 1;
        return null_mut();
    }
    let fd = open(cstr!("/proc/thread-self/ns/net"), O_RDONLY);
    if fd < 0 {
        (*info).exit_code = 2;
        return null_mut();
    }
    ret = ioctl(fd, NS_GET_ID, &mut (*info).ns_id);
    close(fd);
    if ret < 0 {
        (*info).exit_code = 3;
        return null_mut();
    }
    if write((*info).pipefd, &(*info).ns_id as *const _ as *const c_void, size_of::<__u64>()) != size_of::<__u64>() as ssize_t {
        (*info).exit_code = 4;
        return null_mut();
    }
    let mut sync_byte: c_char = 0;
    if read((*info).syncfd_read, &mut sync_byte as *mut _ as *mut c_void, 1) != 1 {
        (*info).exit_code = 5;
        return null_mut();
    }
    (*info).exit_code = 0;
    null_mut()
}

unsafe fn thread_common(hold_fd: bool) {
    let mut thread: pthread_t = 0;
    let mut info = thread_ns_info { ns_id: 0, pipefd: 0, syncfd_read: 0, syncfd_write: 0, exit_code: 0 };
    let mut pipefd = [0 as c_int; 2];
    let mut syncpipe = [0 as c_int; 2];
    let mut sync_byte: c_char = 0;
    let mut buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    ASSERT_EQ!(pipe(syncpipe.as_mut_ptr()), 0);
    info.pipefd = pipefd[1];
    info.syncfd_read = syncpipe[0];
    info.syncfd_write = -1;
    info.exit_code = -1;
    ASSERT_EQ!(pthread_create(&mut thread, null(), thread_create_namespace, &mut info as *mut _ as *mut c_void), 0);
    let mut ns_id: __u64 = 0;
    let ret = read(pipefd[0], &mut ns_id as *mut _ as *mut c_void, size_of::<__u64>());
    if ret != size_of::<__u64>() as ssize_t {
        sync_byte = b'X' as c_char;
        write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1);
        pthread_join(thread, null_mut());
        close(pipefd[0]);
        close(pipefd[1]);
        close(syncpipe[0]);
        close(syncpipe[1]);
        return;
    }
    let handle = init_nsfs_handle(buf.as_mut_ptr(), ns_id);
    let mut nsfd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_GE!(nsfd, 0);
    if !hold_fd {
        close(nsfd);
    }
    sync_byte = b'X' as c_char;
    if !hold_fd {
        ASSERT_EQ!(write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1), 1);
    } else {
        write(syncpipe[1], &sync_byte as *const _ as *const c_void, 1);
    }
    close(syncpipe[1]);
    if !hold_fd {
        ASSERT_EQ!(pthread_join(thread, null_mut()), 0);
    } else {
        pthread_join(thread, null_mut());
    }
    close(pipefd[0]);
    close(pipefd[1]);
    close(syncpipe[0]);
    if info.exit_code != 0 {
        if hold_fd { close(nsfd); }
        return;
    }
    if hold_fd {
        let nsfd2 = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
        ASSERT_GE!(nsfd2, 0);
        let mut st1: stat = zeroed();
        let mut st2: stat = zeroed();
        ASSERT_EQ!(fstat(nsfd, &mut st1), 0);
        ASSERT_EQ!(fstat(nsfd2, &mut st2), 0);
        ASSERT_EQ!(st1.st_ino, st2.st_ino);
        close(nsfd2);
        close(nsfd);
    }
    nsfd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    ASSERT_LT!(nsfd, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
}

/*
 * Test that namespace becomes inactive after thread exits.
 * This verifies active reference counting works with threads, not just processes.
 */
unsafe fn thread_ns_inactive_after_exit() {
    thread_common(false);
}

/*
 * Test that a namespace remains active while a thread holds an fd to it.
 * Even after the thread exits, the namespace should remain active as long as
 * another thread holds a file descriptor to it.
 */
unsafe fn thread_ns_fd_keeps_active() {
    thread_common(true);
}

/* Structure for thread data in subprocess */
#[repr(C)]
struct thread_sleep_data {
    syncfd_read: c_int,
}

unsafe extern "C" fn thread_sleep_and_wait(arg: *mut c_void) -> *mut c_void {
    let data = arg as *mut thread_sleep_data;
    let mut sync_byte: c_char = 0;
    read((*data).syncfd_read, &mut sync_byte as *mut _ as *mut c_void, 1);
    null_mut()
}

/*
 * Test that namespaces become inactive after subprocess with multiple threads exits.
 * Create a subprocess that unshares user and network namespaces, then creates two
 * threads that share those namespaces. Verify that after all threads and subprocess
 * exit, the namespaces are no longer listed by listns() and cannot be opened by
 * open_by_handle_at().
 */
unsafe fn thread_subprocess_ns_inactive_after_all_exit() {
    let mut pipefd = [0 as c_int; 2];
    let mut sv = [0 as c_int; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let (mut user_id, mut net_id) = (0_u64, 0_u64);
    let mut user_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut net_buf = [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ];
    let mut sync_byte: c_char = 0;
    let mut ret: ssize_t;
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);
    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sv.as_mut_ptr()), 0);
    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        close(pipefd[0]);
        close(sv[0]);
        if setup_userns() < 0 {
            close(pipefd[1]);
            close(sv[1]);
            exit(1);
        }
        if !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/user"), &mut user_id) {
            close(sv[1]);
            exit(1);
        }
        if unshare(CLONE_NEWNET) < 0 {
            close(pipefd[1]);
            close(sv[1]);
            exit(1);
        }
        if !child_write_ns_id(pipefd[1], cstr!("/proc/self/ns/net"), &mut net_id) {
            close(sv[1]);
            exit(1);
        }
        if write(pipefd[1], &user_id as *const _ as *const c_void, size_of::<__u64>()) != size_of::<__u64>() as ssize_t {
            exit(1);
        }
        if write(pipefd[1], &net_id as *const _ as *const c_void, size_of::<__u64>()) != size_of::<__u64>() as ssize_t {
            exit(1);
        }
        close(pipefd[1]);
        let mut thread1: pthread_t = 0;
        let mut thread2: pthread_t = 0;
        let mut data = thread_sleep_data { syncfd_read: sv[1] };
        let mut ret_thread = pthread_create(&mut thread1, null(), thread_sleep_and_wait, &mut data as *mut _ as *mut c_void);
        if ret_thread != 0 {
            close(sv[1]);
            exit(1);
        }
        ret_thread = pthread_create(&mut thread2, null(), thread_sleep_and_wait, &mut data as *mut _ as *mut c_void);
        if ret_thread != 0 {
            close(sv[1]);
            pthread_cancel(thread1);
            exit(1);
        }
        pthread_join(thread1, null_mut());
        pthread_join(thread2, null_mut());
        close(sv[1]);
        exit(0);
    }
    close(pipefd[1]);
    close(sv[1]);
    ret = read(pipefd[0], &mut user_id as *mut _ as *mut c_void, size_of::<__u64>());
    if ret != size_of::<__u64>() as ssize_t {
        close(pipefd[0]);
        sync_byte = b'X' as c_char;
        write(sv[0], &sync_byte as *const _ as *const c_void, 1);
        close(sv[0]);
        waitpid(pid, null_mut(), 0);
        return;
    }
    ret = read(pipefd[0], &mut net_id as *mut _ as *mut c_void, size_of::<__u64>());
    close(pipefd[0]);
    if ret != size_of::<__u64>() as ssize_t {
        sync_byte = b'X' as c_char;
        write(sv[0], &sync_byte as *const _ as *const c_void, 1);
        close(sv[0]);
        waitpid(pid, null_mut(), 0);
        return;
    }
    let user_handle = init_nsfs_handle(user_buf.as_mut_ptr(), user_id);
    let net_handle = init_nsfs_handle(net_buf.as_mut_ptr(), net_id);
    let mut user_fd = open_by_handle_at(FD_NSFS_ROOT, user_handle, O_RDONLY);
    ASSERT_GE!(user_fd, 0);
    let mut net_fd = open_by_handle_at(FD_NSFS_ROOT, net_handle, O_RDONLY);
    ASSERT_GE!(net_fd, 0);
    close(user_fd);
    close(net_fd);
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_uint,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER as __u64,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids = [0 as __u64; 256];
    let mut nr_ids = sys_listns(&mut req, ns_ids.as_mut_ptr(), 256, 0);
    if nr_ids >= 0 {
        let mut found_user = 0;
        let mut i = 0;
        while i < nr_ids {
            if ns_ids[i as usize] == user_id {
                found_user = 1;
                break;
            }
            i += 1;
        }
        ASSERT_TRUE!(found_user != 0);
        req.ns_type = CLONE_NEWNET as __u64;
        nr_ids = sys_listns(&mut req, ns_ids.as_mut_ptr(), 256, 0);
        if nr_ids >= 0 {
            let mut found_net = 0;
            let mut j = 0;
            while j < nr_ids {
                if ns_ids[j as usize] == net_id {
                    found_net = 1;
                    break;
                }
                j += 1;
            }
            ASSERT_TRUE!(found_net != 0);
        }
    }
    sync_byte = b'X' as c_char;
    ASSERT_EQ!(write(sv[0], &sync_byte as *const _ as *const c_void, 1), 1);
    ASSERT_EQ!(write(sv[0], &sync_byte as *const _ as *const c_void, 1), 1);
    close(sv[0]);
    waitpid(pid, &mut status, 0);
    ASSERT_TRUE!(WIFEXITED(status));
    if WEXITSTATUS(status) != 0 {
        return;
    }
    user_fd = open_by_handle_at(FD_NSFS_ROOT, user_handle, O_RDONLY);
    ASSERT_LT!(user_fd, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
    net_fd = open_by_handle_at(FD_NSFS_ROOT, net_handle, O_RDONLY);
    ASSERT_LT!(net_fd, 0);
    ASSERT_TRUE!(errno() == ENOENT || errno() == ESTALE);
    memset(&mut req as *mut _ as *mut c_void, 0, size_of::<ns_id_req>());
    req.size = size_of::<ns_id_req>() as c_uint;
    req.ns_type = CLONE_NEWUSER as __u64;
    nr_ids = sys_listns(&mut req, ns_ids.as_mut_ptr(), 256, 0);
    if nr_ids >= 0 {
        let mut found_user = 0;
        let mut i = 0;
        while i < nr_ids {
            if ns_ids[i as usize] == user_id {
                found_user = 1;
                break;
            }
            i += 1;
        }
        ASSERT_FALSE!(found_user != 0);
        req.ns_type = CLONE_NEWNET as __u64;
        nr_ids = sys_listns(&mut req, ns_ids.as_mut_ptr(), 256, 0);
        if nr_ids >= 0 {
            let mut found_net = 0;
            let mut j = 0;
            while j < nr_ids {
                if ns_ids[j as usize] == net_id {
                    found_net = 1;
                    break;
                }
                j += 1;
            }
            ASSERT_FALSE!(found_net != 0);
        }
    }
}

fn main() {
    unsafe {
        init_ns_always_active();
        ns_inactive_after_exit();
        ns_active_with_multiple_processes();
        userns_active_ref_lifecycle();
        pidns_active_ref_lifecycle();
        ns_fd_keeps_active();
        ns_parent_always_reachable();
        ns_bind_mount_keeps_in_tree();
        ns_multilevel_hierarchy();
        ns_multiple_children_same_parent();
        ns_different_types_same_owner();
        ns_deep_hierarchy_propagation();
        ns_parent_multiple_children_refcount();
        ns_userns_child_propagation();
        ns_mixed_types_same_owner();
        thread_ns_inactive_after_exit();
        thread_ns_fd_keeps_active();
        thread_subprocess_ns_inactive_after_all_exit();
    }
}
