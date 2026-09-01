// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

use core::ffi::{c_char, c_int};

extern "C" {
    static PERF_TYPE_SOFTWARE: u32;
    static PERF_SAMPLE_BRANCH_STACK: u64;

    fn event_init_opts(event: *mut event, config: u64, type_: u32, name: *const c_char);
    fn event_open(event: *mut event) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/*
 * The exact definitions of `struct event` and its nested attr type are supplied
 * by the translated equivalents of ../event.h and related headers.
 */
#[repr(C)]
pub struct perf_event_attr {
    pub sample_period: u64,
    pub sample_type: u64,
    pub disabled: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
}

/*
 * A perf sampling test for making sure
 * enabling branch stack doesn't crash in any
 * environment, say:
 *  - With generic compat PMU
 *  - without any PMU registered
 *  - With platform specific PMU
 *  A fix for bhrb sampling crash was added in kernel
 *  via commit: b460b512417a ("powerpc/perf: Fix crashes
 *  with generic_compat_pmu & BHRB")
 *
 * This testcase exercises this code by doing branch
 * stack enable for software event. s/w event is used
 * since software event will work even in platform
 * without PMU.
 */
unsafe extern "C" fn bhrb_no_crash_wo_pmu_test() -> c_int {
    let mut event: event = core::mem::zeroed();

    /*
     * Init the event for the sampling test.
     * This uses software event which works on
     * any platform.
     */
    event_init_opts(
        &mut event,
        0,
        PERF_TYPE_SOFTWARE,
        b"cycles\0".as_ptr() as *const c_char,
    );

    event.attr.sample_period = 1000;
    event.attr.sample_type = PERF_SAMPLE_BRANCH_STACK;
    event.attr.disabled = 1;

    /*
     * Return code of event_open is not
     * considered since test just expects no crash from
     * using PERF_SAMPLE_BRANCH_STACK. Also for environment
     * like generic compat PMU, branch stack is unsupported.
     */
    event_open(&mut event);

    event_close(&mut event);
    0
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    test_harness(
        bhrb_no_crash_wo_pmu_test,
        b"bhrb_no_crash_wo_pmu_test\0".as_ptr() as *const c_char,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
