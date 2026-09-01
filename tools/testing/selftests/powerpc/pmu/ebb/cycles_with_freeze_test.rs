// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * Original C dependencies:
 *   #include <stdio.h>
 *   #include <stdlib.h>
 *   #include <stdbool.h>
 *   #include "ebb.h"
 */

use core::ffi::{c_char, c_int, c_ulong};

/*
 * Test of counting cycles while using MMCR0_FC (freeze counters) to only count
 * parts of the code. This is complicated by the fact that FC is set by the
 * hardware when the event overflows. We may take the EBB after we have set FC,
 * so we have to be careful about whether we clear FC at the end of the EBB
 * handler or not.
 */

#[repr(C)]
pub struct event_attr {
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
pub struct event {
    pub attr: event_attr,
}

#[repr(C)]
pub struct ebb_stats {
    pub spurious: u64,
    pub ebb_count: u64,
}

#[repr(C)]
pub struct ebb_state_struct {
    pub stats: ebb_stats,
    pub trace: *mut core::ffi::c_void,
}

unsafe extern "C" {
    static mut ebb_state: ebb_state_struct;
    static sample_period: u64;

    static MMCR0_PMAO: u64;
    static MMCR0_FC: u64;
    static SPRN_BESCR: c_int;
    static SPRN_MMCR0: c_int;
    static SPRN_PMC1: c_int;
    static BESCR_PMEO: u64;

    fn mfspr(spr: c_int) -> u64;
    fn mtspr(spr: c_int, val: u64);
    fn trace_log_counter(trace: *mut core::ffi::c_void, val: u64);
    fn trace_log_reg(trace: *mut core::ffi::c_void, reg: c_int, val: u64);
    fn trace_log_string(trace: *mut core::ffi::c_void, string: *const c_char);
    fn count_pmc(pmc: c_int, period: u64);
    fn reset_ebb_with_clear_mask(mask: u64);
    fn ebb_is_supported() -> bool;
    fn event_init_named(event: *mut event, code: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn pmc_sample_period(period: u64) -> u64;
    fn mb();
    fn core_busy_loop() -> c_int;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_ebb_state();
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

static mut counters_frozen: bool = false;
static mut ebbs_while_frozen: c_int = 0;

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

unsafe extern "C" fn ebb_callee() {
    let mut mask: u64;
    let mut val: u64;

    unsafe {
        mask = MMCR0_PMAO | MMCR0_FC;

        val = mfspr(SPRN_BESCR);
        if !(val & BESCR_PMEO != 0) {
            ebb_state.stats.spurious += 1;
            reset_ebb_with_clear_mask(mask);
            return;
        }

        ebb_state.stats.ebb_count += 1;
        trace_log_counter(ebb_state.trace, ebb_state.stats.ebb_count);

        val = mfspr(SPRN_MMCR0);
        trace_log_reg(ebb_state.trace, SPRN_MMCR0, val);

        if counters_frozen {
            trace_log_string(ebb_state.trace, c"frozen".as_ptr());
            ebbs_while_frozen += 1;
            mask &= !MMCR0_FC;
        }

        count_pmc(1, sample_period);
        reset_ebb_with_clear_mask(mask);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cycles_with_freeze() -> c_int {
    let mut event = event {
        attr: event_attr {
            exclude_kernel: 0,
            exclude_hv: 0,
            exclude_idle: 0,
        },
    };
    let mut val: u64;
    let mut fc_cleared: bool;

    unsafe {
        SKIP_IF!(!ebb_is_supported());

        event_init_named(&mut event, 0x1001e, c"cycles".as_ptr());
        event_leader_ebb_init(&mut event);

        event.attr.exclude_kernel = 1;
        event.attr.exclude_hv = 1;
        event.attr.exclude_idle = 1;

        FAIL_IF!(event_open(&mut event) != 0);

        setup_ebb_handler(ebb_callee);
        ebb_global_enable();
        FAIL_IF!(ebb_event_enable(&mut event) != 0);

        mtspr(SPRN_PMC1, pmc_sample_period(sample_period));

        fc_cleared = false;

        /* Make sure we loop until we take at least one EBB */
        while (ebb_state.stats.ebb_count < 20 && !fc_cleared) || ebb_state.stats.ebb_count < 1 {
            counters_frozen = false;
            mb();
            mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & !MMCR0_FC);

            FAIL_IF!(core_busy_loop() != 0);

            counters_frozen = true;
            mb();
            mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) | MMCR0_FC);

            val = mfspr(SPRN_MMCR0);
            if !(val & MMCR0_FC != 0) {
                printf(
                    c"Outside of loop, FC NOT set MMCR0 0x%lx\n".as_ptr(),
                    val as c_ulong,
                );
                fc_cleared = true;
            }
        }

        ebb_global_disable();
        ebb_freeze_pmcs();

        dump_ebb_state();

        printf(c"EBBs while frozen %d\n".as_ptr(), ebbs_while_frozen);

        event_close(&mut event);

        FAIL_IF!(ebb_state.stats.ebb_count == 0);
        FAIL_IF!(fc_cleared);

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { test_harness(cycles_with_freeze, c"cycles_with_freeze".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
