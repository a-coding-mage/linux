// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Author: Aleksa Sarai <cyphar@cyphar.com>
 * Copyright (C) 2018-2019 SUSE LLC.
 */

// C dependencies: <fcntl.h>, <sched.h>, <sys/stat.h>, <sys/types.h>,
// <sys/mount.h>, <stdlib.h>, <stdbool.h>, <string.h>, "helpers.h",
// "kselftest_harness.h".

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
struct open_how {
    flags: c_ulonglong,
    mode: c_ulonglong,
    resolve: c_ulonglong,
}

#[repr(C)]
union resolve_test_out {
    err: c_int,
    path: *const c_char,
}

#[repr(C)]
struct resolve_test {
    name: *const c_char,
    dir: *const c_char,
    path: *const c_char,
    how: open_how,
    pass: bool,
    out: resolve_test_out,
}

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct openat2_resolve {
    rootfd: c_int,
    hardcoded_fd: c_int,
    hardcoded_fdpath: *mut c_char,
    procselfexe: *mut c_char,
}

const O_RDONLY: c_int = 0;
const O_CREAT: c_ulonglong = 0o100;
const O_NOFOLLOW: c_ulonglong = 0o400000;
const O_PATH: c_int = 0o10000000;
const O_DIRECTORY: c_int = 0o200000;
const CLONE_NEWNS: c_int = 0x00020000;
const MS_RDONLY: c_ulonglong = 1;
const MS_NOSUID: c_ulonglong = 2;
const MS_NODEV: c_ulonglong = 4;
const MS_PRIVATE: c_ulonglong = 1 << 18;
const EXDEV: c_int = 18;
const ELOOP: c_int = 40;
const RESOLVE_NO_XDEV: c_ulonglong = 0x01;
const RESOLVE_NO_MAGICLINKS: c_ulonglong = 0x02;
const RESOLVE_NO_SYMLINKS: c_ulonglong = 0x04;
const RESOLVE_BENEATH: c_ulonglong = 0x08;
const RESOLVE_IN_ROOT: c_ulonglong = 0x10;

unsafe extern "C" {
    static openat2_supported: bool;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup(oldfd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn strerror(errnum: c_int) -> *mut c_char;
    fn geteuid() -> c_uint;
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulonglong,
        data: *const c_void,
    ) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: c_uint) -> c_int;
    fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn getpid() -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn sys_openat2(dfd: c_int, path: *const c_char, how: *const open_how) -> c_int;
    fn fdequal(
        _metadata: *mut __test_metadata,
        fd: c_int,
        rootfd: c_int,
        path: *const c_char,
    ) -> bool;
    fn fdreadlink(_metadata: *mut __test_metadata, fd: c_int) -> *mut c_char;
    fn touchat(dfd: c_int, path: *const c_char) -> c_int;
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! TH_LOG {
    ($($tt:tt)*) => {};
}
macro_rules! ASSERT_GE {
    ($($tt:tt)*) => {};
}
macro_rules! ASSERT_EQ {
    ($($tt:tt)*) => {};
}
macro_rules! ASSERT_NE {
    ($($tt:tt)*) => {};
}
macro_rules! EXPECT_GE {
    ($($tt:tt)*) => {};
}
macro_rules! EXPECT_EQ {
    ($($tt:tt)*) => {};
}
macro_rules! EXPECT_TRUE {
    ($($tt:tt)*) => {};
}
macro_rules! SKIP {
    ($($tt:tt)*) => {
        return
    };
}

macro_rules! rt_err {
    ($name:literal, $dir:expr, $path:expr, $flags:expr, $mode:expr, $resolve:expr, $err:expr) => {
        resolve_test {
            name: c!($name),
            dir: $dir,
            path: $path,
            how: open_how {
                flags: $flags as c_ulonglong,
                mode: $mode as c_ulonglong,
                resolve: $resolve as c_ulonglong,
            },
            pass: false,
            out: resolve_test_out { err: $err },
        }
    };
}

