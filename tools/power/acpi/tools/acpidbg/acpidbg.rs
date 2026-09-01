// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI AML interfacing userspace utility
 *
 * Copyright (C) 2015, Intel Corporation
 * Authors: Lv Zheng <lv.zheng@intel.com>
 */

/* Translated from C. External ACPI/libc symbols are declarations. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct fd_set {
    fds_bits: [c_long; 16],
}

#[repr(C)]
struct circ_buf {
    buf: *mut c_char,
    head: c_int,
    tail: c_int,
}

unsafe extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut optarg: *mut c_char;

    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strlen(s: *const c_char) -> size_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        exceptfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
}

const STDIN_FILENO: c_int = 0;
const STDOUT_FILENO: c_int = 1;

const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const O_RDWR: c_int = 0o00000002;
const O_NONBLOCK: c_int = 0o00004000;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

/* From ACPICA headers. */
const ACPI_DEBUGGER_COMMAND_PROMPT: c_char = b'-' as c_char;
const ACPI_DEBUGGER_EXECUTE_PROMPT: c_char = b'=' as c_char;

const ACPI_AML_FILE: &[u8] = b"/sys/kernel/debug/acpi/acpidbg\0";
const ACPI_AML_SEC_TICK: c_long = 1;
const ACPI_AML_USEC_PEEK: c_long = 200;
const ACPI_AML_BUF_SIZE: c_int = 4096;

const ACPI_AML_BATCH_WRITE_CMD: c_ulong = 0x00; /* Write command to kernel */
const ACPI_AML_BATCH_READ_LOG: c_ulong = 0x01; /* Read log from kernel */
const ACPI_AML_BATCH_WRITE_LOG: c_ulong = 0x02; /* Write log to console */

const ACPI_AML_LOG_START: c_ulong = 0x00;
const ACPI_AML_PROMPT_START: c_ulong = 0x01;
const ACPI_AML_PROMPT_STOP: c_ulong = 0x02;
const ACPI_AML_LOG_STOP: c_ulong = 0x03;
const ACPI_AML_PROMPT_ROLL: c_ulong = 0x04;

const ACPI_AML_INTERACTIVE: c_ulong = 0x00;
const ACPI_AML_BATCH: c_ulong = 0x01;

fn CIRC_CNT(head: c_int, tail: c_int, size: c_int) -> c_int {
    (head - tail) & (size - 1)
}

fn CIRC_SPACE(head: c_int, tail: c_int, size: c_int) -> c_int {
    CIRC_CNT(tail, head + 1, size)
}

fn CIRC_CNT_TO_END(head: c_int, tail: c_int, size: c_int) -> c_int {
    let end = size - tail;
    let n = (head + end) & (size - 1);
    if n < end { n } else { end }
}

fn CIRC_SPACE_TO_END(head: c_int, tail: c_int, size: c_int) -> c_int {
    let end = size - 1 - head;
    let n = (end + tail) & (size - 1);
    if n <= end { n } else { end + 1 }
}

unsafe fn circ_count(circ: *mut circ_buf) -> c_int {
    CIRC_CNT((*circ).head, (*circ).tail, ACPI_AML_BUF_SIZE)
}

unsafe fn circ_count_to_end(circ: *mut circ_buf) -> c_int {
    CIRC_CNT_TO_END((*circ).head, (*circ).tail, ACPI_AML_BUF_SIZE)
}

unsafe fn circ_space(circ: *mut circ_buf) -> c_int {
    CIRC_SPACE((*circ).head, (*circ).tail, ACPI_AML_BUF_SIZE)
}

unsafe fn circ_space_to_end(circ: *mut circ_buf) -> c_int {
    CIRC_SPACE_TO_END((*circ).head, (*circ).tail, ACPI_AML_BUF_SIZE)
}

unsafe fn acpi_aml_cmd_count() -> c_int {
    circ_count(&raw mut ACPI_AML_CMD_CRC)
}

unsafe fn acpi_aml_log_count() -> c_int {
    circ_count(&raw mut ACPI_AML_LOG_CRC)
}

unsafe fn acpi_aml_cmd_space() -> c_int {
    circ_space(&raw mut ACPI_AML_CMD_CRC)
}

unsafe fn acpi_aml_log_space() -> c_int {
    circ_space(&raw mut ACPI_AML_LOG_CRC)
}

