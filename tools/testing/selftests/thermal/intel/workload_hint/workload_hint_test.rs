// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_short, c_ulong, c_void};

const WORKLOAD_NOTIFICATION_DELAY_ATTRIBUTE: *const c_char =
    b"/sys/bus/pci/devices/0000:00:04.0/workload_hint/notification_delay_ms\0".as_ptr()
        as *const c_char;
const WORKLOAD_ENABLE_ATTRIBUTE: *mut c_char =
    b"/sys/bus/pci/devices/0000:00:04.0/workload_hint/workload_hint_enable\0".as_ptr()
        as *mut c_char;
const WORKLOAD_SLOW_ENABLE_ATTRIBUTE: *mut c_char =
    b"/sys/bus/pci/devices/0000:00:04.0/workload_hint/workload_slow_hint_enable\0".as_ptr()
        as *mut c_char;
const WORKLOAD_TYPE_INDEX_ATTRIBUTE: *const c_char =
    b"/sys/bus/pci/devices/0000:00:04.0/workload_hint/workload_type_index\0".as_ptr()
        as *const c_char;

static workload_types: [*const c_char; 5] = [
    b"idle\0".as_ptr() as *const c_char,
    b"battery_life\0".as_ptr() as *const c_char,
    b"sustained\0".as_ptr() as *const c_char,
    b"bursty\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

static mut wlt_slow: c_int = 0;
static mut wlt_enable_attr: *mut c_char = core::ptr::null_mut();

const WORKLOAD_TYPE_MAX_INDEX: c_int = 3;

const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 2;
const SEEK_SET: c_int = 0;
const POLLPRI: c_short = 0x002;
const SIGINT: c_int = 2;
const SIGHUP: c_int = 1;
const SIGTERM: c_int = 15;
const SIG_IGN: sighandler_t = 1usize;

type size_t = c_ulong;
type ssize_t = c_long;
type off_t = c_long;
type nfds_t = c_ulong;
type sighandler_t = usize;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
}

extern "C" fn workload_hint_exit(_signum: c_int) {
    let fd: c_int;

    /* Disable feature via sysfs knob */

    unsafe {
        fd = open(wlt_enable_attr as *const c_char, O_RDWR);
        if fd < 0 {
            perror(b"Unable to open workload type feature enable file\0".as_ptr() as *const c_char);
            exit(1);
        }

        if write(fd, b"0\n\0".as_ptr() as *const c_void, 2) < 0 {
            perror(b"Can't disable workload hints\0".as_ptr() as *const c_char);
            exit(1);
        }

        printf(b"Disabled workload type prediction\n\0".as_ptr() as *const c_char);

        close(fd);
    }
}

unsafe fn update_delay(delay_str: *mut c_char) {
    let fd: c_int;

    printf(
        b"Setting notification delay in ms to %s\n\0".as_ptr() as *const c_char,
        delay_str,
    );

    fd = open(WORKLOAD_NOTIFICATION_DELAY_ATTRIBUTE, O_RDWR);
    if fd < 0 {
        perror(b"Unable to open workload notification delay\0".as_ptr() as *const c_char);
        exit(1);
    }

    if write(fd, delay_str as *const c_void, strlen(delay_str as *const c_char)) < 0 {
        perror(b"Can't set delay\0".as_ptr() as *const c_char);
        exit(1);
    }

    close(fd);
}

