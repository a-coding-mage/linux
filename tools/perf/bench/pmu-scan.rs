// SPDX-License-Identifier: GPL-2.0
/*
 * Benchmark scanning sysfs files for PMU information.
 *
 * Copyright 2023 Google LLC.
 */

use std::ffi::{c_char, c_double, c_int, c_uint, c_void};
use std::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;

const ENOMEM: c_int = 12;
const EXIT_FAILURE: c_int = 1;
const USEC_PER_SEC: i64 = 1000000;

static mut iterations: c_uint = 100;

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct perf_pmu {
    name: *mut c_char,
    is_core: bool_,
    nr_caps: u32,
    format: list_head,
}

#[repr(C)]
struct stats {
    _private: [u8; 0],
}

#[repr(C)]
struct timeval {
    tv_sec: i64,
    tv_usec: i64,
}

#[repr(C)]
struct option {
    _private: [u8; 0],
}

#[repr(C)]
struct pmu_scan_result {
    name: *mut c_char,
    nr_aliases: c_int,
    nr_formats: c_int,
    nr_caps: c_int,
    is_core: bool_,
}

// OPT_UINTEGER('i', "iterations", &iterations,
//      "Number of iterations used to compute average"),
// OPT_END()
static options: [option; 0] = [];

static bench_usage_0: &[u8] = b"perf bench internals pmu-scan <options>\0";
static bench_usage_1: *const c_char = ptr::null();
static bench_usage: [*const c_char; 2] = [
    bench_usage_0.as_ptr() as *const c_char,
    bench_usage_1,
];

static mut nr_pmus: c_int = 0;
static mut results: *mut pmu_scan_result = ptr::null_mut();

unsafe extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn exit(status: c_int) -> !;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;

    fn perf_pmus__scan(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmus__scan_core(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmus__destroy();
    fn perf_pmus__find(name: *const c_char) -> *mut perf_pmu;
    fn perf_pmu__num_events(pmu: *mut perf_pmu) -> c_int;

    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> c_double;
    fn stddev_stats(stats: *mut stats) -> c_double;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

unsafe fn list_for_each_count(head: *mut list_head) -> c_int {
    let mut nr: c_int = 0;
    let mut list = (*head).next;

    while list != head {
        nr += 1;
        list = (*list).next;
    }

    nr
}

unsafe fn timersub(a: *const timeval, b: *const timeval, res: *mut timeval) {
    (*res).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*res).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*res).tv_usec < 0 {
        (*res).tv_sec -= 1;
        (*res).tv_usec += USEC_PER_SEC;
    }
}

unsafe fn save_result() -> c_int {
    let mut pmu: *mut perf_pmu = ptr::null_mut();
    let mut r: *mut pmu_scan_result;

    loop {
        pmu = perf_pmus__scan(pmu);
        if pmu.is_null() {
            break;
        }

        r = realloc(
            results as *mut c_void,
            ((nr_pmus + 1) as usize) * std::mem::size_of::<pmu_scan_result>(),
        ) as *mut pmu_scan_result;
        if r.is_null() {
            return -ENOMEM;
        }

        results = r;
        r = results.add(nr_pmus as usize);

        (*r).name = strdup((*pmu).name);
        (*r).is_core = (*pmu).is_core;
        (*r).nr_caps = (*pmu).nr_caps as c_int;

        (*r).nr_aliases = perf_pmu__num_events(pmu);

        (*r).nr_formats = list_for_each_count(&mut (*pmu).format);

        pr_debug(
            b"pmu[%d] name=%s, nr_caps=%d, nr_aliases=%d, nr_formats=%d\n\0".as_ptr()
                as *const c_char,
            nr_pmus,
            (*r).name,
            (*r).nr_caps,
            (*r).nr_aliases,
            (*r).nr_formats,
        );
        nr_pmus += 1;
    }

    perf_pmus__destroy();
    0
}

