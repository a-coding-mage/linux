// SPDX-License-Identifier: GPL-2.0

// C dependencies translated from:
// errno.h, fcntl.h, linux/kernel.h, limits.h, stdbool.h, stdio.h, stdlib.h,
// string.h, syscall.h, unistd.h, sys/resource.h, linux/close_range.h,
// kselftest_harness.h, and ../clone3/clone3_selftests.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::ffi::c_char;
use std::os::raw::{c_int, c_long, c_uint, c_void};

type pid_t = c_int;
type ssize_t = isize;
type size_t = usize;
type rlim_t = u64;

const CLONE_FILES: u64 = 0x0000_0400;
const SIGCHLD: u64 = 17;
const O_RDWR: c_int = 0o2;
const SEEK_SET: c_int = 0;
const RLIMIT_NOFILE: c_int = 7;
const EMFILE: c_int = 24;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

#[repr(C)]
struct __clone_args {
    flags: u64,
    pidfd: u64,
    child_tid: u64,
    parent_tid: u64,
    exit_signal: u64,
    stack: u64,
    stack_size: u64,
    tls: u64,
    set_tid: u64,
    set_tid_size: u64,
    cgroup: u64,
}

#[repr(C)]
struct rlimit {
    rlim_cur: rlim_t,
    rlim_max: rlim_t,
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn sys_clone3(args: *mut __clone_args, size: size_t) -> pid_t;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {
        assert!($left > $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert!($left == $right)
    };
}

// TEST(unshare_EMFILE)
#[no_mangle]
pub unsafe extern "C" fn unshare_EMFILE() {
    let mut pid: pid_t;
    let mut status: c_int = 0;
    let mut args = __clone_args {
        flags: CLONE_FILES,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: SIGCHLD,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };
    let fd: c_int;
    let mut n: ssize_t;
    let mut n2: ssize_t;
    static mut buf: [c_char; 512] = [0; 512];
    static mut buf2: [c_char; 512] = [0; 512];
    let mut rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let mut nr_open: c_int = 0;

    fd = unsafe { open(c"/proc/sys/fs/nr_open".as_ptr(), O_RDWR) };
    ASSERT_GE!(fd, 0);

    n = unsafe {
        read(
            fd,
            (&raw mut buf).cast::<c_void>(),
            std::mem::size_of_val(&*(&raw const buf)),
        )
    };
    ASSERT_GT!(n, 0);
    ASSERT_EQ!(unsafe { buf[(n - 1) as usize] }, b'\n' as c_char);

    ASSERT_EQ!(
        unsafe { sscanf((&raw const buf).cast::<c_char>(), c"%d".as_ptr(), &mut nr_open) },
        1
    );

    ASSERT_EQ!(0, unsafe { getrlimit(RLIMIT_NOFILE, &mut rlimit) });

    /* bump fs.nr_open */
    n2 = unsafe {
        sprintf(
            (&raw mut buf2).cast::<c_char>(),
            c"%d\n".as_ptr(),
            nr_open + 1024,
        ) as ssize_t
    };
    unsafe {
        lseek(fd, 0, SEEK_SET);
        write(fd, (&raw const buf2).cast::<c_void>(), n2 as size_t);
    }

    /* bump ulimit -n */
    rlimit.rlim_cur = (nr_open + 1024) as rlim_t;
    rlimit.rlim_max = (nr_open + 1024) as rlim_t;
    if unsafe { setrlimit(RLIMIT_NOFILE, &rlimit) } != 0 {
        unsafe {
            lseek(fd, 0, SEEK_SET);
            write(fd, (&raw const buf).cast::<c_void>(), n as size_t);
            exit(EXIT_FAILURE);
        }
    }

    /* get a descriptor past the old fs.nr_open */
    if unsafe { dup2(2, nr_open + 64) } < 0 {
        unsafe {
            lseek(fd, 0, SEEK_SET);
            write(fd, (&raw const buf).cast::<c_void>(), n as size_t);
            exit(EXIT_FAILURE);
        }
    }

    /* get descriptor table shared */
    pid = unsafe { sys_clone3(&mut args, std::mem::size_of_val(&args)) };
    if pid < 0 {
        unsafe {
            lseek(fd, 0, SEEK_SET);
            write(fd, (&raw const buf).cast::<c_void>(), n as size_t);
            exit(EXIT_FAILURE);
        }
    }

    if pid == 0 {
        let err: c_int;

        /* restore fs.nr_open */
        unsafe {
            lseek(fd, 0, SEEK_SET);
            write(fd, (&raw const buf).cast::<c_void>(), n as size_t);
        }
        /* ... and now unshare(CLONE_FILES) must fail with EMFILE */
        err = unsafe { unshare(CLONE_FILES as c_int) };
        if err != -1 {
            unsafe { exit(EXIT_FAILURE) };
        }
        if unsafe { errno_value() } != EMFILE {
            unsafe { exit(EXIT_FAILURE) };
        }
        unsafe { exit(EXIT_SUCCESS) };
    }

    ASSERT_EQ!(unsafe { waitpid(pid, &mut status, 0) }, pid);
    ASSERT_EQ!(true, WIFEXITED(status));
    ASSERT_EQ!(0, WEXITSTATUS(status));
}

// TEST_HARNESS_MAIN
