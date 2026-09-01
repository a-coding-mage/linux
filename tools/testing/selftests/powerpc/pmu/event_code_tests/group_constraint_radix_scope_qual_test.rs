// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies: <stdio.h>, "../event.h", "../sampling_tests/misc.h"

#[repr(C)]
pub struct event {
    pub fd: ::std::os::raw::c_int,
}

unsafe extern "C" {
    fn platform_check_for_tests() -> ::std::os::raw::c_int;
    fn have_hwcap2(feature: ::std::os::raw::c_ulong) -> ::std::os::raw::c_int;
    fn event_init(event: *mut event, event_code: ::std::os::raw::c_ulong);
    fn event_open(event: *mut event) -> ::std::os::raw::c_int;
    fn event_open_with_group(event: *mut event, group_fd: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
    fn event_close(event: *mut event);
    fn test_harness(
        test: unsafe extern "C" fn() -> ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

unsafe extern "C" {
    static PPC_FEATURE2_ARCH_3_1: ::std::os::raw::c_ulong;
}

/* PM_DATA_RADIX_PROCESS_L2_PTE_FROM_L2 */
const EventCode_1: ::std::os::raw::c_ulong = 0x14242;
/* PM_DATA_RADIX_PROCESS_L2_PTE_FROM_L3 */
const EventCode_2: ::std::os::raw::c_ulong = 0x24242;

unsafe fn skip_if(cond: bool) -> ::std::os::raw::c_int {
    if cond {
        return 4;
    }
    0
}

unsafe fn fail_if(cond: bool) -> ::std::os::raw::c_int {
    if cond {
        return 1;
    }
    0
}

/*
 * Testcase for group constraint check for radix_scope_qual
 * field which is used to program Monitor Mode Control
 * egister (MMCR1)  bit 18.
 * All events in the group should match radix_scope_qual,
 * bits otherwise event_open for the group should fail.
 */
unsafe extern "C" fn group_constraint_radix_scope_qual() -> ::std::os::raw::c_int {
    let mut event: event = ::std::mem::zeroed();
    let mut leader: event = ::std::mem::zeroed();

    /*
     * Check for platform support for the test.
     * This test is aplicable on ISA v3.1 only.
     */
    let mut ret = skip_if(platform_check_for_tests() != 0);
    if ret != 0 {
        return ret;
    }
    ret = skip_if(!have_hwcap2(PPC_FEATURE2_ARCH_3_1) != 0);
    if ret != 0 {
        return ret;
    }

    /* Init the events for the group contraint check for radix_scope_qual bits */
    event_init(&mut leader, EventCode_1);
    ret = fail_if(event_open(&mut leader) != 0);
    if ret != 0 {
        return ret;
    }

    event_init(&mut event, 0x200fc);

    /* Expected to fail as sibling event doesn't request same radix_scope_qual bits as leader */
    ret = fail_if(!event_open_with_group(&mut event, leader.fd) != 0);
    if ret != 0 {
        return ret;
    }

    event_init(&mut event, EventCode_2);
    /* Expected to pass as sibling event request same radix_scope_qual bits as leader */
    ret = fail_if(event_open_with_group(&mut event, leader.fd) != 0);
    if ret != 0 {
        return ret;
    }

    event_close(&mut leader);
    event_close(&mut event);
    return 0;
}

fn main() -> ::std::os::raw::c_int {
    unsafe {
        return test_harness(
            group_constraint_radix_scope_qual,
            b"group_constraint_radix_scope_qual\0".as_ptr() as *const ::std::os::raw::c_char,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
