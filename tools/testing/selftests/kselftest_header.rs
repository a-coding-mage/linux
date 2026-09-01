/* SPDX-License-Identifier: GPL-2.0 */
/*
 * kselftest.h:	low-level kselftest framework to include from
 *		selftest programs. When possible, please use
 *		kselftest_harness.h instead.
 *
 * Copyright (c) 2014 Shuah Khan <shuahkh@osg.samsung.com>
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 *
 * Using this API consists of first counting how many tests your code
 * has to run, and then starting up the reporting:
 *
 *     ksft_print_header();
 *     ksft_set_plan(total_number_of_tests);
 *
 * For each test, report any progress, debugging, etc with:
 *
 *     ksft_print_msg(fmt, ...);
 *     ksft_perror(msg);
 *
 * and finally report the pass/fail/skip/xfail/xpass state of the test
 * with one of:
 *
 *     ksft_test_result(condition, fmt, ...);
 *     ksft_test_result_report(result, fmt, ...);
 *     ksft_test_result_pass(fmt, ...);
 *     ksft_test_result_fail(fmt, ...);
 *     ksft_test_result_skip(fmt, ...);
 *     ksft_test_result_xfail(fmt, ...);
 *     ksft_test_result_xpass(fmt, ...);
 *     ksft_test_result_error(fmt, ...);
 *     ksft_test_result_code(exit_code, test_name, fmt, ...);
 *
 * When all tests are finished, clean up and exit the program with one of:
 *
 *    ksft_finished();
 *    ksft_exit(condition);
 *    ksft_exit_pass();
 *    ksft_exit_fail();
 *
 * If the program wants to report details on why the entire program has
 * failed, it can instead exit with a message (this is usually done when
 * the program is aborting before finishing all tests):
 *
 *    ksft_exit_fail_msg(fmt, ...);
 *    ksft_exit_fail_perror(msg);
 *
 */

use core::ffi::{c_char, c_int, c_uint, c_void, VaList};

/* C header guard and include directives removed.  The C source depended on
 * errno.h, stdlib.h, unistd.h, stdarg.h, stdbool.h, string.h, stdio.h, and
 * sys/utsname.h when NOLIBC was not defined.
 */

#[macro_export]
macro_rules! ARRAY_SIZE {
    ($arr:expr) => {
        (::core::mem::size_of_val(&$arr) / ::core::mem::size_of_val(&$arr[0]))
    };
}

/* On i386/x86_64 the C header conditionally supplied a __cpuid_count macro
 * using inline assembly when the compiler headers did not provide one.
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_export]
macro_rules! __cpuid_count {
    ($level:expr, $count:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        ::core::arch::asm!(
            "cpuid",
            inout("eax") $level => $a,
            lateout("ebx") $b,
            inout("ecx") $count => $c,
            lateout("edx") $d,
            options(nostack, preserves_flags)
        );
    }};
}

/* define kselftest exit codes */
pub const KSFT_PASS: c_int = 0;
pub const KSFT_FAIL: c_int = 1;
pub const KSFT_XFAIL: c_int = 2;
pub const KSFT_XPASS: c_int = 3;
pub const KSFT_SKIP: c_int = 4;

pub const _IOLBF: c_int = 1;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

unsafe extern "C" {
    pub static mut errno: c_int;
    pub static mut stdout: *mut FILE;

    pub fn setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: usize) -> c_int;
    pub fn getenv(name: *const c_char) -> *mut c_char;
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn vprintf(format: *const c_char, arg: VaList<'_, '_>) -> c_int;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn exit(status: c_int) -> !;
    pub fn uname(buf: *mut utsname) -> c_int;
    pub fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
}

/* counters */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksft_count {
    pub ksft_pass: c_uint,
    pub ksft_fail: c_uint,
    pub ksft_xfail: c_uint,
    pub ksft_xpass: c_uint,
    pub ksft_xskip: c_uint,
    pub ksft_error: c_uint,
}

pub static mut ksft_cnt: ksft_count = ksft_count {
    ksft_pass: 0,
    ksft_fail: 0,
    ksft_xfail: 0,
    ksft_xpass: 0,
    ksft_xskip: 0,
    ksft_error: 0,
};
pub static mut ksft_plan: c_uint = 0;
pub static mut ksft_debug_enabled: bool = false;

pub unsafe fn ksft_test_num() -> c_uint {
    unsafe {
        ksft_cnt
            .ksft_pass
            .wrapping_add(ksft_cnt.ksft_fail)
            .wrapping_add(ksft_cnt.ksft_xfail)
            .wrapping_add(ksft_cnt.ksft_xpass)
            .wrapping_add(ksft_cnt.ksft_xskip)
            .wrapping_add(ksft_cnt.ksft_error)
    }
}

