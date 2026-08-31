// SPDX-License-Identifier: GPL-2.0
/*
 * vdso_full_test.c: Sample code to test all the timers.
 * Copyright (c) 2019 Arm Ltd.
 *
 * Compile with:
 * gcc -std=gnu99 vdso_full_test.c parse_vdso.c
 *
 */

// C dependencies: stdint.h, elf.h, stdio.h, time.h, sys/auxv.h, sys/time.h,
// unistd.h, sys/syscall.h, kselftest.h, vdso_config.h, vdso_call.h,
// parse_vdso.h.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type clockid_t = c_int;
type time_t = c_long;

const AT_SYSINFO_EHDR: c_ulong = 33;
const CLOCK_REALTIME: clockid_t = 0;
const CLOCK_MONOTONIC: clockid_t = 1;
const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
const CLOCK_MONOTONIC_RAW: clockid_t = 4;
const CLOCK_REALTIME_COARSE: clockid_t = 5;
const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
const CLOCK_BOOTTIME: clockid_t = 7;
const CLOCK_REALTIME_ALARM: clockid_t = 8;
const CLOCK_BOOTTIME_ALARM: clockid_t = 9;
const CLOCK_TAI: clockid_t = 11;
const VDSO_TEST_PLAN: c_uint = 38;

type c_uint = u32;

static mut VERSION: *const c_char = core::ptr::null();
static mut NAME: *mut *const c_char = core::ptr::null_mut();

/* The same as struct __kernel_timespec */
#[repr(C)]
struct vdso_timespec64 {
    tv_sec: u64,
    tv_nsec: u64,
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: c_long,
}

#[repr(C)]
struct timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

type vdso_gettimeofday_t =
    unsafe extern "C" fn(tv: *mut timeval, tz: *mut timezone) -> c_long;
type vdso_clock_gettime_t =
    unsafe extern "C" fn(clk_id: clockid_t, ts: *mut timespec) -> c_long;
type vdso_clock_gettime64_t =
    unsafe extern "C" fn(clk_id: clockid_t, ts: *mut vdso_timespec64) -> c_long;
type vdso_clock_getres_t =
    unsafe extern "C" fn(clk_id: clockid_t, ts: *mut timespec) -> c_long;
type vdso_clock_getres_time64_t =
    unsafe extern "C" fn(clk_id: clockid_t, ts: *mut vdso_timespec64) -> c_long;
type vdso_time_t = unsafe extern "C" fn(t: *mut time_t) -> time_t;

static VDSO_CLOCK_NAME_0: &[u8] = b"CLOCK_REALTIME\0";
static VDSO_CLOCK_NAME_1: &[u8] = b"CLOCK_MONOTONIC\0";
static VDSO_CLOCK_NAME_2: &[u8] = b"CLOCK_PROCESS_CPUTIME_ID\0";
static VDSO_CLOCK_NAME_3: &[u8] = b"CLOCK_THREAD_CPUTIME_ID\0";
static VDSO_CLOCK_NAME_4: &[u8] = b"CLOCK_MONOTONIC_RAW\0";
static VDSO_CLOCK_NAME_5: &[u8] = b"CLOCK_REALTIME_COARSE\0";
static VDSO_CLOCK_NAME_6: &[u8] = b"CLOCK_MONOTONIC_COARSE\0";
static VDSO_CLOCK_NAME_7: &[u8] = b"CLOCK_BOOTTIME\0";
static VDSO_CLOCK_NAME_8: &[u8] = b"CLOCK_REALTIME_ALARM\0";
static VDSO_CLOCK_NAME_9: &[u8] = b"CLOCK_BOOTTIME_ALARM\0";
static VDSO_CLOCK_NAME_10: &[u8] = b"CLOCK_SGI_CYCLE\0";
static VDSO_CLOCK_NAME_11: &[u8] = b"CLOCK_TAI\0";

