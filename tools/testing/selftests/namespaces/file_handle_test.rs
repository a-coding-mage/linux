// SPDX-License-Identifier: GPL-2.0
// C source included: errno.h, fcntl.h, grp.h, limits.h, sched.h, stdio.h,
// stdlib.h, string.h, sys/mount.h, sys/stat.h, sys/types.h, sys/wait.h,
// unistd.h, linux/unistd.h, and "kselftest_harness.h".

use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

const FD_NSFS_ROOT: c_int = -10003; /* Root of the nsfs filesystem */
const MAX_HANDLE_SZ: usize = 128;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_TRUNC: c_int = 0o1000;
const O_DIRECTORY: c_int = 0o200000;
const O_DIRECT: c_int = 0o40000;
const O_TMPFILE: c_int = 0o20200000;
const O_WRONLY_CONST: c_int = 1;
const AT_EMPTY_PATH: c_int = 0x1000;
const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const ESTALE: c_int = 116;
const ENOTDIR: c_int = 20;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWIPC: c_int = 0x08000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const CLONE_NEWNET: c_int = 0x40000000;
const CLONE_NEWCGROUP: c_int = 0x02000000;
const CLONE_NEWTIME: c_int = 0x00000080;

#[repr(C)]
struct file_handle {
    handle_bytes: c_uint,
    handle_type: c_int,
    f_handle: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    _rest: [u8; 128],
}

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn setresgid(rgid: c_uint, egid: c_uint, sgid: c_uint) -> c_int;
    fn setresuid(ruid: c_uint, euid: c_uint, suid: c_uint) -> c_int;
    fn name_to_handle_at(
        dirfd: c_int,
        pathname: *const c_char,
        handle: *mut file_handle,
        mount_id: *mut c_int,
        flags: c_int,
    ) -> c_int;
    fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn getuid() -> c_uint;
    fn getgid() -> c_uint;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn skip(message: &str) {
    eprintln!("SKIP: {}", message);
}

unsafe fn alloc_handle() -> *mut file_handle {
    malloc(mem::size_of::<file_handle>() + MAX_HANDLE_SZ) as *mut file_handle
}

unsafe fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

unsafe fn assert_eq_int(left: c_int, right: c_int) {
    assert_eq!(left, right);
}

unsafe fn assert_ge_int(left: c_int, right: c_int) {
    assert!(left >= right);
}

unsafe fn assert_gt_uint(left: c_uint, right: c_uint) {
    assert!(left > right);
}

unsafe fn assert_lt_int(left: c_int, right: c_int) {
    assert!(left < right);
}

unsafe fn common_nsfs_handle(path: &str, unavailable: bool, check_eperm: bool) {
    let mut mount_id: c_int = 0;
    let mut ret: c_int;
    let mut fd: c_int;
    let ns_fd: c_int;
    let mut st1: stat = mem::zeroed();
    let mut st2: stat = mem::zeroed();

    /* Drop to unprivileged uid/gid */
    assert_eq_int(setresgid(65534, 65534, 65534), 0); /* nogroup */
    assert_eq_int(setresuid(65534, 65534, 65534), 0); /* nobody */

    let handle = alloc_handle();
    assert!(!handle.is_null());

    /* Open namespace file descriptor */
    let path_c = cstr(path);
    ns_fd = open(path_c.as_ptr(), O_RDONLY);
    if unavailable && ns_fd < 0 {
        skip(if path.ends_with("cgroup") {
            "cgroup namespace not available"
        } else {
            "time namespace not available"
        });
        free(handle as *mut c_void);
        return;
    }
    assert_ge_int(ns_fd, 0);

    /* Get handle for the namespace */
    (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
    let empty = cstr("");
    ret = name_to_handle_at(ns_fd, empty.as_ptr(), handle, &mut mount_id, AT_EMPTY_PATH);
    if ret < 0 && errno() == EOPNOTSUPP {
        skip("nsfs doesn't support file handles");
        free(handle as *mut c_void);
        close(ns_fd);
        return;
    }
    assert_eq_int(ret, 0);
    assert_gt_uint((*handle).handle_bytes, 0);

    /* Try to open using FD_NSFS_ROOT */
    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);
    if fd < 0 && (errno() == EINVAL || errno() == EOPNOTSUPP) {
        skip("open_by_handle_at with FD_NSFS_ROOT not supported");
        free(handle as *mut c_void);
        close(ns_fd);
        return;
    }
    if check_eperm && fd < 0 && errno() == EPERM {
        skip("Permission denied for unprivileged user (expected)");
        free(handle as *mut c_void);
        close(ns_fd);
        return;
    }
    assert_ge_int(fd, 0);

    /* Verify we opened the correct namespace */
    assert_eq_int(fstat(ns_fd, &mut st1), 0);
    assert_eq_int(fstat(fd, &mut st2), 0);
    assert_eq!(st1.st_ino, st2.st_ino);
    assert_eq!(st1.st_dev, st2.st_dev);

    close(fd);
    close(ns_fd);
    free(handle as *mut c_void);
}