pub unsafe fn ksft_inc_pass_cnt() {
    unsafe { ksft_cnt.ksft_pass = ksft_cnt.ksft_pass.wrapping_add(1) };
}
pub unsafe fn ksft_inc_fail_cnt() {
    unsafe { ksft_cnt.ksft_fail = ksft_cnt.ksft_fail.wrapping_add(1) };
}
pub unsafe fn ksft_inc_xfail_cnt() {
    unsafe { ksft_cnt.ksft_xfail = ksft_cnt.ksft_xfail.wrapping_add(1) };
}
pub unsafe fn ksft_inc_xpass_cnt() {
    unsafe { ksft_cnt.ksft_xpass = ksft_cnt.ksft_xpass.wrapping_add(1) };
}
pub unsafe fn ksft_inc_xskip_cnt() {
    unsafe { ksft_cnt.ksft_xskip = ksft_cnt.ksft_xskip.wrapping_add(1) };
}
pub unsafe fn ksft_inc_error_cnt() {
    unsafe { ksft_cnt.ksft_error = ksft_cnt.ksft_error.wrapping_add(1) };
}

pub unsafe fn ksft_get_pass_cnt() -> c_int {
    unsafe { ksft_cnt.ksft_pass as c_int }
}
pub unsafe fn ksft_get_fail_cnt() -> c_int {
    unsafe { ksft_cnt.ksft_fail as c_int }
}
pub unsafe fn ksft_get_xfail_cnt() -> c_int {
    unsafe { ksft_cnt.ksft_xfail as c_int }
}
pub unsafe fn ksft_get_xpass_cnt() -> c_int {
    unsafe { ksft_cnt.ksft_xpass as c_int }
}
pub unsafe fn ksft_get_xskip_cnt() -> c_int {
    unsafe { ksft_cnt.ksft_xskip as c_int }
}
pub unsafe fn ksft_get_error_cnt() -> c_int {
    unsafe { ksft_cnt.ksft_error as c_int }
}

pub unsafe fn ksft_print_header() {
    unsafe {
        /*
         * Force line buffering; If stdout is not connected to a terminal, it
         * will otherwise default to fully buffered, which can cause output
         * duplication if there is content in the buffer when fork()ing. If
         * there is a crash, line buffering also means the most recent output
         * line will be visible.
         */
        setvbuf(stdout, core::ptr::null_mut(), _IOLBF, 0);

        if getenv(c"KSFT_TAP_LEVEL".as_ptr()).is_null() {
            printf(c"TAP version 13\n".as_ptr());
        }
    }
}

pub unsafe fn ksft_set_plan(plan: c_uint) {
    unsafe {
        ksft_plan = plan;
        printf(c"1..%u\n".as_ptr(), ksft_plan);
    }
}

pub unsafe fn ksft_print_cnts() {
    unsafe {
        if ksft_cnt.ksft_xskip > 0 {
            printf(
                c"# %u skipped test(s) detected. Consider enabling relevant config options to improve coverage.\n"
                    .as_ptr(),
                ksft_cnt.ksft_xskip,
            );
        }
        if ksft_plan != ksft_test_num() {
            printf(
                c"# Planned tests != run tests (%u != %u)\n".as_ptr(),
                ksft_plan,
                ksft_test_num(),
            );
        }
        printf(
            c"# Totals: pass:%u fail:%u xfail:%u xpass:%u skip:%u error:%u\n".as_ptr(),
            ksft_cnt.ksft_pass,
            ksft_cnt.ksft_fail,
            ksft_cnt.ksft_xfail,
            ksft_cnt.ksft_xpass,
            ksft_cnt.ksft_xskip,
            ksft_cnt.ksft_error,
        );
    }
}

pub unsafe extern "C" fn ksft_print_msg(msg: *const c_char, mut args: ...) {
    unsafe {
        let saved_errno: c_int = errno;

        printf(c"# ".as_ptr());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());
    }
}

pub unsafe extern "C" fn ksft_print_dbg_msg(msg: *const c_char, mut args: ...) {
    unsafe {
        if !ksft_debug_enabled {
            return;
        }

        ksft_print_msg(msg, args.as_va_list());
    }
}

pub unsafe fn ksft_perror(msg: *const c_char) {
    unsafe {
        ksft_print_msg(c"%s: %s (%d)\n".as_ptr(), msg, strerror(errno), errno);
    }
}

pub unsafe extern "C" fn ksft_test_result_pass(msg: *const c_char, mut args: ...) {
    unsafe {
        let saved_errno: c_int = errno;

        ksft_cnt.ksft_pass = ksft_cnt.ksft_pass.wrapping_add(1);

        printf(c"ok %u ".as_ptr(), ksft_test_num());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());
    }
}

