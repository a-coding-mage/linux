// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

use core::ffi::{c_char, c_int, c_uint};

// Dependencies from:
// #include "../event.h"
// #include "utils.h"
// #include "../sampling_tests/misc.h"

#[repr(C)]
pub struct event {
    pub fd: c_int,
}

unsafe extern "C" {
    fn event_init(e: *mut event, event_code: c_uint);
    fn event_open(e: *mut event) -> c_int;
    fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    fn event_close(e: *mut event);
    fn platform_check_for_tests() -> c_int;
    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    static PPC_FEATURE2_ARCH_3_1: c_ulong;
}

#[cfg(target_pointer_width = "64")]
type c_ulong = u64;
#[cfg(target_pointer_width = "32")]
type c_ulong = u32;

const TEST_SKIP: c_int = 4;

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return TEST_SKIP;
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

/* All successful D-side store dispatches for this thread with PMC 2 */
const EventCode_1: c_uint = 0x26080;
/* All successful D-side store dispatches for this thread with PMC 4 */
const EventCode_2: c_uint = 0x46080;
/* All successful D-side store dispatches for this thread that were L2 Miss with PMC 3 */
const EventCode_3: c_uint = 0x36880;

/*
 * Testcase for group constraint check of unit and pmc bits which is
 * used to program corresponding unit and pmc field in Monitor Mode
 * Control Register 1 (MMCR1)
 * One of the event in the group should use PMC 4 incase units field
 * value is within 6 to 9 otherwise event_open for the group will fail.
 */
unsafe extern "C" fn group_constraint_unit() -> c_int {
    let mut e: *mut event;
    let mut events: [event; 3] = [
        event { fd: 0 },
        event { fd: 0 },
        event { fd: 0 },
    ];

    /*
     * Check for platform support for the test.
     * Constraint to use PMC4 with one of the event in group,
     * when the unit is within 6 to 9 is only applicable on
     * power9.
     */
    SKIP_IF!(unsafe { platform_check_for_tests() });
    SKIP_IF!(unsafe { have_hwcap2(PPC_FEATURE2_ARCH_3_1) });

    /* Init the events for the group contraint check for unit bits */
    e = &mut events[0];
    unsafe {
        event_init(e, EventCode_1);
    }

    /* Expected to fail as PMC 4 is not used with unit field value 6 to 9 */
    FAIL_IF!(unsafe { event_open(&mut events[0]) } == 0);

    /* Init the events for the group contraint check for unit bits */
    e = &mut events[1];
    unsafe {
        event_init(e, EventCode_2);
    }

    /* Expected to pass as PMC 4 is used with unit field value 6 to 9 */
    FAIL_IF!(unsafe { event_open(&mut events[1]) } != 0);

    /* Init the event for the group contraint unit test */
    e = &mut events[2];
    unsafe {
        event_init(e, EventCode_3);
    }

    /* Expected to fail as PMC4 is not being used */
    FAIL_IF!(unsafe { event_open_with_group(&mut events[2], events[0].fd) } == 0);

    /* Expected to succeed as event using PMC4 */
    FAIL_IF!(unsafe { event_open_with_group(&mut events[2], events[1].fd) } != 0);

    unsafe {
        event_close(&mut events[0]);
        event_close(&mut events[1]);
        event_close(&mut events[2]);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { test_harness(group_constraint_unit, c"group_constraint_unit".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
