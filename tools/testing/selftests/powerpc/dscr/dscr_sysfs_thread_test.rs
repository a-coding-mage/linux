// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Data Stream Control Register (DSCR) sysfs thread test
 *
 * This test updates the system wide DSCR default value through
 * sysfs interface which should then update all the CPU specific
 * DSCR default values which must also be then visible to threads
 * executing on individual CPUs on the system.
 *
 * Copyright 2015, Anshuman Khandual, IBM Corporation.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C source dependencies:
// #define _GNU_SOURCE
// #include "dscr.h"

const CPU_SETSIZE: c_int = 1024;

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; CPU_SETSIZE as usize / (8 * core::mem::size_of::<c_ulong>())],
}

unsafe extern "C" {
    static COUNT: c_int;
    static DSCR_MAX: c_int;
    static PPC_FEATURE2_DSCR: c_ulong;

    fn get_dscr() -> c_ulong;
    fn get_dscr_usr() -> c_ulong;
    fn get_default_dscr() -> c_ulong;
    fn set_default_dscr(val: c_ulong);
    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn sched_getcpu() -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;

    // Macro supplied by the selftest harness in C.
    fn SKIP_IF(cond: c_int);
}

fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        (*set).__bits.fill(0);
    }
}

fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let bit = cpu as usize;
    let bits_per_word = 8 * core::mem::size_of::<c_ulong>();
    unsafe {
        (*set).__bits[bit / bits_per_word] |= (1 as c_ulong) << (bit % bits_per_word);
    }
}

unsafe fn test_thread_dscr(val: c_ulong) -> c_int {
    let cur_dscr: c_ulong;
    let cur_dscr_usr: c_ulong;

    cur_dscr = unsafe { get_dscr() };
    cur_dscr_usr = unsafe { get_dscr_usr() };

    if val != cur_dscr {
        unsafe {
            printf(
                c"[cpu %d] Kernel DSCR should be %ld but is %ld\n".as_ptr(),
                sched_getcpu(),
                val,
                cur_dscr,
            );
        }
        return 1;
    }

    if val != cur_dscr_usr {
        unsafe {
            printf(
                c"[cpu %d] User DSCR should be %ld but is %ld\n".as_ptr(),
                sched_getcpu(),
                val,
                cur_dscr_usr,
            );
        }
        return 1;
    }
    0
}

unsafe fn check_cpu_dscr_thread(val: c_ulong) -> c_int {
    let mut mask: cpu_set_t = cpu_set_t {
        __bits: [0; CPU_SETSIZE as usize / (8 * core::mem::size_of::<c_ulong>())],
    };
    let mut cpu: c_int;

    cpu = 0;
    while cpu < CPU_SETSIZE {
        CPU_ZERO(&mut mask);
        CPU_SET(cpu, &mut mask);
        if unsafe { sched_setaffinity(0, core::mem::size_of_val(&mask), &mask) } != 0 {
            cpu += 1;
            continue;
        }

        if unsafe { test_thread_dscr(val) } != 0 {
            return 1;
        }
        cpu += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dscr_sysfs_thread() -> c_int {
    let orig_dscr_default: c_ulong;
    let mut i: c_int;
    let mut j: c_int;

    unsafe {
        SKIP_IF((have_hwcap2(PPC_FEATURE2_DSCR) == 0) as c_int);
    }

    orig_dscr_default = unsafe { get_default_dscr() };
    i = 0;
    while i < unsafe { COUNT } {
        j = 0;
        while j < unsafe { DSCR_MAX } {
            unsafe {
                set_default_dscr(j as c_ulong);
            }
            if unsafe { check_cpu_dscr_thread(j as c_ulong) } != 0 {
                unsafe {
                    set_default_dscr(orig_dscr_default);
                }
                return 1;
            }
            j += 1;
        }
        i += 1;
    }
    unsafe {
        set_default_dscr(orig_dscr_default);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe {
        test_harness(
            dscr_sysfs_thread,
            c"dscr_sysfs_thread_test".as_ptr() as *const c_char,
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
