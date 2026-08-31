// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/event-times.c. C include dependencies are kept as
// external declarations below.

use core::ffi::c_void;
use std::ffi::{c_char, c_int};
use std::ptr;

const STRERR_BUFSIZE: usize = 128;
const EACCES: c_int = 13;
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;

#[repr(C)]
pub struct perf_event_attr {
    pub disabled: u64,
    pub enable_on_exec: u64,
    pub read_format: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

unsafe extern "C" {
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__create_maps(evlist: *mut evlist, target: *mut target) -> c_int;
    fn evlist__prepare_workload(
        evlist: *mut evlist,
        target: *mut target,
        argv: *const *const c_char,
        pipe_output: bool,
        exec_error: *mut c_void,
    ) -> c_int;
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__start_workload(evlist: *mut evlist) -> c_int;
    fn evlist__workload_pid(evlist: *mut evlist) -> c_int;
    fn evlist__new() -> *mut evlist;
    fn evlist__put(evlist: *mut evlist);

    fn thread_map__new_by_tid(tid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);

    fn evsel__open_per_thread(evsel: *mut evsel, threads: *mut perf_thread_map) -> c_int;
    fn evsel__open_per_cpu(evsel: *mut evsel, cpus: *mut perf_cpu_map, cpu_map_idx: c_int) -> c_int;
    fn evsel__enable(evsel: *mut evsel) -> c_int;
    fn perf_evsel__read(evsel: *mut evsel_core, cpu_map_idx: c_int, thread: c_int, count: *mut perf_counts_values);

    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *const c_char;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn getpid() -> c_int;
    fn pr_debug(fmt: *const c_char, ...);

    static mut errno: c_int;
}

unsafe extern "C" fn attach__enable_on_exec(evlist: *mut evlist) -> c_int {
    let evsel = evlist__last(evlist);
    let mut target: target = core::mem::zeroed();
    let argv: [*const c_char; 2] = [c"true".as_ptr(), ptr::null()];
    let mut sbuf = [0 as c_char; STRERR_BUFSIZE];
    let mut err: c_int;

    pr_debug(c"attaching to spawned child, enable on exec\n".as_ptr());

    err = evlist__create_maps(evlist, &mut target);
    if err < 0 {
        pr_debug(c"Not enough memory to create thread/cpu maps\n".as_ptr());
        return err;
    }

    err = evlist__prepare_workload(evlist, &mut target, argv.as_ptr(), false, ptr::null_mut());
    if err < 0 {
        pr_debug(c"Couldn't run the workload!\n".as_ptr());
        return err;
    }

    (*evsel).core.attr.enable_on_exec = 1;

    err = evlist__open(evlist);
    if err < 0 {
        pr_debug(
            c"perf_evlist__open: %s\n".as_ptr(),
            str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()),
        );
        return err;
    }

    if evlist__start_workload(evlist) == 1 {
        TEST_OK
    } else {
        TEST_FAIL
    }
}

unsafe extern "C" fn detach__enable_on_exec(evlist: *mut evlist) -> c_int {
    waitpid(evlist__workload_pid(evlist), ptr::null_mut(), 0);
    0
}

unsafe extern "C" fn attach__current_disabled(evlist: *mut evlist) -> c_int {
    let evsel = evlist__last(evlist);
    let threads: *mut perf_thread_map;
    let err: c_int;

    pr_debug(c"attaching to current thread as disabled\n".as_ptr());

    threads = thread_map__new_by_tid(getpid());
    if threads.is_null() {
        pr_debug(c"thread_map__new\n".as_ptr());
        return -1;
    }

    (*evsel).core.attr.disabled = 1;

    err = evsel__open_per_thread(evsel, threads);
    if err != 0 {
        pr_debug(c"Failed to open event cpu-clock:u\n".as_ptr());
        return err;
    }

    perf_thread_map__put(threads);
    if evsel__enable(evsel) == 0 {
        TEST_OK
    } else {
        TEST_FAIL
    }
}

unsafe extern "C" fn attach__current_enabled(evlist: *mut evlist) -> c_int {
    let evsel = evlist__last(evlist);
    let threads: *mut perf_thread_map;
    let err: c_int;

    pr_debug(c"attaching to current thread as enabled\n".as_ptr());

    threads = thread_map__new_by_tid(getpid());
    if threads.is_null() {
        pr_debug(c"failed to call thread_map__new\n".as_ptr());
        return -1;
    }

    err = evsel__open_per_thread(evsel, threads);

    perf_thread_map__put(threads);
    if err == 0 {
        TEST_OK
    } else {
        TEST_FAIL
    }
}

unsafe extern "C" fn detach__disable(evlist: *mut evlist) -> c_int {
    let evsel = evlist__last(evlist);

    evsel__enable(evsel)
}

