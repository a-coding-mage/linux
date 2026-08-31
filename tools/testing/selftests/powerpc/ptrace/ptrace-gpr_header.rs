/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_ulong};

pub const GPR_1: c_int = 1;
pub const GPR_2: c_int = 2;
pub const GPR_3: c_int = 3;
pub const GPR_4: c_int = 4;

pub const FPR_1: f64 = 0.001;
pub const FPR_2: f64 = 0.002;
pub const FPR_3: f64 = 0.003;
pub const FPR_4: f64 = 0.004;

pub const FPR_1_REP: u64 = 0x3f50624dd2f1a9fc_u64;
pub const FPR_2_REP: u64 = 0x3f60624dd2f1a9fc_u64;
pub const FPR_3_REP: u64 = 0x3f689374bc6a7efa_u64;
pub const FPR_4_REP: u64 = 0x3f70624dd2f1a9fc_u64;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}

/* TEST_FAIL and TEST_PASS are expected from the translated test dependencies. */

/* Buffer must have 18 elements */
#[no_mangle]
pub unsafe extern "C" fn validate_gpr(gpr: *mut c_ulong, val: c_ulong) -> c_int {
    let mut i: c_int;
    let mut found: c_int = 1;

    i = 0;
    while i < 18 {
        let gpr_i = unsafe { *gpr.add(i as usize) };
        if gpr_i != val {
            unsafe {
                printf(
                    b"GPR[%d]: %lx Expected: %lx\n\0".as_ptr() as *const c_char,
                    i + 14,
                    gpr_i,
                    val,
                );
            }
            found = 0;
        }
        i += 1;
    }

    if found == 0 {
        return TEST_FAIL;
    }
    TEST_PASS
}

/* Buffer must have 32 elements */
#[no_mangle]
pub unsafe extern "C" fn validate_fpr(fpr: *mut u64, val: u64) -> c_int {
    let mut i: c_int;
    let mut found: c_int = 1;

    i = 0;
    while i < 32 {
        let fpr_i = unsafe { *fpr.add(i as usize) };
        if fpr_i != val {
            unsafe {
                printf(
                    b"FPR[%d]: %llx Expected: %llx\n\0".as_ptr() as *const c_char,
                    i,
                    fpr_i,
                    val,
                );
            }
            found = 0;
        }
        i += 1;
    }

    if found == 0 {
        return TEST_FAIL;
    }
    TEST_PASS
}

/* Buffer must have 32 elements */
#[no_mangle]
pub unsafe extern "C" fn validate_fpr_double(fpr: *mut f64, val: f64) -> c_int {
    let mut i: c_int;
    let mut found: c_int = 1;

    i = 0;
    while i < 32 {
        let fpr_i = unsafe { *fpr.add(i as usize) };
        if fpr_i != val {
            unsafe {
                printf(
                    b"FPR[%d]: %f Expected: %f\n\0".as_ptr() as *const c_char,
                    i,
                    fpr_i,
                    val,
                );
            }
            found = 0;
        }
        i += 1;
    }

    if found == 0 {
        return TEST_FAIL;
    }
    TEST_PASS
}