#[test]
unsafe fn nsfs_net_handle() {
    common_nsfs_handle("/proc/self/ns/net", false, true);
}

#[test]
unsafe fn nsfs_uts_handle() {
    common_nsfs_handle("/proc/self/ns/uts", false, false);
}

#[test]
unsafe fn nsfs_ipc_handle() {
    common_nsfs_handle("/proc/self/ns/ipc", false, false);
}

#[test]
unsafe fn nsfs_pid_handle() {
    common_nsfs_handle("/proc/self/ns/pid", false, false);
}

#[test]
unsafe fn nsfs_mnt_handle() {
    common_nsfs_handle("/proc/self/ns/mnt", false, false);
}

#[test]
unsafe fn nsfs_user_handle() {
    common_nsfs_handle("/proc/self/ns/user", false, false);
}

#[test]
unsafe fn nsfs_cgroup_handle() {
    common_nsfs_handle("/proc/self/ns/cgroup", true, false);
}

#[test]
unsafe fn nsfs_time_handle() {
    common_nsfs_handle("/proc/self/ns/time", true, false);
}

unsafe fn write_byte(fd: c_int, ch: u8) {
    write(fd, &ch as *const u8 as *const c_void, 1);
}

unsafe fn setup_user_mapping(pipe_write: c_int) -> bool {
    /* First create new user namespace to drop privileges */
    let mut ret = unshare(CLONE_NEWUSER);
    if ret < 0 {
        write_byte(pipe_write, b'U'); /* Unable to create user namespace */
        close(pipe_write);
        exit(0);
    }

    /* Write uid/gid mappings to maintain some capabilities */
    let uid_map = cstr("/proc/self/uid_map");
    let gid_map = cstr("/proc/self/gid_map");
    let setgroups = cstr("/proc/self/setgroups");
    let uid_map_fd = open(uid_map.as_ptr(), O_WRONLY_CONST);
    let gid_map_fd = open(gid_map.as_ptr(), O_WRONLY_CONST);
    let setgroups_fd = open(setgroups.as_ptr(), O_WRONLY_CONST);

    if uid_map_fd < 0 || gid_map_fd < 0 || setgroups_fd < 0 {
        write_byte(pipe_write, b'M'); /* Unable to set mappings */
        close(pipe_write);
        exit(0);
    }

    /* Disable setgroups to allow gid mapping */
    write(setgroups_fd, b"deny".as_ptr() as *const c_void, 4);
    close(setgroups_fd);

    /* Map current uid/gid to root in the new namespace */
    let mut mapping = [0 as c_char; 64];
    let fmt = cstr("0 %d 1");
    snprintf(mapping.as_mut_ptr(), mapping.len(), fmt.as_ptr(), getuid());
    write(uid_map_fd, mapping.as_ptr() as *const c_void, strlen(mapping.as_ptr()));
    close(uid_map_fd);

    ret = snprintf(mapping.as_mut_ptr(), mapping.len(), fmt.as_ptr(), getgid());
    let _ = ret;
    write(gid_map_fd, mapping.as_ptr() as *const c_void, strlen(mapping.as_ptr()));
    close(gid_map_fd);
    true
}

unsafe fn try_open_parent_handle(pipe_write: c_int, handle: *mut file_handle) {
    let fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);

    if fd >= 0 {
        /* Should NOT succeed - we're in a different user namespace */
        write_byte(pipe_write, b'S'); /* Unexpected success */
        close(fd);
    } else if errno() == ESTALE {
        /* Expected: Stale file handle */
        write_byte(pipe_write, b'P');
    } else {
        /* Other error */
        write_byte(pipe_write, b'F');
    }
}

