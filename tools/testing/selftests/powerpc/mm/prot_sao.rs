// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016, Michael Ellerman, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>
// #include <sys/mman.h>
// #include <unistd.h>
// #include <asm/cputable.h>
// #include "utils.h"

use core::ffi::{c_char, c_int, c_long, c_void};

const SIZE: usize = 64 * 1024;

// Provided by system headers / the selftest harness.
extern "C" {
    static PPC_FEATURE_ARCH_2_06: c_long;
    static PPC_FEATURE2_ARCH_3_1: c_long;
    static PROT_READ: c_int;
    static PROT_WRITE: c_int;
    static PROT_SAO: c_int;
    static MAP_ANONYMOUS: c_int;
    static MAP_PRIVATE: c_int;
    static MAP_FAILED: *mut c_void;

    fn have_hwcap(feature: c_long) -> c_int;
    fn have_hwcap2(feature: c_long) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

const NULL: *mut c_void = core::ptr::null_mut();
const F_OK: c_int = 0;

// Translated from utils.h test macros. The exact return codes are supplied by
// that dependency in the original source.
extern "C" {
    fn SKIP_IF(condition: bool);
    fn FAIL_IF(condition: bool);
}

#[no_mangle]
pub unsafe extern "C" fn test_prot_sao() -> c_int {
    let mut p: *mut c_char;

    /*
     * SAO was introduced in 2.06 and removed in 3.1. It's disabled in
     * guests/LPARs by default, so also skip if we are running in a guest.
     */
    SKIP_IF(
        have_hwcap(PPC_FEATURE_ARCH_2_06) == 0
            || have_hwcap2(PPC_FEATURE2_ARCH_3_1) != 0
            || access(
                b"/proc/device-tree/rtas/ibm,hypertas-functions\0".as_ptr() as *const c_char,
                F_OK,
            ) == 0,
    );

    /*
     * Ensure we can ask for PROT_SAO.
     * We can't really verify that it does the right thing, but at least we
     * confirm the kernel will accept it.
     */
    p = mmap(
        NULL,
        SIZE,
        PROT_READ | PROT_WRITE | PROT_SAO,
        MAP_ANONYMOUS | MAP_PRIVATE,
        -1,
        0,
    ) as *mut c_char;
    FAIL_IF(p as *mut c_void == MAP_FAILED);

    /* Write to the mapping, to at least cause a fault */
    memset(p as *mut c_void, 0xaa, SIZE);

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    return test_harness(test_prot_sao, b"prot-sao\0".as_ptr() as *const c_char);
}
