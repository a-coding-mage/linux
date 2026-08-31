// SPDX-License-Identifier: GPL-2.0
/*
 * Real Time Clock Periodic Interrupt test program
 *
 * Since commit 6610e0893b8bc ("RTC: Rework RTC code to use timerqueue for
 * events"), PIE are completely handled using hrtimers, without actually using
 * any underlying hardware RTC.
 *
 */

// C dependencies removed from executable Rust:
// stdio.h, linux/rtc.h, sys/ioctl.h, sys/time.h, sys/types.h, fcntl.h,
// unistd.h, stdlib.h, errno.h, and "kselftest.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;
}

// Constants supplied by included C headers.
const O_RDONLY: c_int = 0;
const EINVAL: c_int = 22;
const KSFT_SKIP: c_int = 4;

// ioctl request constants come from <linux/rtc.h>.
// They are declared here as external dependency symbols for this translation.
unsafe extern "C" {
    static RTC_IRQP_READ: c_ulong;
    static RTC_IRQP_SET: c_ulong;
    static RTC_PIE_ON: c_ulong;
    static RTC_PIE_OFF: c_ulong;
}

/*
 * This expects the new RTC class driver framework, working with
 * clocks that will often not be clones of what the PC-AT had.
 * Use the command line to specify another RTC if you need one.
 */
static DEFAULT_RTC: &[u8; 10] = b"/dev/rtc0\0";

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn timersub(a: *const timeval, b: *const timeval, res: *mut timeval) {
    unsafe {
        (*res).tv_sec = (*a).tv_sec - (*b).tv_sec;
        (*res).tv_usec = (*a).tv_usec - (*b).tv_usec;
        if (*res).tv_usec < 0 {
            (*res).tv_sec -= 1;
            (*res).tv_usec += 1000000;
        }
    }
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut i: c_int;
    let mut fd: c_int;
    let mut retval: c_int;
    let mut tmp: c_ulong;
    let mut data: c_ulong = 0;
    let mut old_pie_rate: c_ulong = 0;
    let mut rtc: *const c_char = DEFAULT_RTC.as_ptr() as *const c_char;
    let mut start = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut end = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut diff = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    unsafe {
        match argc {
            2 => {
                rtc = *argv.add(1);
            }
            1 => {
                fd = open(DEFAULT_RTC.as_ptr() as *const c_char, O_RDONLY);
                if fd == -1 {
                    printf(
                        b"Default RTC %s does not exist. Test Skipped!\n\0".as_ptr()
                            as *const c_char,
                        DEFAULT_RTC.as_ptr() as *const c_char,
                    );
                    exit(KSFT_SKIP);
                }
                close(fd);
            }
            _ => {
                fprintf(
                    stderr,
                    b"usage:  rtctest [rtcdev] [d]\n\0".as_ptr() as *const c_char,
                );
                return 1;
            }
        }

        fd = open(rtc, O_RDONLY);

        if fd == -1 {
            perror(rtc);
            exit(errno_value());
        }

        /* Read periodic IRQ rate */
        retval = ioctl(
            fd,
            RTC_IRQP_READ,
            &mut old_pie_rate as *mut c_ulong,
        );
        if retval == -1 {
            /* not all RTCs support periodic IRQs */
            if errno_value() == EINVAL {
                fprintf(stderr, b"\nNo periodic IRQ support\n\0".as_ptr() as *const c_char);
                goto_done(fd, old_pie_rate);
                return 0;
            }
            perror(b"RTC_IRQP_READ ioctl\0".as_ptr() as *const c_char);
            exit(errno_value());
        }
        fprintf(
            stderr,
            b"\nPeriodic IRQ rate is %ldHz.\n\0".as_ptr() as *const c_char,
            old_pie_rate,
        );

        fprintf(stderr, b"Counting 20 interrupts at:\0".as_ptr() as *const c_char);
        fflush(stderr);

        /* The frequencies 128Hz, 256Hz, ... 8192Hz are only allowed for root. */
        tmp = 2;
        while tmp <= 64 {
            retval = ioctl(fd, RTC_IRQP_SET, tmp);
            if retval == -1 {
                /* not all RTCs can change their periodic IRQ rate */
                if errno_value() == EINVAL {
                    fprintf(
                        stderr,
                        b"\n...Periodic IRQ rate is fixed\n\0".as_ptr() as *const c_char,
                    );
                    goto_done(fd, old_pie_rate);
                    return 0;
                }
                perror(b"RTC_IRQP_SET ioctl\0".as_ptr() as *const c_char);
                exit(errno_value());
            }

            fprintf(stderr, b"\n%ldHz:\t\0".as_ptr() as *const c_char, tmp);
            fflush(stderr);

            /* Enable periodic interrupts */
            retval = ioctl(fd, RTC_PIE_ON, 0);
            if retval == -1 {
                perror(b"RTC_PIE_ON ioctl\0".as_ptr() as *const c_char);
                exit(errno_value());
            }

            i = 1;
            while i < 21 {
                gettimeofday(&mut start as *mut timeval, core::ptr::null_mut());
                /* This blocks */
                retval = read(
                    fd,
                    &mut data as *mut c_ulong as *mut c_void,
                    core::mem::size_of::<c_ulong>(),
                ) as c_int;
                if retval == -1 {
                    perror(b"read\0".as_ptr() as *const c_char);
                    exit(errno_value());
                }
                gettimeofday(&mut end as *mut timeval, core::ptr::null_mut());
                timersub(
                    &end as *const timeval,
                    &start as *const timeval,
                    &mut diff as *mut timeval,
                );
                if diff.tv_sec > 0 || diff.tv_usec > (((1000000 as c_ulong / tmp) as f64 * 1.10) as c_long) {
                    fprintf(
                        stderr,
                        b"\nPIE delta error: %ld.%06ld should be close to 0.%06ld\n\0".as_ptr()
                            as *const c_char,
                        diff.tv_sec,
                        diff.tv_usec,
                        1000000 as c_ulong / tmp,
                    );
                    fflush(stdout);
                    exit(-1);
                }

                fprintf(stderr, b" %d\0".as_ptr() as *const c_char, i);
                fflush(stderr);
                i += 1;
            }

            /* Disable periodic interrupts */
            retval = ioctl(fd, RTC_PIE_OFF, 0);
            if retval == -1 {
                perror(b"RTC_PIE_OFF ioctl\0".as_ptr() as *const c_char);
                exit(errno_value());
            }

            tmp *= 2;
        }

        goto_done(fd, old_pie_rate);

        return 0;
    }
}

unsafe fn goto_done(fd: c_int, old_pie_rate: c_ulong) {
    unsafe {
        ioctl(fd, RTC_IRQP_SET, old_pie_rate);

        fprintf(
            stderr,
            b"\n\n\t\t\t *** Test complete ***\n\0".as_ptr() as *const c_char,
        );

        close(fd);
    }
}