unsafe fn common_user_namespace_isolation(path: &str, clone_flag: c_int, unavailable: bool, name: &str) {
    let mut mount_id: c_int = 0;
    let mut ret: c_int;
    let mut fd: c_int;
    let ns_fd: c_int;
    let mut pid: c_int;
    let mut status: c_int = 0;
    let mut pipefd = [0 as c_int; 2];
    let mut result: c_char = 0;

    let handle = alloc_handle();
    assert!(!handle.is_null());

    /* Create pipe for communication */
    assert_eq_int(pipe(pipefd.as_mut_ptr()), 0);

    /* Get handle for current namespace */
    let path_c = cstr(path);
    ns_fd = open(path_c.as_ptr(), O_RDONLY);
    if unavailable && ns_fd < 0 {
        skip(if name == "cgroup" {
            "cgroup namespace not available"
        } else {
            "time namespace not available"
        });
        free(handle as *mut c_void);
        close(pipefd[0]);
        close(pipefd[1]);
        return;
    }
    assert_ge_int(ns_fd, 0);

    (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
    let empty = cstr("");
    ret = name_to_handle_at(ns_fd, empty.as_ptr(), handle, &mut mount_id, AT_EMPTY_PATH);
    if ret < 0 && errno() == EOPNOTSUPP {
        skip("nsfs doesn't support file handles");
        free(handle as *mut c_void);
        close(ns_fd);
        close(pipefd[0]);
        close(pipefd[1]);
        return;
    }
    assert_eq_int(ret, 0);
    close(ns_fd);

    pid = fork();
    assert_ge_int(pid, 0);

    if pid == 0 {
        /* Child process */
        close(pipefd[0]);
        setup_user_mapping(pipefd[1]);

        /* Now create new namespace */
        ret = unshare(clone_flag);
        if ret < 0 {
            write_byte(pipefd[1], b'N'); /* Unable to create namespace */
            close(pipefd[1]);
            exit(0);
        }

        /* Try to open parent's namespace handle from new user+namespace */
        fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDONLY);

        if fd >= 0 {
            /* Should NOT succeed - we're in a different user namespace */
            write_byte(pipefd[1], b'S'); /* Unexpected success */
            close(fd);
        } else if errno() == ESTALE {
            /* Expected: Stale file handle */
            write_byte(pipefd[1], b'P');
        } else {
            /* Other error */
            write_byte(pipefd[1], b'F');
        }

        close(pipefd[1]);
        exit(0);
    }

    /* Parent process */
    close(pipefd[1]);
    assert_eq!(read(pipefd[0], &mut result as *mut c_char as *mut c_void, 1), 1);

    waitpid(pid, &mut status, 0);
    assert!(wifexited(status));
    assert_eq_int(wexitstatus(status), 0);

    if result == b'U' as c_char {
        skip("Cannot create new user namespace");
        free(handle as *mut c_void);
        close(pipefd[0]);
        return;
    }
    if result == b'M' as c_char {
        skip("Cannot set uid/gid mappings");
        free(handle as *mut c_void);
        close(pipefd[0]);
        return;
    }
    if result == b'N' as c_char {
        let msg = format!("Cannot create new {} namespace", name);
        skip(&msg);
        free(handle as *mut c_void);
        close(pipefd[0]);
        return;
    }

    /* Should fail with ESTALE since we're in a different user namespace */
    assert_eq!(result, b'P' as c_char);

    close(pipefd[0]);
    free(handle as *mut c_void);
}

#[test]
unsafe fn nsfs_user_net_namespace_isolation() {
    common_user_namespace_isolation("/proc/self/ns/net", CLONE_NEWNET, false, "network");
}

#[test]
unsafe fn nsfs_user_uts_namespace_isolation() {
    common_user_namespace_isolation("/proc/self/ns/uts", CLONE_NEWUTS, false, "UTS");
}

#[test]
unsafe fn nsfs_user_ipc_namespace_isolation() {
    common_user_namespace_isolation("/proc/self/ns/ipc", CLONE_NEWIPC, false, "IPC");
}

#[test]
unsafe fn nsfs_user_mnt_namespace_isolation() {
    common_user_namespace_isolation("/proc/self/ns/mnt", CLONE_NEWNS, false, "mount");
}

#[test]
unsafe fn nsfs_user_cgroup_namespace_isolation() {
    common_user_namespace_isolation("/proc/self/ns/cgroup", CLONE_NEWCGROUP, true, "cgroup");
}

