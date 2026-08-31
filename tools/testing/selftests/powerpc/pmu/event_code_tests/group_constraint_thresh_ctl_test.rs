// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "utils.h"
// #include "../sampling_tests/misc.h"

use core::ffi::{c_char, c_int, c_ulonglong};

#[repr(C)]
struct event {
    fd: c_int,
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

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond != 0 {
            return 0;
        }
    };
}

/*
 * Primary PMU events used here are PM_MRK_INST_CMPL (0x401e0) and
 * PM_THRESH_MET (0x101ec).
 * Threshold event selection used is issue to complete and issue to
 * finished for cycles
 * Sampling criteria is Load or Store only sampling
 */
const EventCode_1: c_ulonglong = 0x35340401e0;
const EventCode_2: c_ulonglong = 0x34340101ec;
const EventCode_3: c_ulonglong = 0x35340101ec;

/*
 * Testcase for group constraint check of thresh_ctl bits which is
 * used to program thresh compare field in Monitor Mode Control Register A
 * (MMCR0: 48-55).
 * All events in the group should match thresh ctl bits otherwise
 * event_open for the group will fail.
 */
unsafe extern "C" fn group_constraint_thresh_ctl() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut leader: event = core::mem::zeroed();

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /* Init the events for the group contraint thresh control test */
    event_init(&mut leader, EventCode_1);
    FAIL_IF!(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_2);

    /* Expected to fail as sibling and leader event request different thresh_ctl bits */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) == 0);

    event_close(&mut event);

    /* Init the event for the group contraint thresh control test */
    event_init(&mut event, EventCode_3);

    /* Expected to succeed as sibling and leader event request same thresh_ctl bits */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    0
}

pub unsafe fn main() -> c_int {
    test_harness(
        Some(group_constraint_thresh_ctl),
        b"group_constraint_thresh_ctl\0".as_ptr() as *const c_char,
    )
}