static mut ACPI_AML_CMD_BUF: [c_char; ACPI_AML_BUF_SIZE as usize] = [0; ACPI_AML_BUF_SIZE as usize];
static mut ACPI_AML_LOG_BUF: [c_char; ACPI_AML_BUF_SIZE as usize] = [0; ACPI_AML_BUF_SIZE as usize];
static mut ACPI_AML_CMD_CRC: circ_buf = circ_buf {
    buf: ptr::null_mut(),
    head: 0,
    tail: 0,
};
static mut ACPI_AML_LOG_CRC: circ_buf = circ_buf {
    buf: ptr::null_mut(),
    head: 0,
    tail: 0,
};
static mut acpi_aml_file_path: *const c_char = ACPI_AML_FILE.as_ptr() as *const c_char;
static mut acpi_aml_mode: c_ulong = ACPI_AML_INTERACTIVE;
static mut acpi_aml_exit: bool = false;

static mut acpi_aml_batch_drain: bool = false;
static mut acpi_aml_batch_state: c_ulong = 0;
static mut acpi_aml_batch_prompt: c_char = 0;
static mut acpi_aml_batch_roll: c_char = 0;
static mut acpi_aml_log_state: c_ulong = 0;
static mut acpi_aml_batch_cmd: *mut c_char = ptr::null_mut();
static mut acpi_aml_batch_pos: *mut c_char = ptr::null_mut();

unsafe fn init_circs() {
    ACPI_AML_CMD_CRC.buf = (&raw mut ACPI_AML_CMD_BUF) as *mut c_char;
    ACPI_AML_LOG_CRC.buf = (&raw mut ACPI_AML_LOG_BUF) as *mut c_char;
}

unsafe fn FD_ZERO(set: *mut fd_set) {
    ptr::write_bytes(set as *mut u8, 0, mem::size_of::<fd_set>());
}

unsafe fn FD_SET(fd: c_int, set: *mut fd_set) {
    let bits_per_long = 8 * mem::size_of::<c_long>() as c_int;
    (*set).fds_bits[(fd / bits_per_long) as usize] |= 1 << (fd % bits_per_long);
}

unsafe fn FD_ISSET(fd: c_int, set: *mut fd_set) -> bool {
    let bits_per_long = 8 * mem::size_of::<c_long>() as c_int;
    ((*set).fds_bits[(fd / bits_per_long) as usize] & (1 << (fd % bits_per_long))) != 0
}

unsafe fn acpi_aml_set_fl(fd: c_int, mut flags: c_int) -> c_int {
    let mut ret: c_int;

    ret = fcntl(fd, F_GETFL, 0);
    if ret < 0 {
        perror(c"fcntl(F_GETFL)".as_ptr());
        return ret;
    }
    flags |= ret;
    ret = fcntl(fd, F_SETFL, flags);
    if ret < 0 {
        perror(c"fcntl(F_SETFL)".as_ptr());
        return ret;
    }
    ret
}

unsafe fn acpi_aml_set_fd(fd: c_int, mut maxfd: c_int, set: *mut fd_set) -> c_int {
    if fd > maxfd {
        maxfd = fd;
    }
    FD_SET(fd, set);
    maxfd
}

unsafe fn acpi_aml_read(fd: c_int, crc: *mut circ_buf) -> c_int {
    let p: *mut c_char;
    let mut len: c_int;

    p = (*crc).buf.add((*crc).head as usize);
    len = circ_space_to_end(crc);
    len = read(fd, p as *mut c_void, len as size_t) as c_int;
    if len < 0 {
        perror(c"read".as_ptr());
    } else if len > 0 {
        (*crc).head = ((*crc).head + len) & (ACPI_AML_BUF_SIZE - 1);
    }
    len
}

unsafe fn acpi_aml_read_batch_cmd(_unused: c_int, crc: *mut circ_buf) -> c_int {
    let p: *mut c_char;
    let mut len: c_int;
    let remained: c_int = strlen(acpi_aml_batch_pos) as c_int;

    p = (*crc).buf.add((*crc).head as usize);
    len = circ_space_to_end(crc);
    if len > remained {
        memcpy(p as *mut c_void, acpi_aml_batch_pos as *const c_void, remained as size_t);
        acpi_aml_batch_pos = acpi_aml_batch_pos.add(remained as usize);
        len = remained;
    } else {
        memcpy(p as *mut c_void, acpi_aml_batch_pos as *const c_void, len as size_t);
        acpi_aml_batch_pos = acpi_aml_batch_pos.add(len as usize);
    }
    if len > 0 {
        (*crc).head = ((*crc).head + len) & (ACPI_AML_BUF_SIZE - 1);
    }
    len
}

