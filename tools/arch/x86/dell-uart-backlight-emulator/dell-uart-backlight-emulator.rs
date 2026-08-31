// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Dell AIO Serial Backlight board emulator for testing
 * the Linux dell-uart-backlight driver.
 *
 * Copyright (C) 2024 Hans de Goede <hansg@kernel.org>
 */

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

type size_t = usize;
type ssize_t = isize;
type speed_t = c_uint;
type tcflag_t = c_uint;
type cc_t = u8;
type pid_t = c_int;

const NCCS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
struct termios {
    c_iflag: tcflag_t,
    c_oflag: tcflag_t,
    c_cflag: tcflag_t,
    c_lflag: tcflag_t,
    c_line: cc_t,
    c_cc: [cc_t; NCCS],
    c_ispeed: speed_t,
    c_ospeed: speed_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigaction {
    sa_handler: extern "C" fn(c_int),
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<extern "C" fn()>,
}

const O_RDWR: c_int = 0o2;
const O_NOCTTY: c_int = 0o400;
const TCSANOW: c_int = 0;
const CSTOPB: tcflag_t = 0o0000100;
const CRTSCTS: tcflag_t = 0o20000000000;
const CLOCAL: tcflag_t = 0o0004000;
const CREAD: tcflag_t = 0o0000200;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn __errno_location() -> *mut c_int;
    fn cfmakeraw(termios_p: *mut termios);
    fn cfsetspeed(termios_p: *mut termios, speed: speed_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
}

static mut serial_fd: c_int = 0;
static mut brightness: c_int = 50;

unsafe fn dell_uart_checksum(buf: *mut u8, mut len: c_int) -> u8 {
    let mut val: u8 = 0;

    while {
        len -= 1;
        len >= 0
    } {
        val = val.wrapping_add(*buf.offset(len as isize));
    }

    val ^ 0xff
}

/* read() will return -1 on SIGINT / SIGTERM causing the mainloop to cleanly exit */
extern "C" fn signalhdlr(_signum: c_int) {}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let sigact = sigaction {
        sa_handler: signalhdlr,
        sa_mask: mem::zeroed(),
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut buf: [u8; 4] = [0; 4];
    let mut csum: u8;
    let mut response: [u8; 32] = [0; 32];
    let version_str = cstr(b"PHI23-V321\0");
    let mut tty: termios = mem::zeroed();
    let mut saved_tty: termios;
    let mut ret: c_int;
    let mut idx: c_int;
    let mut len: c_int = 0;

    if argc != 2 {
        fprintf(stderr, cstr(b"Invalid or missing arguments\n\0"));
        fprintf(
            stderr,
            cstr(b"Usage: %s <serial-port>\n\0"),
            *argv.offset(0),
        );
        return 1;
    }

    serial_fd = open(*argv.offset(1), O_RDWR | O_NOCTTY);
    if serial_fd == -1 {
        fprintf(
            stderr,
            cstr(b"Error opening %s: %s\n\0"),
            *argv.offset(1),
            strerror(errno()),
        );
        return 1;
    }

    ret = tcgetattr(serial_fd, &mut tty);
    if ret == -1 {
        fprintf(
            stderr,
            cstr(b"Error getting tcattr: %s\n\0"),
            strerror(errno()),
        );
        ret = 1;
        close(serial_fd);
        return ret;
    }
    saved_tty = tty;

    cfsetspeed(&mut tty, 9600);
    cfmakeraw(&mut tty);
    tty.c_cflag &= !CSTOPB;
    tty.c_cflag &= !CRTSCTS;
    tty.c_cflag |= CLOCAL | CREAD;

    ret = tcsetattr(serial_fd, TCSANOW, &tty);
    if ret == -1 {
        fprintf(
            stderr,
            cstr(b"Error setting tcattr: %s\n\0"),
            strerror(errno()),
        );
        tcsetattr(serial_fd, TCSANOW, &saved_tty);
        close(serial_fd);
        return ret;
    }

    sigaction(SIGINT, &sigact, ptr::null_mut());
    sigaction(SIGTERM, &sigact, ptr::null_mut());

    idx = 0;
    while read(
        serial_fd,
        buf.as_mut_ptr().offset(idx as isize) as *mut c_void,
        1,
    ) == 1
    {
        if idx == 0 {
            match buf[0] {
                /* 3 MSB bits: cmd-len + 01010 SOF marker */
                0x6a => len = 3,
                0x8a => len = 4,
                _ => {
                    fprintf(
                        stderr,
                        cstr(b"Error unexpected first byte: 0x%02x\n\0"),
                        buf[0] as c_int,
                    );
                    continue; /* Try to sync up with sender */
                }
            }
        }

        /* Process msg when len bytes have been received */
        if idx != len - 1 {
            idx += 1;
            continue;
        }

        /* Reset idx for next command */
        idx = 0;

        csum = dell_uart_checksum(buf.as_mut_ptr(), len - 1);
        if buf[(len - 1) as usize] != csum {
            fprintf(
                stderr,
                cstr(b"Error checksum mismatch got 0x%02x expected 0x%02x\n\0"),
                buf[(len - 1) as usize] as c_int,
                csum as c_int,
            );
            continue;
        }

        match ((buf[0] as c_int) << 8) | buf[1] as c_int {
            0x6a06 => {
                /* cmd = 0x06, get version */
                len = strlen(version_str) as c_int;
                strcpy(response.as_mut_ptr().offset(2) as *mut c_char, version_str);
                printf(cstr(b"Get version, reply: %s\n\0"), version_str);
            }
            0x8a0b => {
                /* cmd = 0x0b, set brightness */
                if buf[2] > 100 {
                    fprintf(
                        stderr,
                        cstr(b"Error invalid brightness param: %d\n\0"),
                        buf[2] as c_int,
                    );
                    continue;
                }

                len = 0;
                brightness = buf[2] as c_int;
                printf(cstr(b"Set brightness %d\n\0"), brightness);
            }
            0x6a0c => {
                /* cmd = 0x0c, get brightness */
                len = 1;
                response[2] = brightness as u8;
                printf(cstr(b"Get brightness, reply: %d\n\0"), brightness);
            }
            0x8a0e => {
                /* cmd = 0x0e, set backlight power */
                if buf[2] != 0 && buf[2] != 1 {
                    fprintf(
                        stderr,
                        cstr(b"Error invalid set power param: %d\n\0"),
                        buf[2] as c_int,
                    );
                    continue;
                }

                len = 0;
                printf(cstr(b"Set power %d\n\0"), buf[2] as c_int);
            }
            _ => {
                fprintf(
                    stderr,
                    cstr(b"Error unknown cmd 0x%04x\n\0"),
                    ((buf[0] as c_int) << 8) | buf[1] as c_int,
                );
                continue;
            }
        }

        /* Respond with <total-len> <cmd> <data...> <csum> */
        response[0] = (len + 3) as u8; /* response length in bytes */
        response[1] = buf[1]; /* ack cmd */
        csum = dell_uart_checksum(response.as_mut_ptr(), len + 2);
        response[(len + 2) as usize] = csum;
        ret = write(serial_fd, response.as_ptr() as *const c_void, response[0] as size_t) as c_int;
        if ret != response[0] as c_int {
            fprintf(
                stderr,
                cstr(b"Error writing %d bytes: %d\n\0"),
                response[0] as c_int,
                ret,
            );
        }
    }

    ret = 0;
    tcsetattr(serial_fd, TCSANOW, &saved_tty);
    close(serial_fd);
    ret
}

fn main() {
    let args: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();
    argv.push(ptr::null_mut());

    let ret = unsafe { main_impl(args.len() as c_int, argv.as_mut_ptr()) };
    std::process::exit(ret);
}
