// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Data Stream Control Register (DSCR) sysfs interface test
 *
 * This test updates to system wide DSCR default through the sysfs interface
 * and then verifies that all the CPU specific DSCR defaults are updated as
 * well verified from their sysfs interfaces.
 *
 * Copyright 2015, Anshuman Khandual, IBM Corporation.
 */
// C dependency intent: #include "dscr.h"

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_ulong,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

const DT_DIR: u8 = 4;
const F_OK: c_int = 0;

unsafe extern "C" {
    static CPU_PATH: *const c_char;
    static LEN_MAX: c_int;
    static DSCR_MAX: c_int;
    static PPC_FEATURE2_DSCR: c_ulong;

    fn read_ulong(file: *const c_char, val: *mut c_ulong, base: c_int) -> c_int;
    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn get_default_dscr() -> c_ulong;
    fn set_default_dscr(val: c_ulong);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
}

// Rust translation of the C SKIP_IF macro intent from the external selftest
// harness dependency.
unsafe extern "C" {
    fn SKIP_IF(condition: bool);
}

unsafe extern "C" fn check_cpu_dscr_default(file: *mut c_char, val: c_ulong) -> c_int {
    let mut cpu_dscr: c_ulong = 0;
    let mut err: c_int;

    err = unsafe { read_ulong(file, &mut cpu_dscr, 16) };
    if err != 0 {
        return err;
    }

    if cpu_dscr != val {
        unsafe {
            printf(
                c"DSCR match failed: %ld (system) %ld (cpu)\n".as_ptr(),
                val,
                cpu_dscr,
            );
        }
        return 1;
    }
    0
}

unsafe extern "C" fn check_all_cpu_dscr_defaults(val: c_ulong) -> c_int {
    let mut sysfs: *mut DIR;
    let mut dp: *mut dirent;
    let mut file: Vec<c_char> = vec![0; unsafe { LEN_MAX as usize }];

    sysfs = unsafe { opendir(CPU_PATH) };
    if sysfs.is_null() {
        unsafe {
            perror(c"opendir() failed".as_ptr());
        }
        return 1;
    }

    loop {
        dp = unsafe { readdir(sysfs) };
        if dp.is_null() {
            break;
        }

        let mut len: c_int;

        if unsafe { (*dp).d_type } & DT_DIR == 0 {
            continue;
        }
        if unsafe { strcmp((*dp).d_name.as_ptr(), c"cpuidle".as_ptr()) } == 0 {
            continue;
        }
        if unsafe { strstr((*dp).d_name.as_ptr(), c"cpu".as_ptr()) }.is_null() {
            continue;
        }

        len = unsafe {
            snprintf(
                file.as_mut_ptr(),
                LEN_MAX as usize,
                c"%s%s/dscr".as_ptr(),
                CPU_PATH,
                (*dp).d_name.as_ptr(),
            )
        };
        if len >= unsafe { LEN_MAX } {
            continue;
        }
        if unsafe { access(file.as_ptr(), F_OK) } != 0 {
            continue;
        }

        if unsafe { check_cpu_dscr_default(file.as_mut_ptr(), val) } != 0 {
            unsafe {
                closedir(sysfs);
            }
            return 1;
        }
    }
    unsafe {
        closedir(sysfs);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dscr_sysfs() -> c_int {
    let mut orig_dscr_default: c_ulong;

    unsafe {
        SKIP_IF(have_hwcap2(PPC_FEATURE2_DSCR) == 0);
    }

    orig_dscr_default = unsafe { get_default_dscr() };
    let mut i: c_int = 0;
    while i < unsafe { DSCR_MAX } {
        unsafe {
            set_default_dscr(i as c_ulong);
        }
        if unsafe { check_all_cpu_dscr_defaults(i as c_ulong) } != 0 {
            unsafe {
                set_default_dscr(orig_dscr_default);
            }
            return 1;
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
    unsafe { test_harness(dscr_sysfs, c"dscr_sysfs_test".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