unsafe fn acpi_aml_read_batch_log(fd: c_int, crc: *mut circ_buf) -> c_int {
    let mut p: *mut c_char;
    let mut len: c_int;
    let mut ret: c_int = 0;

    p = (*crc).buf.add((*crc).head as usize);
    len = circ_space_to_end(crc);
    while ret < len && acpi_aml_log_state != ACPI_AML_LOG_STOP {
        if acpi_aml_log_state == ACPI_AML_PROMPT_ROLL {
            *p = acpi_aml_batch_roll;
            len = 1;
            (*crc).head = ((*crc).head + 1) & (ACPI_AML_BUF_SIZE - 1);
            ret += 1;
            acpi_aml_log_state = ACPI_AML_LOG_START;
        } else {
            len = read(fd, p as *mut c_void, 1) as c_int;
            if len <= 0 {
                if len < 0 {
                    perror(c"read".as_ptr());
                }
                ret = len;
                break;
            }
        }
        match acpi_aml_log_state {
            ACPI_AML_LOG_START => {
                if *p == b'\n' as c_char {
                    acpi_aml_log_state = ACPI_AML_PROMPT_START;
                }
                (*crc).head = ((*crc).head + 1) & (ACPI_AML_BUF_SIZE - 1);
                ret += 1;
            }
            ACPI_AML_PROMPT_START => {
                if *p == ACPI_DEBUGGER_COMMAND_PROMPT || *p == ACPI_DEBUGGER_EXECUTE_PROMPT {
                    acpi_aml_batch_prompt = *p;
                    acpi_aml_log_state = ACPI_AML_PROMPT_STOP;
                } else {
                    if *p != b'\n' as c_char {
                        acpi_aml_log_state = ACPI_AML_LOG_START;
                    }
                    (*crc).head = ((*crc).head + 1) & (ACPI_AML_BUF_SIZE - 1);
                    ret += 1;
                }
            }
            ACPI_AML_PROMPT_STOP => {
                if *p == b' ' as c_char {
                    acpi_aml_log_state = ACPI_AML_LOG_STOP;
                    acpi_aml_exit = true;
                } else {
                    /* Roll back */
                    acpi_aml_log_state = ACPI_AML_PROMPT_ROLL;
                    acpi_aml_batch_roll = *p;
                    *p = acpi_aml_batch_prompt;
                    (*crc).head = ((*crc).head + 1) & (ACPI_AML_BUF_SIZE - 1);
                    ret += 1;
                }
            }
            _ => {
                assert!(false);
            }
        }
        p = (*crc).buf.add((*crc).head as usize);
    }
    ret
}

unsafe fn acpi_aml_write(fd: c_int, crc: *mut circ_buf) -> c_int {
    let p: *mut c_char;
    let mut len: c_int;

    p = (*crc).buf.add((*crc).tail as usize);
    len = circ_count_to_end(crc);
    len = write(fd, p as *const c_void, len as size_t) as c_int;
    if len < 0 {
        perror(c"write".as_ptr());
    } else if len > 0 {
        (*crc).tail = ((*crc).tail + len) & (ACPI_AML_BUF_SIZE - 1);
    }
    len
}

unsafe fn acpi_aml_write_batch_log(fd: c_int, crc: *mut circ_buf) -> c_int {
    let p: *mut c_char;
    let mut len: c_int;

    p = (*crc).buf.add((*crc).tail as usize);
    len = circ_count_to_end(crc);
    if !acpi_aml_batch_drain {
        len = write(fd, p as *const c_void, len as size_t) as c_int;
        if len < 0 {
            perror(c"write".as_ptr());
        }
    }
    if len > 0 {
        (*crc).tail = ((*crc).tail + len) & (ACPI_AML_BUF_SIZE - 1);
    }
    len
}

unsafe fn acpi_aml_write_batch_cmd(fd: c_int, crc: *mut circ_buf) -> c_int {
    let len: c_int;

    len = acpi_aml_write(fd, crc);
    if circ_count_to_end(crc) == 0 {
        acpi_aml_batch_state = ACPI_AML_BATCH_READ_LOG;
    }
    len
}

unsafe fn acpi_aml_do(fd: c_int, op: unsafe fn(c_int, *mut circ_buf) -> c_int, crc: *mut circ_buf, buf: *const c_char, opname: *const c_char) -> bool {
    let ret = op(fd, crc);
    if ret == 0 {
        fprintf(stderr, c"%s %s pipe closed.\n".as_ptr(), buf, opname);
        return false;
    }
    true
}

