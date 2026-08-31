// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Program that atomically exchanges two paths using
 * the renameat2() system call RENAME_EXCHANGE flag.
 *
 * Copyright 2022 Red Hat Inc.
 * Author: Javier Martinez Canillas <javierm@redhat.com>
 */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint};

const AT_FDCWD: c_int = -100;
const RENAME_EXCHANGE: c_uint = 1 << 1;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn renameat2(
        olddirfd: c_int,
        oldpath: *const c_char,
        newdirfd: c_int,
        newpath: *const c_char,
        flags: c_uint,
    ) -> c_int;
}

unsafe fn print_usage(program: *const c_char) {
    unsafe {
        printf(
            c"Usage: %s [oldpath] [newpath]\n".as_ptr(),
            CStr::from_ptr(program).as_ptr(),
        );
        printf(c"Atomically exchange oldpath and newpath\n".as_ptr());
    }
}

unsafe fn main_c(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let ret: c_int;

    unsafe {
        if argc != 3 {
            print_usage(*argv.offset(0));
            exit(EXIT_FAILURE);
        }

        ret = renameat2(
            AT_FDCWD,
            *argv.offset(1),
            AT_FDCWD,
            *argv.offset(2),
            RENAME_EXCHANGE,
        );
        if ret != 0 {
            perror(c"rename exchange failed".as_ptr());
            exit(EXIT_FAILURE);
        }

        exit(EXIT_SUCCESS);
    }
}

fn main() {
    let mut args: Vec<*mut c_char> = std::env::args()
        .map(|arg| {
            std::ffi::CString::new(arg)
                .expect("argument contains interior NUL byte")
                .into_raw()
        })
        .collect();

    unsafe {
        main_c(args.len() as c_int, args.as_mut_ptr());
    }
}
