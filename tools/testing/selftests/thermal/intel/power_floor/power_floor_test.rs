// SPDX-License-Identifier: GPL-2.0

// Original C source requested _GNU_SOURCE and included:
// stdio.h, string.h, stdlib.h, unistd.h, fcntl.h, poll.h, signal.h.

use core::ffi::{c_char, c_int, c_short, c_ulong, c_void};

const POWER_FLOOR_ENABLE_ATTRIBUTE: &[u8] =
    b"/sys/bus/pci/devices/0000:00:04.0/power_limits/power_floor_enable\0";
const POWER_FLOOR_STATUS_ATTRIBUTE: &[u8] =
    b"/sys/bus/pci/devices/0000:00:04.0/power_limits/power_floor_status\0";

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const SEEK_SET: c_int = 0;
const POLLPRI: c_short = 0x002;
const SIGHUP: c_int = 1;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIG_IGN: usize = 1;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;
    fn lseek(fd: c_int, offset: i64, whence: c_int) -> i64;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
}

unsafe extern "C" fn power_floor_exit(_signum: c_int) {
    let fd: c_int;

    /* Disable feature via sysfs knob */

    fd = unsafe { open(POWER_FLOOR_ENABLE_ATTRIBUTE.as_ptr() as *const c_char, O_RDWR) };
    if fd < 0 {
        unsafe { perror(c"Unable to open power floor enable file\n".as_ptr()) };
        unsafe { exit(1) };
    }

    if unsafe { write(fd, c"0\n".as_ptr() as *const c_void, 2) } < 0 {
        unsafe { perror(c"Can' disable power floor notifications\n".as_ptr()) };
        unsafe { exit(1) };
    }

    unsafe { printf(c"Disabled power floor notifications\n".as_ptr()) };

    unsafe { close(fd) };
}

fn main() {
    unsafe {
        let mut ufd: pollfd = core::mem::zeroed();
        let mut status_str: [c_char; 3] = [0; 3];
        let mut fd: c_int;
        let mut ret: c_int;

        if signal(SIGINT, power_floor_exit as usize) == SIG_IGN {
            signal(SIGINT, SIG_IGN);
        }
        if signal(SIGHUP, power_floor_exit as usize) == SIG_IGN {
            signal(SIGHUP, SIG_IGN);
        }
        if signal(SIGTERM, power_floor_exit as usize) == SIG_IGN {
            signal(SIGTERM, SIG_IGN);
        }

        /* Enable feature via sysfs knob */
        fd = open(POWER_FLOOR_ENABLE_ATTRIBUTE.as_ptr() as *const c_char, O_RDWR);
        if fd < 0 {
            perror(c"Unable to open power floor enable file\n".as_ptr());
            exit(1);
        }

        if write(fd, c"1\n".as_ptr() as *const c_void, 2) < 0 {
            perror(c"Can't enable power floor notifications\n".as_ptr());
            exit(1);
        }

        close(fd);

        printf(c"Enabled power floor notifications\n".as_ptr());

        loop {
            fd = open(POWER_FLOOR_STATUS_ATTRIBUTE.as_ptr() as *const c_char, O_RDONLY);
            if fd < 0 {
                perror(c"Unable to power floor status file\n".as_ptr());
                exit(1);
            }

            if lseek(fd, 0_i64, SEEK_SET) < 0 {
                fprintf(stderr, c"Failed to set pointer to beginning\n".as_ptr());
                exit(1);
            }

            if read(
                fd,
                status_str.as_mut_ptr() as *mut c_void,
                core::mem::size_of_val(&status_str),
            ) < 0
            {
                fprintf(
                    stderr,
                    c"Failed to read from:%s\n".as_ptr(),
                    POWER_FLOOR_STATUS_ATTRIBUTE.as_ptr() as *const c_char,
                );
                exit(1);
            }

            ufd.fd = fd;
            ufd.events = POLLPRI;

            ret = poll(&mut ufd as *mut pollfd, 1, -1);
            if ret < 0 {
                perror(c"poll error".as_ptr());
                exit(1);
            } else if ret == 0 {
                printf(c"Poll Timeout\n".as_ptr());
            } else {
                if lseek(fd, 0_i64, SEEK_SET) < 0 {
                    fprintf(stderr, c"Failed to set pointer to beginning\n".as_ptr());
                    exit(1);
                }

                if read(
                    fd,
                    status_str.as_mut_ptr() as *mut c_void,
                    core::mem::size_of_val(&status_str),
                ) < 0
                {
                    exit(0);
                }

                printf(c"power floor status: %s\n".as_ptr(), status_str.as_ptr());
            }

            close(fd);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