macro_rules! rt_path {
    ($name:literal, $dir:expr, $path:expr, $flags:expr, $mode:expr, $resolve:expr, $out:expr) => {
        resolve_test {
            name: c!($name),
            dir: $dir,
            path: $path,
            how: open_how {
                flags: $flags as c_ulonglong,
                mode: $mode as c_ulonglong,
                resolve: $resolve as c_ulonglong,
            },
            pass: true,
            out: resolve_test_out { path: $out },
        }
    };
}

/*
 * Verify a single resolve test case. This must be called from within a TEST_F
 * function with _metadata in scope.
 */
unsafe fn verify_resolve_test(
    _metadata: *mut __test_metadata,
    rootfd: c_int,
    hardcoded_fd: c_int,
    test: *const resolve_test,
) {
    let test = &*test;
    let mut how: open_how = test.how;
    let dfd: c_int;
    let fd: c_int;
    let mut fdpath: *mut c_char = ptr::null_mut();

    /* Auto-set O_PATH. */
    if (how.flags & O_CREAT) == 0 {
        how.flags |= O_PATH as c_ulonglong;
    }

    if !test.dir.is_null() {
        dfd = openat(rootfd, test.dir, O_PATH | O_DIRECTORY);
    } else {
        dfd = dup(rootfd);
    }
    ASSERT_GE!(dfd, 0);
    TH_LOG!("failed to open dir '%s': %m", if !test.dir.is_null() { test.dir } else { c!(".") });
    ASSERT_EQ!(dup2(dfd, hardcoded_fd), hardcoded_fd);

    fd = sys_openat2(dfd, test.path, &how);

    if test.pass {
        EXPECT_GE!(fd, 0);
        if fd < 0 {
            TH_LOG!(
                "%s: expected success, got %d (%s)",
                test.name,
                fd,
                strerror(-fd)
            );
        }
        if fd >= 0 {
            EXPECT_TRUE!(fdequal(_metadata, fd, rootfd, test.out.path));
            if !fdequal(_metadata, fd, rootfd, test.out.path) {
                fdpath = fdreadlink(_metadata, fd);
                TH_LOG!(
                    "%s: wrong path '%s', expected '%s'",
                    test.name,
                    fdpath,
                    if !test.out.path.is_null() { test.out.path } else { c!(".") }
                );
                free(fdpath as *mut c_void);
            }
        }
    } else {
        EXPECT_EQ!(test.out.err, fd);
        if test.out.err != fd {
            if fd >= 0 {
                fdpath = fdreadlink(_metadata, fd);
                TH_LOG!(
                    "%s: expected %d (%s), got %d['%s']",
                    test.name,
                    test.out.err,
                    strerror(-test.out.err),
                    fd,
                    fdpath
                );
                free(fdpath as *mut c_void);
            } else {
                TH_LOG!(
                    "%s: expected %d (%s), got %d (%s)",
                    test.name,
                    test.out.err,
                    strerror(-test.out.err),
                    fd,
                    strerror(-fd)
                );
            }
        }
    }

    if fd >= 0 {
        close(fd);
    }
    close(dfd);
}

/*
 * Construct a test directory with the following structure:
 *
 * root/
 * |-- procexe -> /proc/self/exe
 * |-- procroot -> /proc/self/root
 * |-- root/
 * |-- mnt/ [mountpoint]
 * |   |-- self -> ../mnt/
 * |   `-- absself -> /mnt/
 * |-- etc/
 * |   `-- passwd
 * |-- creatlink -> /newfile3
 * |-- reletc -> etc/
 * |-- relsym -> etc/passwd
 * |-- absetc -> /etc/
 * |-- abssym -> /etc/passwd
 * |-- abscheeky -> /cheeky
 * `-- cheeky/
 *     |-- absself -> /
 *     |-- self -> ../../root/
 *     |-- garbageself -> /../../root/
 *     |-- passwd -> ../cheeky/../etc/../etc/passwd
 *     |-- abspasswd -> /../cheeky/../etc/../etc/passwd
 *     |-- dotdotlink -> ../../../../../../../../../../../../../../etc/passwd
 *     `-- garbagelink -> /../../../../../../../../../../../../../../etc/passwd
 */
