// SPDX-License-Identifier: GPL-2.0+

/*
 * Copyright 2018 IBM Corporation.
 * Copyright 2020 Canonical Ltd.
 */

// C includes translated as external declarations supplied by the surrounding
// selftest build: utils.h, flush_utils.h, libc, and Linux type definitions.

use std::ffi::{c_char, c_int, c_void};

type __u64 = u64;

#[repr(C)]
struct perf_event_read {
    l1d_misses: __u64,
}

unsafe extern "C" {
    static PPC_FEATURE_ARCH_2_06: u64;
    static PERF_TYPE_HW_CACHE: u32;
    static PERF_L1D_READ_MISS_CONFIG: u64;
    static CACHELINE_SIZE: u64;

    fn geteuid() -> u32;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn memalign(alignment: usize, size: usize) -> *mut c_void;

    fn SKIP_IF(cond: c_int);
    fn FAIL_IF(cond: c_int);

    fn have_hwcap(feature: u64) -> c_int;
    fn read_debugfs_int(name: *const c_char, value: *mut c_int) -> c_int;
    fn write_debugfs_int(name: *const c_char, value: c_int) -> c_int;
    fn perf_event_open_counter(type_: u32, config: u64, pid: c_int) -> c_int;
    fn perf_event_enable(fd: c_int) -> c_int;
    fn perf_event_reset(fd: c_int) -> c_int;
    fn perf_event_disable(fd: c_int) -> c_int;
    fn set_dscr(value: c_int);
    fn syscall_loop_uaccess(p: *mut c_char, iterations: u64, zero_size: u64);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

unsafe extern "C" fn uaccess_flush_test() -> c_int {
    let p: *mut c_char;
    let repetitions: c_int = 10;
    let fd: c_int;
    let mut passes: c_int = 0;
    let mut iter: c_int;
    let mut rc: c_int = 0;
    let mut v: perf_event_read = perf_event_read { l1d_misses: 0 };
    let mut l1d_misses_total: __u64 = 0;
    let iterations: u64 = 100000;
    let zero_size: u64 = 24 * 1024;
    let l1d_misses_expected: u64;
    let mut rfi_flush_orig: c_int = 0;
    let mut entry_flush_orig: c_int = 0;
    let mut uaccess_flush: c_int;
    let mut uaccess_flush_orig: c_int = 0;

    SKIP_IF((geteuid() != 0) as c_int);

    // The PMU event we use only works on Power7 or later
    SKIP_IF((have_hwcap(PPC_FEATURE_ARCH_2_06) == 0) as c_int);

    if read_debugfs_int(
        c"powerpc/rfi_flush".as_ptr(),
        &mut rfi_flush_orig as *mut c_int,
    ) < 0
    {
        perror(c"Unable to read powerpc/rfi_flush debugfs file".as_ptr());
        SKIP_IF(1);
    }

    if read_debugfs_int(
        c"powerpc/entry_flush".as_ptr(),
        &mut entry_flush_orig as *mut c_int,
    ) < 0
    {
        perror(c"Unable to read powerpc/entry_flush debugfs file".as_ptr());
        SKIP_IF(1);
    }

    if read_debugfs_int(
        c"powerpc/uaccess_flush".as_ptr(),
        &mut uaccess_flush_orig as *mut c_int,
    ) < 0
    {
        perror(c"Unable to read powerpc/entry_flush debugfs file".as_ptr());
        SKIP_IF(1);
    }

    if rfi_flush_orig != 0 {
        if write_debugfs_int(c"powerpc/rfi_flush".as_ptr(), 0) < 0 {
            perror(c"error writing to powerpc/rfi_flush debugfs file".as_ptr());
            FAIL_IF(1);
        }
    }

    if entry_flush_orig != 0 {
        if write_debugfs_int(c"powerpc/entry_flush".as_ptr(), 0) < 0 {
            perror(c"error writing to powerpc/entry_flush debugfs file".as_ptr());
            FAIL_IF(1);
        }
    }

    uaccess_flush = uaccess_flush_orig;

    fd = perf_event_open_counter(PERF_TYPE_HW_CACHE, PERF_L1D_READ_MISS_CONFIG, -1);
    FAIL_IF((fd < 0) as c_int);

    p = memalign(zero_size as usize, CACHELINE_SIZE as usize) as *mut c_char;

    FAIL_IF(perf_event_enable(fd));

    // disable L1 prefetching
    set_dscr(1);

    iter = repetitions;

    /*
     * We expect to see l1d miss for each cacheline access when entry_flush
     * is set. Allow a small variation on this.
     */
    l1d_misses_expected = iterations * (zero_size / CACHELINE_SIZE - 2);

    'again: loop {
        FAIL_IF(perf_event_reset(fd));

        syscall_loop_uaccess(p, iterations, zero_size);

        FAIL_IF(
            (read(
                fd,
                &mut v as *mut perf_event_read as *mut c_void,
                std::mem::size_of::<perf_event_read>(),
            ) != std::mem::size_of::<perf_event_read>() as isize) as c_int,
        );

        if uaccess_flush != 0 && v.l1d_misses >= l1d_misses_expected {
            passes += 1;
        } else if uaccess_flush == 0 && v.l1d_misses < (l1d_misses_expected / 2) {
            passes += 1;
        }

        l1d_misses_total = l1d_misses_total.wrapping_add(v.l1d_misses);

        iter -= 1;
        if iter != 0 {
            continue 'again;
        }

        if passes < repetitions {
            printf(
                c"FAIL (L1D misses with uaccess_flush=%d: %llu %c %lu) [%d/%d failures]\n"
                    .as_ptr(),
                uaccess_flush,
                l1d_misses_total,
                if uaccess_flush != 0 { '<' as c_int } else { '>' as c_int },
                if uaccess_flush != 0 {
                    repetitions as u64 * l1d_misses_expected
                } else {
                    repetitions as u64 * l1d_misses_expected / 2
                },
                repetitions - passes,
                repetitions,
            );
            rc = 1;
        } else {
            printf(
                c"PASS (L1D misses with uaccess_flush=%d: %llu %c %lu) [%d/%d pass]\n"
                    .as_ptr(),
                uaccess_flush,
                l1d_misses_total,
                if uaccess_flush != 0 { '>' as c_int } else { '<' as c_int },
                if uaccess_flush != 0 {
                    repetitions as u64 * l1d_misses_expected
                } else {
                    repetitions as u64 * l1d_misses_expected / 2
                },
                passes,
                repetitions,
            );
        }

        if uaccess_flush == uaccess_flush_orig {
            uaccess_flush = (uaccess_flush_orig == 0) as c_int;
            if write_debugfs_int(c"powerpc/uaccess_flush".as_ptr(), uaccess_flush) < 0 {
                perror(c"error writing to powerpc/uaccess_flush debugfs file".as_ptr());
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

    if write_debugfs_int(c"powerpc/rfi_flush".as_ptr(), rfi_flush_orig) < 0 {
        perror(c"unable to restore original value of powerpc/rfi_flush debugfs file".as_ptr());
        return 1;
    }

    if write_debugfs_int(c"powerpc/entry_flush".as_ptr(), entry_flush_orig) < 0 {
        perror(c"unable to restore original value of powerpc/entry_flush debugfs file".as_ptr());
        return 1;
    }

    if write_debugfs_int(c"powerpc/uaccess_flush".as_ptr(), uaccess_flush_orig) < 0 {
        perror(c"unable to restore original value of powerpc/uaccess_flush debugfs file".as_ptr());
        return 1;
    }

    rc
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            uaccess_flush_test,
            c"uaccess_flush_test".as_ptr(),
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