unsafe fn common_user_forked_namespace_isolation(path: &str, clone_flag: c_int, unavailable: bool, name: &str) {
    let mut mount_id: c_int = 0;
    let mut ret: c_int;
    let ns_fd: c_int;
    let mut pid: c_int;
    let mut status: c_int = 0;
    let mut pipefd = [0 as c_int; 2];
    let mut result: c_char = 0;

    let handle = alloc_handle();
    assert!(!handle.is_null());

    /* Create pipe for communication */
    assert_eq_int(pipe(pipefd.as_mut_ptr()), 0);

    /* Get handle for current namespace */
    let path_c = cstr(path);
    ns_fd = open(path_c.as_ptr(), O_RDONLY);
    if unavailable && ns_fd < 0 {
        skip(if name == "time" {
            "time namespace not available"
        } else {
            "namespace not available"
        });
        free(handle as *mut c_void);
        close(pipefd[0]);
        close(pipefd[1]);
        return;
    }
    assert_ge_int(ns_fd, 0);

    (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
    let empty = cstr("");
    ret = name_to_handle_at(ns_fd, empty.as_ptr(), handle, &mut mount_id, AT_EMPTY_PATH);
    if ret < 0 && errno() == EOPNOTSUPP {
        skip("nsfs doesn't support file handles");
        free(handle as *mut c_void);
        close(ns_fd);
        close(pipefd[0]);
        close(pipefd[1]);
        return;
    }
    assert_eq_int(ret, 0);
    close(ns_fd);

    pid = fork();
    assert_ge_int(pid, 0);

    if pid == 0 {
        /* Child process */
        close(pipefd[0]);
        setup_user_mapping(pipefd[1]);

        /* Now create new namespace - requires fork to take effect */
        ret = unshare(clone_flag);
        if ret < 0 {
            write_byte(pipefd[1], b'N'); /* Unable to create namespace */
            close(pipefd[1]);
            exit(0);
        }

        /* Fork again for namespace to take effect */
        let child_pid = fork();
        if child_pid < 0 {
            write_byte(pipefd[1], b'N'); /* Unable to fork in namespace */
            close(pipefd[1]);
            exit(0);
        }

        if child_pid == 0 {
            /* Grandchild in new namespace */
            /* Try to open parent's namespace handle from new user+namespace */
            try_open_parent_handle(pipefd[1], handle);
            close(pipefd[1]);
            exit(0);
        }

        /* Wait for grandchild */
        waitpid(child_pid, ptr::null_mut(), 0);
        exit(0);
    }

    /* Parent process */
    close(pipefd[1]);
    assert_eq!(read(pipefd[0], &mut result as *mut c_char as *mut c_void, 1), 1);

    waitpid(pid, &mut status, 0);
    assert!(wifexited(status));
    assert_eq_int(wexitstatus(status), 0);

    if result == b'U' as c_char {
        skip("Cannot create new user namespace");
        free(handle as *mut c_void);
        close(pipefd[0]);
        return;
    }
    if result == b'M' as c_char {
        skip("Cannot set uid/gid mappings");
        free(handle as *mut c_void);
        close(pipefd[0]);
        return;
    }
    if result == b'N' as c_char {
        let msg = format!("Cannot create new {} namespace", name);
        skip(&msg);
        free(handle as *mut c_void);
        close(pipefd[0]);
        return;
    }

    /* Should fail with ESTALE since we're in a different user namespace */
    assert_eq!(result, b'P' as c_char);

    close(pipefd[0]);
    free(handle as *mut c_void);
}

#[test]
unsafe fn nsfs_user_pid_namespace_isolation() {
    common_user_forked_namespace_isolation("/proc/self/ns/pid", CLONE_NEWPID, false, "PID");
}

#[test]
unsafe fn nsfs_user_time_namespace_isolation() {
    common_user_forked_namespace_isolation("/proc/self/ns/time", CLONE_NEWTIME, true, "time");
}

#[test]
unsafe fn nsfs_open_flags() {
    let mut mount_id: c_int = 0;
    let mut ret: c_int;
    let mut fd: c_int;
    let ns_fd: c_int;

    let handle = alloc_handle();
    assert!(!handle.is_null());

    /* Open a namespace file descriptor */
    let net = cstr("/proc/self/ns/net");
    ns_fd = open(net.as_ptr(), O_RDONLY);
    assert_ge_int(ns_fd, 0);

    /* Get handle for the namespace */
    (*handle).handle_bytes = MAX_HANDLE_SZ as c_uint;
    let empty = cstr("");
    ret = name_to_handle_at(ns_fd, empty.as_ptr(), handle, &mut mount_id, AT_EMPTY_PATH);
    if ret < 0 && errno() == EOPNOTSUPP {
        skip("nsfs doesn't support file handles");
        free(handle as *mut c_void);
        close(ns_fd);
        return;
    }
    assert_eq_int(ret, 0);
    assert_gt_uint((*handle).handle_bytes, 0);

    /* Test invalid flags that should fail */
    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_WRONLY);
    assert_lt_int(fd, 0);
    assert_eq_int(errno(), EPERM);

    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_RDWR);
    assert_lt_int(fd, 0);
    assert_eq_int(errno(), EPERM);

    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_TRUNC);
    assert_lt_int(fd, 0);
    assert_eq_int(errno(), EPERM);

    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_DIRECT);
    assert_lt_int(fd, 0);
    assert_eq_int(errno(), EINVAL);

    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_TMPFILE);
    assert_lt_int(fd, 0);
    assert_eq_int(errno(), EINVAL);

    fd = open_by_handle_at(FD_NSFS_ROOT, handle, O_DIRECTORY);
    assert_lt_int(fd, 0);
    assert_eq_int(errno(), ENOTDIR);

    close(ns_fd);
    free(handle as *mut c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
