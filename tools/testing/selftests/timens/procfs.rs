// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external libc/test-helper declarations:
// errno.h, fcntl.h, math.h, sched.h, stdio.h, stdbool.h, stdlib.h,
// sys/stat.h, sys/syscall.h, sys/types.h, time.h, unistd.h, "log.h",
// and "timens.h".

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_ulonglong};

const O_RDONLY: c_int = 0;
const CLONE_NEWTIME: c_int = 0x00000080;
const CLOCK_BOOTTIME: c_int = 7;

/*
 * Test shouldn't be run for a day, so add 10 days to child
 * time and check parent's time to be in the same day.
 */
const MAX_TEST_TIME_SEC: c_int = 60 * 5;
const DAY_IN_SEC: c_int = 60 * 60 * 24;
const TEN_DAYS_IN_SEC: c_int = 10 * DAY_IN_SEC;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[allow(non_camel_case_types)]
type time_t = c_long;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: c_ulong,
    pub st_size: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [c_long; 3],
}

static mut CHILD_NS: c_int = 0;
static mut PARENT_NS: c_int = 0;

unsafe extern "C" {
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn difftime(time1: time_t, time0: time_t) -> c_double;
    fn fabs(x: c_double) -> c_double;
    fn __errno_location() -> *mut c_int;

    fn pr_perror(format: *const c_char, ...) -> c_int;
    fn pr_err(format: *const c_char, ...) -> c_int;
    fn pr_fail(format: *const c_char, ...) -> c_int;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_exit_fail();
    fn ksft_exit_pass();
    fn nscheck();
    fn unshare_timens() -> c_int;
    fn _settime(clockid: c_int, offset: time_t) -> c_int;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn switch_ns(fd: c_int) -> c_int {
    if setns(fd, CLONE_NEWTIME) != 0 {
        return pr_perror(c"setns()".as_ptr());
    }

    0
}

unsafe fn init_namespaces() -> c_int {
    let path = *b"/proc/self/ns/time_for_children\0";
    let path = path.as_ptr() as *const c_char;
    let mut st1: stat = core::mem::zeroed();
    let mut st2: stat = core::mem::zeroed();

    PARENT_NS = open(path, O_RDONLY);
    if PARENT_NS <= 0 {
        return pr_perror(c"Unable to open %s".as_ptr(), path);
    }

    if fstat(PARENT_NS, &mut st1) != 0 {
        return pr_perror(c"Unable to stat the parent timens".as_ptr());
    }

    if unshare_timens() != 0 {
        return -1;
    }

    CHILD_NS = open(path, O_RDONLY);
    if CHILD_NS <= 0 {
        return pr_perror(c"Unable to open %s".as_ptr(), path);
    }

    if fstat(CHILD_NS, &mut st2) != 0 {
        return pr_perror(c"Unable to stat the timens".as_ptr());
    }

    if st1.st_ino == st2.st_ino {
        return pr_err(c"The same child_ns after CLONE_NEWTIME".as_ptr());
    }

    if _settime(CLOCK_BOOTTIME, TEN_DAYS_IN_SEC as time_t) != 0 {
        return -1;
    }

    0
}

unsafe fn read_proc_uptime(uptime: *mut timespec) -> c_int {
    let mut up_sec: c_ulong = 0;
    let mut up_nsec: c_ulong = 0;
    let proc: *mut FILE;

    proc = fopen(c"/proc/uptime".as_ptr(), c"r".as_ptr());
    if proc.is_null() {
        pr_perror(c"Unable to open /proc/uptime".as_ptr());
        return -1;
    }

    if fscanf(
        proc,
        c"%lu.%02lu".as_ptr(),
        &mut up_sec as *mut c_ulong,
        &mut up_nsec as *mut c_ulong,
    ) != 2
    {
        if errno() != 0 {
            pr_perror(c"fscanf".as_ptr());
            return -errno();
        }
        pr_err(c"failed to parse /proc/uptime".as_ptr());
        return -1;
    }
    fclose(proc);

    (*uptime).tv_sec = up_sec as time_t;
    (*uptime).tv_nsec = up_nsec as c_long;
    0
}

unsafe fn read_proc_stat_btime(boottime_sec: *mut c_ulonglong) -> c_int {
    let proc: *mut FILE;
    let mut line_buf = [0 as c_char; 2048];

    proc = fopen(c"/proc/stat".as_ptr(), c"r".as_ptr());
    if proc.is_null() {
        pr_perror(c"Unable to open /proc/stat".as_ptr());
        return -1;
    }

    while !fgets(line_buf.as_mut_ptr(), 2048, proc).is_null() {
        if sscanf(line_buf.as_ptr(), c"btime %llu".as_ptr(), boottime_sec) != 1 {
            continue;
        }
        fclose(proc);
        return 0;
    }
    if errno() != 0 {
        pr_perror(c"fscanf".as_ptr());
        fclose(proc);
        return -errno();
    }
    pr_err(c"failed to parse /proc/stat".as_ptr());
    fclose(proc);
    -1
}

unsafe fn check_uptime() -> c_int {
    let mut uptime_new: timespec = core::mem::zeroed();
    let mut uptime_old: timespec = core::mem::zeroed();
    let uptime_expected: time_t;
    let prec: c_double = MAX_TEST_TIME_SEC as c_double;

    if switch_ns(PARENT_NS) != 0 {
        return pr_err(c"switch_ns(%d)".as_ptr(), PARENT_NS);
    }

    if read_proc_uptime(&mut uptime_old) != 0 {
        return 1;
    }

    if switch_ns(CHILD_NS) != 0 {
        return pr_err(c"switch_ns(%d)".as_ptr(), CHILD_NS);
    }

    if read_proc_uptime(&mut uptime_new) != 0 {
        return 1;
    }

    uptime_expected = uptime_old.tv_sec + TEN_DAYS_IN_SEC as time_t;
    if fabs(difftime(uptime_new.tv_sec, uptime_expected)) > prec {
        pr_fail(
            c"uptime in /proc/uptime: old %ld, new %ld [%ld]".as_ptr(),
            uptime_old.tv_sec,
            uptime_new.tv_sec,
            uptime_old.tv_sec + TEN_DAYS_IN_SEC as time_t,
        );
        return 1;
    }

    ksft_test_result_pass(c"Passed for /proc/uptime\n".as_ptr());
    0
}

unsafe fn check_stat_btime() -> c_int {
    let mut btime_new: c_ulonglong = 0;
    let mut btime_old: c_ulonglong = 0;
    let btime_expected: c_ulonglong;

    if switch_ns(PARENT_NS) != 0 {
        return pr_err(c"switch_ns(%d)".as_ptr(), PARENT_NS);
    }

    if read_proc_stat_btime(&mut btime_old) != 0 {
        return 1;
    }

    if switch_ns(CHILD_NS) != 0 {
        return pr_err(c"switch_ns(%d)".as_ptr(), CHILD_NS);
    }

    if read_proc_stat_btime(&mut btime_new) != 0 {
        return 1;
    }

    btime_expected = btime_old.wrapping_sub(TEN_DAYS_IN_SEC as c_ulonglong);
    if btime_new != btime_expected {
        pr_fail(
            c"btime in /proc/stat: old %llu, new %llu [%llu]".as_ptr(),
            btime_old,
            btime_new,
            btime_expected,
        );
        return 1;
    }

    ksft_test_result_pass(c"Passed for /proc/stat btime\n".as_ptr());
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int = 0;

    ksft_print_header();

    nscheck();

    ksft_set_plan(2);

    if init_namespaces() != 0 {
        return 1;
    }

    ret |= check_uptime();
    ret |= check_stat_btime();

    if ret != 0 {
        ksft_exit_fail();
    }
    ksft_exit_pass();
    ret
}
