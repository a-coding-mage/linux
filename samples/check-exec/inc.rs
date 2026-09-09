// SPDX-License-Identifier: BSD-3-Clause
/*
 * Very simple script interpreter that can evaluate two different commands (one
 * per line):
 * - "?" to initialize a counter from user's input;
 * - "+" to increment the counter (which is set to 0 by default).
 */

use std::ffi::{c_char, c_int, c_longlong, c_void};
use std::ptr;

const AT_EMPTY_PATH: c_int = 0x1000;
const AT_EXECVE_CHECK: c_int = 0x2000;
const PR_GET_SECUREBITS: c_int = 27;
const SECBIT_EXEC_DENY_INTERACTIVE: c_int = 1 << 24;
const SECBIT_EXEC_RESTRICT_FILE: c_int = 1 << 25;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    fn syscall(number: c_longlong, ...) -> c_longlong;
    fn fileno(stream: *mut FILE) -> c_int;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn scanf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fread(ptr: *mut c_void, size: usize, count: usize, stream: *mut FILE) -> usize;
    fn prctl(option: c_int, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
    static mut stdin: *mut FILE;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
}

unsafe fn sys_execveat(
    dirfd: c_int,
    pathname: *const c_char,
    argv: *const *mut c_char,
    envp: *const *mut c_char,
    flags: c_int,
) -> c_int {
    syscall(322, dirfd, pathname, argv, envp, flags) as c_int
}

/* Returns 1 on error, 0 otherwise. */
unsafe fn interpret_buffer(buffer: *mut c_char, _buffer_size: usize) -> c_int {
    let mut saveptr: *mut c_char = ptr::null_mut();
    let mut number: c_longlong = 0;
    let newline = b"\n\0".as_ptr() as *const c_char;
    let mut line = strtok_r(buffer, newline, &mut saveptr);
    while !line.is_null() {
        if *line != b'#' as c_char && strlen(line) != 1 {
            fprintf(stderr(), b"# ERROR: Unknown string\n\0".as_ptr() as _,);
            return 1;
        }
        match *line as u8 {
            b'#' => {}
            b'+' => {
                number = number.wrapping_add(1);
                printf(b"%lld\n\0".as_ptr() as _, number);
            }
            b'?' => {
                fprintf(stderr(), b"> Enter new number: \n\0".as_ptr() as _);
                if scanf(b"%lld\0".as_ptr() as _, &mut number) != 1 {
                    fprintf(stderr(), b"# WARNING: Failed to read number from stdin\n\0".as_ptr() as _);
                }
            }
            ch => {
                fprintf(stderr(), b"# ERROR: Unknown character '%c'\n\0".as_ptr() as _, ch as c_int);
                return 1;
            }
        }
        line = strtok_r(ptr::null_mut(), newline, &mut saveptr);
    }
    0
}

unsafe fn stderr() -> *mut FILE {
    // The C standard error stream is supplied by the C runtime.
    extern "C" {
        static mut stderr: *mut FILE;
    }
    stderr
}

/* Returns 1 on error, 0 otherwise. */
unsafe fn interpret_stream(
    script: *mut FILE,
    script_name: *mut c_char,
    envp: *const *mut c_char,
    restrict_stream: bool,
) -> c_int {
    let mut script_argv = [script_name, ptr::null_mut()];
    let mut buf = [0i8; 128];
    let err = sys_execveat(fileno(script), b"\0".as_ptr() as _, script_argv.as_mut_ptr(), envp, AT_EMPTY_PATH | AT_EXECVE_CHECK);
    if err != 0 && restrict_stream {
        perror(b"ERROR: Script execution check\0".as_ptr() as _);
        return 1;
    }
    let buf_size = fread(buf.as_mut_ptr() as _, 1, buf.len() - 1, script);
    interpret_buffer(buf.as_mut_ptr(), buf_size)
}

unsafe fn print_usage(argv0: *const c_char) {
    fprintf(stderr(), b"usage: %s <script.inc> | -i | -c <command>\n\n\0".as_ptr() as _, argv0);
    fprintf(stderr(), b"Example:\n\0".as_ptr() as _);
    fprintf(stderr(), b"  ./set-exec -fi -- ./inc -i < script-exec.inc\n\0".as_ptr() as _);
}

pub unsafe fn main(argc: c_int, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int {
    let secbits = prctl(PR_GET_SECUREBITS);
    if secbits == -1 {
        perror(b"ERROR: Failed to get securebits\0".as_ptr() as _);
        return 1;
    }
    let deny_interactive = (secbits & SECBIT_EXEC_DENY_INTERACTIVE) != 0;
    let restrict_file = (secbits & SECBIT_EXEC_RESTRICT_FILE) != 0;
    let mut cmd: *mut c_char = ptr::null_mut();
    let mut interpret_stdin = false;
    loop {
        let opt = getopt(argc, argv, b"c:i\0".as_ptr() as _);
        if opt == -1 { break; }
        match opt as u8 {
            b'c' => { if !cmd.is_null() { fprintf(stderr(), b"ERROR: Command already set\0".as_ptr() as _); return 1; } cmd = optarg; }
            b'i' => interpret_stdin = true,
            _ => { print_usage(*argv); return 1; }
        }
    }
    let arg_nb = (!cmd.is_null() as usize) + (interpret_stdin as usize);
    let mut script_name: *mut c_char = ptr::null_mut();
    if arg_nb == 0 && argc == 2 { script_name = *argv.add(1); }
    else if arg_nb != 1 { print_usage(*argv); return 1; }
    if !cmd.is_null() {
        if deny_interactive { fprintf(stderr(), b"ERROR: Interactive interpretation denied.\n\0".as_ptr() as _); return 1; }
        return interpret_buffer(cmd, strlen(cmd));
    }
    if interpret_stdin && script_name.is_null() {
        script_name = b"/proc/self/fd/0\0".as_ptr() as *mut c_char;
        return interpret_stream(stdin, script_name, envp, deny_interactive);
    } else if !script_name.is_null() && !interpret_stdin {
        let script_file = fopen(script_name, b"r\0".as_ptr() as _);
        if script_file.is_null() { perror(b"ERROR: Failed to open script\0".as_ptr() as _); return 1; }
        return interpret_stream(script_file, script_name, envp, restrict_file);
    }
    print_usage(*argv);
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
