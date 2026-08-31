// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Michael Neuling, IBM Corp.
 * Original: Michael Neuling 19/7/2013
 * Edited: Rashmica Gupta 01/12/2015
 *
 * Do some transactions, see if the tar is corrupted.
 * If the transaction is aborted, the TAR should be rolled back to the
 * checkpointed value before the transaction began. The value written to
 * TAR in suspended mode should only remain in TAR if the transaction
 * completes.
 */

use core::arch::asm;
use std::ffi::c_char;
use std::os::raw::c_int;

/* From tm.h / utils.h in the original C source. */
unsafe extern "C" {
    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn is_ppc64le() -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

/* SPRN_TAR, the Target Address Register SPR number. */
const SPRN_TAR: i32 = 815;

static mut num_loops: c_int = 10000;

/* Rust equivalent of the SKIP_IF() test helper macro supplied by utils.h. */
macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn test_tar() -> c_int {
    let mut i: c_int;

    SKIP_IF!(have_htm() == 0);
    SKIP_IF!(htm_is_synthetic() != 0);
    SKIP_IF!(is_ppc64le() == 0);

    i = 0;
    while i < num_loops {
        let mut result: u64;
        asm!(
            "li     7, 1;",
            "mtspr  {tar}, 7;",    /* tar = 1 */
            "tbegin.;",
            "beq    3f;",
            "li     4, 0x7000;",   /* Loop lots, to use time */
            "2:",                  /* Start loop */
            "li     7, 2;",
            "mtspr  {tar}, 7;",    /* tar = 2 */
            "tsuspend.;",
            "li     7, 3;",
            "mtspr  {tar}, 7;",    /* tar = 3 */
            "tresume.;",
            "subi   4, 4, 1;",
            "cmpdi  4, 0;",
            "bne    2b;",
            "tend.;",

            /* Transaction sucess! TAR should be 3 */
            "mfspr  7, {tar};",
            "ori    {res}, 7, 4;",  // res = 3|4 = 7
            "b      4f;",

            /* Abort handler. TAR should be rolled back to 1 */
            "3:",
            "mfspr  7, {tar};",
            "ori    {res}, 7, 8;",  // res = 1|8 = 9
            "4:",

            res = lateout(reg) result,
            tar = const SPRN_TAR,
            options(nostack, preserves_flags),
        );

        /* If result is anything else other than 7 or 9, the tar
         * value must have been corrupted. */
        if (result != 7) && (result != 9) {
            return 1;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    /* A low number of iterations (eg 100) can cause a false pass */
    if argc > 1 {
        if strcmp(*argv.offset(1), c"-h".as_ptr()) == 0 {
            printf(
                c"Syntax:\n\t%s [<num loops>]\n".as_ptr(),
                *argv.offset(0),
            );
            return 1;
        } else {
            num_loops = atoi(*argv.offset(1));
        }
    }

    printf(c"Starting, %d loops\n".as_ptr(), num_loops);

    test_harness(test_tar, c"tm_tar".as_ptr())
}
