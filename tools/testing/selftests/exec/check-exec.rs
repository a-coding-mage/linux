// SPDX-License-Identifier: GPL-2.0
/*
 * Test execveat(2) with AT_EXECVE_CHECK, and prctl(2) with
 * SECBIT_EXEC_RESTRICT_FILE, SECBIT_EXEC_DENY_INTERACTIVE, and their locked
 * counterparts.
 *
 * Copyright © 2018-2020 ANSSI
 * Copyright © 2024 Microsoft Corporation
 *
 * Author: Mickaël Salaün <mic@digikod.net>
 */

/*
 * Dependencies from the C source:
 * asm-generic/unistd.h, errno.h, fcntl.h, linux/prctl.h,
 * linux/securebits.h, stdio.h, stdlib.h, sys/capability.h, sys/mount.h,
 * sys/prctl.h, sys/socket.h, sys/stat.h, sys/syscall.h, sys/sysmacros.h,
 * unistd.h, linux/fcntl.h, and kselftest_harness.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type size_t = usize;
type ssize_t = isize;
type mode_t = c_uint;
type dev_t = u64;
type pid_t = c_int;
type cap_t = *mut c_void;

const NULL: *mut c_void = core::ptr::null_mut();

extern "C" {
    static mut errno: c_int;

    fn syscall(number: c_long, ...) -> c_long;
    fn cap_get_secbits() -> c_int;
    fn cap_set_secbits(bits: c_uint) -> c_int;
    fn cap_get_proc() -> cap_t;
    fn cap_clear(cap_p: cap_t) -> c_int;
    fn cap_set_proc(cap_p: cap_t) -> c_int;
    fn cap_free(cap_p: cap_t) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn mkdir(pathname: *const c_char, mode: mode_t) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn mknod(pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn vfork() -> pid_t;
    fn _exit(status: c_int) -> !;
}

type c_ulong = u64;

/* External constants supplied by the translated headers/build. */
extern "C" {
    static __NR_execveat: c_long;
    static AT_EMPTY_PATH: c_int;
    static AT_EXECVE_CHECK: c_int;
    static SECBIT_NOROOT: c_uint;
    static SECBIT_NOROOT_LOCKED: c_uint;
    static SECBIT_EXEC_RESTRICT_FILE: c_uint;
    static SECBIT_EXEC_DENY_INTERACTIVE: c_uint;
    static SECBIT_EXEC_RESTRICT_FILE_LOCKED: c_uint;
    static SECBIT_EXEC_DENY_INTERACTIVE_LOCKED: c_uint;
    static PR_SET_SECUREBITS: c_int;
    static PR_GET_SECUREBITS: c_int;
    static MS_MGC_VAL: c_ulong;
    static MS_NOEXEC: c_ulong;
    static MFD_CLOEXEC: c_uint;
    static O_CLOEXEC: c_int;
    static O_RDONLY: c_int;
    static O_RDWR: c_int;
    static O_WRONLY: c_int;
    static O_NONBLOCK: c_int;
    static S_IFREG: mode_t;
    static S_IFCHR: mode_t;
    static S_IFBLK: mode_t;
    static S_IFIFO: mode_t;
    static AF_UNIX: c_int;
    static SOCK_DGRAM: c_int;
    static SOCK_CLOEXEC: c_int;
    static EACCES: c_int;
    static EPERM: c_int;
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        expect_eq!($left, $right)
    };
}
macro_rules! EXPECT_NE {
    ($left:expr, $right:expr) => {
        expect_ne!($left, $right)
    };
}
macro_rules! EXPECT_LE {
    ($left:expr, $right:expr) => {
        expect_le!($left, $right)
    };
}
macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}
macro_rules! ASSERT_LE {
    ($left:expr, $right:expr) => {
        assert_le!($left, $right)
    };
}
macro_rules! ASSERT_LT {
    ($left:expr, $right:expr) => {
        assert_lt!($left, $right)
    };
}
macro_rules! TH_LOG {
    ($($arg:tt)*) => {
        th_log!($($arg)*)
    };
}

