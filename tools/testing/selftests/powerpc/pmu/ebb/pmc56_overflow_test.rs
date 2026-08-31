// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: <stdio.h>, <stdlib.h>, and "ebb.h".

use core::ffi::{c_char, c_int, c_ulonglong};

/*
 * Test that PMC5 & 6 are frozen (ie. don't overflow) when they are not being
 * used. Tests the MMCR0_FC56 logic in the kernel.
 */

static mut pmc56_overflowed: c_int = 0;

extern "C" {
    static mut ebb_state: EbbState;
    static sample_period: u64;

    static SPRN_BESCR: c_int;
    static SPRN_PMC2: c_int;
    static SPRN_PMC5: c_int;
    static SPRN_PMC6: c_int;
    static BESCR_PMEO: u64;
    static COUNTER_OVERFLOW: u64;

    fn mfspr(sprn: c_int) -> u64;
    fn mtspr(sprn: c_int, val: u64);
    fn count_pmc(pmc: c_int, value: u64);
    fn reset_ebb();
    fn ebb_is_supported() -> c_int;
    fn event_init(event: *mut event, config: c_ulonglong);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn setup_ebb_handler(handler: extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn pmc_sample_period(period: u64) -> u64;
    fn core_busy_loop() -> c_int;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_ebb_state();
    fn printf(format: *const c_char, ...) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

#[repr(C)]
pub struct EbbStats {
    pub spurious: c_int,
    pub ebb_count: c_int,
}

#[repr(C)]
pub struct EbbState {
    pub stats: EbbStats,
}

#[repr(C)]
pub struct perf_event_attr {
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
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

extern "C" fn ebb_callee() {
    let mut val: u64;

    unsafe {
        val = mfspr(SPRN_BESCR);
        if (val & BESCR_PMEO) == 0 {
            ebb_state.stats.spurious += 1;
            reset_ebb();
            return;
        }

        ebb_state.stats.ebb_count += 1;
        count_pmc(2, sample_period);

        val = mfspr(SPRN_PMC5);
        if val >= COUNTER_OVERFLOW {
            pmc56_overflowed += 1;
        }

        count_pmc(5, COUNTER_OVERFLOW);

        val = mfspr(SPRN_PMC6);
        if val >= COUNTER_OVERFLOW {
            pmc56_overflowed += 1;
        }

        count_pmc(6, COUNTER_OVERFLOW);

        reset_ebb();
    }
}

#[no_mangle]
pub extern "C" fn pmc56_overflow() -> c_int {
    unsafe {
        let mut event: event = core::mem::zeroed();

        SKIP_IF!(ebb_is_supported() == 0);

        /* Use PMC2 so we set PMCjCE, which enables PMC5/6 */
        event_init(&mut event, 0x2001e);
        event_leader_ebb_init(&mut event);

        event.attr.exclude_kernel = 1;
        event.attr.exclude_hv = 1;
        event.attr.exclude_idle = 1;

        FAIL_IF!(event_open(&mut event) != 0);

        setup_ebb_handler(ebb_callee);
        ebb_global_enable();

        FAIL_IF!(ebb_event_enable(&mut event) != 0);

        mtspr(SPRN_PMC2, pmc_sample_period(sample_period));
        mtspr(SPRN_PMC5, 0);
        mtspr(SPRN_PMC6, 0);

        while ebb_state.stats.ebb_count < 10 {
            FAIL_IF!(core_busy_loop() != 0);
        }

        ebb_global_disable();
        ebb_freeze_pmcs();

        dump_ebb_state();

        printf(
            b"PMC5/6 overflow %d\n\0".as_ptr() as *const c_char,
            pmc56_overflowed,
        );

        event_close(&mut event);

        FAIL_IF!(ebb_state.stats.ebb_count == 0 || pmc56_overflowed != 0);

        0
    }
}

fn main() -> c_int {
    unsafe { test_harness(pmc56_overflow, b"pmc56_overflow\0".as_ptr() as *const c_char) }
}
