// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: <stdio.h>, <stdlib.h>, "ebb.h"
use crate::ebb::*;

/*
 * Test various attributes of the EBB event are enforced.
 */
pub unsafe extern "C" fn event_attributes() -> i32 {
    let mut event: event = ::core::mem::zeroed();
    let mut leader: event = ::core::mem::zeroed();

    SKIP_IF!(!ebb_is_supported());

    event_init(&mut event, 0x1001e);
    event_leader_ebb_init(&mut event);
    /* Expected to succeed */
    FAIL_IF!(event_open(&mut event) != 0);
    event_close(&mut event);

    event_init(&mut event, 0x001e); /* CYCLES - no PMC specified */
    event_leader_ebb_init(&mut event);
    /* Expected to fail, no PMC specified */
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x2001e);
    event_leader_ebb_init(&mut event);
    event.attr.exclusive = 0;
    /* Expected to fail, not exclusive */
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x3001e);
    event_leader_ebb_init(&mut event);
    event.attr.freq = 1;
    /* Expected to fail, sets freq */
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x4001e);
    event_leader_ebb_init(&mut event);
    event.attr.sample_period = 1;
    /* Expected to fail, sets sample_period */
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x1001e);
    event_leader_ebb_init(&mut event);
    event.attr.enable_on_exec = 1;
    /* Expected to fail, sets enable_on_exec */
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut event, 0x1001e);
    event_leader_ebb_init(&mut event);
    event.attr.inherit = 1;
    /* Expected to fail, sets inherit */
    FAIL_IF!(event_open(&mut event) == 0);

    event_init(&mut leader, 0x1001e);
    event_leader_ebb_init(&mut leader);
    FAIL_IF!(event_open(&mut leader) != 0);

    event_init(&mut event, 0x20002);
    event_ebb_init(&mut event);

    /* Expected to succeed */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) != 0);
    event_close(&mut leader);
    event_close(&mut event);

    event_init(&mut leader, 0x1001e);
    event_leader_ebb_init(&mut leader);
    FAIL_IF!(event_open(&mut leader) != 0);

    event_init(&mut event, 0x20002);

    /* Expected to fail, event doesn't request EBB, leader does */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) == 0);
    event_close(&mut leader);

    event_init(&mut leader, 0x1001e);
    event_leader_ebb_init(&mut leader);
    /* Clear the EBB flag */
    leader.attr.config &= !(1u64 << 63);

    FAIL_IF!(event_open(&mut leader) != 0);

    event_init(&mut event, 0x20002);
    event_ebb_init(&mut event);

    /* Expected to fail, leader doesn't request EBB */
    FAIL_IF!(event_open_with_group(&mut event, leader.fd) == 0);
    event_close(&mut leader);

    event_init(&mut leader, 0x1001e);
    event_leader_ebb_init(&mut leader);
    leader.attr.exclusive = 0;
    /* Expected to fail, leader isn't exclusive */
    FAIL_IF!(event_open(&mut leader) == 0);

    event_init(&mut leader, 0x1001e);
    event_leader_ebb_init(&mut leader);
    leader.attr.pinned = 0;
    /* Expected to fail, leader isn't pinned */
    FAIL_IF!(event_open(&mut leader) == 0);

    event_init(&mut event, 0x1001e);
    event_leader_ebb_init(&mut event);
    /* Expected to fail, not a task event */
    SKIP_IF!(require_paranoia_below(1) != 0);
    FAIL_IF!(event_open_with_cpu(&mut event, 0) == 0);

    0
}

pub unsafe extern "C" fn main() -> i32 {
    test_harness(event_attributes, "event_attributes\0".as_ptr() as *const i8)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