static VDSO_CLOCK_NAME: [*const c_char; 12] = [
    VDSO_CLOCK_NAME_0.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_1.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_2.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_3.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_4.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_5.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_6.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_7.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_8.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_9.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_10.as_ptr() as *const c_char,
    VDSO_CLOCK_NAME_11.as_ptr() as *const c_char,
];

unsafe extern "C" {
    static versions: [*const c_char; 0];
    static names: [*const *const c_char; 0];
    static VDSO_VERSION: usize;
    static VDSO_NAMES: usize;

    fn getauxval(type_: c_ulong) -> c_ulong;
    fn syscall(number: c_long, ...) -> c_long;
    fn vdso_sym(version: *const c_char, name: *const c_char) -> *mut c_void;
    fn vdso_init_from_sysinfo_ehdr(ehdr: c_ulong);

    fn ksft_print_header();
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_finished() -> !;
}

unsafe fn vdso_name(index: isize) -> *const c_char {
    *NAME.offset(index)
}

unsafe fn vdso_clock_name(clk_id: clockid_t) -> *const c_char {
    VDSO_CLOCK_NAME[clk_id as usize]
}

unsafe fn vdso_test_gettimeofday() {
    /* Find gettimeofday. */
    let vdso_gettimeofday: Option<vdso_gettimeofday_t> =
        core::mem::transmute(vdso_sym(VERSION, vdso_name(0)));

    if vdso_gettimeofday.is_none() {
        ksft_print_msg(
            b"Couldn't find %s\n\0".as_ptr() as *const c_char,
            vdso_name(0),
        );
        ksft_test_result_skip(b"%s\n\0".as_ptr() as *const c_char, vdso_name(0));
        return;
    }

    let mut tv: timeval = core::mem::zeroed();
    let ret = vdso_gettimeofday.unwrap()(&mut tv, core::ptr::null_mut());

    if ret == 0 {
        ksft_print_msg(
            b"The time is %lld.%06lld\n\0".as_ptr() as *const c_char,
            tv.tv_sec as c_long,
            tv.tv_usec as c_long,
        );
        ksft_test_result_pass(b"%s\n\0".as_ptr() as *const c_char, vdso_name(0));
    } else {
        ksft_test_result_fail(b"%s\n\0".as_ptr() as *const c_char, vdso_name(0));
    }
}

unsafe fn vdso_test_clock_gettime64(clk_id: clockid_t) {
    /* Find clock_gettime64. */
    let vdso_clock_gettime64: Option<vdso_clock_gettime64_t> =
        core::mem::transmute(vdso_sym(VERSION, vdso_name(5)));

    if vdso_clock_gettime64.is_none() {
        ksft_print_msg(
            b"Couldn't find %s\n\0".as_ptr() as *const c_char,
            vdso_name(5),
        );
        ksft_test_result_skip(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(5),
            vdso_clock_name(clk_id),
        );
        return;
    }

    let mut ts: vdso_timespec64 = core::mem::zeroed();
    let ret = vdso_clock_gettime64.unwrap()(clk_id, &mut ts);

    if ret == 0 {
        ksft_print_msg(
            b"The time is %lld.%06lld\n\0".as_ptr() as *const c_char,
            ts.tv_sec as c_long,
            ts.tv_nsec as c_long,
        );
        ksft_test_result_pass(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(5),
            vdso_clock_name(clk_id),
        );
    } else {
        ksft_test_result_fail(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(5),
            vdso_clock_name(clk_id),
        );
    }
}

unsafe fn vdso_test_clock_gettime(clk_id: clockid_t) {
    /* Find clock_gettime. */
    let vdso_clock_gettime: Option<vdso_clock_gettime_t> =
        core::mem::transmute(vdso_sym(VERSION, vdso_name(1)));

    if vdso_clock_gettime.is_none() {
        ksft_print_msg(
            b"Couldn't find %s\n\0".as_ptr() as *const c_char,
            vdso_name(1),
        );
        ksft_test_result_skip(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(1),
            vdso_clock_name(clk_id),
        );
        return;
    }

    let mut ts: timespec = core::mem::zeroed();
    let ret = vdso_clock_gettime.unwrap()(clk_id, &mut ts);

    if ret == 0 {
        ksft_print_msg(
            b"The time is %lld.%06lld\n\0".as_ptr() as *const c_char,
            ts.tv_sec as c_long,
            ts.tv_nsec as c_long,
        );
        ksft_test_result_pass(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(1),
            vdso_clock_name(clk_id),
        );
    } else {
        ksft_test_result_fail(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(1),
            vdso_clock_name(clk_id),
        );
    }
}

