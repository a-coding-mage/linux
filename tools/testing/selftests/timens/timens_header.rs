/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies removed from executable Rust:
// <fcntl.h>, <unistd.h>, <stdlib.h>, <stdbool.h>, and "kselftest.h".

pub type c_int = i32;
pub type c_long = i64;
pub type c_char = i8;
pub type c_void = core::ffi::c_void;
pub type size_t = usize;
pub type ssize_t = isize;
pub type clockid_t = c_int;
pub type time_t = c_long;

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

pub const CLONE_NEWTIME: c_int = 0x00000080;

pub static mut config_posix_timers: c_int = 1;
pub static mut config_alarm_timers: c_int = 1;

unsafe extern "C" {
    fn timer_create(clockid: clockid_t, sevp: *mut c_void, timerid: *mut c_void) -> c_int;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn ksft_test_result_skip(msg: *const c_char, ...);
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn pr_perror(msg: *const c_char, ...) -> c_int;
}

// Constants supplied by the C headers included above.
unsafe extern "C" {
    static CLOCK_BOOTTIME_ALARM: clockid_t;
    static CLOCK_BOOTTIME: clockid_t;
    static CLOCK_MONOTONIC: clockid_t;
    static CLOCK_REALTIME: clockid_t;
    static CLOCK_MONOTONIC_COARSE: clockid_t;
    static CLOCK_MONOTONIC_RAW: clockid_t;
    static SYS_clock_gettime: c_long;
    static ENOSYS: c_int;
    static EINVAL: c_int;
    static EPERM: c_int;
    static O_WRONLY: c_int;
    static F_OK: c_int;
}

#[inline]
unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

#[inline]
pub unsafe fn check_supported_timers() {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    if unsafe {
        timer_create(
            -1,
            core::ptr::null_mut::<c_void>(),
            core::ptr::null_mut::<c_void>(),
        ) == -1
            && errno() == ENOSYS
    } {
        unsafe {
            config_posix_timers = 0;
        }
    }

    if unsafe { clock_gettime(CLOCK_BOOTTIME_ALARM, &mut ts) == -1 && errno() == EINVAL } {
        unsafe {
            config_alarm_timers = 0;
        }
    }
}

#[inline]
pub unsafe fn check_skip(clockid: c_int) -> bool {
    if unsafe { config_alarm_timers == 0 && clockid == CLOCK_BOOTTIME_ALARM } {
        unsafe {
            ksft_test_result_skip(b"CLOCK_BOOTTIME_ALARM isn't supported\n\0".as_ptr() as *const c_char);
        }
        return true;
    }

    if unsafe { config_posix_timers != 0 } {
        return false;
    }

    unsafe {
        if clockid == CLOCK_BOOTTIME || clockid == CLOCK_MONOTONIC || clockid == CLOCK_REALTIME {
            return false;
        }
    }

    unsafe {
        ksft_test_result_skip(
            b"Posix Clocks & timers are not supported\n\0".as_ptr() as *const c_char,
        );
    }
    true
}

#[inline]
pub unsafe fn unshare_timens() -> c_int {
    if unsafe { unshare(CLONE_NEWTIME) != 0 } {
        if unsafe { errno() == EPERM } {
            unsafe {
                ksft_exit_skip(b"need to run as root\n\0".as_ptr() as *const c_char);
            }
        }
        return unsafe { pr_perror(b"Can't unshare() timens\0".as_ptr() as *const c_char) };
    }
    0
}

#[inline]
pub unsafe fn _settime(mut clk_id: clockid_t, offset: time_t) -> c_int {
    let fd: c_int;
    let len: c_int;
    let mut buf: [c_char; 4096] = [0; 4096];

    if unsafe { clk_id == CLOCK_MONOTONIC_COARSE || clk_id == CLOCK_MONOTONIC_RAW } {
        unsafe {
            clk_id = CLOCK_MONOTONIC;
        }
    }

    len = unsafe {
        snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of_val(&buf),
            b"%d %ld 0\0".as_ptr() as *const c_char,
            clk_id,
            offset,
        )
    };

    fd = unsafe { open(b"/proc/self/timens_offsets\0".as_ptr() as *const c_char, O_WRONLY) };
    if fd < 0 {
        return unsafe {
            pr_perror(b"/proc/self/timens_offsets\0".as_ptr() as *const c_char)
        };
    }

    if unsafe { write(fd, buf.as_ptr() as *const c_void, len as size_t) != len as ssize_t } {
        return unsafe {
            pr_perror(b"/proc/self/timens_offsets\0".as_ptr() as *const c_char)
        };
    }

    unsafe {
        close(fd);
    }

    0
}

#[inline]
pub unsafe fn _gettime(clk_id: clockid_t, res: *mut timespec, raw_syscall: bool) -> c_int {
    let err: c_int;

    if !raw_syscall {
        if unsafe { clock_gettime(clk_id, res) != 0 } {
            unsafe {
                pr_perror(
                    b"clock_gettime(%d)\0".as_ptr() as *const c_char,
                    clk_id as c_int,
                );
            }
            return -1;
        }
        return 0;
    }

    err = unsafe { syscall(SYS_clock_gettime, clk_id, res) as c_int };
    if err != 0 {
        unsafe {
            pr_perror(
                b"syscall(SYS_clock_gettime(%d))\0".as_ptr() as *const c_char,
                clk_id as c_int,
            );
        }
    }

    err
}

#[inline]
pub unsafe fn nscheck() {
    if unsafe { access(b"/proc/self/ns/time\0".as_ptr() as *const c_char, F_OK) < 0 } {
        unsafe {
            ksft_exit_skip(b"Time namespaces are not supported\n\0".as_ptr() as *const c_char);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
