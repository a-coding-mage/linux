// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

/*
 * C dependencies removed from executable Rust:
 *   <stdio.h>, <stdlib.h>, "../event.h", "misc.h", "utils.h"
 * The declarations below name the external items this translation uses.
 */

/*
 * A perf sampling test to check bhrb filter
 * map. All the branch filters are not supported
 * in powerpc. Supported filters in:
 * power10/power11: any, any_call, ind_call, cond
 * power9: any, any_call
 *
 * Testcase checks event open for invalid bhrb filter
 * types should fail and valid filter types should pass.
 * Testcase does validity check for these branch
 * sample types.
 */

#[repr(C)]
pub struct perf_event_attr {
    pub sample_period: u64,
    pub sample_type: u64,
    pub disabled: u64,
    pub branch_sample_type: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
}

unsafe extern "C" {
    static PERF_SAMPLE_BRANCH_ANY: i32;
    static PERF_SAMPLE_BRANCH_ANY_CALL: i32;
    static PERF_SAMPLE_BRANCH_IND_CALL: i32;
    static PERF_SAMPLE_BRANCH_COND: i32;

    static PERF_SAMPLE_BRANCH_STACK: u64;
    static PERF_SAMPLE_BRANCH_USER_SHIFT: i32;
    static PERF_SAMPLE_BRANCH_MAX_SHIFT: i32;
    static PERF_SAMPLE_BRANCH_ANY_SHIFT: i32;
    static PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT: i32;
    static PERF_SAMPLE_BRANCH_IND_CALL_SHIFT: i32;
    static PERF_SAMPLE_BRANCH_COND_SHIFT: i32;

    static SPRN_PVR: u64;
    static POWER11: u64;
    static POWER10: u64;

    fn platform_check_for_tests() -> i32;
    fn check_for_generic_compat_pmu() -> i32;
    fn event_init(event: *mut event, code: u32);
    fn event_open(event: *mut event) -> i32;
    fn event_close(event: *mut event);
    fn mfspr(sprn: u64) -> u64;
    fn PVR_VER(pvr: u64) -> u64;
    fn test_harness(test: unsafe extern "C" fn() -> i32, name: *const core::ffi::c_char) -> i32;
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
            return -1;
        }
    };
}

/* Invalid types for powerpc */
/* Valid bhrb filters in power9/power10/power11 */
static mut bhrb_filter_map_valid_common: [i32; 2] = unsafe {
    [
        PERF_SAMPLE_BRANCH_ANY,
        PERF_SAMPLE_BRANCH_ANY_CALL,
    ]
};

/* Valid bhrb filters in power10/power11 */
static mut bhrb_filter_map_valid_p10: [i32; 2] = unsafe {
    [
        PERF_SAMPLE_BRANCH_IND_CALL,
        PERF_SAMPLE_BRANCH_COND,
    ]
};

const EventCode: u32 = 0x1001e;

unsafe extern "C" fn bhrb_filter_map_test() -> i32 {
    let mut event: event = core::mem::zeroed();
    let mut i: i32;

    /* Check for platform support for the test */
    SKIP_IF!(platform_check_for_tests());

    /*
     * Skip for Generic compat PMU since
     * bhrb filters is not supported
     */
    SKIP_IF!(check_for_generic_compat_pmu());

    /* Init the event for the sampling test */
    event_init(&mut event, EventCode);

    event.attr.sample_period = 1000;
    event.attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    event.attr.disabled = 1;

    /* Invalid filter maps which are expected to fail in event_open */
    i = PERF_SAMPLE_BRANCH_USER_SHIFT;
    while i < PERF_SAMPLE_BRANCH_MAX_SHIFT {
        /* Skip the valid branch sample type */
        if i == PERF_SAMPLE_BRANCH_ANY_SHIFT
            || i == PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT
            || i == PERF_SAMPLE_BRANCH_IND_CALL_SHIFT
            || i == PERF_SAMPLE_BRANCH_COND_SHIFT
        {
            i += 1;
            continue;
        }
        event.attr.branch_sample_type = 1u64 << i;
        FAIL_IF!(event_open(&mut event) == 0);
        i += 1;
    }

    /* valid filter maps for power9/power10/power11 which are expected to pass in event_open */
    i = 0;
    while i < bhrb_filter_map_valid_common.len() as i32 {
        event.attr.branch_sample_type = bhrb_filter_map_valid_common[i as usize] as u64;
        FAIL_IF!(event_open(&mut event) != 0);
        event_close(&mut event);
        i += 1;
    }

    /*
     * filter maps which are valid in power10/power11 and invalid in power9.
     * PVR check is used here since PMU specific data like bhrb filter
     * alternative tests is handled by respective PMU driver code and
     * using PVR will work correctly for all cases including generic
     * compat mode.
     */
    match PVR_VER(mfspr(SPRN_PVR)) {
        v if v == POWER11 || v == POWER10 => {
            i = 0;
            while i < bhrb_filter_map_valid_p10.len() as i32 {
                event.attr.branch_sample_type = bhrb_filter_map_valid_p10[i as usize] as u64;
                FAIL_IF!(event_open(&mut event) != 0);
                event_close(&mut event);
                i += 1;
            }
        }
        _ => {
            i = 0;
            while i < bhrb_filter_map_valid_p10.len() as i32 {
                event.attr.branch_sample_type = bhrb_filter_map_valid_p10[i as usize] as u64;
                FAIL_IF!(event_open(&mut event) == 0);
                i += 1;
            }
        }
    }

    /*
     * Combine filter maps which includes a valid branch filter and an invalid branch
     * filter. Example: any ( PERF_SAMPLE_BRANCH_ANY) and any_call
     * (PERF_SAMPLE_BRANCH_ANY_CALL).
     * The perf_event_open should fail in this case.
     */
    event.attr.branch_sample_type =
        (PERF_SAMPLE_BRANCH_ANY | PERF_SAMPLE_BRANCH_ANY_CALL) as u64;
    FAIL_IF!(event_open(&mut event) == 0);

    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    return test_harness(
        bhrb_filter_map_test,
        c"bhrb_filter_map_test".as_ptr(),
    );
}