unsafe fn acpi_aml_batch_do(fd: c_int, op: unsafe fn(c_int, *mut circ_buf) -> c_int, crc: *mut circ_buf) -> bool {
    let ret = op(fd, crc);
    ret != 0
}

unsafe fn acpi_aml_loop(fd: c_int) {
    let mut rfds: fd_set = mem::zeroed();
    let mut wfds: fd_set = mem::zeroed();
    let mut tv: timeval = mem::zeroed();
    let mut ret: c_int;
    let mut maxfd: c_int = 0;

    if acpi_aml_mode == ACPI_AML_BATCH {
        acpi_aml_log_state = ACPI_AML_LOG_START;
        acpi_aml_batch_pos = acpi_aml_batch_cmd;
        if acpi_aml_batch_drain {
            acpi_aml_batch_state = ACPI_AML_BATCH_READ_LOG;
        } else {
            acpi_aml_batch_state = ACPI_AML_BATCH_WRITE_CMD;
        }
    }
    acpi_aml_exit = false;
    while !acpi_aml_exit {
        tv.tv_sec = ACPI_AML_SEC_TICK;
        tv.tv_usec = 0;
        FD_ZERO(&mut rfds);
        FD_ZERO(&mut wfds);

        if acpi_aml_cmd_space() != 0 {
            if acpi_aml_mode == ACPI_AML_INTERACTIVE {
                maxfd = acpi_aml_set_fd(STDIN_FILENO, maxfd, &mut rfds);
            } else if strlen(acpi_aml_batch_pos) != 0
                && acpi_aml_batch_state == ACPI_AML_BATCH_WRITE_CMD
            {
                if !acpi_aml_batch_do(STDIN_FILENO, acpi_aml_read_batch_cmd, &raw mut ACPI_AML_CMD_CRC) {
                    return;
                }
            }
        }
        if acpi_aml_cmd_count() != 0
            && (acpi_aml_mode == ACPI_AML_INTERACTIVE
                || acpi_aml_batch_state == ACPI_AML_BATCH_WRITE_CMD)
        {
            maxfd = acpi_aml_set_fd(fd, maxfd, &mut wfds);
        }
        if acpi_aml_log_space() != 0
            && (acpi_aml_mode == ACPI_AML_INTERACTIVE
                || acpi_aml_batch_state == ACPI_AML_BATCH_READ_LOG)
        {
            maxfd = acpi_aml_set_fd(fd, maxfd, &mut rfds);
        }
        if acpi_aml_log_count() != 0 {
            maxfd = acpi_aml_set_fd(STDOUT_FILENO, maxfd, &mut wfds);
        }

        ret = select(maxfd + 1, &mut rfds, &mut wfds, ptr::null_mut(), &mut tv);
        if ret < 0 {
            perror(c"select".as_ptr());
            break;
        }
        if ret > 0 {
            if FD_ISSET(STDIN_FILENO, &mut rfds) {
                if !acpi_aml_do(STDIN_FILENO, acpi_aml_read, &raw mut ACPI_AML_CMD_CRC, c"cmd".as_ptr(), c"read".as_ptr()) {
                    return;
                }
            }
            if FD_ISSET(fd, &mut wfds) {
                if acpi_aml_mode == ACPI_AML_BATCH {
                    if !acpi_aml_batch_do(fd, acpi_aml_write_batch_cmd, &raw mut ACPI_AML_CMD_CRC) {
                        return;
                    }
                } else if !acpi_aml_do(fd, acpi_aml_write, &raw mut ACPI_AML_CMD_CRC, c"cmd".as_ptr(), c"write".as_ptr()) {
                    return;
                }
            }
            if FD_ISSET(fd, &mut rfds) {
                if acpi_aml_mode == ACPI_AML_BATCH {
                    if !acpi_aml_batch_do(fd, acpi_aml_read_batch_log, &raw mut ACPI_AML_LOG_CRC) {
                        return;
                    }
                } else if !acpi_aml_do(fd, acpi_aml_read, &raw mut ACPI_AML_LOG_CRC, c"log".as_ptr(), c"read".as_ptr()) {
                    return;
                }
            }
            if FD_ISSET(STDOUT_FILENO, &mut wfds) {
                if acpi_aml_mode == ACPI_AML_BATCH {
                    if !acpi_aml_batch_do(STDOUT_FILENO, acpi_aml_write_batch_log, &raw mut ACPI_AML_LOG_CRC) {
                        return;
                    }
                } else if !acpi_aml_do(STDOUT_FILENO, acpi_aml_write, &raw mut ACPI_AML_LOG_CRC, c"log".as_ptr(), c"write".as_ptr()) {
                    return;
                }
            }
        }
    }
}

