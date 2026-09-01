// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Data Stream Control Register (DSCR) SPR test
 *
 * This test modifies the DSCR value through both the SPR number
 * based mtspr instruction and then makes sure that the same is
 * reflected through mfspr instruction using either of the SPR
 * numbers.
 *
 * When using the privilege state SPR, the instructions such as
 * mfspr or mtspr are privileged and the kernel emulates them
 * for us. Instructions using problem state SPR can be executed
 * directly without any emulation if the HW supports them. Else
 * they also get emulated by the kernel.
 *
 * Copyright 2013, Anton Blanchard, IBM Corporation.
 * Copyright 2015, Anshuman Khandual, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_ulong};

/*
 * C dependency: #include "dscr.h"
 *
 * These declarations are supplied by that dependency in the original source.
 * SKIP_IF is a C macro there; it is represented here as an external dependency
 * preserving the source-level call site.
 */
unsafe extern "C" {
    static COUNT: c_int;
    static PPC_FEATURE2_DSCR: c_ulong;

    fn get_dscr() -> c_ulong;
    fn get_dscr_usr() -> c_ulong;
    fn set_dscr(val: c_ulong);
    fn set_dscr_usr(val: c_ulong);
    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn SKIP_IF(condition: bool);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

unsafe extern "C" fn check_dscr(str_: *mut c_char) -> c_int {
    let cur_dscr: c_ulong;
    let cur_dscr_usr: c_ulong;

    cur_dscr = unsafe { get_dscr() };
    cur_dscr_usr = unsafe { get_dscr_usr() };
    if cur_dscr != cur_dscr_usr {
        unsafe {
            printf(
                b"%s set, kernel get %lx != user get %lx\n\0".as_ptr() as *const c_char,
                str_,
                cur_dscr,
                cur_dscr_usr,
            );
        }
        return 1;
    }
    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dscr_user() -> c_int {
    let mut i: c_int;

    unsafe {
        SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR) != 0);
    }

    unsafe {
        check_dscr(b"\0".as_ptr() as *mut c_char);
    }

    i = 0;
    while i < unsafe { COUNT } {
        unsafe {
            set_dscr(i as c_ulong);
            if check_dscr(b"kernel\0".as_ptr() as *mut c_char) != 0 {
                return 1;
            }
        }
        i += 1;
    }

    i = 0;
    while i < unsafe { COUNT } {
        unsafe {
            set_dscr_usr(i as c_ulong);
            if check_dscr(b"user\0".as_ptr() as *mut c_char) != 0 {
                return 1;
            }
        }
        i += 1;
    }
    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe { test_harness(dscr_user, b"dscr_user_test\0".as_ptr() as *const c_char) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