extern "Rust" {
    fn expect_eq<T, U>(left: T, right: U);
    fn expect_ne<T, U>(left: T, right: U);
    fn expect_le<T, U>(left: T, right: U);
    fn assert_eq<T, U>(left: T, right: U);
    fn assert_le<T, U>(left: T, right: U);
    fn assert_lt<T, U>(left: T, right: U);
    fn th_log(format: *const c_char, ...);
}

unsafe fn makedev(major: c_uint, minor: c_uint) -> dev_t {
    ((major as dev_t) << 8) | (minor as dev_t)
}

unsafe fn sys_execveat(
    dirfd: c_int,
    pathname: *const c_char,
    argv: *mut *mut c_char,
    envp: *mut *mut c_char,
    flags: c_int,
) -> c_int {
    syscall(__NR_execveat, dirfd, pathname, argv, envp, flags) as c_int
}

unsafe fn drop_privileges(_metadata: *mut __test_metadata) {
    let noroot: c_uint = SECBIT_NOROOT | SECBIT_NOROOT_LOCKED;
    let cap_p: cap_t;

    if (cap_get_secbits() as c_uint & noroot) != noroot {
        EXPECT_EQ!(0, cap_set_secbits(noroot));
    }

    cap_p = cap_get_proc();
    EXPECT_NE!(NULL, cap_p);
    EXPECT_NE!(-1, cap_clear(cap_p));

    /*
     * Drops everything, especially CAP_SETPCAP, CAP_DAC_OVERRIDE, and
     * CAP_DAC_READ_SEARCH.
     */
    EXPECT_NE!(-1, cap_set_proc(cap_p));
    EXPECT_NE!(-1, cap_free(cap_p));
}

unsafe fn test_secbits_set(secbits: c_uint) -> c_int {
    let err: c_int;

    err = prctl(PR_SET_SECUREBITS, secbits);
    if err != 0 {
        return errno;
    }
    0
}

#[repr(C)]
struct access {
    memfd: c_int,
    pipefd: c_int,
    pipe_fds: [c_int; 2],
    socket_fds: [c_int; 2],
}

#[repr(C)]
struct access_variant {
    mount_exec: bool,
    file_exec: bool,
}

static mount_exec_file_exec: access_variant = access_variant {
    mount_exec: true,
    file_exec: true,
};

static mount_exec_file_noexec: access_variant = access_variant {
    mount_exec: true,
    file_exec: false,
};

static mount_noexec_file_exec: access_variant = access_variant {
    mount_exec: false,
    file_exec: true,
};

static mount_noexec_file_noexec: access_variant = access_variant {
    mount_exec: false,
    file_exec: false,
};

static binary_path: &[u8] = b"./false\0";
static workdir_path: &[u8] = b"./test-mount\0";
static reg_file_path: &[u8] = b"./test-mount/regular_file\0";
static dir_path: &[u8] = b"./test-mount/directory\0";
static block_dev_path: &[u8] = b"./test-mount/block_device\0";
static char_dev_path: &[u8] = b"./test-mount/character_device\0";
static fifo_path: &[u8] = b"./test-mount/fifo\0";

