// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// Dependencies from the original C file:
// #include <stdio.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

use core::ffi::{c_char, c_int, c_ulonglong};

const EventCode_1: c_ulonglong = 0x35340401e0;
const EventCode_2: c_ulonglong = 0x353c0101ec;
const EventCode_3: c_ulonglong = 0x35340101ec;

// Layout is supplied by ../event.h in the original source. The fd field is used
// directly by this file.
#[repr(C)]
pub struct event {
    pub fd: c_int,
}

extern "C" {
    fn platform_check_for_tests() -> c_int;
    fn event_init(event: *mut event, event_code: c_ulonglong);
    fn event_open(event: *mut event) -> c_int;
    fn event_open_with_group(event: *mut event, group_fd: c_int) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return 0;
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
 * Test that using different sample bits in
 * event code cause failure in schedule for
 * group of events.
 */
unsafe extern "C" fn group_constraint_mmcra_sample() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut leader: event = core::mem::zeroed();

    SKIP_IF!(platform_check_for_tests());

    /*
     * Events with different "sample" field values
     * in a group will fail to schedule.
     * Use event with load only sampling mode as
     * group leader. Use event with store only sampling
     * as sibling event.
     */
    event_init(&mut leader, EventCode_1);
    FAIL_IF!(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_2);

    /* Expected to fail as sibling event doesn't use same sampling bits as leader */
    FAIL_IF!(!(event_open_with_group(&mut event, leader.fd) != 0));

    event_init(&mut event, EventCode_3);

    /* Expected to pass as sibling event use same sampling bits as leader */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    0
}

fn main() -> c_int {
    unsafe {
        test_harness(
            group_constraint_mmcra_sample,
            b"group_constraint_mmcra_sample\0".as_ptr() as *const c_char,
        )
    }
}
