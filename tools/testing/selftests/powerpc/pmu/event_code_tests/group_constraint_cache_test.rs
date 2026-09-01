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

/* All L1 D cache load references counted at finish, gated by reject */
const EventCode_1: u64 = 0x1100fc;
/* Load Missed L1 */
const EventCode_2: u64 = 0x23e054;
/* Load Missed L1 */
const EventCode_3: u64 = 0x13e054;

/*
 * Testcase for group constraint check of data and instructions
 * cache qualifier bits which is used to program cache select field in
 * Monitor Mode Control Register 1 (MMCR1: 16-17) for l1 cache.
 * All events in the group should match cache select bits otherwise
 * event_open for the group will fail.
 */
unsafe fn group_constraint_cache() -> i32 {
    let mut event: event = core::mem::zeroed();
    let mut leader: event = core::mem::zeroed();

    /* Check for platform support for the test */
    SKIP_IF(platform_check_for_tests());

    /* Init the events for the group contraint check for l1 cache select bits */
    event_init(&mut leader, EventCode_1);
    FAIL_IF(event_open(&mut leader));

    event_init(&mut event, EventCode_2);

    /* Expected to fail as sibling event doesn't request same l1 cache select bits as leader */
    FAIL_IF(!event_open_with_group(&mut event, leader.fd));

    event_close(&mut event);

    /* Init the event for the group contraint l1 cache select test */
    event_init(&mut event, EventCode_3);

    /* Expected to succeed as sibling event request same l1 cache select bits as leader */
    FAIL_IF(event_open_with_group(&mut event, leader.fd));

    event_close(&mut leader);
    event_close(&mut event);

    0
}

fn main() -> i32 {
    unsafe { test_harness(group_constraint_cache, "group_constraint_cache") }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
