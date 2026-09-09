// SPDX-License-Identifier: BSD-3-Clause
/*
 * Simple tool to set SECBIT_EXEC_RESTRICT_FILE, SECBIT_EXEC_DENY_INTERACTIVE,
 * before executing a command.
 *
 * Copyright © 2024 Microsoft Corporation
 */

use std::ffi::{c_char, c_int, c_void, CStr};

// Supplied by the system headers/libraries in the final build.
extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn execvpe(
        file: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...);
    static mut optind: c_int;
    static mut stderr: *mut c_void;
    static mut errno: c_int;
}

// Linux securebits and prctl constants are supplied by the corresponding headers.
const PR_GET_SECUREBITS: c_int = 27;
const PR_SET_SECUREBITS: c_int = 28;
const SECBIT_EXEC_RESTRICT_FILE: c_int = 1 << 20;
const SECBIT_EXEC_RESTRICT_FILE_LOCKED: c_int = 1 << 21;
const SECBIT_EXEC_DENY_INTERACTIVE: c_int = 1 << 22;
const SECBIT_EXEC_DENY_INTERACTIVE_LOCKED: c_int = 1 << 23;

unsafe fn print_usage(argv0: *const c_char) {
    fprintf(stderr, b"usage: %s -f|-i -- <cmd> [args]...\n\n\0".as_ptr() as *const c_char, argv0);
    fprintf(stderr, b"Execute a command with\n\0".as_ptr() as *const c_char);
    fprintf(stderr, b"- SECBIT_EXEC_RESTRICT_FILE set: -f\n\0".as_ptr() as *const c_char);
    fprintf(stderr, b"- SECBIT_EXEC_DENY_INTERACTIVE set: -i\n\0".as_ptr() as *const c_char);
}

pub unsafe fn main(
    argc: c_int,
    argv: *const *mut c_char,
    envp: *const *mut c_char,
) -> c_int {
    let mut secbits_cur: c_int;
    let mut secbits_new: c_int;
    let mut has_policy = false;

    secbits_cur = prctl(PR_GET_SECUREBITS);
    if secbits_cur == -1 {
        /*
         * This should never happen, except with a buggy seccomp
         * filter.
         */
        perror(b"ERROR: Failed to get securebits\0".as_ptr() as *const c_char);
        return 1;
    }

    secbits_new = secbits_cur;
    let optstring = b"fi\0";
    loop {
        let opt = getopt(argc, argv, optstring.as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }
        match opt as u8 as char {
            'f' => {
                secbits_new |= SECBIT_EXEC_RESTRICT_FILE | SECBIT_EXEC_RESTRICT_FILE_LOCKED;
                has_policy = true;
            }
            'i' => {
                secbits_new |= SECBIT_EXEC_DENY_INTERACTIVE | SECBIT_EXEC_DENY_INTERACTIVE_LOCKED;
                has_policy = true;
            }
            _ => {
                print_usage(*argv);
                return 1;
            }
        }
    }

    let cmd_argv = argv.add(optind as usize);
    if (*cmd_argv).is_null() || !has_policy {
        print_usage(*argv);
        return 1;
    }

    if secbits_cur != secbits_new && prctl(PR_SET_SECUREBITS, secbits_new) != 0 {
        perror(b"Failed to set secure bit(s).\0".as_ptr() as *const c_char);
        fprintf(
            stderr,
            b"Hint: The running kernel may not support this feature.\n\0".as_ptr()
                as *const c_char,
        );
        return 1;
    }

    let cmd_path = *cmd_argv;
    fprintf(stderr, b"Executing command...\n\0".as_ptr() as *const c_char);
    execvpe(cmd_path, cmd_argv, envp);
    fprintf(
        stderr,
        b"Failed to execute \"%s\": %s\n\0".as_ptr() as *const c_char,
        cmd_path,
        strerror(errno),
    );
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