pub unsafe extern "C" fn ksft_test_result_fail(msg: *const c_char, mut args: ...) {
    unsafe {
        let saved_errno: c_int = errno;

        ksft_cnt.ksft_fail = ksft_cnt.ksft_fail.wrapping_add(1);

        printf(c"not ok %u ".as_ptr(), ksft_test_num());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());
    }
}

/**
 * ksft_test_result() - Report test success based on truth of condition
 *
 * @condition: if true, report test success, otherwise failure.
 */
#[macro_export]
macro_rules! ksft_test_result {
    ($condition:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        if ($condition) != 0 {
            unsafe { ksft_test_result_pass($fmt $(, $arg)*); }
        } else {
            unsafe { ksft_test_result_fail($fmt $(, $arg)*); }
        }
    }};
}

pub unsafe extern "C" fn ksft_test_result_xfail(msg: *const c_char, mut args: ...) {
    unsafe {
        let saved_errno: c_int = errno;

        ksft_cnt.ksft_xfail = ksft_cnt.ksft_xfail.wrapping_add(1);

        printf(c"ok %u # XFAIL ".as_ptr(), ksft_test_num());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());
    }
}

pub unsafe extern "C" fn ksft_test_result_xpass(msg: *const c_char, mut args: ...) {
    unsafe {
        let saved_errno: c_int = errno;

        ksft_cnt.ksft_xpass = ksft_cnt.ksft_xpass.wrapping_add(1);

        printf(c"ok %u # XPASS ".as_ptr(), ksft_test_num());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());
    }
}

pub unsafe extern "C" fn ksft_test_result_skip(msg: *const c_char, mut args: ...) {
    unsafe {
        let saved_errno: c_int = errno;

        ksft_cnt.ksft_xskip = ksft_cnt.ksft_xskip.wrapping_add(1);

        printf(c"ok %u # SKIP ".as_ptr(), ksft_test_num());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());
    }
}

/* TODO: how does "error" differ from "fail" or "skip"? */
pub unsafe extern "C" fn ksft_test_result_error(msg: *const c_char, mut args: ...) {
    unsafe {
        let saved_errno: c_int = errno;

        ksft_cnt.ksft_error = ksft_cnt.ksft_error.wrapping_add(1);

        printf(c"not ok %u # error ".as_ptr(), ksft_test_num());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());
    }
}

pub unsafe extern "C" fn ksft_test_result_code(
    exit_code: c_int,
    test_name: *const c_char,
    msg: *const c_char,
    mut args: ...
) {
    unsafe {
        let mut tap_code: *const c_char = c"ok".as_ptr();
        let mut directive: *const c_char = c"".as_ptr();
        let saved_errno: c_int = errno;

        match exit_code {
            KSFT_PASS => {
                ksft_cnt.ksft_pass = ksft_cnt.ksft_pass.wrapping_add(1);
            }
            KSFT_XFAIL => {
                directive = c" # XFAIL ".as_ptr();
                ksft_cnt.ksft_xfail = ksft_cnt.ksft_xfail.wrapping_add(1);
            }
            KSFT_XPASS => {
                directive = c" # XPASS ".as_ptr();
                ksft_cnt.ksft_xpass = ksft_cnt.ksft_xpass.wrapping_add(1);
            }
            KSFT_SKIP => {
                directive = c" # SKIP ".as_ptr();
                ksft_cnt.ksft_xskip = ksft_cnt.ksft_xskip.wrapping_add(1);
            }
            KSFT_FAIL | _ => {
                tap_code = c"not ok".as_ptr();
                ksft_cnt.ksft_fail = ksft_cnt.ksft_fail.wrapping_add(1);
            }
        }

        /* Docs seem to call for double space if directive is absent */
        if *directive == 0 && !msg.is_null() {
            directive = c" #  ".as_ptr();
        }

        printf(
            c"%s %u %s%s".as_ptr(),
            tap_code,
            ksft_test_num(),
            test_name,
            directive,
        );
        errno = saved_errno;
        if !msg.is_null() {
            vprintf(msg, args.as_va_list());
        }
        printf(c"\n".as_ptr());
    }
}

/**
 * ksft_test_result_report() - Report test result based on a kselftest exit code
 *
 * @result: a kselftest exit code
 */