unsafe fn vdso_test_time() {
    /* Find time. */
    let vdso_time: Option<vdso_time_t> = core::mem::transmute(vdso_sym(VERSION, vdso_name(2)));

    if vdso_time.is_none() {
        ksft_print_msg(
            b"Couldn't find %s\n\0".as_ptr() as *const c_char,
            vdso_name(2),
        );
        ksft_test_result_skip(b"%s\n\0".as_ptr() as *const c_char, vdso_name(2));
        return;
    }

    let ret = vdso_time.unwrap()(core::ptr::null_mut());

    if ret > 0 {
        ksft_print_msg(
            b"The time in hours since January 1, 1970 is %lld\n\0".as_ptr() as *const c_char,
            (ret / 3600) as c_long,
        );
        ksft_test_result_pass(b"%s\n\0".as_ptr() as *const c_char, vdso_name(2));
    } else {
        ksft_test_result_fail(b"%s\n\0".as_ptr() as *const c_char, vdso_name(2));
    }
}

unsafe fn vdso_test_clock_getres(clk_id: clockid_t) {
    let mut clock_getres_fail: c_int = 0;

    /* Find clock_getres. */
    let vdso_clock_getres: Option<vdso_clock_getres_t> =
        core::mem::transmute(vdso_sym(VERSION, vdso_name(3)));

    if vdso_clock_getres.is_none() {
        ksft_print_msg(
            b"Couldn't find %s\n\0".as_ptr() as *const c_char,
            vdso_name(3),
        );
        ksft_test_result_skip(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(3),
            vdso_clock_name(clk_id),
        );
        return;
    }

    let mut ts: timespec = core::mem::zeroed();
    let mut sys_ts: timespec = core::mem::zeroed();
    let mut ret = vdso_clock_getres.unwrap()(clk_id, &mut ts);

    if ret == 0 {
        ksft_print_msg(
            b"The vdso resolution is %lld %lld\n\0".as_ptr() as *const c_char,
            ts.tv_sec as c_long,
            ts.tv_nsec as c_long,
        );
    } else {
        clock_getres_fail += 1;
    }

    ret = syscall(__NR_clock_getres as c_long, clk_id, &mut sys_ts);

    ksft_print_msg(
        b"The syscall resolution is %lld %lld\n\0".as_ptr() as *const c_char,
        sys_ts.tv_sec as c_long,
        sys_ts.tv_nsec as c_long,
    );

    if sys_ts.tv_sec != ts.tv_sec || sys_ts.tv_nsec != ts.tv_nsec {
        clock_getres_fail += 1;
    }

    if clock_getres_fail > 0 {
        ksft_test_result_fail(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(3),
            vdso_clock_name(clk_id),
        );
    } else {
        ksft_test_result_pass(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(3),
            vdso_clock_name(clk_id),
        );
    }
}

unsafe extern "C" {
    static __NR_clock_getres: c_long;
}