unsafe fn access_setup(
    _metadata: *mut __test_metadata,
    self_: *mut access,
    variant: *const access_variant,
) {
    let mut procfd_path_size: c_int;
    static path_template: &[u8] = b"/proc/self/fd/%d\0";
    let mut procfd_path = [0 as c_char; path_template.len() + 10];

    /* Makes sure we are not already restricted nor locked. */
    EXPECT_EQ!(0, test_secbits_set(0));

    /*
     * Cleans previous workspace if any error previously happened (don't
     * check errors).
     */
    umount(workdir_path.as_ptr() as *const c_char);
    rmdir(workdir_path.as_ptr() as *const c_char);

    /* Creates a clean mount point. */
    ASSERT_EQ!(0, mkdir(workdir_path.as_ptr() as *const c_char, 0o0700));
    ASSERT_EQ!(
        0,
        mount(
            b"test\0".as_ptr() as *const c_char,
            workdir_path.as_ptr() as *const c_char,
            b"tmpfs\0".as_ptr() as *const c_char,
            MS_MGC_VAL | if (*variant).mount_exec { 0 } else { MS_NOEXEC },
            b"mode=0700,size=9m\0".as_ptr() as *const c_void,
        )
    );

    /* Creates a regular file. */
    ASSERT_EQ!(
        0,
        mknod(
            reg_file_path.as_ptr() as *const c_char,
            S_IFREG | if (*variant).file_exec { 0o0700 } else { 0o0600 },
            0,
        )
    );
    /* Creates a directory. */
    ASSERT_EQ!(
        0,
        mkdir(
            dir_path.as_ptr() as *const c_char,
            if (*variant).file_exec { 0o0700 } else { 0o0600 },
        )
    );
    /* Creates a character device: /dev/null. */
    ASSERT_EQ!(
        0,
        mknod(
            char_dev_path.as_ptr() as *const c_char,
            S_IFCHR | 0o0400,
            makedev(1, 3),
        )
    );
    /* Creates a block device: /dev/loop0 */
    ASSERT_EQ!(
        0,
        mknod(
            block_dev_path.as_ptr() as *const c_char,
            S_IFBLK | 0o0400,
            makedev(7, 0),
        )
    );
    /* Creates a fifo. */
    ASSERT_EQ!(
        0,
        mknod(fifo_path.as_ptr() as *const c_char, S_IFIFO | 0o0600, 0)
    );

    /* Creates a regular file without user mount point. */
    (*self_).memfd = memfd_create(b"test-exec-probe\0".as_ptr() as *const c_char, MFD_CLOEXEC);
    ASSERT_LE!(0, (*self_).memfd);
    /* Sets mode, which must be ignored by the exec check. */
    ASSERT_EQ!(
        0,
        fchmod((*self_).memfd, if (*variant).file_exec { 0o0700 } else { 0o0600 })
    );

    /* Creates a pipefs file descriptor. */
    ASSERT_EQ!(0, pipe((*self_).pipe_fds.as_mut_ptr()));
    procfd_path_size = snprintf(
        procfd_path.as_mut_ptr(),
        procfd_path.len(),
        path_template.as_ptr() as *const c_char,
        (*self_).pipe_fds[0],
    );
    ASSERT_LT!(procfd_path_size, procfd_path.len());
    (*self_).pipefd = open(procfd_path.as_ptr(), O_RDWR | O_CLOEXEC);
    ASSERT_LE!(0, (*self_).pipefd);
    ASSERT_EQ!(
        0,
        fchmod((*self_).pipefd, if (*variant).file_exec { 0o0700 } else { 0o0600 })
    );

    /* Creates a socket file descriptor. */
    ASSERT_EQ!(
        0,
        socketpair(
            AF_UNIX,
            SOCK_DGRAM | SOCK_CLOEXEC,
            0,
            (*self_).socket_fds.as_mut_ptr(),
        )
    );
}

unsafe fn access_teardown_parent(_metadata: *mut __test_metadata) {
    /* There is no need to unlink the test files. */
    EXPECT_EQ!(0, umount(workdir_path.as_ptr() as *const c_char));
    EXPECT_EQ!(0, rmdir(workdir_path.as_ptr() as *const c_char));
}

unsafe fn fill_exec_fd(_metadata: *mut __test_metadata, fd_out: c_int) {
    let mut buf = [0u8; 1024];
    let mut len: ssize_t;
    let fd_in: c_int;

    fd_in = open(binary_path.as_ptr() as *const c_char, O_CLOEXEC | O_RDONLY);
    ASSERT_LE!(0, fd_in);
    /* Cannot use copy_file_range(2) because of EXDEV. */
    len = read(fd_in, buf.as_mut_ptr() as *mut c_void, buf.len());
    EXPECT_LE!(0, len);
    while len > 0 {
        let write_ret = write(fd_out, buf.as_ptr() as *const c_void, len as size_t);
        EXPECT_EQ!(len, write_ret);
        if len != write_ret {
            TH_LOG!(
                b"Failed to write: %s (%d)\0".as_ptr() as *const c_char,
                strerror(errno),
                errno
            );
        }
        len = read(fd_in, buf.as_mut_ptr() as *mut c_void, buf.len());
        EXPECT_LE!(0, len);
    }
    EXPECT_EQ!(0, close(fd_in));
}