unsafe fn acpi_aml_readable(fd: c_int) -> bool {
    let mut rfds: fd_set = mem::zeroed();
    let mut tv: timeval = mem::zeroed();
    let ret: c_int;
    let mut maxfd: c_int = 0;

    tv.tv_sec = 0;
    tv.tv_usec = ACPI_AML_USEC_PEEK;
    FD_ZERO(&mut rfds);
    maxfd = acpi_aml_set_fd(fd, maxfd, &mut rfds);
    ret = select(maxfd + 1, &mut rfds, ptr::null_mut(), ptr::null_mut(), &mut tv);
    if ret < 0 {
        perror(c"select".as_ptr());
    }
    if ret > 0 && FD_ISSET(fd, &mut rfds) {
        return true;
    }
    false
}

/*
 * This is a userspace IO flush implementation, replying on the prompt
 * characters and can be turned into a flush() call after kernel implements
 * .flush() filesystem operation.
 */
unsafe fn acpi_aml_flush(fd: c_int) {
    while acpi_aml_readable(fd) {
        acpi_aml_batch_drain = true;
        acpi_aml_loop(fd);
        acpi_aml_batch_drain = false;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage(file: *mut FILE, progname: *mut c_char) {
    fprintf(file, c"usage: %s [-b cmd] [-f file] [-h]\n".as_ptr(), progname);
    fprintf(file, c"\nOptions:\n".as_ptr());
    fprintf(file, c"  -b     Specify command to be executed in batch mode\n".as_ptr());
    fprintf(file, c"  -f     Specify interface file other than".as_ptr());
    fprintf(file, c"         /sys/kernel/debug/acpi/acpidbg\n".as_ptr());
    fprintf(file, c"  -h     Print this help message\n".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut fd: c_int = -1;
    let mut ch: c_int;
    let mut len: c_int;
    let mut ret: c_int = EXIT_SUCCESS;

    init_circs();

    loop {
        ch = getopt(argc, argv, c"b:f:h".as_ptr());
        if ch == -1 {
            break;
        }
        match ch {
            x if x == b'b' as c_int => {
                if !acpi_aml_batch_cmd.is_null() {
                    fprintf(stderr, c"Already specify %s\n".as_ptr(), acpi_aml_batch_cmd);
                    ret = EXIT_FAILURE;
                    break;
                }
                len = strlen(optarg) as c_int;
                acpi_aml_batch_cmd = calloc((len + 2) as size_t, 1) as *mut c_char;
                if acpi_aml_batch_cmd.is_null() {
                    perror(c"calloc".as_ptr());
                    ret = EXIT_FAILURE;
                    break;
                }
                memcpy(
                    acpi_aml_batch_cmd as *mut c_void,
                    optarg as *const c_void,
                    len as size_t,
                );
                *acpi_aml_batch_cmd.add(len as usize) = b'\n' as c_char;
                acpi_aml_mode = ACPI_AML_BATCH;
            }
            x if x == b'f' as c_int => {
                acpi_aml_file_path = optarg;
            }
            x if x == b'h' as c_int => {
                usage(stdout, *argv.add(0));
                break;
            }
            _ => {
                usage(stderr, *argv.add(0));
                ret = EXIT_FAILURE;
                break;
            }
        }
    }

    if ret == EXIT_SUCCESS && ch != b'h' as c_int {
        fd = open(acpi_aml_file_path, O_RDWR | O_NONBLOCK);
        if fd < 0 {
            perror(c"open".as_ptr());
            ret = EXIT_FAILURE;
        } else {
            acpi_aml_set_fl(STDIN_FILENO, O_NONBLOCK);
            acpi_aml_set_fl(STDOUT_FILENO, O_NONBLOCK);

            if acpi_aml_mode == ACPI_AML_BATCH {
                acpi_aml_flush(fd);
            }
            acpi_aml_loop(fd);
        }
    }

    if fd >= 0 {
        close(fd);
    }
    if !acpi_aml_batch_cmd.is_null() {
        free(acpi_aml_batch_cmd as *mut c_void);
    }
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
