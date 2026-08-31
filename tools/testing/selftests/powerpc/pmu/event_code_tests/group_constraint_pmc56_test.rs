// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

use core::ffi::{c_char, c_int, c_ulonglong};

#[repr(C)]
pub struct event {
    _private: [u8; 0],
}

extern "C" {
    fn platform_check_for_tests() -> c_int;
    fn event_init(event: *mut event, code: c_ulonglong);
    fn event_open(event: *mut event) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond != 0 {
            return 0;
        }
    };
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

/*
 * Testcase for checking constraint checks for
 * Performance Monitor Counter 5 (PMC5) and also
 * Performance Monitor Counter 6 (PMC6). Events using
 * PMC5/PMC6 shouldn't have other fields in event
 * code like cache bits, thresholding or marked bit.
 */
unsafe extern "C" fn group_constraint_pmc56() -> c_int {
    let mut event: event = core::mem::zeroed();

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /*
     * Events using PMC5 and PMC6 with cache bit
     * set in event code is expected to fail.
     */
    event_init(&mut event, 0x2500fa);
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x2600f4);
    FAIL_IF!(event_open(&mut event) == 0);

    /*
     * PMC5 and PMC6 only supports base events:
     * ie 500fa and 600f4. Other combinations
     * should fail.
     */
    event_init(&mut event, 0x501e0);
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x6001e);
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x501fa);
    FAIL_IF!(event_open(&mut event) == 0);

    /*
     * Events using PMC5 and PMC6 with random
     * sampling bits set in event code should fail
     * to schedule.
     */
    event_init(&mut event, 0x35340500fa);
    FAIL_IF!(event_open(&mut event) == 0);

    0
}

fn main() -> c_int {
    unsafe { test_harness(group_constraint_pmc56, b"group_constraint_pmc56\0".as_ptr().cast()) }
}
