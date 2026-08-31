// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies: <stdio.h>, "../event.h", <sys/prctl.h>, <limits.h>,
// "../sampling_tests/misc.h"

use std::os::raw::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct event {
    pub fd: c_int,
}

extern "C" {
    fn platform_check_for_tests() -> c_int;
    fn event_init(e: *mut event, event_code: c_ulong);
    fn event_open(e: *mut event) -> c_int;
    fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    fn event_close(e: *mut event);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
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
        if $cond != 0 {
            return 1;
        }
    };
}

/*
 * Testcase for group constraint check for
 * Performance Monitor Counter 5 (PMC5) and also
 * Performance Monitor Counter 6 (PMC6).
 * Test that pmc5/6 is excluded from constraint
 * check when scheduled along with group of events.
 */
unsafe extern "C" fn group_pmc56_exclude_constraints() -> c_int {
    let mut e: *mut event;
    let mut events: [event; 3] = std::mem::zeroed();
    let mut i: c_int;

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /*
     * PMC5/6 is excluded from constraint bit
     * check along with group of events. Use
     * group of events with PMC5, PMC6 and also
     * event with cache bit (dc_ic) set. Test expects
     * this set of events to go in as a group.
     */
    e = &mut events[0];
    event_init(e, 0x500fa);

    e = &mut events[1];
    event_init(e, 0x600f4);

    e = &mut events[2];
    event_init(e, 0x22C040);

    FAIL_IF!(event_open(&mut events[0]));

    /*
     * The event_open will fail if constraint check fails.
     * Since we are asking for events in a group and since
     * PMC5/PMC6 is excluded from group constraints, even_open
     * should pass.
     */
    i = 1;
    while i < 3 {
        FAIL_IF!(event_open_with_group(
            &mut events[i as usize],
            events[0].fd,
        ));
        i += 1;
    }

    i = 0;
    while i < 3 {
        event_close(&mut events[i as usize]);
        i += 1;
    }

    return 0;
}

pub fn main() -> c_int {
    unsafe {
        return test_harness(
            group_pmc56_exclude_constraints,
            b"group_pmc56_exclude_constraints\0".as_ptr() as *const c_char,
        );
    }
}
