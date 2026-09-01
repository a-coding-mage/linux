// SPDX-License-Identifier: GPL-2.0
/*
 * led_hw_brightness_mon.c
 *
 * This program monitors LED brightness level changes having its origin
 * in hardware/firmware, i.e. outside of kernel control.
 * A timestamp and brightness value is printed each time the brightness changes.
 *
 * Usage: led_hw_brightness_mon <device-name>
 *
 * <device-name> is the name of the LED class device to be monitored. Pressing
 * CTRL+C will exit.
 */

use core::ffi::c_void;
use core::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_long};

const LED_MAX_NAME_SIZE: usize = 64;
const O_RDONLY: c_int = 0;
const POLLPRI: i16 = 0x002;
const CLOCK_MONOTONIC: c_int = 1;
const SEEK_SET: c_int = 0;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: i16,
    revents: i16,
}

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn poll(fds: *mut pollfd, nfds: usize, timeout: c_int) -> c_int;
    fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    fn atoi(nptr: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut fd: c_int;
    let mut ret: c_int;
    let mut brightness_file_path: [c_char; LED_MAX_NAME_SIZE + 11] =
        [0; LED_MAX_NAME_SIZE + 11];
    let mut pollfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut buf: MaybeUninit<[c_char; 11]> = MaybeUninit::uninit();

    if argc != 2 {
        fprintf(
            stderr,
            b"Requires <device-name> argument\n\0".as_ptr() as *const c_char,
        );
        return 1;
    }

    snprintf(
        brightness_file_path.as_mut_ptr(),
        LED_MAX_NAME_SIZE,
        b"/sys/class/leds/%s/brightness_hw_changed\0".as_ptr() as *const c_char,
        *argv.add(1),
    );

    fd = open(brightness_file_path.as_ptr(), O_RDONLY);
    if fd == -1 {
        printf(
            b"Failed to open %s file\n\0".as_ptr() as *const c_char,
            brightness_file_path.as_ptr(),
        );
        return 1;
    }

    /*
     * read may fail if no hw brightness change has occurred so far,
     * but it is required to avoid spurious poll notifications in
     * the opposite case.
     */
    read(fd, buf.as_mut_ptr() as *mut c_void, 11);

    pollfd.fd = fd;
    pollfd.events = POLLPRI;

    loop {
        ret = poll(&mut pollfd, 1, -1);
        if ret == -1 {
            printf(
                b"Failed to poll %s file (%d)\n\0".as_ptr() as *const c_char,
                brightness_file_path.as_ptr(),
                ret,
            );
            ret = 1;
            break;
        }

        clock_gettime(CLOCK_MONOTONIC, &mut ts);

        ret = read(fd, buf.as_mut_ptr() as *mut c_void, 11) as c_int;
        if ret < 0 {
            break;
        }

        ret = lseek(pollfd.fd, 0, SEEK_SET) as c_int;
        if ret < 0 {
            printf(b"lseek failed (%d)\n\0".as_ptr() as *const c_char, ret);
            break;
        }

        printf(
            b"[%ld.%09ld] %d\n\0".as_ptr() as *const c_char,
            ts.tv_sec,
            ts.tv_nsec,
            atoi(buf.as_ptr() as *const c_char),
        );
    }

    close(fd);

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