unsafe fn openat2_resolve_setup(self_: *mut openat2_resolve) {
    let mut dirname = *b"/tmp/ksft-openat2-testdir.XXXXXX\0";
    let mut dfd: c_int;
    let mut tmpfd: c_int;

    (*self_).rootfd = -1;
    (*self_).hardcoded_fd = -1;
    (*self_).hardcoded_fdpath = ptr::null_mut();
    (*self_).procselfexe = ptr::null_mut();

    /* NOTE: We should be checking for CAP_SYS_ADMIN here... */
    if geteuid() != 0 {
        SKIP!(return, "all tests require euid == 0");
    }
    if !openat2_supported {
        SKIP!(return, "openat2(2) not supported");
    }

    /* Unshare and make /tmp a new directory. */
    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(mount(c!(""), c!("/tmp"), c!(""), MS_PRIVATE, c!("") as *const c_void), 0);

    /* Make the top-level directory. */
    ASSERT_NE!(mkdtemp(dirname.as_mut_ptr() as *mut c_char), ptr::null_mut());
    dfd = open(dirname.as_ptr() as *const c_char, O_PATH | O_DIRECTORY);
    ASSERT_GE!(dfd, 0);

    /* A sub-directory which is actually used for tests. */
    ASSERT_EQ!(mkdirat(dfd, c!("root"), 0o755), 0);
    tmpfd = openat(dfd, c!("root"), O_PATH | O_DIRECTORY);
    ASSERT_GE!(tmpfd, 0);
    close(dfd);
    dfd = tmpfd;

    ASSERT_EQ!(symlinkat(c!("/proc/self/exe"), dfd, c!("procexe")), 0);
    ASSERT_EQ!(symlinkat(c!("/proc/self/root"), dfd, c!("procroot")), 0);
    ASSERT_EQ!(mkdirat(dfd, c!("root"), 0o755), 0);

    /* There is no mountat(2), so use chdir. */
    ASSERT_EQ!(mkdirat(dfd, c!("mnt"), 0o755), 0);
    ASSERT_EQ!(fchdir(dfd), 0);
    ASSERT_EQ!(mount(c!("tmpfs"), c!("./mnt"), c!("tmpfs"), MS_NOSUID | MS_NODEV, c!("") as *const c_void), 0);
    ASSERT_EQ!(symlinkat(c!("../mnt/"), dfd, c!("mnt/self")), 0);
    ASSERT_EQ!(symlinkat(c!("/mnt/"), dfd, c!("mnt/absself")), 0);

    ASSERT_EQ!(mkdirat(dfd, c!("etc"), 0o755), 0);
    ASSERT_GE!(touchat(dfd, c!("etc/passwd")), 0);

    ASSERT_EQ!(symlinkat(c!("/newfile3"), dfd, c!("creatlink")), 0);
    ASSERT_EQ!(symlinkat(c!("etc/"), dfd, c!("reletc")), 0);
    ASSERT_EQ!(symlinkat(c!("etc/passwd"), dfd, c!("relsym")), 0);
    ASSERT_EQ!(symlinkat(c!("/etc/"), dfd, c!("absetc")), 0);
    ASSERT_EQ!(symlinkat(c!("/etc/passwd"), dfd, c!("abssym")), 0);
    ASSERT_EQ!(symlinkat(c!("/cheeky"), dfd, c!("abscheeky")), 0);

    ASSERT_EQ!(mkdirat(dfd, c!("cheeky"), 0o755), 0);

    ASSERT_EQ!(symlinkat(c!("/"), dfd, c!("cheeky/absself")), 0);
    ASSERT_EQ!(symlinkat(c!("../../root/"), dfd, c!("cheeky/self")), 0);
    ASSERT_EQ!(symlinkat(c!("/../../root/"), dfd, c!("cheeky/garbageself")), 0);

    ASSERT_EQ!(symlinkat(c!("../cheeky/../etc/../etc/passwd"), dfd, c!("cheeky/passwd")), 0);
    ASSERT_EQ!(symlinkat(c!("/../cheeky/../etc/../etc/passwd"), dfd, c!("cheeky/abspasswd")), 0);

    ASSERT_EQ!(symlinkat(c!("../../../../../../../../../../../../../../etc/passwd"), dfd, c!("cheeky/dotdotlink")), 0);
    ASSERT_EQ!(symlinkat(c!("/../../../../../../../../../../../../../../etc/passwd"), dfd, c!("cheeky/garbagelink")), 0);

    (*self_).rootfd = dfd;

    (*self_).hardcoded_fd = open(c!("/dev/null"), O_RDONLY);
    ASSERT_GE!((*self_).hardcoded_fd, 0);
    ASSERT_GE!(asprintf(&mut (*self_).hardcoded_fdpath, c!("self/fd/%d"), (*self_).hardcoded_fd), 0);
    ASSERT_GE!(asprintf(&mut (*self_).procselfexe, c!("/proc/%d/exe"), getpid()), 0);
}

