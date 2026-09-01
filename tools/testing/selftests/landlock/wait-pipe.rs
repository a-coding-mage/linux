// SPDX-License-Identifier: GPL-2.0
/*
 * Write in a pipe and wait.
 *
 * Used by layout1.umount_sandboxer from fs_test.c
 *
 * Copyright © 2024-2025 Microsoft Corporation
 */

// C source used _GNU_SOURCE and included <stdio.h>, <stdlib.h>, and <unistd.h>.

use std::env;
use std::ffi::CString;
use std::io::{self, Write};
use std::os::raw::{c_char, c_int, c_void};

unsafe extern "C" {
    fn atoi(nptr: *const c_char) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn perror(s: *const c_char);
}

fn main() {
    let args: Vec<CString> = env::args()
        .map(|arg| CString::new(arg).expect("argument contains interior NUL byte"))
        .collect();
    let argc = args.len() as c_int;

    let pipe_child: c_int;
    let pipe_parent: c_int;
    let mut buf: c_char = 0;

    /* The first argument must be the file descriptor number of a pipe. */
    if argc != 3 {
        let _ = writeln!(io::stderr(), "Wrong number of arguments (not two)");
        std::process::exit(1);
    }

    unsafe {
        pipe_child = atoi(args[1].as_ptr());
        pipe_parent = atoi(args[2].as_ptr());

        /* Signals that we are waiting. */
        if write(pipe_child, b".\0".as_ptr() as *const c_void, 1) != 1 {
            perror(c"Failed to write to first argument".as_ptr());
            std::process::exit(1);
        }

        /* Waits for the parent do its test. */
        if read(pipe_parent, &mut buf as *mut c_char as *mut c_void, 1) != 1 {
            perror(c"Failed to write to the second argument".as_ptr());
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
