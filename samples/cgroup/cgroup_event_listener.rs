// SPDX-License-Identifier: GPL-2.0
/*
 * cgroup_event_listener.c - Simple listener of cgroup events
 *
 * Copyright (C) Kirill A. Shutemov <kirill@shutemov.name>
 */

use std::os::raw::{c_char, c_int, c_void};

// These constants correspond to the platform limits supplied by <limits.h>.
const PATH_MAX: usize = 4096;
const LINE_MAX: usize = 2048;
const USAGE_STR: &[u8] = b"Usage: cgroup_event_listener <path-to-control-file> <args>\0";

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const W_OK: c_int = 2;
const EINTR: c_int = 4;
const ENOENT: c_int = 2;

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn eventfd(initval: u32, flags: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn err(eval: c_int, format: *const c_char, ... ) -> !;
    fn errx(eval: c_int, format: *const c_char, ... ) -> !;
}

#[inline]
unsafe fn errno() -> c_int {
    *__errno_location()
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut efd: c_int = -1;
    let mut cfd: c_int = -1;
    let mut event_control: c_int = -1;
    let mut event_control_path = [0 as c_char; PATH_MAX];
    let mut line = [0 as c_char; LINE_MAX];
    let mut ret: c_int;

    if argc != 3 {
        errx(1, b"%s\0".as_ptr() as *const c_char, USAGE_STR.as_ptr());
    }

    cfd = open(*argv.add(1), O_RDONLY);
    if cfd == -1 {
        err(1, b"Cannot open %s\0".as_ptr() as *const c_char, *argv.add(1));
    }

    ret = snprintf(
        event_control_path.as_mut_ptr(), PATH_MAX,
        b"%s/cgroup.event_control\0".as_ptr() as *const c_char,
        dirname(*argv.add(1)),
    );
    if ret >= PATH_MAX as c_int {
        errx(1, b"Path to cgroup.event_control is too long\0".as_ptr() as *const c_char);
    }

    event_control = open(event_control_path.as_ptr(), O_WRONLY);
    if event_control == -1 {
        err(1, b"Cannot open %s\0".as_ptr() as *const c_char, event_control_path.as_ptr());
    }

    efd = eventfd(0, 0);
    if efd == -1 {
        err(1, b"eventfd() failed\0".as_ptr() as *const c_char);
    }

    ret = snprintf(
        line.as_mut_ptr(), LINE_MAX,
        b"%d %d %s\0".as_ptr() as *const c_char,
        efd, cfd, *argv.add(2),
    );
    if ret >= LINE_MAX as c_int {
        errx(1, b"Arguments string is too long\0".as_ptr() as *const c_char);
    }

    ret = write(event_control, line.as_ptr() as *const c_void, strlen(line.as_ptr()) + 1) as c_int;
    if ret == -1 {
        err(1, b"Cannot write to cgroup.event_control\0".as_ptr() as *const c_char);
    }

    loop {
        let mut result: u64 = 0;

        ret = read(efd, &mut result as *mut u64 as *mut c_void, std::mem::size_of::<u64>()) as c_int;
        if ret == -1 {
            if errno() == EINTR {
                continue;
            }
            err(1, b"Cannot read from eventfd\0".as_ptr() as *const c_char);
        }
        assert!(ret == std::mem::size_of::<u64>() as c_int);

        ret = access(event_control_path.as_ptr(), W_OK);
        if ret == -1 && errno() == ENOENT {
            puts(b"The cgroup seems to have removed.\0".as_ptr() as *const c_char);
            break;
        }

        if ret == -1 {
            err(1, b"cgroup.event_control is not accessible any more\0".as_ptr() as *const c_char);
        }

        printf(
            b"%s %s: crossed\n\0".as_ptr() as *const c_char,
            *argv.add(1), *argv.add(2),
        );
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
