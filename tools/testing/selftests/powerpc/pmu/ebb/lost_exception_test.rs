// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * C dependencies:
 *   <sched.h>
 *   <signal.h>
 *   <stdio.h>
 *   <stdlib.h>
 *   <sys/mman.h>
 *   "ebb.h"
 */

use core::ffi::{c_char, c_int, c_ulong};

const SPRN_PMC4: c_int = 4;

#[repr(C)]
pub struct perf_event_attr {
    pub exclude_kernel: c_ulong,
    pub exclude_hv: c_ulong,
    pub exclude_idle: c_ulong,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct ebb_stats {
    pub ebb_count: c_int,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

extern "C" {
    static mut sample_period: c_int;
    static mut ebb_state: ebb_state_t;

    static standard_ebb_callee: unsafe extern "C" fn();

    fn sched_yield() -> c_int;

    fn ebb_is_supported() -> c_int;
    fn event_init_named(event: *mut event, config: c_ulong, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn ebb_enable_pmc_counting(pmc: c_int);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn mtspr(spr: c_int, value: c_ulong);
    fn pmc_sample_period(period: c_int) -> c_ulong;
    fn ebb_freeze_pmcs();
    fn ebb_global_disable();
    fn dump_summary_ebb_state();
    fn dump_ebb_hw_state();
    fn event_close(event: *mut event);
    fn ebb_check_count(pmc: c_int, period: c_int, fudge: c_int) -> c_int;
    fn eat_cpu(test: unsafe extern "C" fn() -> c_int) -> c_int;
    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn SKIP_IF(condition: c_int);
    fn FAIL_IF(condition: c_int);
}

/*
 * Test that tries to trigger CPU_FTR_PMAO_BUG. Which is a hardware defect
 * where an exception triggers but we context switch before it is delivered and
 * lose the exception.
 */

unsafe extern "C" fn test_body() -> c_int {
    let mut i: c_int;
    let orig_period: c_int;
    let mut max_period: c_int;
    let mut event: event = core::mem::zeroed();

    SKIP_IF((ebb_is_supported() == 0) as c_int);

    /* We use PMC4 to make sure the kernel switches all counters correctly */
    event_init_named(&mut event, 0x40002, c"instructions".as_ptr());
    event_leader_ebb_init(&mut event);

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    FAIL_IF(event_open(&mut event));

    ebb_enable_pmc_counting(4);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    FAIL_IF(ebb_event_enable(&mut event));

    /*
     * We want a low sample period, but we also want to get out of the EBB
     * handler without tripping up again.
     *
     * This value picked after much experimentation.
     */
    sample_period = 400;
    max_period = sample_period;
    orig_period = max_period;

    mtspr(SPRN_PMC4, pmc_sample_period(sample_period));

    while ebb_state.stats.ebb_count < 1000000 {
        /*
         * We are trying to get the EBB exception to race exactly with
         * us entering the kernel to do the syscall. We then need the
         * kernel to decide our timeslice is up and context switch to
         * the other thread. When we come back our EBB will have been
         * lost and we'll spin in this while loop forever.
         */

        i = 0;
        while i < 100000 {
            sched_yield();
            i += 1;
        }

        /* Change the sample period slightly to try and hit the race */
        if sample_period >= orig_period + 200 {
            sample_period = orig_period;
        } else {
            sample_period += 1;
        }

        if sample_period > max_period {
            max_period = sample_period;
        }
    }

    ebb_freeze_pmcs();
    ebb_global_disable();

    mtspr(SPRN_PMC4, 0xdead);

    dump_summary_ebb_state();
    dump_ebb_hw_state();

    event_close(&mut event);

    FAIL_IF((ebb_state.stats.ebb_count == 0) as c_int);

    /* We vary our sample period so we need extra fudge here */
    FAIL_IF((ebb_check_count(4, orig_period, 2 * (max_period - orig_period)) == 0) as c_int);

    0
}

unsafe extern "C" fn lost_exception() -> c_int {
    eat_cpu(test_body)
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    test_harness_set_timeout(300);
    test_harness(lost_exception, c"lost_exception".as_ptr())
}
