// SPDX-License-Identifier: GPL-2.0-or-later
/* fd-based mount test.
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use std::ffi::{c_char, c_int, c_uint, c_void};
use std::ptr;

extern "C" {
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
    static mut stderr: *mut c_void;
}

type c_long = isize;

unsafe fn check_messages(fd: c_int) {
    let mut buf = [0u8; 4096];
    let err = *__errno_location();

    loop {
        let n = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if n < 0 {
            break;
        }
        let n = n - 2;

        match buf[0] {
            b'e' => {
                fprintf(stderr, b"Error: %*.*s\n\0".as_ptr() as *const c_char,
                        n as c_int, n as c_int, buf.as_ptr().add(2));
            }
            b'w' => {
                fprintf(stderr, b"Warning: %*.*s\n\0".as_ptr() as *const c_char,
                        n as c_int, n as c_int, buf.as_ptr().add(2));
            }
            b'i' => {
                fprintf(stderr, b"Info: %*.*s\n\0".as_ptr() as *const c_char,
                        n as c_int, n as c_int, buf.as_ptr().add(2));
            }
            _ => {}
        }
    }

    *__errno_location() = err;
}

unsafe fn mount_error(fd: c_int, s: *const c_char) -> ! {
    check_messages(fd);
    perror(s);
    exit(1);
}

// Hope -1 isn't a syscall. These values are supplied by the system headers
// when the corresponding syscall numbers are available.
#[allow(dead_code)]
const NR_FSOPEN: c_long = -1;
#[allow(dead_code)]
const NR_FSMOUNT: c_long = -1;
#[allow(dead_code)]
const NR_FSCONFIG: c_long = -1;
#[allow(dead_code)]
const NR_MOVE_MOUNT: c_long = -1;

unsafe fn fsopen(fs_name: *const c_char, flags: c_uint) -> c_int {
    syscall(NR_FSOPEN, fs_name, flags) as c_int
}

unsafe fn fsmount(fsfd: c_int, flags: c_uint, ms_flags: c_uint) -> c_int {
    syscall(NR_FSMOUNT, fsfd, flags, ms_flags) as c_int
}

unsafe fn fsconfig(fsfd: c_int, cmd: c_uint, key: *const c_char,
                   val: *const c_void, aux: c_int) -> c_int {
    syscall(NR_FSCONFIG, fsfd, cmd, key, val, aux) as c_int
}

unsafe fn move_mount(from_dfd: c_int, from_pathname: *const c_char,
                     to_dfd: c_int, to_pathname: *const c_char,
                     flags: c_uint) -> c_int {
    syscall(NR_MOVE_MOUNT, from_dfd, from_pathname,
            to_dfd, to_pathname, flags) as c_int
}

unsafe fn e_fsconfig(fd: c_int, cmd: c_uint, key: *const c_char,
                     val: *const c_void, aux: c_int) {
    if fsconfig(fd, cmd, key, val, aux) == -1 {
        let message = if key.is_null() {
            b"create\0".as_ptr() as *const c_char
        } else {
            key
        };
        mount_error(fd, message);
    }
}

fn main() {
    unsafe {
        let fsfd: c_int;
        let mfd: c_int;

        /* Mount a publically available AFS filesystem */
        fsfd = fsopen(b"afs\0".as_ptr() as *const c_char, 0);
        if fsfd == -1 {
            perror(b"fsopen\0".as_ptr() as *const c_char);
            exit(1);
        }

        e_fsconfig(fsfd, FSCONFIG_SET_STRING, b"source\0".as_ptr() as *const c_char,
                   b"#grand.central.org:root.cell.\0".as_ptr() as *const c_void, 0);
        e_fsconfig(fsfd, FSCONFIG_CMD_CREATE, ptr::null(), ptr::null(), 0);

        mfd = fsmount(fsfd, 0, MOUNT_ATTR_RDONLY);
        if mfd < 0 {
            mount_error(fsfd, b"fsmount\0".as_ptr() as *const c_char);
        }
        if close(fsfd) == -1 {
            perror(b"close(fsfd)\0".as_ptr() as *const c_char);
            exit(1);
        }

        if move_mount(mfd, b"\0".as_ptr() as *const c_char, AT_FDCWD,
                      b"/mnt\0".as_ptr() as *const c_char,
                      MOVE_MOUNT_F_EMPTY_PATH) < 0 {
            perror(b"move_mount\0".as_ptr() as *const c_char);
            exit(1);
        }

        if close(mfd) == -1 {
            perror(b"close(mfd)\0".as_ptr() as *const c_char);
            exit(1);
        }
        exit(0);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
