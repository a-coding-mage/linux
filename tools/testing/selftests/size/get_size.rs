// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014 Sony Mobile Communications Inc.
 *
 * Selftest for runtime system size
 *
 * Prints the amount of RAM that the currently running system is using.
 *
 * This program tries to be as small as possible itself, to
 * avoid perturbing the system memory utilization with its
 * own execution.  It also attempts to have as few dependencies
 * on kernel features as possible.
 *
 * It should be statically linked, with startup libs avoided.  It uses
 * no library calls except the syscall() function for the following 3
 * syscalls:
 *   sysinfo(), write(), and _exit()
 *
 * For output, it avoids printf (which in some C libraries
 * has large external dependencies) by  implementing it's own
 * number output and print routines, and using __builtin_strlen()
 *
 * The test may crash if any of the above syscalls fails because in some
 * libc implementations (e.g. the GNU C Library) errno is saved in
 * thread-local storage, which does not get initialized due to avoiding
 * startup libs.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_ulonglong, c_void};

// C dependencies: <sys/sysinfo.h>, <unistd.h>, and <sys/syscall.h>.
const STDOUT_FILENO: c_int = 1;

const SYS_WRITE: c_long = 1;
const SYS_SYSINFO: c_long = 99;
const SYS_EXIT: c_long = 60;

#[repr(C)]
struct sysinfo {
    uptime: c_long,
    loads: [c_ulong; 3],
    totalram: c_ulong,
    freeram: c_ulong,
    sharedram: c_ulong,
    bufferram: c_ulong,
    totalswap: c_ulong,
    freeswap: c_ulong,
    procs: u16,
    pad: u16,
    totalhigh: c_ulong,
    freehigh: c_ulong,
    mem_unit: u32,
    _f: [c_char; 0],
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
}

unsafe fn print(s: *const c_char) -> c_int {
    let mut len: usize = 0;

    while unsafe { *s.add(len) } != 0 {
        len += 1;
    }

    unsafe { syscall(SYS_WRITE, STDOUT_FILENO, s, len) as c_int }
}

unsafe fn num_to_str(mut num: c_ulong, mut buf: *mut c_char, len: c_int) -> *mut c_char {
    let mut digit: u32;

    /* put digits in buffer from back to front */
    buf = unsafe { buf.add((len - 1) as usize) };
    unsafe {
        *buf = 0;
    }
    loop {
        digit = (num % 10) as u32;
        buf = unsafe { buf.sub(1) };
        unsafe {
            *buf = (digit + b'0' as u32) as c_char;
        }
        num /= 10;
        if num == 0 {
            break;
        }
    }

    buf
}

unsafe fn print_num(num: c_ulong) -> c_int {
    let mut num_buf: [c_char; 30] = [0; 30];

    unsafe { print(num_to_str(num, num_buf.as_mut_ptr(), num_buf.len() as c_int)) }
}

unsafe fn print_k_value(s: *const c_char, mut num: c_ulong, units: c_ulong) -> c_int {
    let mut temp: c_ulonglong;
    let ccode: c_int;

    unsafe {
        print(s);
    }

    temp = num as c_ulonglong;
    temp = (temp * units as c_ulonglong) / 1024;
    num = temp as c_ulong;
    ccode = unsafe { print_num(num) };
    unsafe {
        print(c_str(b"\n\0"));
    }
    ccode
}

const fn c_str(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

/* this program has no main(), as startup libraries are not used */
#[no_mangle]
pub unsafe extern "C" fn _start() {
    let mut ccode: c_int;
    let mut info: sysinfo = unsafe { core::mem::zeroed() };
    let used: c_ulong;
    let test_name: *const c_char = c_str(b" get runtime memory use\n\0");

    unsafe {
        print(c_str(b"TAP version 13\n\0"));
        print(c_str(b"# Testing system size.\n\0"));
    }

    ccode = unsafe { syscall(SYS_SYSINFO, &mut info as *mut sysinfo as *mut c_void) as c_int };
    if ccode < 0 {
        unsafe {
            print(c_str(b"not ok 1\0"));
            print(test_name);
            print(c_str(b" ---\n reason: \"could not get sysinfo\"\n ...\n\0"));
            syscall(SYS_EXIT, ccode);
        }
    }
    unsafe {
        print(c_str(b"ok 1\0"));
        print(test_name);
    }

    /* ignore cache complexities for now */
    used = info.totalram - info.freeram - info.bufferram;
    unsafe {
        print(c_str(b"# System runtime memory report (units in Kilobytes):\n\0"));
        print(c_str(b" ---\n\0"));
        print_k_value(c_str(b" Total:  \0"), info.totalram, info.mem_unit as c_ulong);
        print_k_value(c_str(b" Free:   \0"), info.freeram, info.mem_unit as c_ulong);
        print_k_value(c_str(b" Buffer: \0"), info.bufferram, info.mem_unit as c_ulong);
        print_k_value(c_str(b" In use: \0"), used, info.mem_unit as c_ulong);
        print(c_str(b" ...\n\0"));
        print(c_str(b"1..1\n\0"));

        syscall(SYS_EXIT, 0);
    }
}
