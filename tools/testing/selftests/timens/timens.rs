// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/timens/timens.c.
// C dependencies removed: errno.h, fcntl.h, sched.h, stdio.h, stdbool.h,
// sys/stat.h, sys/syscall.h, sys/types.h, time.h, unistd.h, string.h,
// "log.h", and "timens.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint};

type clockid_t = c_int;
type time_t = c_long;

const O_RDONLY: c_int = 0;
const CLONE_NEWTIME: c_int = 0x00000080;

const CLOCK_MONOTONIC: clockid_t = 1;
const CLOCK_MONOTONIC_RAW: clockid_t = 4;
const CLOCK_BOOTTIME: clockid_t = 7;
const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
const CLOCK_BOOTTIME_ALARM: clockid_t = 9;

/*
 * Test shouldn't be run for a day, so add 10 days to child
 * time and check parent's time to be in the same day.
 */
const DAY_IN_SEC: time_t = 60 * 60 * 24;
const TEN_DAYS_IN_SEC: time_t = 10 * DAY_IN_SEC;

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct stat {
    st_dev: u64,
    st_ino: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct test_clock {
    id: clockid_t,
    name: *mut c_char,
    /*
     * off_id is -1 if a clock has own offset, or it contains an index
     * which contains a right offset of this clock.
     */
    off_id: c_int,
    offset: time_t,
}

const fn c_name(bytes: &'static [u8]) -> *mut c_char {
    bytes.as_ptr() as *mut c_char
}

static mut clocks: [test_clock; 5] = [
    test_clock {
        id: CLOCK_BOOTTIME,
        name: c_name(b"CLOCK_BOOTTIME\0"),
        off_id: -1,
        offset: 0,
    },
    test_clock {
        id: CLOCK_BOOTTIME_ALARM,
        name: c_name(b"CLOCK_BOOTTIME_ALARM\0"),
        off_id: 1,
        offset: 0,
    },
    test_clock {
        id: CLOCK_MONOTONIC,
        name: c_name(b"CLOCK_MONOTONIC\0"),
        off_id: -1,
        offset: 0,
    },
    test_clock {
        id: CLOCK_MONOTONIC_COARSE,
        name: c_name(b"CLOCK_MONOTONIC_COARSE\0"),
        off_id: 1,
        offset: 0,
    },
    test_clock {
        id: CLOCK_MONOTONIC_RAW,
        name: c_name(b"CLOCK_MONOTONIC_RAW\0"),
        off_id: 1,
        offset: 0,
    },
];

static mut child_ns: c_int = 0;
static mut parent_ns: c_int = -1;

unsafe extern "C" {
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn clock_settime(clk_id: clockid_t, tp: *const timespec) -> c_int;
    fn difftime(time1: time_t, time0: time_t) -> c_double;

    fn pr_perror(fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;

    fn nscheck();
    fn check_supported_timers();
    fn check_skip(clock: clockid_t) -> c_int;
    fn unshare_timens() -> c_int;
    fn _gettime(clock: clockid_t, ts: *mut timespec, raw_syscall: bool) -> c_int;
    fn _settime(clock: clockid_t, offset: time_t) -> c_int;
}

unsafe fn switch_ns(fd: c_int) -> c_int {
    if setns(fd, CLONE_NEWTIME) != 0 {
        pr_perror(c"setns()".as_ptr());
        return -1;
    }

    0
}

unsafe fn init_namespaces() -> c_int {
    let path = c"/proc/self/ns/time_for_children";
    let mut st1: stat = core::mem::zeroed();
    let mut st2: stat = core::mem::zeroed();

    if parent_ns == -1 {
        parent_ns = open(path.as_ptr(), O_RDONLY);
        if parent_ns <= 0 {
            return pr_perror(c"Unable to open %s".as_ptr(), path.as_ptr());
        }
    }

    if fstat(parent_ns, &mut st1) != 0 {
        return pr_perror(c"Unable to stat the parent timens".as_ptr());
    }

    if unshare_timens() != 0 {
        return -1;
    }

    child_ns = open(path.as_ptr(), O_RDONLY);
    if child_ns <= 0 {
        return pr_perror(c"Unable to open %s".as_ptr(), path.as_ptr());
    }

    if fstat(child_ns, &mut st2) != 0 {
        return pr_perror(c"Unable to stat the timens".as_ptr());
    }

    if st1.st_ino == st2.st_ino {
        return pr_perror(c"The same child_ns after CLONE_NEWTIME".as_ptr());
    }

    0
}

unsafe fn test_gettime(clock_index: clockid_t, raw_syscall: bool, offset: time_t) -> c_int {
    let mut child_ts_new: timespec = core::mem::zeroed();
    let mut parent_ts_old: timespec = core::mem::zeroed();
    let mut cur_ts: timespec = core::mem::zeroed();
    let entry = if raw_syscall {
        c"syscall".as_ptr()
    } else {
        c"vdso".as_ptr()
    };
    let mut precision: c_double = 0.0;
    let clock_index_usize = clock_index as usize;

    if check_skip(clocks[clock_index_usize].id) != 0 {
        return 0;
    }

    match clocks[clock_index_usize].id {
        CLOCK_MONOTONIC_COARSE | CLOCK_MONOTONIC_RAW => {
            precision = -2.0;
        }
        _ => {}
    }

    if switch_ns(parent_ns) != 0 {
        return pr_err(c"switch_ns(%d)".as_ptr(), child_ns);
    }

    if _gettime(
        clocks[clock_index_usize].id,
        &mut parent_ts_old,
        raw_syscall,
    ) != 0
    {
        return -1;
    }

    child_ts_new.tv_nsec = parent_ts_old.tv_nsec;
    child_ts_new.tv_sec = parent_ts_old.tv_sec + offset;

    if switch_ns(child_ns) != 0 {
        return pr_err(c"switch_ns(%d)".as_ptr(), child_ns);
    }

    if _gettime(clocks[clock_index_usize].id, &mut cur_ts, raw_syscall) != 0 {
        return -1;
    }

    if difftime(cur_ts.tv_sec, child_ts_new.tv_sec) < precision {
        ksft_test_result_fail(
            c"Child's %s (%s) time has not changed: %lu -> %lu [%lu]\n".as_ptr(),
            clocks[clock_index_usize].name,
            entry,
            parent_ts_old.tv_sec,
            child_ts_new.tv_sec,
            cur_ts.tv_sec,
        );
        return -1;
    }

    if switch_ns(parent_ns) != 0 {
        return pr_err(c"switch_ns(%d)".as_ptr(), parent_ns);
    }

    if _gettime(clocks[clock_index_usize].id, &mut cur_ts, raw_syscall) != 0 {
        return -1;
    }

    if difftime(cur_ts.tv_sec, parent_ts_old.tv_sec) > DAY_IN_SEC as c_double {
        ksft_test_result_fail(
            c"Parent's %s (%s) time has changed: %lu -> %lu [%lu]\n".as_ptr(),
            clocks[clock_index_usize].name,
            entry,
            parent_ts_old.tv_sec,
            child_ts_new.tv_sec,
            cur_ts.tv_sec,
        );
        /* Let's play nice and put it closer to original */
        clock_settime(clocks[clock_index_usize].id, &cur_ts);
        return -1;
    }

    ksft_test_result_pass(
        c"Passed for %s (%s)\n".as_ptr(),
        clocks[clock_index_usize].name,
        entry,
    );
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut i: c_uint;
    let mut offset: time_t;
    let mut ret: c_int = 0;

    ksft_print_header();

    nscheck();

    check_supported_timers();

    ksft_set_plan((clocks.len() * 2) as c_uint);

    if init_namespaces() != 0 {
        return 1;
    }

    /* Offsets have to be set before tasks enter the namespace. */
    i = 0;
    while (i as usize) < clocks.len() {
        if clocks[i as usize].off_id != -1 {
            i += 1;
            continue;
        }
        offset = TEN_DAYS_IN_SEC + i as time_t * 1000;
        clocks[i as usize].offset = offset;
        if _settime(clocks[i as usize].id, offset) != 0 {
            return 1;
        }
        i += 1;
    }

    i = 0;
    while (i as usize) < clocks.len() {
        if clocks[i as usize].off_id != -1 {
            offset = clocks[clocks[i as usize].off_id as usize].offset;
        } else {
            offset = clocks[i as usize].offset;
        }
        ret |= test_gettime(i as clockid_t, true, offset);
        ret |= test_gettime(i as clockid_t, false, offset);
        i += 1;
    }

    if ret != 0 {
        ksft_exit_fail();
    }

    ksft_exit_pass();
    (ret != 0) as c_int
}