unsafe fn openat2_resolve_teardown(self_: *mut openat2_resolve) {
    free((*self_).procselfexe as *mut c_void);
    free((*self_).hardcoded_fdpath as *mut c_void);
    if (*self_).hardcoded_fd >= 0 {
        close((*self_).hardcoded_fd);
    }
    if (*self_).rootfd >= 0 {
        close((*self_).rootfd);
    }
}

unsafe fn run_resolve_tests(
    _metadata: *mut __test_metadata,
    self_: *mut openat2_resolve,
    tests: &[resolve_test],
) {
    for i in 0..tests.len() {
        verify_resolve_test(
            _metadata,
            (*self_).rootfd,
            (*self_).hardcoded_fd,
            &tests[i],
        );
    }
}

/* Attempts to cross the dirfd should be blocked with -EXDEV. */
unsafe fn test_openat2_resolve_resolve_beneath(
    _metadata: *mut __test_metadata,
    self_: *mut openat2_resolve,
) {
    let tests = [
        rt_err!("[beneath] jump to /", ptr::null(), c!("/"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] absolute link to $root", ptr::null(), c!("cheeky/absself"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] chained absolute links to $root", ptr::null(), c!("abscheeky/absself"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] jump outside $root", ptr::null(), c!(".."), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] temporary jump outside $root", ptr::null(), c!("../root/"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] symlink temporary jump outside $root", ptr::null(), c!("cheeky/self"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] chained symlink temporary jump outside $root", ptr::null(), c!("abscheeky/self"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] garbage links to $root", ptr::null(), c!("cheeky/garbageself"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] chained garbage links to $root", ptr::null(), c!("abscheeky/garbageself"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        /* Only relative paths that stay inside dirfd should work. */
        rt_path!("[beneath] ordinary path to 'root'", ptr::null(), c!("root"), 0, 0, RESOLVE_BENEATH, c!("root")),
        rt_path!("[beneath] ordinary path to 'etc'", ptr::null(), c!("etc"), 0, 0, RESOLVE_BENEATH, c!("etc")),
        rt_path!("[beneath] ordinary path to 'etc/passwd'", ptr::null(), c!("etc/passwd"), 0, 0, RESOLVE_BENEATH, c!("etc/passwd")),
        rt_path!("[beneath] relative symlink inside $root", ptr::null(), c!("relsym"), 0, 0, RESOLVE_BENEATH, c!("etc/passwd")),
        rt_path!("[beneath] chained-'..' relative symlink inside $root", ptr::null(), c!("cheeky/passwd"), 0, 0, RESOLVE_BENEATH, c!("etc/passwd")),
        rt_err!("[beneath] absolute symlink component outside $root", ptr::null(), c!("abscheeky/passwd"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] absolute symlink target outside $root", ptr::null(), c!("abssym"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] absolute path outside $root", ptr::null(), c!("/etc/passwd"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] cheeky absolute path outside $root", ptr::null(), c!("cheeky/abspasswd"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] chained cheeky absolute path outside $root", ptr::null(), c!("abscheeky/abspasswd"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        /* Tricky paths should fail. */
        rt_err!("[beneath] tricky '..'-chained symlink outside $root", ptr::null(), c!("cheeky/dotdotlink"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] tricky absolute + '..'-chained symlink outside $root", ptr::null(), c!("abscheeky/dotdotlink"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] tricky garbage link outside $root", ptr::null(), c!("cheeky/garbagelink"), 0, 0, RESOLVE_BENEATH, -EXDEV),
        rt_err!("[beneath] tricky absolute + garbage link outside $root", ptr::null(), c!("abscheeky/garbagelink"), 0, 0, RESOLVE_BENEATH, -EXDEV),
    ];

    run_resolve_tests(_metadata, self_, &tests);
}

/* All attempts to cross the dirfd will be scoped-to-root. */
unsafe fn test_openat2_resolve_resolve_in_root(
    _metadata: *mut __test_metadata,
    self_: *mut openat2_resolve,
) {
    let tests = [
        rt_path!("[in_root] jump to /", ptr::null(), c!("/"), 0, 0, RESOLVE_IN_ROOT, ptr::null()),
        rt_path!("[in_root] absolute symlink to /root", ptr::null(), c!("cheeky/absself"), 0, 0, RESOLVE_IN_ROOT, ptr::null()),
        rt_path!("[in_root] chained absolute symlinks to /root", ptr::null(), c!("abscheeky/absself"), 0, 0, RESOLVE_IN_ROOT, ptr::null()),
        rt_path!("[in_root] '..' at root", ptr::null(), c!(".."), 0, 0, RESOLVE_IN_ROOT, ptr::null()),
        rt_path!("[in_root] '../root' at root", ptr::null(), c!("../root/"), 0, 0, RESOLVE_IN_ROOT, c!("root")),
        rt_path!("[in_root] relative symlink containing '..' above root", ptr::null(), c!("cheeky/self"), 0, 0, RESOLVE_IN_ROOT, c!("root")),
        rt_path!("[in_root] garbage link to /root", ptr::null(), c!("cheeky/garbageself"), 0, 0, RESOLVE_IN_ROOT, c!("root")),
        rt_path!("[in_root] chained garbage links to /root", ptr::null(), c!("abscheeky/garbageself"), 0, 0, RESOLVE_IN_ROOT, c!("root")),
        rt_path!("[in_root] relative path to 'root'", ptr::null(), c!("root"), 0, 0, RESOLVE_IN_ROOT, c!("root")),
        rt_path!("[in_root] relative path to 'etc'", ptr::null(), c!("etc"), 0, 0, RESOLVE_IN_ROOT, c!("etc")),
        rt_path!("[in_root] relative path to 'etc/passwd'", ptr::null(), c!("etc/passwd"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] relative symlink to 'etc/passwd'", ptr::null(), c!("relsym"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] chained-'..' relative symlink to 'etc/passwd'", ptr::null(), c!("cheeky/passwd"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] chained-'..' absolute + relative symlink to 'etc/passwd'", ptr::null(), c!("abscheeky/passwd"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] absolute symlink to 'etc/passwd'", ptr::null(), c!("abssym"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] absolute path 'etc/passwd'", ptr::null(), c!("/etc/passwd"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] cheeky absolute path 'etc/passwd'", ptr::null(), c!("cheeky/abspasswd"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] chained cheeky absolute path 'etc/passwd'", ptr::null(), c!("abscheeky/abspasswd"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] tricky '..'-chained symlink outside $root", ptr::null(), c!("cheeky/dotdotlink"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] tricky absolute + '..'-chained symlink outside $root", ptr::null(), c!("abscheeky/dotdotlink"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] tricky absolute path + absolute + '..'-chained symlink outside $root", ptr::null(), c!("/../../../../abscheeky/dotdotlink"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] tricky garbage link outside $root", ptr::null(), c!("cheeky/garbagelink"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] tricky absolute + garbage link outside $root", ptr::null(), c!("abscheeky/garbagelink"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        rt_path!("[in_root] tricky absolute path + absolute + garbage link outside $root", ptr::null(), c!("/../../../../abscheeky/garbagelink"), 0, 0, RESOLVE_IN_ROOT, c!("etc/passwd")),
        /* O_CREAT should handle trailing symlinks correctly. */
        rt_path!("[in_root] O_CREAT of relative path inside $root", ptr::null(), c!("newfile1"), O_CREAT, 0o700, RESOLVE_IN_ROOT, c!("newfile1")),
        rt_path!("[in_root] O_CREAT of absolute path", ptr::null(), c!("/newfile2"), O_CREAT, 0o700, RESOLVE_IN_ROOT, c!("newfile2")),
        rt_path!("[in_root] O_CREAT of tricky symlink outside root", ptr::null(), c!("/creatlink"), O_CREAT, 0o700, RESOLVE_IN_ROOT, c!("newfile3")),
    ];

    run_resolve_tests(_metadata, self_, &tests);
}

/* Crossing mount boundaries should be blocked. */
unsafe fn test_openat2_resolve_resolve_no_xdev(
    _metadata: *mut __test_metadata,
    self_: *mut openat2_resolve,
) {
    let tests = [
        /* Crossing *down* into a mountpoint is disallowed. */
        rt_err!("[no_xdev] cross into $mnt", ptr::null(), c!("mnt"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        rt_err!("[no_xdev] cross into $mnt/", ptr::null(), c!("mnt/"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        rt_err!("[no_xdev] cross into $mnt/.", ptr::null(), c!("mnt/."), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        /* Crossing *up* out of a mountpoint is disallowed. */
        rt_path!("[no_xdev] goto mountpoint root", c!("mnt"), c!("."), 0, 0, RESOLVE_NO_XDEV, c!("mnt")),
        rt_err!("[no_xdev] cross up through '..'", c!("mnt"), c!(".."), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        rt_err!("[no_xdev] temporary cross up through '..'", c!("mnt"), c!("../mnt"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        rt_err!("[no_xdev] temporary relative symlink cross up", c!("mnt"), c!("self"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        rt_err!("[no_xdev] temporary absolute symlink cross up", c!("mnt"), c!("absself"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        /* Jumping to "/" is ok, but later components cannot cross. */
        rt_path!("[no_xdev] jump to / directly", c!("mnt"), c!("/"), 0, 0, RESOLVE_NO_XDEV, c!("/")),
        rt_path!("[no_xdev] jump to / (from /) directly", c!("/"), c!("/"), 0, 0, RESOLVE_NO_XDEV, c!("/")),
        rt_err!("[no_xdev] jump to / then proc", ptr::null(), c!("/proc/1"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        rt_err!("[no_xdev] jump to / then tmp", ptr::null(), c!("/tmp"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        /* Magic-links are blocked since they can switch vfsmounts. */
        rt_err!("[no_xdev] cross through magic-link to self/root", c!("/proc"), c!("self/root"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        rt_err!("[no_xdev] cross through magic-link to self/cwd", c!("/proc"), c!("self/cwd"), 0, 0, RESOLVE_NO_XDEV, -EXDEV),
        /* Except magic-link jumps inside the same vfsmount. */
        rt_path!("[no_xdev] jump through magic-link to same procfs", c!("/proc"), (*self_).hardcoded_fdpath, 0, 0, RESOLVE_NO_XDEV, c!("/proc")),
    ];

    run_resolve_tests(_metadata, self_, &tests);
}

/* Procfs-style magic-link resolution should be blocked. */
unsafe fn test_openat2_resolve_resolve_no_magiclinks(
    _metadata: *mut __test_metadata,
    self_: *mut openat2_resolve,
) {
    let tests = [
        /* Regular symlinks should work. */
        rt_path!("[no_magiclinks] ordinary relative symlink", ptr::null(), c!("relsym"), 0, 0, RESOLVE_NO_MAGICLINKS, c!("etc/passwd")),
        /* Magic-links should not work. */
        rt_err!("[no_magiclinks] symlink to magic-link", ptr::null(), c!("procexe"), 0, 0, RESOLVE_NO_MAGICLINKS, -ELOOP),
        rt_err!("[no_magiclinks] normal path to magic-link", ptr::null(), c!("/proc/self/exe"), 0, 0, RESOLVE_NO_MAGICLINKS, -ELOOP),
        rt_path!("[no_magiclinks] normal path to magic-link with O_NOFOLLOW", ptr::null(), c!("/proc/self/exe"), O_NOFOLLOW, 0, RESOLVE_NO_MAGICLINKS, (*self_).procselfexe),
        rt_err!("[no_magiclinks] symlink to magic-link path component", ptr::null(), c!("procroot/etc"), 0, 0, RESOLVE_NO_MAGICLINKS, -ELOOP),
        rt_err!("[no_magiclinks] magic-link path component", ptr::null(), c!("/proc/self/root/etc"), 0, 0, RESOLVE_NO_MAGICLINKS, -ELOOP),
        rt_err!("[no_magiclinks] magic-link path component with O_NOFOLLOW", ptr::null(), c!("/proc/self/root/etc"), O_NOFOLLOW, 0, RESOLVE_NO_MAGICLINKS, -ELOOP),
    ];

    run_resolve_tests(_metadata, self_, &tests);
}

/* All symlink resolution should be blocked. */
unsafe fn test_openat2_resolve_resolve_no_symlinks(
    _metadata: *mut __test_metadata,
    self_: *mut openat2_resolve,
) {
    let tests = [
        /* Normal paths should work. */
        rt_path!("[no_symlinks] ordinary path to '.'", ptr::null(), c!("."), 0, 0, RESOLVE_NO_SYMLINKS, ptr::null()),
        rt_path!("[no_symlinks] ordinary path to 'root'", ptr::null(), c!("root"), 0, 0, RESOLVE_NO_SYMLINKS, c!("root")),
        rt_path!("[no_symlinks] ordinary path to 'etc'", ptr::null(), c!("etc"), 0, 0, RESOLVE_NO_SYMLINKS, c!("etc")),
        rt_path!("[no_symlinks] ordinary path to 'etc/passwd'", ptr::null(), c!("etc/passwd"), 0, 0, RESOLVE_NO_SYMLINKS, c!("etc/passwd")),
        /* Regular symlinks are blocked. */
        rt_err!("[no_symlinks] relative symlink target", ptr::null(), c!("relsym"), 0, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        rt_err!("[no_symlinks] relative symlink component", ptr::null(), c!("reletc/passwd"), 0, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        rt_err!("[no_symlinks] absolute symlink target", ptr::null(), c!("abssym"), 0, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        rt_err!("[no_symlinks] absolute symlink component", ptr::null(), c!("absetc/passwd"), 0, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        rt_err!("[no_symlinks] cheeky garbage link", ptr::null(), c!("cheeky/garbagelink"), 0, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        rt_err!("[no_symlinks] cheeky absolute + garbage link", ptr::null(), c!("abscheeky/garbagelink"), 0, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        rt_err!("[no_symlinks] cheeky absolute + absolute symlink", ptr::null(), c!("abscheeky/absself"), 0, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        /* Trailing symlinks with NO_FOLLOW. */
        rt_path!("[no_symlinks] relative symlink with O_NOFOLLOW", ptr::null(), c!("relsym"), O_NOFOLLOW, 0, RESOLVE_NO_SYMLINKS, c!("relsym")),
        rt_path!("[no_symlinks] absolute symlink with O_NOFOLLOW", ptr::null(), c!("abssym"), O_NOFOLLOW, 0, RESOLVE_NO_SYMLINKS, c!("abssym")),
        rt_path!("[no_symlinks] trailing symlink with O_NOFOLLOW", ptr::null(), c!("cheeky/garbagelink"), O_NOFOLLOW, 0, RESOLVE_NO_SYMLINKS, c!("cheeky/garbagelink")),
        rt_err!("[no_symlinks] multiple symlink components with O_NOFOLLOW", ptr::null(), c!("abscheeky/absself"), O_NOFOLLOW, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
        rt_err!("[no_symlinks] multiple symlink (and garbage link) components with O_NOFOLLOW", ptr::null(), c!("abscheeky/garbagelink"), O_NOFOLLOW, 0, RESOLVE_NO_SYMLINKS, -ELOOP),
    ];

    run_resolve_tests(_metadata, self_, &tests);
}

fn main() {}
