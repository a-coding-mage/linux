// SPDX-License-Identifier: GPL-2.0+

/*
 * Copyright 2018 IBM Corporation.
 */

// C dependencies:
// #define __SANE_USERSPACE_TYPES__
// #include <sys/types.h>
// #include <stdint.h>
// #include <malloc.h>
// #include <unistd.h>
// #include <stdlib.h>
// #include <string.h>
// #include <stdio.h>
// #include "utils.h"
// #include "flush_utils.h"

use std::ffi::{c_char, c_int, c_ulong, c_void};
use std::mem::size_of;

type __u64 = u64;

#[repr(C)]
struct perf_event_read {
    l1d_misses: __u64,
}

unsafe extern "C" {
    static PPC_FEATURE_ARCH_2_06: c_ulong;
    static PERF_TYPE_HW_CACHE: c_int;
    static PERF_L1D_READ_MISS_CONFIG: c_ulong;
    static CACHELINE_SIZE: c_ulong;

    fn geteuid() -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn memalign(alignment: usize, size: usize) -> *mut c_void;

    fn have_hwcap(feature: c_ulong) -> c_int;
    fn read_debugfs_int(name: *const c_char, value: *mut c_int) -> c_int;
    fn write_debugfs_int(name: *const c_char, value: c_int) -> c_int;
    fn perf_event_open_counter(type_: c_int, config: c_ulong, cpu: c_int) -> c_int;
    fn perf_event_enable(fd: c_int) -> c_int;
    fn perf_event_reset(fd: c_int) -> c_int;
    fn perf_event_disable(fd: c_int) -> c_int;
    fn set_dscr(value: c_int);
    fn syscall_loop(p: *mut c_char, iterations: c_ulong, zero_size: c_ulong);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

unsafe extern "C" fn rfi_flush_test() -> c_int {
    let p: *mut c_char;
    let repetitions: c_int = 10;
    let fd: c_int;
    let mut passes: c_int = 0;
    let mut iter: c_int;
    let mut rc: c_int = 0;
    let mut v: perf_event_read = perf_event_read { l1d_misses: 0 };
    let mut l1d_misses_total: __u64 = 0;
    let iterations: c_ulong = 100000;
    let zero_size: c_ulong = 24 * 1024;
    let l1d_misses_expected: c_ulong;
    let rfi_flush_orig: c_int;
    let mut rfi_flush: c_int;
    let mut have_entry_flush: c_int;
    let mut entry_flush_orig: c_int = 0;
    let mut rfi_flush_orig_tmp: c_int = 0;

    SKIP_IF!(geteuid() != 0);

    // The PMU event we use only works on Power7 or later
    SKIP_IF!(have_hwcap(PPC_FEATURE_ARCH_2_06) == 0);

    if read_debugfs_int(
        b"powerpc/rfi_flush\0".as_ptr() as *const c_char,
        &mut rfi_flush_orig_tmp,
    ) < 0
    {
        perror(b"Unable to read powerpc/rfi_flush debugfs file\0".as_ptr() as *const c_char);
        SKIP_IF!(true);
    }
    rfi_flush_orig = rfi_flush_orig_tmp;

    if read_debugfs_int(
        b"powerpc/entry_flush\0".as_ptr() as *const c_char,
        &mut entry_flush_orig,
    ) < 0
    {
        have_entry_flush = 0;
    } else {
        have_entry_flush = 1;

        if entry_flush_orig != 0 {
            if write_debugfs_int(b"powerpc/entry_flush\0".as_ptr() as *const c_char, 0) < 0 {
                perror(
                    b"error writing to powerpc/entry_flush debugfs file\0".as_ptr()
                        as *const c_char,
                );
                return 1;
            }
        }
    }

    rfi_flush = rfi_flush_orig;

    fd = perf_event_open_counter(PERF_TYPE_HW_CACHE, PERF_L1D_READ_MISS_CONFIG, -1);
    FAIL_IF!(fd < 0);

    p = memalign(zero_size as usize, CACHELINE_SIZE as usize) as *mut c_char;

    FAIL_IF!(perf_event_enable(fd) != 0);

    // disable L1 prefetching
    set_dscr(1);

    iter = repetitions;

    /*
     * We expect to see l1d miss for each cacheline access when rfi_flush
     * is set. Allow a small variation on this.
     */
    l1d_misses_expected = iterations * (zero_size / CACHELINE_SIZE - 2);

    'again: loop {
        FAIL_IF!(perf_event_reset(fd) != 0);

        syscall_loop(p, iterations, zero_size);

        FAIL_IF!(
            read(
                fd,
                &mut v as *mut perf_event_read as *mut c_void,
                size_of::<perf_event_read>(),
            ) != size_of::<perf_event_read>() as isize
        );

        if rfi_flush != 0 && v.l1d_misses >= l1d_misses_expected as __u64 {
            passes += 1;
        } else if rfi_flush == 0 && v.l1d_misses < (l1d_misses_expected / 2) as __u64 {
            passes += 1;
        }

        l1d_misses_total = l1d_misses_total.wrapping_add(v.l1d_misses);

        iter -= 1;
        if iter != 0 {
            continue 'again;
        }

        if passes < repetitions {
            printf(
                b"FAIL (L1D misses with rfi_flush=%d: %llu %c %lu) [%d/%d failures]\n\0"
                    .as_ptr() as *const c_char,
                rfi_flush,
                l1d_misses_total,
                if rfi_flush != 0 { '<' as c_int } else { '>' as c_int },
                if rfi_flush != 0 {
                    repetitions as c_ulong * l1d_misses_expected
                } else {
                    repetitions as c_ulong * l1d_misses_expected / 2
                },
                repetitions - passes,
                repetitions,
            );
            rc = 1;
        } else {
            printf(
                b"PASS (L1D misses with rfi_flush=%d: %llu %c %lu) [%d/%d pass]\n\0"
                    .as_ptr() as *const c_char,
                rfi_flush,
                l1d_misses_total,
                if rfi_flush != 0 { '>' as c_int } else { '<' as c_int },
                if rfi_flush != 0 {
                    repetitions as c_ulong * l1d_misses_expected
                } else {
                    repetitions as c_ulong * l1d_misses_expected / 2
                },
                passes,
                repetitions,
            );
        }

        if rfi_flush == rfi_flush_orig {
            rfi_flush = if rfi_flush_orig == 0 { 1 } else { 0 };
            if write_debugfs_int(
                b"powerpc/rfi_flush\0".as_ptr() as *const c_char,
                rfi_flush,
            ) < 0
            {
                perror(
                    b"error writing to powerpc/rfi_flush debugfs file\0".as_ptr()
                        as *const c_char,
                );
                return 1;
            }
            iter = repetitions;
            l1d_misses_total = 0;
            passes = 0;
            continue 'again;
        }

        break 'again;
    }

    perf_event_disable(fd);
    close(fd);

    set_dscr(0);

    if write_debugfs_int(
        b"powerpc/rfi_flush\0".as_ptr() as *const c_char,
        rfi_flush_orig,
    ) < 0
    {
        perror(
            b"unable to restore original value of powerpc/rfi_flush debugfs file\0".as_ptr()
                as *const c_char,
        );
        return 1;
    }

    if have_entry_flush != 0 {
        if write_debugfs_int(
            b"powerpc/entry_flush\0".as_ptr() as *const c_char,
            entry_flush_orig,
        ) < 0
        {
            perror(
                b"unable to restore original value of powerpc/entry_flush debugfs file\0"
                    .as_ptr() as *const c_char,
            );
            return 1;
        }
    }

    rc
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_harness(rfi_flush_test, b"rfi_flush_test\0".as_ptr() as *const c_char)
}

fn main() {
    unsafe {
        let args: Vec<*mut c_char> = std::env::args()
            .map(|_| std::ptr::null_mut())
            .collect();
        std::process::exit(main_impl(args.len() as c_int, args.as_ptr() as *mut *mut c_char));
    }
}