unsafe fn check_result(core_only: bool_) -> c_int {
    let mut r: *mut pmu_scan_result;
    let mut pmu: *mut perf_pmu;
    let mut nr: c_int;

    let mut i: c_int = 0;
    while i < nr_pmus {
        r = results.add(i as usize);
        if core_only && !(*r).is_core {
            i += 1;
            continue;
        }

        pmu = perf_pmus__find((*r).name);
        if pmu.is_null() {
            pr_err(
                b"Cannot find PMU %s\n\0".as_ptr() as *const c_char,
                (*r).name,
            );
            return -1;
        }

        if (*pmu).nr_caps != (*r).nr_caps as u32 {
            pr_err(
                b"Unmatched number of event caps in %s: expect %d vs got %d\n\0".as_ptr()
                    as *const c_char,
                (*pmu).name,
                (*r).nr_caps,
                (*pmu).nr_caps,
            );
            return -1;
        }

        nr = perf_pmu__num_events(pmu);
        if nr != (*r).nr_aliases {
            pr_err(
                b"Unmatched number of event aliases in %s: expect %d vs got %d\n\0".as_ptr()
                    as *const c_char,
                (*pmu).name,
                (*r).nr_aliases,
                nr,
            );
            return -1;
        }

        nr = list_for_each_count(&mut (*pmu).format);
        if nr != (*r).nr_formats {
            pr_err(
                b"Unmatched number of event formats in %s: expect %d vs got %d\n\0".as_ptr()
                    as *const c_char,
                (*pmu).name,
                (*r).nr_formats,
                nr,
            );
            return -1;
        }

        i += 1;
    }
    0
}

unsafe fn delete_result() {
    let mut i: c_int = 0;
    while i < nr_pmus {
        free((*results.add(i as usize)).name as *mut c_void);
        i += 1;
    }
    free(results as *mut c_void);

    results = ptr::null_mut();
    nr_pmus = 0;
}

unsafe fn run_pmu_scan() -> c_int {
    let mut stats: stats = std::mem::zeroed();
    let mut start: timeval = std::mem::zeroed();
    let mut end: timeval = std::mem::zeroed();
    let mut diff: timeval = std::mem::zeroed();
    let mut time_average: c_double;
    let mut time_stddev: c_double;
    let mut runtime_us: u64;
    let mut ret: c_int;

    init_stats(&mut stats);
    pr_info(
        b"Computing performance of sysfs PMU event scan for %u times\n\0".as_ptr()
            as *const c_char,
        iterations,
    );

    if save_result() < 0 {
        pr_err(b"Failed to initialize PMU scan result\n\0".as_ptr() as *const c_char);
        return -1;
    }

    let mut j: c_int = 0;
    while j < 2 {
        let core_only: bool_ = j == 0;

        let mut i: c_uint = 0;
        while i < iterations {
            gettimeofday(&mut start, ptr::null_mut());
            if core_only {
                perf_pmus__scan_core(ptr::null_mut());
            } else {
                perf_pmus__scan(ptr::null_mut());
            }
            gettimeofday(&mut end, ptr::null_mut());
            timersub(&end, &start, &mut diff);
            runtime_us = (diff.tv_sec * USEC_PER_SEC + diff.tv_usec) as u64;
            update_stats(&mut stats, runtime_us);

            ret = check_result(core_only);
            perf_pmus__destroy();
            if ret < 0 {
                break;
            }

            i += 1;
        }
        time_average = avg_stats(&mut stats);
        time_stddev = stddev_stats(&mut stats);
        pr_info(
            b"  Average%s PMU scanning took: %.3f usec (+- %.3f usec)\n\0".as_ptr()
                as *const c_char,
            if core_only {
                b" core\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            time_average,
            time_stddev,
        );

        j += 1;
    }
    delete_result();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_pmu_scan(argc: c_int, argv: *const *const c_char) -> c_int {
    let mut err: c_int = 0;
    let argc = parse_options(argc, argv, options.as_ptr(), bench_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(bench_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    err = run_pmu_scan();

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