unsafe fn fill_exec_path(_metadata: *mut __test_metadata, path: *const c_char) {
    let fd_out: c_int;

    fd_out = open(path, O_CLOEXEC | O_WRONLY);
    ASSERT_LE!(0, fd_out);
    if fd_out < 0 {
        TH_LOG!(
            b"Failed to open %s: %s\0".as_ptr() as *const c_char,
            path,
            strerror(errno)
        );
    }
    fill_exec_fd(_metadata, fd_out);
    EXPECT_EQ!(0, close(fd_out));
}

unsafe fn test_exec_fd(_metadata: *mut __test_metadata, fd: c_int, err_code: c_int) {
    let mut argv: [*mut c_char; 2] = [b"\0".as_ptr() as *mut c_char, core::ptr::null_mut()];
    let access_ret: c_int;
    let access_errno: c_int;

    /*
     * If we really execute fd, filled with the "false" binary, the current
     * thread will exits with an error, which will be interpreted by the
     * test framework as an error.  With AT_EXECVE_CHECK, we only check a
     * potential successful execution.
     */
    access_ret = sys_execveat(
        fd,
        b"\0".as_ptr() as *const c_char,
        argv.as_mut_ptr(),
        core::ptr::null_mut(),
        AT_EMPTY_PATH | AT_EXECVE_CHECK,
    );
    access_errno = errno;
    if err_code != 0 {
        EXPECT_EQ!(-1, access_ret);
        EXPECT_EQ!(err_code, access_errno);
        if err_code != access_errno {
            TH_LOG!(
                b"Wrong error for execveat(2): %s (%d)\0".as_ptr() as *const c_char,
                strerror(access_errno),
                errno
            );
        }
    } else {
        EXPECT_EQ!(0, access_ret);
        if access_ret != 0 {
            TH_LOG!(
                b"Access denied: %s\0".as_ptr() as *const c_char,
                strerror(access_errno)
            );
        }
    }
}

unsafe fn test_exec_path(_metadata: *mut __test_metadata, path: *const c_char, err_code: c_int) {
    let mut flags: c_int = O_CLOEXEC;
    let fd: c_int;

    /* Do not block on pipes. */
    if path == fifo_path.as_ptr() as *const c_char {
        flags |= O_NONBLOCK;
    }

    fd = open(path, flags | O_RDONLY);
    ASSERT_LE!(0, fd);
    if fd < 0 {
        TH_LOG!(
            b"Failed to open %s: %s\0".as_ptr() as *const c_char,
            path,
            strerror(errno)
        );
    }
    test_exec_fd(_metadata, fd, err_code);
    EXPECT_EQ!(0, close(fd));
}

/* Tests that we don't get ENOEXEC. */
unsafe fn access_regular_file_empty(
    _metadata: *mut __test_metadata,
    self_: *mut access,
    variant: *const access_variant,
) {
    let exec: c_int = ((*variant).mount_exec && (*variant).file_exec) as c_int;

    test_exec_path(
        _metadata,
        reg_file_path.as_ptr() as *const c_char,
        if exec != 0 { 0 } else { EACCES },
    );

    drop_privileges(_metadata);
    test_exec_path(
        _metadata,
        reg_file_path.as_ptr() as *const c_char,
        if exec != 0 { 0 } else { EACCES },
    );
}

unsafe fn access_regular_file_elf(
    _metadata: *mut __test_metadata,
    self_: *mut access,
    variant: *const access_variant,
) {
    let exec: c_int = ((*variant).mount_exec && (*variant).file_exec) as c_int;

    fill_exec_path(_metadata, reg_file_path.as_ptr() as *const c_char);

    test_exec_path(
        _metadata,
        reg_file_path.as_ptr() as *const c_char,
        if exec != 0 { 0 } else { EACCES },
    );

    drop_privileges(_metadata);
    test_exec_path(
        _metadata,
        reg_file_path.as_ptr() as *const c_char,
        if exec != 0 { 0 } else { EACCES },
    );
}

