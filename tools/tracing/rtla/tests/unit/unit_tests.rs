// SPDX-License-Identifier: GPL-2.0

// C source used _GNU_SOURCE and included:
// <check.h>, <stdbool.h>, "../../src/utils.h", "../../src/cli.h"

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct Suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SRunner {
    _private: [u8; 0],
}

const CK_VERBOSE: c_int = 3;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

unsafe extern "C" {
    static mut in_unit_test: bool;

    fn utils_suite() -> *mut Suite;
    fn actions_suite() -> *mut Suite;
    fn osnoise_top_cli_suite() -> *mut Suite;
    fn osnoise_hist_cli_suite() -> *mut Suite;
    fn timerlat_top_cli_suite() -> *mut Suite;
    fn timerlat_hist_cli_suite() -> *mut Suite;
    fn cli_opt_callback_suite() -> *mut Suite;

    fn srunner_create(s: *mut Suite) -> *mut SRunner;
    fn srunner_add_suite(sr: *mut SRunner, s: *mut Suite);
    fn srunner_run_all(sr: *mut SRunner, print_mode: c_int);
    fn srunner_ntests_failed(sr: *mut SRunner) -> c_int;
    fn srunner_free(sr: *mut SRunner);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let num_failed: c_int;
    let sr: *mut SRunner;

    unsafe {
        in_unit_test = true;

        sr = srunner_create(utils_suite());
        srunner_add_suite(sr, cli_opt_callback_suite());
        srunner_add_suite(sr, actions_suite());
        srunner_add_suite(sr, osnoise_top_cli_suite());
        srunner_add_suite(sr, osnoise_hist_cli_suite());
        srunner_add_suite(sr, timerlat_top_cli_suite());
        srunner_add_suite(sr, timerlat_hist_cli_suite());

        srunner_run_all(sr, CK_VERBOSE);
        num_failed = srunner_ntests_failed(sr);

        srunner_free(sr);
    }

    if num_failed == 0 {
        EXIT_SUCCESS
    } else {
        EXIT_FAILURE
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