#[macro_export]
macro_rules! ksft_test_result_report {
    ($result:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        match $result {
            KSFT_PASS => unsafe { ksft_test_result_pass($fmt $(, $arg)*); },
            KSFT_FAIL => unsafe { ksft_test_result_fail($fmt $(, $arg)*); },
            KSFT_XFAIL => unsafe { ksft_test_result_xfail($fmt $(, $arg)*); },
            KSFT_XPASS => unsafe { ksft_test_result_xpass($fmt $(, $arg)*); },
            KSFT_SKIP => unsafe { ksft_test_result_skip($fmt $(, $arg)*); },
            _ => {}
        }
    }};
}

pub unsafe fn ksft_exit_pass() -> ! {
    unsafe {
        ksft_print_cnts();
        exit(KSFT_PASS);
    }
}

pub unsafe fn ksft_exit_fail() -> ! {
    unsafe {
        ksft_print_cnts();
        exit(KSFT_FAIL);
    }
}

/**
 * ksft_exit() - Exit selftest based on truth of condition
 *
 * @condition: if true, exit self test with success, otherwise fail.
 */
#[macro_export]
macro_rules! ksft_exit {
    ($condition:expr) => {{
        if ($condition) != 0 {
            unsafe { ksft_exit_pass(); }
        } else {
            unsafe { ksft_exit_fail(); }
        }
    }};
}

/**
 * ksft_finished() - Exit selftest with success if all tests passed
 */
#[macro_export]
macro_rules! ksft_finished {
    () => {{
        unsafe {
            ksft_exit!(
                (ksft_plan
                    == ksft_cnt
                        .ksft_pass
                        .wrapping_add(ksft_cnt.ksft_xpass)
                        .wrapping_add(ksft_cnt.ksft_xfail)
                        .wrapping_add(ksft_cnt.ksft_xskip)) as c_int
            );
        }
    }};
}

pub unsafe extern "C" fn ksft_exit_fail_msg(msg: *const c_char, mut args: ...) -> ! {
    unsafe {
        let saved_errno: c_int = errno;

        printf(c"Bail out! ".as_ptr());
        errno = saved_errno;
        vprintf(msg, args.as_va_list());

        ksft_print_cnts();
        exit(KSFT_FAIL);
    }
}

pub unsafe fn ksft_exit_fail_perror(msg: *const c_char) -> ! {
    unsafe {
        ksft_exit_fail_msg(c"%s: %s (%d)\n".as_ptr(), msg, strerror(errno), errno);
    }
}

pub unsafe fn ksft_exit_xfail() -> ! {
    unsafe {
        ksft_print_cnts();
        exit(KSFT_XFAIL);
    }
}

pub unsafe fn ksft_exit_xpass() -> ! {
    unsafe {
        ksft_print_cnts();
        exit(KSFT_XPASS);
    }
}

pub unsafe extern "C" fn ksft_exit_skip(msg: *const c_char, mut args: ...) -> ! {
    unsafe {
        let saved_errno: c_int = errno;

        /*
         * FIXME: several tests misuse ksft_exit_skip so produce
         * something sensible if some tests have already been run
         * or a plan has been printed.  Those tests should use
         * ksft_test_result_skip or ksft_exit_fail_msg instead.
         */
        if ksft_plan != 0 || ksft_test_num() != 0 {
            ksft_cnt.ksft_xskip = ksft_cnt.ksft_xskip.wrapping_add(1);
            printf(c"ok %u # SKIP ".as_ptr(), ksft_test_num());
        } else {
            printf(c"1..0 # SKIP ".as_ptr());
        }
        if !msg.is_null() {
            errno = saved_errno;
            vprintf(msg, args.as_va_list());
        }
        if ksft_test_num() != 0 {
            ksft_print_cnts();
        }
        exit(KSFT_SKIP);
    }
}

pub unsafe fn ksft_min_kernel_version(min_major: c_uint, min_minor: c_uint) -> c_int {
    unsafe {
        let mut major: c_uint = 0;
        let mut minor: c_uint = 0;
        let mut info: utsname = core::mem::zeroed();

        if uname(&mut info) != 0
            || sscanf(
                info.release.as_ptr(),
                c"%u.%u.".as_ptr(),
                &mut major as *mut c_uint,
                &mut minor as *mut c_uint,
            ) != 2
        {
            ksft_exit_fail_msg(c"Can't parse kernel version\n".as_ptr());
        }

        (major > min_major || (major == min_major && minor >= min_minor)) as c_int
    }
}

pub unsafe fn ksft_reset_state() {
    unsafe {
        ksft_cnt.ksft_pass = 0;
        ksft_cnt.ksft_fail = 0;
        ksft_cnt.ksft_xfail = 0;
        ksft_cnt.ksft_xpass = 0;
        ksft_cnt.ksft_xskip = 0;
        ksft_cnt.ksft_error = 0;
        ksft_plan = 0;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