/* Tests that we don't get ENOEXEC. */
unsafe fn access_memfd_empty(
    _metadata: *mut __test_metadata,
    self_: *mut access,
    variant: *const access_variant,
) {
    let exec: c_int = (*variant).file_exec as c_int;

    test_exec_fd(_metadata, (*self_).memfd, if exec != 0 { 0 } else { EACCES });

    drop_privileges(_metadata);
    test_exec_fd(_metadata, (*self_).memfd, if exec != 0 { 0 } else { EACCES });
}

unsafe fn access_memfd_elf(
    _metadata: *mut __test_metadata,
    self_: *mut access,
    variant: *const access_variant,
) {
    let exec: c_int = (*variant).file_exec as c_int;

    fill_exec_fd(_metadata, (*self_).memfd);

    test_exec_fd(_metadata, (*self_).memfd, if exec != 0 { 0 } else { EACCES });

    drop_privileges(_metadata);
    test_exec_fd(_metadata, (*self_).memfd, if exec != 0 { 0 } else { EACCES });
}

unsafe fn access_non_regular_files(
    _metadata: *mut __test_metadata,
    self_: *mut access,
    variant: *const access_variant,
) {
    test_exec_path(_metadata, dir_path.as_ptr() as *const c_char, EACCES);
    test_exec_path(_metadata, block_dev_path.as_ptr() as *const c_char, EACCES);
    test_exec_path(_metadata, char_dev_path.as_ptr() as *const c_char, EACCES);
    test_exec_path(_metadata, fifo_path.as_ptr() as *const c_char, EACCES);
    test_exec_fd(_metadata, (*self_).socket_fds[0], EACCES);
    test_exec_fd(_metadata, (*self_).pipefd, EACCES);
}

#[repr(C)]
struct secbits {}

#[repr(C)]
struct secbits_variant {
    is_privileged: bool,
    error: c_int,
}

static priv_: secbits_variant = secbits_variant {
    is_privileged: true,
    error: 0,
};

static unpriv: secbits_variant = secbits_variant {
    is_privileged: false,
    error: EPERM,
};

unsafe fn secbits_setup(
    _metadata: *mut __test_metadata,
    self_: *mut secbits,
    variant: *const secbits_variant,
) {
    /* Makes sure no exec bits are set. */
    EXPECT_EQ!(0, test_secbits_set(0));
    EXPECT_EQ!(0, prctl(PR_GET_SECUREBITS));

    if !(*variant).is_privileged {
        drop_privileges(_metadata);
    }
}

unsafe fn secbits_teardown(_metadata: *mut __test_metadata, self_: *mut secbits) {}

unsafe fn secbits_legacy(
    _metadata: *mut __test_metadata,
    self_: *mut secbits,
    variant: *const secbits_variant,
) {
    EXPECT_EQ!((*variant).error, test_secbits_set(0));
}

macro_rules! CHILD {
    ($($body:tt)*) => {{
        let child: pid_t = vfork();
        EXPECT_LE!(0, child);
        if child == 0 {
            $($body)*
            _exit(0);
        }
    }};
}

unsafe fn secbits_exec(
    _metadata: *mut __test_metadata,
    self_: *mut secbits,
    variant: *const secbits_variant,
) {
    let mut secbits: c_uint = prctl(PR_GET_SECUREBITS) as c_uint;

    secbits |= SECBIT_EXEC_RESTRICT_FILE;
    EXPECT_EQ!(0, test_secbits_set(secbits));
    EXPECT_EQ!(secbits, prctl(PR_GET_SECUREBITS) as c_uint);
    CHILD!(EXPECT_EQ!(secbits, prctl(PR_GET_SECUREBITS) as c_uint););

    secbits |= SECBIT_EXEC_DENY_INTERACTIVE;
    EXPECT_EQ!(0, test_secbits_set(secbits));
    EXPECT_EQ!(secbits, prctl(PR_GET_SECUREBITS) as c_uint);
    CHILD!(EXPECT_EQ!(secbits, prctl(PR_GET_SECUREBITS) as c_uint););

    secbits &= !(SECBIT_EXEC_RESTRICT_FILE | SECBIT_EXEC_DENY_INTERACTIVE);
    EXPECT_EQ!(0, test_secbits_set(secbits));
    EXPECT_EQ!(secbits, prctl(PR_GET_SECUREBITS) as c_uint);
    CHILD!(EXPECT_EQ!(secbits, prctl(PR_GET_SECUREBITS) as c_uint););
}

