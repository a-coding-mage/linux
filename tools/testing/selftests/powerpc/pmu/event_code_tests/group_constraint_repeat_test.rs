// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

use core::ffi::{c_char, c_int, c_ulong};

/* The processor's L1 data cache was reloaded */
const EventCode1: c_ulong = 0x21C040;
const EventCode2: c_ulong = 0x22C040;

// Layout is supplied by ../event.h in the original source. The fd field is used
// directly by this file.
#[repr(C)]
pub struct event {
    pub fd: c_int,
}

extern "C" {
    fn platform_check_for_tests() -> c_int;
    fn event_init(event: *mut event, event_code: c_ulong);
    fn event_open(event: *mut event) -> c_int;
    fn event_open_with_group(event: *mut event, group_fd: c_int) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

unsafe fn skip_if(condition: c_int) -> Option<c_int> {
    if condition != 0 {
        Some(0)
    } else {
        None
    }
}

unsafe fn fail_if(condition: bool) -> Option<c_int> {
    if condition {
        Some(1)
    } else {
        None
    }
}

/*
 * Testcase for group constraint check
 * when using events with same PMC.
 * Multiple events in a group shouldn't
 * ask for same PMC. If so it should fail.
 */

unsafe extern "C" fn group_constraint_repeat() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut leader: event = core::mem::zeroed();

    /* Check for platform support for the test */
    if let Some(ret) = skip_if(platform_check_for_tests()) {
        return ret;
    }

    /*
     * Two events in a group using same PMC
     * should fail to get scheduled. Usei same PMC2
     * for leader and sibling event which is expected
     * to fail.
     */
    event_init(&mut leader, EventCode1);
    if let Some(ret) = fail_if(event_open(&mut leader) != 0) {
        return ret;
    }

    event_init(&mut event, EventCode1);

    /* Expected to fail since sibling event is requesting same PMC as leader */
    if let Some(ret) = fail_if(!(event_open_with_group(&mut event, leader.fd) != 0)) {
        return ret;
    }

    event_init(&mut event, EventCode2);

    /* Expected to pass since sibling event is requesting different PMC */
    if let Some(ret) = fail_if(event_open_with_group(&mut event, leader.fd) != 0) {
        return ret;
    }

    event_close(&mut leader);
    event_close(&mut event);

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    return test_harness(
        group_constraint_repeat,
        b"group_constraint_repeat\0".as_ptr() as *const c_char,
    );
}