// C conditional: #ifdef __NR_clock_getres_time64
unsafe fn vdso_test_clock_getres_time64(clk_id: clockid_t) {
    let mut clock_getres_fail: c_int = 0;

    /* Find clock_getres. */
    let vdso_clock_getres_time64: Option<vdso_clock_getres_time64_t> =
        core::mem::transmute(vdso_sym(VERSION, vdso_name(7)));

    if vdso_clock_getres_time64.is_none() {
        ksft_print_msg(
            b"Couldn't find %s\n\0".as_ptr() as *const c_char,
            vdso_name(7),
        );
        ksft_test_result_skip(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(7),
            vdso_clock_name(clk_id),
        );
        return;
    }

    let mut ts: vdso_timespec64 = core::mem::zeroed();
    let mut sys_ts: vdso_timespec64 = core::mem::zeroed();
    let mut ret = vdso_clock_getres_time64.unwrap()(clk_id, &mut ts);

    if ret == 0 {
        ksft_print_msg(
            b"The vdso resolution is %lld %lld\n\0".as_ptr() as *const c_char,
            ts.tv_sec as c_long,
            ts.tv_nsec as c_long,
        );
    } else {
        clock_getres_fail += 1;
    }

    ret = syscall(__NR_clock_getres_time64 as c_long, clk_id, &mut sys_ts);

    ksft_print_msg(
        b"The syscall resolution is %lld %lld\n\0".as_ptr() as *const c_char,
        sys_ts.tv_sec as c_long,
        sys_ts.tv_nsec as c_long,
    );

    if sys_ts.tv_sec != ts.tv_sec || sys_ts.tv_nsec != ts.tv_nsec {
        clock_getres_fail += 1;
    }

    if clock_getres_fail > 0 {
        ksft_test_result_fail(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(7),
            vdso_clock_name(clk_id),
        );
    } else {
        ksft_test_result_pass(
            b"%s %s\n\0".as_ptr() as *const c_char,
            vdso_name(7),
            vdso_clock_name(clk_id),
        );
    }
}

unsafe extern "C" {
    static __NR_clock_getres_time64: c_long;
}

// C #else branch for !__NR_clock_getres_time64:
unsafe fn vdso_test_clock_getres_time64_skip(clk_id: clockid_t) {
    ksft_test_result_skip(
        b"%s %s\n\0".as_ptr() as *const c_char,
        vdso_name(7),
        vdso_clock_name(clk_id),
    );
}

/*
 * This function calls vdso_test_clock_gettime and vdso_test_clock_getres
 * with different values for clock_id.
 */
unsafe fn vdso_test_clock(clock_id: clockid_t) {
    ksft_print_msg(
        b"clock_id: %s\n\0".as_ptr() as *const c_char,
        vdso_clock_name(clock_id),
    );

    vdso_test_clock_gettime(clock_id);
    vdso_test_clock_gettime64(clock_id);

    vdso_test_clock_getres(clock_id);
    vdso_test_clock_getres_time64(clock_id);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let sysinfo_ehdr: c_ulong = getauxval(AT_SYSINFO_EHDR);

    ksft_print_header();

    if sysinfo_ehdr == 0 {
        ksft_exit_skip(b"AT_SYSINFO_EHDR is not present!\n\0".as_ptr() as *const c_char);
    }

    ksft_set_plan(VDSO_TEST_PLAN);

    VERSION = versions[VDSO_VERSION];
    NAME = names[VDSO_NAMES] as *mut *const c_char;

    ksft_print_msg(
        b"[vDSO kselftest] VDSO_VERSION: %s\n\0".as_ptr() as *const c_char,
        VERSION,
    );

    vdso_init_from_sysinfo_ehdr(getauxval(AT_SYSINFO_EHDR));

    vdso_test_gettimeofday();

    vdso_test_clock(CLOCK_REALTIME);
    vdso_test_clock(CLOCK_BOOTTIME);
    vdso_test_clock(CLOCK_TAI);
    vdso_test_clock(CLOCK_REALTIME_COARSE);
    vdso_test_clock(CLOCK_MONOTONIC);
    vdso_test_clock(CLOCK_MONOTONIC_RAW);
    vdso_test_clock(CLOCK_MONOTONIC_COARSE);
    vdso_test_clock(CLOCK_PROCESS_CPUTIME_ID);
    vdso_test_clock(CLOCK_THREAD_CPUTIME_ID);

    vdso_test_time();

    ksft_finished();
}
