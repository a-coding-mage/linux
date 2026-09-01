// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock filesystem benchmark
 *
 * This program benchmarks the time required for file access checks.  We use a
 * large number (-d flag) of nested directories where each directory inode has
 * an associated Landlock rule, and we repeatedly (-n flag) exercise a file
 * access for which Landlock has to walk the path all the way up to the root.
 *
 * With an increasing number of nested subdirectories, Landlock's portion of the
 * overall system call time increases, which makes the effects of Landlock
 * refactorings more measurable.
 *
 * This benchmark does *not* measure the building of the Landlock ruleset.  The
 * time required to add all these rules is not large enough to be easily
 * measurable.  A separate benchmark tool would be better to test that, and that
 * tool could then also use a simpler file system layout.
 *
 * Copyright © 2026 Google LLC
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;

type size_t = usize;
type clock_t = c_long;

#[repr(C)]
struct tms {
    tms_utime: clock_t,
    tms_stime: clock_t,
    tms_cutime: clock_t,
    tms_cstime: clock_t,
}

#[repr(C)]
struct landlock_ruleset_attr {
    handled_access_fs: u64,
}

#[repr(C)]
struct landlock_path_beneath_attr {
    allowed_access: u64,
    parent_fd: i32,
}

const LANDLOCK_CREATE_RULESET_VERSION: u32 = 1;
const LANDLOCK_RULE_PATH_BENEATH: i32 = 1;
const LANDLOCK_ACCESS_FS_WRITE_FILE: u64 = 1 << 1;
const LANDLOCK_ACCESS_FS_MAKE_REG: u64 = 1 << 8;
const LANDLOCK_ACCESS_FS_IOCTL_DEV: u64 = 1 << 15;

const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_PATH: c_int = 0o10000000;
const AT_FDCWD: c_int = -100;
const AT_REMOVEDIR: c_int = 0x200;
const EACCES: c_int = 13;
const PR_SET_NO_NEW_PRIVS: c_int = 38;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const CLOCKS_PER_SEC: c_long = 1_000_000;

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn printf(format: *const c_char, ...) -> c_int;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    static mut stdout: *mut c_void;

    fn atoi(nptr: *const c_char) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: c_uint) -> c_int;
    fn unlinkat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn prctl(option: c_int, ...) -> c_int;
    fn times(buf: *mut tms) -> clock_t;
    fn __errno_location() -> *mut c_int;

    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;

    /* From wrappers.h. */
    fn landlock_create_ruleset(
        attr: *const landlock_ruleset_attr,
        size: size_t,
        flags: u32,
    ) -> c_int;
    fn landlock_add_rule(
        ruleset_fd: c_int,
        rule_type: c_int,
        rule_attr: *const c_void,
        flags: u32,
    ) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: u32) -> c_int;
}

unsafe fn usage(argv0: *const c_char) {
    unsafe {
        printf(c"Usage:\n".as_ptr());
        printf(c"  %s [OPTIONS]\n".as_ptr(), argv0);
        printf(c"\n".as_ptr());
        printf(c"  Benchmark expensive Landlock checks for D nested dirs\n".as_ptr());
        printf(c"\n".as_ptr());
        printf(c"Options:\n".as_ptr());
        printf(c"  -h\thelp\n".as_ptr());
        printf(c"  -L\tdisable Landlock (as a baseline)\n".as_ptr());
        printf(c"  -d D\tset directory depth to D\n".as_ptr());
        printf(c"  -n N\tset number of benchmark iterations to N\n".as_ptr());
    }
}

/*
 * Build a deep directory, enforce Landlock and return the FD to the
 * deepest dir.  On any failure, exit the process with an error.
 */
