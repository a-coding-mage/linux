// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 ARM Limited
 *
 * Generic test wrapper for arm64 signal tests.
 *
 * Each test provides its own tde struct tdescr descriptor to link with
 * this wrapper. Framework provides common helpers.
 */

use core::ffi::{c_char, c_int, c_ulong};

// Dependencies supplied by:
// <sys/auxv.h>, <sys/prctl.h>, <kselftest.h>,
// "test_signals.h", and "test_signals_utils.h".

#[repr(C)]
pub struct tdescr {
    pub name: *const c_char,
    pub descr: *const c_char,
    pub result: c_int,
}

unsafe extern "C" {
    static mut tde: tdescr;

    static AT_HWCAP: c_ulong;
    static HWCAP_GCS: c_ulong;
    static PR_SHADOW_STACK_ENABLE: c_int;

    fn getauxval(type_: c_ulong) -> c_ulong;
    fn gcs_set_state(state: c_int);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn test_setup(td: *mut tdescr) -> c_int;
    fn test_init(td: *mut tdescr) -> c_int;
    fn test_run(td: *mut tdescr);
    fn test_cleanup(td: *mut tdescr);
    fn test_result(td: *mut tdescr);
    fn exit(status: c_int) -> !;
}

#[unsafe(no_mangle)]
pub static mut current: *mut tdescr = unsafe { &raw mut tde };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    /*
     * Ensure GCS is at least enabled throughout the tests if
     * supported, otherwise the inability to return from the
     * function that enabled GCS makes it very inconvenient to set
     * up test cases.  The prctl() may fail if GCS was locked by
     * libc setup code.
     */
    unsafe {
        if (getauxval(AT_HWCAP) & HWCAP_GCS) != 0 {
            gcs_set_state(PR_SHADOW_STACK_ENABLE);
        }

        ksft_print_msg(
            c"%s :: %s\n".as_ptr(),
            (*current).name,
            (*current).descr,
        );
        if test_setup(current) != 0 && test_init(current) != 0 {
            test_run(current);
            test_cleanup(current);
        }
        test_result(current);

        /* Do not return in case GCS was enabled */
        exit((*current).result);
    }
}