unsafe fn secbits_check_locked_set(
    _metadata: *mut __test_metadata,
    self_: *mut secbits,
    variant: *const secbits_variant,
) {
    let mut secbits: c_uint = prctl(PR_GET_SECUREBITS) as c_uint;

    secbits |= SECBIT_EXEC_RESTRICT_FILE;
    EXPECT_EQ!(0, test_secbits_set(secbits));
    secbits |= SECBIT_EXEC_RESTRICT_FILE_LOCKED;
    EXPECT_EQ!(0, test_secbits_set(secbits));

    /* Checks lock set but unchanged. */
    EXPECT_EQ!((*variant).error, test_secbits_set(secbits));
    CHILD!(EXPECT_EQ!((*variant).error, test_secbits_set(secbits)););

    secbits &= !SECBIT_EXEC_RESTRICT_FILE;
    EXPECT_EQ!(EPERM, test_secbits_set(0));
    CHILD!(EXPECT_EQ!(EPERM, test_secbits_set(0)););
}

unsafe fn secbits_check_locked_unset(
    _metadata: *mut __test_metadata,
    self_: *mut secbits,
    variant: *const secbits_variant,
) {
    let mut secbits: c_uint = prctl(PR_GET_SECUREBITS) as c_uint;

    secbits |= SECBIT_EXEC_RESTRICT_FILE_LOCKED;
    EXPECT_EQ!(0, test_secbits_set(secbits));

    /* Checks lock unset but unchanged. */
    EXPECT_EQ!((*variant).error, test_secbits_set(secbits));
    CHILD!(EXPECT_EQ!((*variant).error, test_secbits_set(secbits)););

    secbits &= !SECBIT_EXEC_RESTRICT_FILE;
    EXPECT_EQ!(EPERM, test_secbits_set(0));
    CHILD!(EXPECT_EQ!(EPERM, test_secbits_set(0)););
}

unsafe fn secbits_restrict_locked_set(
    _metadata: *mut __test_metadata,
    self_: *mut secbits,
    variant: *const secbits_variant,
) {
    let mut secbits: c_uint = prctl(PR_GET_SECUREBITS) as c_uint;

    secbits |= SECBIT_EXEC_DENY_INTERACTIVE;
    EXPECT_EQ!(0, test_secbits_set(secbits));
    secbits |= SECBIT_EXEC_DENY_INTERACTIVE_LOCKED;
    EXPECT_EQ!(0, test_secbits_set(secbits));

    /* Checks lock set but unchanged. */
    EXPECT_EQ!((*variant).error, test_secbits_set(secbits));
    CHILD!(EXPECT_EQ!((*variant).error, test_secbits_set(secbits)););

    secbits &= !SECBIT_EXEC_DENY_INTERACTIVE;
    EXPECT_EQ!(EPERM, test_secbits_set(0));
    CHILD!(EXPECT_EQ!(EPERM, test_secbits_set(0)););
}

unsafe fn secbits_restrict_locked_unset(
    _metadata: *mut __test_metadata,
    self_: *mut secbits,
    variant: *const secbits_variant,
) {
    let mut secbits: c_uint = prctl(PR_GET_SECUREBITS) as c_uint;

    secbits |= SECBIT_EXEC_DENY_INTERACTIVE_LOCKED;
    EXPECT_EQ!(0, test_secbits_set(secbits));

    /* Checks lock unset but unchanged. */
    EXPECT_EQ!((*variant).error, test_secbits_set(secbits));
    CHILD!(EXPECT_EQ!((*variant).error, test_secbits_set(secbits)););

    secbits &= !SECBIT_EXEC_DENY_INTERACTIVE;
    EXPECT_EQ!(EPERM, test_secbits_set(0));
    CHILD!(EXPECT_EQ!(EPERM, test_secbits_set(0)););
}

/* TEST_HARNESS_MAIN */
