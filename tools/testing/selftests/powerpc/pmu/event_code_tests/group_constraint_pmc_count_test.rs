// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

use core::ffi::c_int;
use core::mem::MaybeUninit;
use crate::event::event;

// Dependencies translated from:
// #include <stdio.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

extern "C" {
    fn platform_check_for_tests() -> c_int;
    fn event_init(e: *mut event, event: u64);
    fn event_open(e: *mut event) -> c_int;
    fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    fn event_close(e: *mut event);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const core::ffi::c_char,
    ) -> c_int;
}

/*
 * Testcase for number of counters in use.
 * The number of programmable counters is from
 * performance monitor counter 1 to performance
 * monitor counter 4 (PMC1-PMC4). If number of
 * counters in use exceeds the limit, next event
 * should fail to schedule.
 */

unsafe extern "C" fn group_constraint_pmc_count() -> c_int {
    let mut e: *mut event;
    let mut events: [MaybeUninit<event>; 5] = MaybeUninit::uninit().assume_init();
    let mut i: c_int;

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /*
     * Test for number of counters in use.
     * Use PMC1 to PMC4 for leader and 3 sibling
     * events. Trying to open fourth event should
     * fail here.
     */
    e = events[0].as_mut_ptr();
    event_init(e, 0x1001a);

    e = events[1].as_mut_ptr();
    event_init(e, 0x200fc);

    e = events[2].as_mut_ptr();
    event_init(e, 0x30080);

    e = events[3].as_mut_ptr();
    event_init(e, 0x40054);

    e = events[4].as_mut_ptr();
    event_init(e, 0x0002c);

    FAIL_IF!(event_open(events[0].as_mut_ptr()));

    /*
     * The event_open will fail on event 4 if constraint
     * check fails
     */
    i = 1;
    while i < 5 {
        if i == 4 {
            FAIL_IF!(!event_open_with_group(
                events[i as usize].as_mut_ptr(),
                (*events[0].as_ptr()).fd,
            ));
        } else {
            FAIL_IF!(event_open_with_group(
                events[i as usize].as_mut_ptr(),
                (*events[0].as_ptr()).fd,
            ));
        }
        i += 1;
    }

    i = 1;
    while i < 4 {
        event_close(events[i as usize].as_mut_ptr());
        i += 1;
    }

    0
}

fn main() -> c_int {
    unsafe {
        test_harness(
            group_constraint_pmc_count,
            b"group_constraint_pmc_count\0".as_ptr() as *const core::ffi::c_char,
        )
    }
}
