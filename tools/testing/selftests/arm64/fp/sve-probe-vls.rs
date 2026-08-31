// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015-2020 ARM Limited.
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

use std::ffi::{c_char, CStr};
use std::os::raw::{c_int, c_ulong, c_uint};

/*
 * C dependencies:
 * #include <errno.h>
 * #include <stdio.h>
 * #include <stdlib.h>
 * #include <string.h>
 * #include <sys/auxv.h>
 * #include <sys/prctl.h>
 * #include <asm/sigcontext.h>
 * #include "kselftest.h"
 * #include "rdvl.h"
 */

extern "C" {
    static SVE_VQ_MAX: c_uint;
    static AT_HWCAP: c_ulong;
    static HWCAP_SVE: c_ulong;
    static PR_SVE_SET_VL: c_int;
    static PR_SVE_VL_LEN_MASK: c_int;

    fn getauxval(type_: c_ulong) -> c_ulong;
    fn prctl(option: c_int, arg2: c_ulong) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_pass() -> !;

    fn rdvl_sve() -> c_int;
    fn sve_vl_valid(vl: c_int) -> bool;
    fn sve_vq_from_vl(vl: c_int) -> c_uint;
}

#[inline]
unsafe fn errno() -> c_int {
    *__errno_location()
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut vq: c_uint;
    let mut vl: c_int;
    static mut VQS: [c_uint; SVE_VQ_MAX as usize] = [0; SVE_VQ_MAX as usize];
    let mut nvqs: c_uint = 0;

    ksft_print_header();
    ksft_set_plan(2);

    if (getauxval(AT_HWCAP) & HWCAP_SVE) == 0 {
        ksft_exit_skip(c"SVE not available\n".as_ptr());
    }

    /*
     * Enumerate up to SVE_VQ_MAX vector lengths
     */
    vq = SVE_VQ_MAX;
    while vq > 0 {
        vl = prctl(PR_SVE_SET_VL, (vq * 16) as c_ulong);
        if vl == -1 {
            let err = errno();
            ksft_exit_fail_msg(
                c"PR_SVE_SET_VL failed: %s (%d)\n".as_ptr(),
                strerror(err),
                err,
            );
        }

        vl &= PR_SVE_VL_LEN_MASK;

        if rdvl_sve() != vl {
            ksft_exit_fail_msg(
                c"PR_SVE_SET_VL reports %d, RDVL %d\n".as_ptr(),
                vl,
                rdvl_sve(),
            );
        }

        if !sve_vl_valid(vl) {
            ksft_exit_fail_msg(c"VL %d invalid\n".as_ptr(), vl);
        }
        vq = sve_vq_from_vl(vl);

        if nvqs >= SVE_VQ_MAX {
            ksft_exit_fail_msg(
                c"Too many VLs %u >= SVE_VQ_MAX\n".as_ptr(),
                nvqs,
            );
        }
        VQS[nvqs as usize] = vq;
        nvqs += 1;

        vq = vq.wrapping_sub(1);
    }
    ksft_test_result_pass(c"Enumerated %d vector lengths\n".as_ptr(), nvqs);
    ksft_test_result_pass(c"All vector lengths valid\n".as_ptr());

    /* Print out the vector lengths in ascending order: */
    while nvqs != 0 {
        nvqs = nvqs.wrapping_sub(1);
        ksft_print_msg(c"%u\n".as_ptr(), 16 * VQS[nvqs as usize]);
    }

    ksft_exit_pass();
}
