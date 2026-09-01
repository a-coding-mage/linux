// SPDX-License-Identifier: GPL-2.0
/*
 * Benchmark find_next_bit and related bit operations.
 *
 * Copyright 2020 Google LLC.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

// C dependencies: stdlib.h, bench.h, ../util/stat.h, linux/bitmap.h,
// linux/bitops.h, linux/time64.h, subcmd/parse-options.h.

const EXIT_FAILURE: c_int = 1;
const USEC_PER_SEC: u64 = 1_000_000;

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
    fn bitmap_zalloc(nbits: c_uint) -> *mut c_ulong;
    fn bitmap_zero(dst: *mut c_ulong, nbits: c_uint);
    fn __set_bit(nr: c_uint, addr: *mut c_ulong);
    fn bitmap_free(bitmap: *mut c_ulong);
    fn find_next_bit(addr: *const c_ulong, size: c_ulong, offset: c_ulong) -> c_ulong;
    fn test_bit(nr: c_long, addr: *const c_ulong) -> bool;

    fn init_stats(stats: *mut stats);
    fn update_stats(stats: *mut stats, val: u64);
    fn avg_stats(stats: *mut stats) -> f64;
    fn stddev_stats(stats: *mut stats) -> f64;

    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *const option,
        usagestr: *const *const c_char,
        flags: c_int,
    ) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
}

static mut outer_iterations: c_uint = 5;
static mut inner_iterations: c_uint = 100000;

// Original C used OPT_UINTEGER/OPT_END initializers from parse-options.h.
// They require the external `struct option` layout/macros and are preserved
// here as a dependency-shaped declaration site rather than reimplemented.
static options: [option; 0] = [];

static BENCH_USAGE_0: &[u8] = b"perf bench mem find_bit <options>\0";
static bench_usage: [*const c_char; 2] = [
    BENCH_USAGE_0.as_ptr() as *const c_char,
    ptr::null(),
];

static mut accumulator: c_uint = 0;
static mut use_of_val: c_uint = 0;

#[inline(never)]
unsafe fn workload(val: c_int) {
    unsafe {
        use_of_val = use_of_val.wrapping_add(val as c_uint);
        accumulator = accumulator.wrapping_add(1);
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn asm_test_bit(nr: c_long, addr: *const c_ulong) -> bool {
    let oldbit: u8;

    unsafe {
        core::arch::asm!(
            "bt {nr}, [{addr}]",
            "setc {old}",
            nr = in(reg) nr,
            addr = in(reg) addr,
            old = lateout(reg_byte) oldbit,
            options(nostack, readonly),
        );
    }

    oldbit != 0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
unsafe fn asm_test_bit(nr: c_long, addr: *const c_ulong) -> bool {
    unsafe { test_bit(nr, addr) }
}

unsafe fn timersub(end: *const timeval, start: *const timeval, diff: *mut timeval) {
    unsafe {
        (*diff).tv_sec = (*end).tv_sec - (*start).tv_sec;
        (*diff).tv_usec = (*end).tv_usec - (*start).tv_usec;
        if (*diff).tv_usec < 0 {
            (*diff).tv_sec -= 1;
            (*diff).tv_usec += USEC_PER_SEC as i64;
        }
    }
}

unsafe fn do_for_each_set_bit(num_bits: c_uint) -> c_int {
    unsafe {
        let to_test: *mut c_ulong = bitmap_zalloc(num_bits);
        let mut start = MaybeUninit::<timeval>::uninit();
        let mut end = MaybeUninit::<timeval>::uninit();
        let mut diff = MaybeUninit::<timeval>::uninit();
        let mut runtime_us: u64;
        let mut fb_time_stats = MaybeUninit::<stats>::uninit();
        let mut tb_time_stats = MaybeUninit::<stats>::uninit();
        let mut time_average: f64;
        let mut time_stddev: f64;
        let mut bit: c_uint;
        let mut i: c_uint;
        let mut j: c_uint;
        let mut set_bits: c_uint;
        let mut skip: c_uint;

        init_stats(fb_time_stats.as_mut_ptr());
        init_stats(tb_time_stats.as_mut_ptr());

        set_bits = 1;
        while set_bits <= num_bits {
            bitmap_zero(to_test, num_bits);
            skip = num_bits / set_bits;
            i = 0;
            while i < num_bits {
                __set_bit(i, to_test);
                i = i.wrapping_add(skip);
            }

            i = 0;
            while i < outer_iterations {
                #[cfg(not(debug_assertions))]
                let old = accumulator;

                gettimeofday(start.as_mut_ptr(), ptr::null_mut());
                j = 0;
                while j < inner_iterations {
                    bit = find_next_bit(to_test as *const c_ulong, num_bits as c_ulong, 0) as c_uint;
                    while bit < num_bits {
                        workload(bit as c_int);
                        bit = find_next_bit(
                            to_test as *const c_ulong,
                            num_bits as c_ulong,
                            bit.wrapping_add(1) as c_ulong,
                        ) as c_uint;
                    }
                    j = j.wrapping_add(1);
                }
                gettimeofday(end.as_mut_ptr(), ptr::null_mut());
                #[cfg(not(debug_assertions))]
                assert!(
                    old.wrapping_add(inner_iterations.wrapping_mul(set_bits)) == accumulator
                );
                timersub(end.as_ptr(), start.as_ptr(), diff.as_mut_ptr());
                runtime_us = ((*diff.as_ptr()).tv_sec as u64)
                    .wrapping_mul(USEC_PER_SEC)
                    .wrapping_add((*diff.as_ptr()).tv_usec as u64);
                update_stats(fb_time_stats.as_mut_ptr(), runtime_us);

                #[cfg(not(debug_assertions))]
                let old = accumulator;

                gettimeofday(start.as_mut_ptr(), ptr::null_mut());
                j = 0;
                while j < inner_iterations {
                    bit = 0;
                    while bit < num_bits {
                        if asm_test_bit(bit as c_long, to_test as *const c_ulong) {
                            workload(bit as c_int);
                        }
                        bit = bit.wrapping_add(1);
                    }
                    j = j.wrapping_add(1);
                }
                gettimeofday(end.as_mut_ptr(), ptr::null_mut());
                #[cfg(not(debug_assertions))]
                assert!(
                    old.wrapping_add(inner_iterations.wrapping_mul(set_bits)) == accumulator
                );
                timersub(end.as_ptr(), start.as_ptr(), diff.as_mut_ptr());
                runtime_us = ((*diff.as_ptr()).tv_sec as u64)
                    .wrapping_mul(USEC_PER_SEC)
                    .wrapping_add((*diff.as_ptr()).tv_usec as u64);
                update_stats(tb_time_stats.as_mut_ptr(), runtime_us);

                i = i.wrapping_add(1);
            }

            printf(
                b"%d operations %d bits set of %d bits\n\0".as_ptr() as *const c_char,
                inner_iterations,
                set_bits,
                num_bits,
            );
            time_average = avg_stats(fb_time_stats.as_mut_ptr());
            time_stddev = stddev_stats(fb_time_stats.as_mut_ptr());
            printf(
                b"  Average for_each_set_bit took: %.3f usec (+- %.3f usec)\n\0".as_ptr()
                    as *const c_char,
                time_average,
                time_stddev,
            );
            time_average = avg_stats(tb_time_stats.as_mut_ptr());
            time_stddev = stddev_stats(tb_time_stats.as_mut_ptr());
            printf(
                b"  Average test_bit loop took:    %.3f usec (+- %.3f usec)\n\0".as_ptr()
                    as *const c_char,
                time_average,
                time_stddev,
            );

            if use_of_val == accumulator {
                /* Try to avoid compiler tricks. */
                printf(b"\n\0".as_ptr() as *const c_char);
            }

            set_bits <<= 1;
        }
        bitmap_free(to_test);
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn bench_mem_find_bit(argc: c_int, argv: *const *const c_char) -> c_int {
    unsafe {
        let mut err: c_int = 0;
        let mut i: c_int;
        let argc = parse_options(argc, argv, options.as_ptr(), bench_usage.as_ptr(), 0);
        if argc != 0 {
            usage_with_options(bench_usage.as_ptr(), options.as_ptr());
            exit(EXIT_FAILURE);
        }

        i = 1;
        while i <= 2048 {
            do_for_each_set_bit(i as c_uint);
            i <<= 1;
        }

        err
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
