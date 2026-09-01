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

/*
 * Primary PMU events used here is PM_MRK_INST_CMPL (0x401e0) and
 * PM_THRESH_MET (0x101ec)
 * Threshold event selection used is issue to complete for cycles
 * Sampling criteria is Load or Store only sampling
 */
const p9_EventCode_1: u64 = 0x13e35340401e0;
const p9_EventCode_2: u64 = 0x17d34340101ec;
const p9_EventCode_3: u64 = 0x13e35340101ec;
const p10_EventCode_1: u64 = 0x35340401e0;
const p10_EventCode_2: u64 = 0x35340101ec;

#[repr(C)]
pub struct perf_event_attr {
    pub config1: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: std::os::raw::c_int,
}

extern "C" {
    static PPC_FEATURE2_ARCH_3_1: std::os::raw::c_ulong;

    fn platform_check_for_tests() -> std::os::raw::c_int;
    fn have_hwcap2(feature: std::os::raw::c_ulong) -> std::os::raw::c_int;
    fn event_init(event: *mut event, config: u64);
    fn event_open(event: *mut event) -> std::os::raw::c_int;
    fn event_open_with_group(
        event: *mut event,
        group_fd: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
    fn event_close(event: *mut event);
    fn test_harness(
        test: unsafe extern "C" fn() -> std::os::raw::c_int,
        name: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
}

unsafe fn skip_if(cond: std::os::raw::c_int) {
    if cond != 0 {
        std::process::exit(4);
    }
}

unsafe fn fail_if(cond: bool) {
    if cond {
        std::process::exit(1);
    }
}

/*
 * Testcase for group constraint check of thresh_cmp bits which is
 * used to program thresh compare field in Monitor Mode Control Register A
 * (MMCRA: 9-18 bits for power9 and MMCRA: 8-18 bits for power10/power11).
 * All events in the group should match thresh compare bits otherwise
 * event_open for the group will fail.
 */
unsafe extern "C" fn group_constraint_thresh_cmp() -> std::os::raw::c_int {
    let mut event = std::mem::MaybeUninit::<event>::uninit();
    let mut leader = std::mem::MaybeUninit::<event>::uninit();

    /* Check for platform support for the test */
    skip_if(platform_check_for_tests());

    if have_hwcap2(PPC_FEATURE2_ARCH_3_1) != 0 {
        /* Init the events for the group contraint check for thresh_cmp bits */
        event_init(leader.as_mut_ptr(), p10_EventCode_1);

        /* Add the thresh_cmp value for leader in config1 */
        (*leader.as_mut_ptr()).attr.config1 = 1000;
        fail_if(event_open(leader.as_mut_ptr()) != 0);

        event_init(event.as_mut_ptr(), p10_EventCode_2);

        /* Add the different thresh_cmp value from the leader event in config1 */
        (*event.as_mut_ptr()).attr.config1 = 2000;

        /* Expected to fail as sibling and leader event request different thresh_cmp bits */
        fail_if(!((event_open_with_group(event.as_mut_ptr(), (*leader.as_ptr()).fd)) != 0));

        event_close(event.as_mut_ptr());

        /* Init the event for the group contraint thresh compare test */
        event_init(event.as_mut_ptr(), p10_EventCode_2);

        /* Add the same thresh_cmp value for leader and sibling event in config1 */
        (*event.as_mut_ptr()).attr.config1 = 1000;

        /* Expected to succeed as sibling and leader event request same thresh_cmp bits */
        fail_if(event_open_with_group(event.as_mut_ptr(), (*leader.as_ptr()).fd) != 0);

        event_close(leader.as_mut_ptr());
        event_close(event.as_mut_ptr());
    } else {
        /* Init the events for the group contraint check for thresh_cmp bits */
        event_init(leader.as_mut_ptr(), p9_EventCode_1);
        fail_if(event_open(leader.as_mut_ptr()) != 0);

        event_init(event.as_mut_ptr(), p9_EventCode_2);

        /* Expected to fail as sibling and leader event request different thresh_cmp bits */
        fail_if(!((event_open_with_group(event.as_mut_ptr(), (*leader.as_ptr()).fd)) != 0));

        event_close(event.as_mut_ptr());

        /* Init the event for the group contraint thresh compare test */
        event_init(event.as_mut_ptr(), p9_EventCode_3);

        /* Expected to succeed as sibling and leader event request same thresh_cmp bits */
        fail_if(event_open_with_group(event.as_mut_ptr(), (*leader.as_ptr()).fd) != 0);

        event_close(leader.as_mut_ptr());
        event_close(event.as_mut_ptr());
    }

    0
}

fn main() {
    let name = b"group_constraint_thresh_cmp\0";

    unsafe {
        std::process::exit(test_harness(
            group_constraint_thresh_cmp,
            name.as_ptr() as *const std::os::raw::c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
