// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include "../event.h"
// #include "../sampling_tests/misc.h"

use core::ffi::{c_char, c_int, c_ulong};

const PM_RUN_CYC_ALT: c_int = 0x200f4;
const PM_INST_DISP: c_int = 0x200f2;
const PM_BR_2PATH: c_int = 0x20036;
const PM_LD_MISS_L1: c_int = 0x3e054;
const PM_RUN_INST_CMPL_ALT: c_int = 0x400fa;

const EventCode_1: c_int = 0x100fc;
const EventCode_2: c_int = 0x200fa;
const EventCode_3: c_int = 0x300fc;
const EventCode_4: c_int = 0x400fc;

extern "C" {
    static SPRN_PVR: c_int;
    static POWER10: c_int;
    static POWER11: c_int;

    fn PVR_VER(value: c_ulong) -> c_int;
    fn mfspr(reg: c_int) -> c_ulong;
    fn platform_check_for_tests() -> c_int;
    fn check_for_generic_compat_pmu() -> c_int;

    fn event_init(e: *mut event, event_code: c_int);
    fn event_open(e: *mut event) -> c_int;
    fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    fn event_close(e: *mut event);

    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct event {
    // Field used directly by this source file. Remaining layout is supplied by ../event.h
    // in the original C build and is intentionally not reconstructed here.
    fd: c_int,
}

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return 4;
        }
    };
}

macro_rules! FAIL_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return 1;
        }
    };
}

/*
 * Check for event alternatives.
 */
unsafe extern "C" fn event_alternatives_tests_p10() -> c_int {
    let mut events: [event; 5] = [event { fd: 0 }; 5];
    let mut e: *mut event;
    let mut i: c_int;
    let pvr: c_int = PVR_VER(mfspr(SPRN_PVR));

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /*
     * PVR check is used here since PMU specific data like
     * alternative events is handled by respective PMU driver
     * code and using PVR will work correctly for all cases
     * including generic compat mode.
     */
    SKIP_IF!(((pvr != POWER10) && (pvr != POWER11)) as c_int);

    SKIP_IF!(check_for_generic_compat_pmu());

    /*
     * Test for event alternative for 0x0001e
     * and 0x00002.
     */
    e = &mut events[0];
    event_init(e, 0x0001e);

    e = &mut events[1];
    event_init(e, EventCode_1);

    e = &mut events[2];
    event_init(e, EventCode_2);

    e = &mut events[3];
    event_init(e, EventCode_3);

    e = &mut events[4];
    event_init(e, EventCode_4);

    FAIL_IF!(event_open(&mut events[0]));

    /*
     * Expected to pass since 0x0001e has alternative event
     * 0x600f4 in PMC6. So it can go in with other events
     * in PMC1 to PMC4.
     */
    i = 1;
    while i < 5 {
        FAIL_IF!(event_open_with_group(
            &mut events[i as usize],
            events[0].fd,
        ));
        i += 1;
    }

    i = 0;
    while i < 5 {
        event_close(&mut events[i as usize]);
        i += 1;
    }

    e = &mut events[0];
    event_init(e, 0x00002);

    e = &mut events[1];
    event_init(e, EventCode_1);

    e = &mut events[2];
    event_init(e, EventCode_2);

    e = &mut events[3];
    event_init(e, EventCode_3);

    e = &mut events[4];
    event_init(e, EventCode_4);

    FAIL_IF!(event_open(&mut events[0]));

    /*
     * Expected to pass since 0x00020 has alternative event
     * 0x500fa in PMC5. So it can go in with other events
     * in PMC1 to PMC4.
     */
    i = 1;
    while i < 5 {
        FAIL_IF!(event_open_with_group(
            &mut events[i as usize],
            events[0].fd,
        ));
        i += 1;
    }

    i = 0;
    while i < 5 {
        event_close(&mut events[i as usize]);
        i += 1;
    }

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    return test_harness(
        event_alternatives_tests_p10,
        b"event_alternatives_tests_p10\0".as_ptr() as *const c_char,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
