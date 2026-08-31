// SPDX-License-Identifier: GPL-2.0
/*
 * Benchmark of /proc/kallsyms parsing.
 *
 * Copyright 2020 Google LLC.
 */

// C dependencies from the original file:
// #include <stdlib.h>
// #include "bench.h"
// #include "../util/stat.h"
// #include <linux/time64.h>
// #include <subcmd/parse-options.h>
// #include <symbol/kallsyms.h>

use core::ffi::{c_char, c_double, c_int, c_uint, c_void};

const NULL: *mut c_void = core::ptr::null_mut();
const EXIT_FAILURE: c_int = 1;
const USEC_PER_SEC: u64 = 1_000_000;
const USEC_PER_MSEC: c_double = 1_000.0;

static mut iterations: c_uint = 100;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut options: [option; 2];
}

// Static initializer translated from:
// static const struct option options[] = {
//     OPT_UINTEGER('i', "iterations", &iterations,
//         "Number of iterations used to compute average"),
//     OPT_END()
// };
// The OPT_UINTEGER and OPT_END macro expansions are supplied by parse-options.h.

#[unsafe(no_mangle)]
pub static bench_usage: [*const c_char; 2] = [
    b"perf bench internals kallsyms-parse <options>\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe extern "C" {
    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> c_double;
    fn stddev_stats(stats: *mut stats) -> c_double;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn kallsyms__parse(
        filename: *const c_char,
        arg: *mut c_void,
        process_symbol: Option<
            unsafe extern "C" fn(
                arg: *mut c_void,
                name: *const c_char,
                type_: c_char,
                start: u64,
            ) -> c_int,
        >,
    ) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn parse_options(
        argc: c_int,
        argv: *mut *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn exit(status: c_int) -> !;
}

unsafe extern "C" fn bench_process_symbol(
    _arg: *mut c_void,
    _name: *const c_char,
    _type: c_char,
    _start: u64,
) -> c_int {
    return 0;
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += USEC_PER_SEC as i64;
    }
}

unsafe fn do_kallsyms_parse() -> c_int {
    let mut start: timeval = core::mem::zeroed();
    let mut end: timeval = core::mem::zeroed();
    let mut diff: timeval = core::mem::zeroed();
    let mut runtime_us: u64;
    let mut i: c_uint;
    let time_average: c_double;
    let time_stddev: c_double;
    let mut err: c_int;
    let mut time_stats: stats = core::mem::zeroed();

    init_stats(&mut time_stats);

    i = 0;
    while i < iterations {
        gettimeofday(&mut start, NULL);
        err = kallsyms__parse(
            b"/proc/kallsyms\0".as_ptr() as *const c_char,
            NULL,
            Some(bench_process_symbol),
        );
        if err != 0 {
            return err;
        }

        gettimeofday(&mut end, NULL);
        timersub(&end, &start, &mut diff);
        runtime_us = (diff.tv_sec as u64)
            .wrapping_mul(USEC_PER_SEC)
            .wrapping_add(diff.tv_usec as u64);
        update_stats(&mut time_stats, runtime_us);
        i = i.wrapping_add(1);
    }

    time_average = avg_stats(&mut time_stats) / USEC_PER_MSEC;
    time_stddev = stddev_stats(&mut time_stats) / USEC_PER_MSEC;
    printf(
        b"  Average kallsyms__parse took: %.3f ms (+- %.3f ms)\n\0".as_ptr() as *const c_char,
        time_average,
        time_stddev,
    );
    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_kallsyms_parse(
    mut argc: c_int,
    argv: *mut *const c_char,
) -> c_int {
    argc = parse_options(argc, argv, options.as_ptr(), bench_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(bench_usage.as_ptr(), options.as_ptr());
        exit(EXIT_FAILURE);
    }

    return do_kallsyms_parse();
}
