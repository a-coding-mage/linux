// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: stdio.h, stdlib.h, setjmp.h, signal.h, "ebb.h"

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn ebb_is_supported() -> c_int;
    fn event_init_named(event: *mut event, event_code: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn mfspr(sprn: c_int) -> u64;
    fn mtspr(sprn: c_int, val: u64);
    fn pmc_sample_period(period: u64) -> u64;
    fn mb();
    fn dump_ebb_state();
    fn event_close(event: *mut event);
    fn test_harness(test: Option<unsafe extern "C" fn() -> c_int>, name: *const c_char) -> c_int;

    static SPRN_EBBHR: c_int;
    static SPRN_PMC1: c_int;
    static SPRN_MMCR0: c_int;
    static mut sample_period: u64;
}

/* Test that things work sanely if we have no handler */

unsafe extern "C" fn no_handler_test() -> c_int {
    let mut event: event = unsafe { core::mem::zeroed() };
    let mut val: u64;
    let mut i: c_int;

    SKIP_IF!(unsafe { ebb_is_supported() == 0 });

    unsafe { event_init_named(&mut event, 0x1001e, c"cycles".as_ptr()) };
    unsafe { event_leader_ebb_init(&mut event) };

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    FAIL_IF!(unsafe { event_open(&mut event) != 0 });
    FAIL_IF!(unsafe { ebb_event_enable(&mut event) != 0 });

    val = unsafe { mfspr(SPRN_EBBHR) };
    FAIL_IF!(val != 0);

    /* Make sure it overflows quickly */
    unsafe {
        sample_period = 1000;
        mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
    }

    /* Spin to make sure the event has time to overflow */
    i = 0;
    while i < 1000 {
        unsafe { mb() };
        i += 1;
    }

    unsafe { dump_ebb_state() };

    /* We expect to see the PMU frozen & PMAO set */
    val = unsafe { mfspr(SPRN_MMCR0) };
    FAIL_IF!(val != 0x0000000080000080);

    unsafe { event_close(&mut event) };

    /* The real test is that we never took an EBB at 0x0 */

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { test_harness(Some(no_handler_test), c"no_handler_test".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
