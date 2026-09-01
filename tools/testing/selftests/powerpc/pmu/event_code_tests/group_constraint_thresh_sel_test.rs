// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

use core::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_ulonglong};

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "utils.h"
// #include "../sampling_tests/misc.h"

#[repr(C)]
pub struct event {
    pub fd: c_int,
}

extern "C" {
    fn event_init(event: *mut event, event_code: c_ulonglong);
    fn event_open(event: *mut event) -> c_int;
    fn event_open_with_group(event: *mut event, group_fd: c_int) -> c_int;
    fn event_close(event: *mut event);
    fn platform_check_for_tests() -> c_int;
    fn test_harness(
        test_function: Option<unsafe extern "C" fn() -> c_int>,
        name: *const c_char,
    ) -> c_int;
}

const KSFT_SKIP: c_int = 4;

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return KSFT_SKIP;
        }
    };
}

macro_rules! FAIL_IF {
    ($condition:expr) => {
        if $condition {
            return 1;
        }
    };
}

/*
 * Primary PMU events used here are PM_MRK_INST_CMPL (0x401e0) and
 * PM_THRESH_MET (0x101ec).
 * Threshold event selection used is issue to complete
 * Sampling criteria is Load or Store only sampling
 */
const EventCode_1: c_ulonglong = 0x35340401e0;
const EventCode_2: c_ulonglong = 0x35540101ec;
const EventCode_3: c_ulonglong = 0x35340101ec;

/*
 * Testcase for group constraint check of thresh_sel bits which is
 * used to program thresh select field in Monitor Mode Control Register A
 * (MMCRA: 45-57).
 * All events in the group should match thresh sel bits otherwise
 * event_open for the group will fail.
 */
unsafe extern "C" fn group_constraint_thresh_sel() -> c_int {
    let mut event = MaybeUninit::<event>::uninit();
    let mut leader = MaybeUninit::<event>::uninit();

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /* Init the events for the group contraint thresh select test */
    event_init(leader.as_mut_ptr(), EventCode_1);
    FAIL_IF!(event_open(leader.as_mut_ptr()) != 0);

    event_init(event.as_mut_ptr(), EventCode_2);

    /* Expected to fail as sibling and leader event request different thresh_sel bits */
    FAIL_IF!(event_open_with_group(event.as_mut_ptr(), (*leader.as_ptr()).fd) == 0);

    event_close(event.as_mut_ptr());

    /* Init the event for the group contraint thresh select test */
    event_init(event.as_mut_ptr(), EventCode_3);

    /* Expected to succeed as sibling and leader event request same thresh_sel bits */
    FAIL_IF!(event_open_with_group(event.as_mut_ptr(), (*leader.as_ptr()).fd) != 0);

    event_close(leader.as_mut_ptr());
    event_close(event.as_mut_ptr());

    0
}

pub unsafe fn main() -> c_int {
    test_harness(
        Some(group_constraint_thresh_sel),
        b"group_constraint_thresh_sel\0".as_ptr() as *const c_char,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