unsafe extern "C" fn attach__cpu_disabled(evlist: *mut evlist) -> c_int {
    let evsel = evlist__last(evlist);
    let cpus: *mut perf_cpu_map;
    let err: c_int;

    pr_debug(c"attaching to CPU 0 as enabled\n".as_ptr());

    cpus = perf_cpu_map__new(c"0".as_ptr());
    if cpus.is_null() {
        pr_debug(c"failed to call perf_cpu_map__new\n".as_ptr());
        return -1;
    }

    (*evsel).core.attr.disabled = 1;

    err = evsel__open_per_cpu(evsel, cpus, -1);
    perf_cpu_map__put(cpus);
    if err != 0 {
        if err == -EACCES {
            return TEST_SKIP;
        }

        pr_debug(c"Failed to open event cpu-clock:u\n".as_ptr());
        return err;
    }

    evsel__enable(evsel)
}

unsafe extern "C" fn attach__cpu_enabled(evlist: *mut evlist) -> c_int {
    let evsel = evlist__last(evlist);
    let cpus: *mut perf_cpu_map;
    let err: c_int;

    pr_debug(c"attaching to CPU 0 as enabled\n".as_ptr());

    cpus = perf_cpu_map__new(c"0".as_ptr());
    if cpus.is_null() {
        pr_debug(c"failed to call perf_cpu_map__new\n".as_ptr());
        return -1;
    }

    err = evsel__open_per_cpu(evsel, cpus, -1);
    perf_cpu_map__put(cpus);
    if err == -EACCES {
        return TEST_SKIP;
    }

    if err != 0 {
        TEST_FAIL
    } else {
        TEST_OK
    }
}

unsafe fn test_assert_val(_msg: *const c_char, val: bool) -> bool {
    val
}

unsafe fn test_times(
    attach: unsafe extern "C" fn(*mut evlist) -> c_int,
    detach: unsafe extern "C" fn(*mut evlist) -> c_int,
) -> c_int {
    let mut count: perf_counts_values = core::mem::zeroed();
    let mut evlist: *mut evlist = ptr::null_mut();
    let evsel: *mut evsel;
    let mut err: c_int = -1;
    let mut i: c_int;

    evlist = evlist__new();
    if evlist.is_null() {
        pr_debug(c"failed to create event list\n".as_ptr());
        return TEST_FAIL;
    }

    err = parse_event(evlist, c"cpu-clock:u".as_ptr());
    if err != 0 {
        pr_debug(c"failed to parse event cpu-clock:u\n".as_ptr());
        evlist__put(evlist);
        return TEST_FAIL;
    }

    evsel = evlist__last(evlist);
    (*evsel).core.attr.read_format |=
        PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;

    err = attach(evlist);
    if err == TEST_SKIP {
        pr_debug(c"  SKIP  : not enough rights\n".as_ptr());
        evlist__put(evlist);
        return err;
    }

    if !test_assert_val(c"failed to attach".as_ptr(), err == 0) {
        evlist__put(evlist);
        return TEST_FAIL;
    }

    i = 0;
    while i < 100000000 {
        i += 1;
    }

    if !test_assert_val(c"failed to detach".as_ptr(), detach(evlist) == 0) {
        evlist__put(evlist);
        return TEST_FAIL;
    }

    perf_evsel__read(&mut (*evsel).core, 0, 0, &mut count);

    err = if !(count.ena == count.run) { 1 } else { 0 };

    pr_debug(
        c"  %s: ena %lu, run %lu\n".as_ptr(),
        if err == 0 {
            c"OK    ".as_ptr()
        } else {
            c"FAILED".as_ptr()
        },
        count.ena,
        count.run,
    );

    evlist__put(evlist);
    if err == 0 {
        TEST_OK
    } else {
        TEST_FAIL
    }
}

/*
 * This test creates software event 'cpu-clock'
 * attaches it in several ways (explained below)
 * and checks that enabled and running times
 * match.
 */
unsafe extern "C" fn test__event_times(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut err: c_int;
    let mut ret: c_int = 0;

    /* attach on newly spawned process after exec */
    err = test_times(attach__enable_on_exec, detach__enable_on_exec);
    if err != 0 && (ret == TEST_OK || ret == TEST_SKIP) {
        ret = err;
    }

    /* attach on current process as enabled */
    err = test_times(attach__current_enabled, detach__disable);
    if err != 0 && (ret == TEST_OK || ret == TEST_SKIP) {
        ret = err;
    }

    /* attach on current process as disabled */
    err = test_times(attach__current_disabled, detach__disable);
    if err != 0 && (ret == TEST_OK || ret == TEST_SKIP) {
        ret = err;
    }

    /* attach on cpu as disabled */
    err = test_times(attach__cpu_disabled, detach__disable);
    if err != 0 && (ret == TEST_OK || ret == TEST_SKIP) {
        ret = err;
    }

    /* attach on cpu as enabled */
    err = test_times(attach__cpu_enabled, detach__disable);
    if err != 0 && (ret == TEST_OK || ret == TEST_SKIP) {
        ret = err;
    }

    ret
}
