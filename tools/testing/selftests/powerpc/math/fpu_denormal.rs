// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright IBM Corp. 2020
 *
 * This test attempts to cause a FP denormal exception on POWER8 CPUs. Unfortunately
 * if the denormal handler is not configured or working properly, this can cause a bad
 * crash in kernel mode when the kernel tries to save FP registers when the process
 * exits.
 */

use std::ffi::c_char;
use std::os::raw::c_int;
use std::ptr;

unsafe extern "C" {
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

extern "C" fn test_denormal_fpu() -> c_int {
    let m32: u32;
    let mut m64: u64 = 0;
    let mut f: f32 = 0.0;
    let mut d: f64 = 0.0;

    /*
     * volatile float f;
     * volatile double d;
     */

    /* try to induce lfs <denormal> ; stfd */

    m32 = 0x00715fcf; /* random denormal */
    unsafe {
        ptr::copy_nonoverlapping(
            &m32 as *const u32 as *const u8,
            &mut f as *mut f32 as *mut u8,
            std::mem::size_of_val(&f),
        );
        ptr::write_volatile(&mut d, ptr::read_volatile(&f));
        ptr::copy_nonoverlapping(
            &d as *const f64 as *const u8,
            &mut m64 as *mut u64 as *mut u8,
            std::mem::size_of_val(&d),
        );
    }

    if (m64 != 0x380c57f3c0000000) as i64 != 0 {
        /* renormalised value */
        return 1;
    }

    0
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;

    unsafe { test_harness(test_denormal_fpu, c"fpu_denormal".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
