// SPDX-License-Identifier: GPL-2.0
/*
 * uledmon.c
 *
 * This program creates a new userspace LED class device and monitors it. A
 * timestamp and brightness value is printed each time the brightness changes.
 *
 * Usage: uledmon <device-name>
 *
 * <device-name> is the name of the LED class device to be created. Pressing
 * CTRL+C will exit.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

const LED_MAX_NAME_SIZE: usize = 64;
const O_RDWR: c_int = 2;
const CLOCK_MONOTONIC: c_int = 1;

#[repr(C)]
struct uleds_user_dev {
    name: [c_char; LED_MAX_NAME_SIZE],
    max_brightness: c_int,
}

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut uleds_dev: uleds_user_dev = core::mem::zeroed();
    let mut fd: c_int;
    let mut ret: isize;
    let mut brightness: c_int = 0;
    let mut ts: timespec = core::mem::zeroed();

    if argc != 2 {
        fprintf(
            stderr,
            c"Requires <device-name> argument\n".as_ptr(),
        );
        return 1;
    }

    strncpy(
        uleds_dev.name.as_mut_ptr(),
        *argv.offset(1),
        LED_MAX_NAME_SIZE,
    );
    uleds_dev.max_brightness = 100;

    fd = open(c"/dev/uleds".as_ptr(), O_RDWR);
    if fd == -1 {
        perror(c"Failed to open /dev/uleds".as_ptr());
        return 1;
    }

    ret = write(
        fd,
        &uleds_dev as *const uleds_user_dev as *const c_void,
        core::mem::size_of_val(&uleds_dev),
    );
    if ret == -1 {
        perror(c"Failed to write to /dev/uleds".as_ptr());
        close(fd);
        return 1;
    }

    loop {
        ret = read(
            fd,
            &mut brightness as *mut c_int as *mut c_void,
            core::mem::size_of_val(&brightness),
        );
        if ret == -1 {
            perror(c"Failed to read from /dev/uleds".as_ptr());
            close(fd);
            return 1;
        }
        clock_gettime(CLOCK_MONOTONIC, &mut ts);
        printf(
            c"[%ld.%09ld] %u\n".as_ptr(),
            ts.tv_sec,
            ts.tv_nsec,
            brightness as u32,
        );
    }

    #[allow(unreachable_code)]
    {
        close(fd);

        return 0;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