unsafe fn build_directory(mut depth: size_t, use_landlock: bool) -> c_int {
    let path = c"d".as_ptr(); /* directory name */
    let mut abi: c_int;
    let mut ruleset_fd: c_int;
    let mut curr: c_int;
    let mut prev: c_int;

    unsafe {
        if use_landlock {
            abi = landlock_create_ruleset(
                core::ptr::null(),
                0,
                LANDLOCK_CREATE_RULESET_VERSION,
            );
            if abi < 7 {
                err(1, c"Landlock ABI too low: got %d, wanted 7+".as_ptr(), abi);
            }
        }

        ruleset_fd = -1;
        if use_landlock {
            let attr = landlock_ruleset_attr {
                handled_access_fs: LANDLOCK_ACCESS_FS_IOCTL_DEV
                    | LANDLOCK_ACCESS_FS_WRITE_FILE
                    | LANDLOCK_ACCESS_FS_MAKE_REG,
            };
            ruleset_fd = landlock_create_ruleset(&attr, size_of::<landlock_ruleset_attr>(), 0_u32);
            if ruleset_fd < 0 {
                err(1, c"landlock_create_ruleset".as_ptr());
            }
        }

        curr = open(c".".as_ptr(), O_PATH);
        if curr < 0 {
            err(1, c"open(.)".as_ptr());
        }

        while depth != 0 {
            depth -= 1;

            if use_landlock {
                let attr = landlock_path_beneath_attr {
                    allowed_access: LANDLOCK_ACCESS_FS_IOCTL_DEV,
                    parent_fd: curr,
                };
                if landlock_add_rule(
                    ruleset_fd,
                    LANDLOCK_RULE_PATH_BENEATH,
                    &attr as *const landlock_path_beneath_attr as *const c_void,
                    0,
                ) < 0
                {
                    err(1, c"landlock_add_rule".as_ptr());
                }
            }

            if mkdirat(curr, path, 0o700) < 0 {
                err(1, c"mkdirat(%s)".as_ptr(), path);
            }

            prev = curr;
            curr = openat(curr, path, O_PATH);
            if curr < 0 {
                err(1, c"openat(%s)".as_ptr(), path);
            }

            close(prev);
        }

        if use_landlock {
            if prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) < 0 {
                err(1, c"prctl".as_ptr());
            }

            if landlock_restrict_self(ruleset_fd, 0) < 0 {
                err(1, c"landlock_restrict_self".as_ptr());
            }
        }

        close(ruleset_fd);
        curr
    }
}

unsafe fn remove_recursively(depth: size_t) {
    let path = c"d".as_ptr(); /* directory name */

    unsafe {
        let mut fd = openat(AT_FDCWD, c".".as_ptr(), O_PATH);

        if fd < 0 {
            err(1, c"openat(.)".as_ptr());
        }

        for _i in 0..depth.wrapping_sub(1) {
            let oldfd = fd;

            fd = openat(fd, path, O_PATH);
            if fd < 0 {
                err(1, c"openat(%s)".as_ptr(), path);
            }
            close(oldfd);
        }

        for _i in 0..depth {
            if unlinkat(fd, path, AT_REMOVEDIR) < 0 {
                err(1, c"unlinkat(%s)".as_ptr(), path);
            }
            let newfd = openat(fd, c"..".as_ptr(), O_PATH);

            close(fd);
            fd = newfd;
        }
        close(fd);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut use_landlock = true;
    let mut num_iterations: size_t = 100000;
    let mut num_subdirs: size_t = 10000;
    let mut c: c_int;
    let curr: c_int;
    let mut fd: c_int;
    let mut start_time = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };
    let mut end_time = tms {
        tms_utime: 0,
        tms_stime: 0,
        tms_cutime: 0,
        tms_cstime: 0,
    };

    unsafe {
        setbuf(stdout, core::ptr::null_mut());
        loop {
            c = getopt(argc, argv, c"hLd:n:".as_ptr());
            if c == -1 {
                break;
            }
            match c {
                104 => {
                    usage(*argv);
                    return EXIT_SUCCESS;
                }
                76 => {
                    use_landlock = false;
                }
                100 => {
                    num_subdirs = atoi(optarg) as size_t;
                }
                110 => {
                    num_iterations = atoi(optarg) as size_t;
                }
                _ => {
                    usage(*argv);
                    return EXIT_FAILURE;
                }
            }
        }

        printf(c"*** Benchmark ***\n".as_ptr());
        printf(
            c"%zu dirs, %zu iterations, %s Landlock\n".as_ptr(),
            num_subdirs,
            num_iterations,
            if use_landlock {
                c"with".as_ptr()
            } else {
                c"without".as_ptr()
            },
        );

        if times(&mut start_time) == -1 {
            err(1, c"times".as_ptr());
        }

        curr = build_directory(num_subdirs, use_landlock);

        for i in 0..num_iterations {
            let _ = i;
            fd = openat(curr, c"file.txt".as_ptr(), O_CREAT | O_TRUNC | O_WRONLY, 0o600);
            if use_landlock {
                if fd == 0 {
                    errx(1, c"openat succeeded, expected EACCES".as_ptr());
                }
                if *__errno_location() != EACCES {
                    err(1, c"openat expected EACCES, but got".as_ptr());
                }
            }
            if fd != -1 {
                close(fd);
            }
        }

        if times(&mut end_time) == -1 {
            err(1, c"times".as_ptr());
        }

        printf(c"*** Benchmark concluded ***\n".as_ptr());
        printf(
            c"System: %ld clocks\n".as_ptr(),
            end_time.tms_stime - start_time.tms_stime,
        );
        printf(
            c"User  : %ld clocks\n".as_ptr(),
            end_time.tms_utime - start_time.tms_utime,
        );
        printf(c"Clocks per second: %ld\n".as_ptr(), CLOCKS_PER_SEC);

        close(curr);

        remove_recursively(num_subdirs);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
