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

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct event {
    pub fd: c_int,
}

extern "C" {
    fn platform_check_for_tests() -> c_int;
    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn event_init(event: *mut event, code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_open_with_group(event: *mut event, group_fd: c_int) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(
        test_function: Option<unsafe extern "C" fn() -> c_int>,
        name: *const c_char,
    ) -> c_int;
}

extern "C" {
    static PPC_FEATURE2_ARCH_3_1: c_ulong;
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

/* All successful D-side store dispatches for this thread */
const EventCode_1: u64 = 0x010000046080;
/* All successful D-side store dispatches for this thread that were L2 Miss */
const EventCode_2: u64 = 0x26880;
/* All successful D-side store dispatches for this thread that were L2 Miss */
const EventCode_3: u64 = 0x010000026880;

/*
 * Testcase for group constraint check of l2l3_sel bits which is
 * used to program l2l3 select field in Monitor Mode Control Register 0
 * (MMCR0: 56-60).
 * All events in the group should match l2l3_sel bits otherwise
 * event_open for the group should fail.
 */
unsafe extern "C" fn group_constraint_l2l3_sel() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut leader: event = core::mem::zeroed();

    /*
     * Check for platform support for the test.
     * This test is only aplicable on ISA v3.1
     */
    SKIP_IF!(platform_check_for_tests());
    SKIP_IF!((have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0) as c_int);

    /* Init the events for the group contraint check for l2l3_sel bits */
    event_init(&mut leader, EventCode_1);
    FAIL_IF!(event_open(&mut leader) != 0);

    event_init(&mut event, EventCode_2);

    /* Expected to fail as sibling event doesn't request same l2l3_sel bits as leader */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) == 0);

    event_close(&mut event);

    /* Init the event for the group contraint l2l3_sel test */
    event_init(&mut event, EventCode_3);

    /* Expected to succeed as sibling event request same l2l3_sel bits as leader */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) != 0);

    event_close(&mut leader);
    event_close(&mut event);

    return 0;
}

pub unsafe fn main_0() -> c_int {
    return test_harness(
        Some(group_constraint_l2l3_sel),
        b"group_constraint_l2l3_sel\0".as_ptr() as *const c_char,
    );
}

fn main() {
    unsafe {
        main_0();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