fn main() {
    unsafe {
        let argc = std::env::args_os().count() as c_int;
        let mut argv_storage: Vec<std::ffi::CString> = std::env::args_os()
            .map(|arg| {
                use std::os::unix::ffi::OsStrExt;
                std::ffi::CString::new(arg.as_bytes()).unwrap()
            })
            .collect();
        let mut argv: Vec<*mut c_char> = argv_storage
            .iter_mut()
            .map(|arg| arg.as_ptr() as *mut c_char)
            .collect();
        argv.push(core::ptr::null_mut());
        let argv = argv.as_mut_ptr();

        let mut ufd: pollfd = core::mem::zeroed();
        let mut index_str: [c_char; 4] = [0; 4];
        let mut fd: c_int;
        let mut ret: c_int;
        let mut index: c_int = 0;
        let mut delay_str: [c_char; 64] = [0; 64];
        let mut delay: c_int = 0;

        printf(
            b"Usage: workload_hint_test [notification delay in milli seconds][slow]\n\0".as_ptr()
                as *const c_char,
        );

        if argc > 1 {
            let mut i: c_int;

            i = 1;
            while i < argc {
                if strcmp(*argv.offset(i as isize), b"slow\0".as_ptr() as *const c_char) == 0 {
                    wlt_slow = 1;
                    i += 1;
                    continue;
                }

                ret = sscanf(
                    *argv.offset(1),
                    b"%d\0".as_ptr() as *const c_char,
                    &mut delay as *mut c_int,
                );
                if ret < 0 {
                    printf(b"Invalid delay\n\0".as_ptr() as *const c_char);
                    exit(1);
                }

                sprintf(
                    delay_str.as_mut_ptr(),
                    b"%s\n\0".as_ptr() as *const c_char,
                    *argv.offset(1),
                );
                update_delay(delay_str.as_mut_ptr());

                i += 1;
            }
        }

        if signal(SIGINT, workload_hint_exit as sighandler_t) == SIG_IGN {
            signal(SIGINT, SIG_IGN);
        }
        if signal(SIGHUP, workload_hint_exit as sighandler_t) == SIG_IGN {
            signal(SIGHUP, SIG_IGN);
        }
        if signal(SIGTERM, workload_hint_exit as sighandler_t) == SIG_IGN {
            signal(SIGTERM, SIG_IGN);
        }

        if wlt_slow != 0 {
            wlt_enable_attr = WORKLOAD_SLOW_ENABLE_ATTRIBUTE;
        } else {
            wlt_enable_attr = WORKLOAD_ENABLE_ATTRIBUTE;
        }

        /* Enable feature via sysfs knob */
        fd = open(wlt_enable_attr as *const c_char, O_RDWR);
        if fd < 0 {
            perror(b"Unable to open workload type feature enable file\0".as_ptr() as *const c_char);
            exit(1);
        }

        if write(fd, b"1\n\0".as_ptr() as *const c_void, 2) < 0 {
            perror(b"Can't enable workload hints\0".as_ptr() as *const c_char);
            exit(1);
        }

        close(fd);

        printf(b"Enabled workload type prediction\n\0".as_ptr() as *const c_char);

        loop {
            fd = open(WORKLOAD_TYPE_INDEX_ATTRIBUTE, O_RDONLY);
            if fd < 0 {
                perror(b"Unable to open workload type file\0".as_ptr() as *const c_char);
                exit(1);
            }

            if lseek(fd, 0 as c_long, SEEK_SET) < 0 {
                fprintf(
                    stderr,
                    b"Failed to set pointer to beginning\n\0".as_ptr() as *const c_char,
                );
                exit(1);
            }

            if read(
                fd,
                index_str.as_mut_ptr() as *mut c_void,
                core::mem::size_of_val(&index_str) as size_t,
            ) < 0
            {
                fprintf(
                    stderr,
                    b"Failed to read from:%s\n\0".as_ptr() as *const c_char,
                    WORKLOAD_TYPE_INDEX_ATTRIBUTE,
                );
                exit(1);
            }

            ufd.fd = fd;
            ufd.events = POLLPRI;

            ret = poll(&mut ufd as *mut pollfd, 1, -1);
            if ret < 0 {
                perror(b"poll error\0".as_ptr() as *const c_char);
                exit(1);
            } else if ret == 0 {
                printf(b"Poll Timeout\n\0".as_ptr() as *const c_char);
            } else {
                if lseek(fd, 0 as c_long, SEEK_SET) < 0 {
                    fprintf(
                        stderr,
                        b"Failed to set pointer to beginning\n\0".as_ptr() as *const c_char,
                    );
                    exit(1);
                }

                if read(
                    fd,
                    index_str.as_mut_ptr() as *mut c_void,
                    core::mem::size_of_val(&index_str) as size_t,
                ) < 0
                {
                    exit(0);
                }

                ret = sscanf(
                    index_str.as_ptr(),
                    b"%d\0".as_ptr() as *const c_char,
                    &mut index as *mut c_int,
                );
                if ret < 0 {
                    break;
                }

                if wlt_slow != 0 {
                    if (index & 0x10) != 0 {
                        printf(
                            b"workload type slow:%s\n\0".as_ptr() as *const c_char,
                            b"power\0".as_ptr() as *const c_char,
                        );
                    } else {
                        printf(
                            b"workload type slow:%s\n\0".as_ptr() as *const c_char,
                            b"performance\0".as_ptr() as *const c_char,
                        );
                    }
                }

                index &= 0x0f;
                if index > WORKLOAD_TYPE_MAX_INDEX {
                    printf(b"Invalid workload type index\n\0".as_ptr() as *const c_char);
                } else {
                    printf(
                        b"workload type:%s\n\0".as_ptr() as *const c_char,
                        workload_types[index as usize],
                    );
                }
            }

            close(fd);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
