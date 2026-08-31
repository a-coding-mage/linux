// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// Rust translation of dependencies supplied by "ebb.h" and system headers.
// The concrete definitions are expected to be provided by the surrounding test
// harness/bindings when this file is integrated.

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

const NUMBER_OF_EBBS: u64 = 50;

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

#[repr(C)]
pub struct ebb_stats {
    pub spurious: u64,
    pub ebb_count: u64,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
    pub trace: *mut c_void,
}

unsafe extern "C" {
    static mut ebb_state: ebb_state_t;
    static mut sample_period: u64;

    static SPRN_BESCR: c_ulonglong;
    static SPRN_SIAR: c_ulonglong;
    static SPRN_PMC1: c_ulonglong;
    static SPRN_MMCR0: c_ulonglong;
    static BESCR_PMEO: u64;
    static MMCR0_PMAO: u64;

    fn mfspr(spr: c_ulonglong) -> u64;
    fn mtspr(spr: c_ulonglong, val: u64);
    fn trace_log_counter(trace: *mut c_void, value: u64);
    fn trace_log_reg(trace: *mut c_void, reg: c_ulonglong, value: u64);
    fn count_pmc(pmc: c_int, period: u64);
    fn reset_ebb_with_clear_mask(mask: u64);
    fn reset_ebb();
    fn ebb_is_supported() -> bool;
    fn event_init_named(event: *mut event, config: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn pmc_sample_period(period: u64) -> u64;
    fn ebb_freeze_pmcs();
    fn ebb_global_enable();
    fn ebb_unfreeze_pmcs();
    fn core_busy_loop() -> c_int;
    fn ebb_global_disable();
    fn dump_ebb_state();
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

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
            return 4;
        }
    };
}

/*
 * Test that if we overflow the counter while in the EBB handler, we take
 * another EBB on exiting from the handler.
 *
 * We do this by counting with a stupidly low sample period, causing us to
 * overflow the PMU while we're still in the EBB handler, leading to another
 * EBB.
 *
 * We get out of what would otherwise be an infinite loop by leaving the
 * counter frozen once we've taken enough EBBs.
 */

unsafe extern "C" fn ebb_callee() {
    let siar: u64;
    let mut val: u64;

    val = unsafe { mfspr(SPRN_BESCR) };
    if (val & unsafe { BESCR_PMEO }) == 0 {
        unsafe {
            ebb_state.stats.spurious += 1;
        }
        // goto out;
    } else {
        unsafe {
            ebb_state.stats.ebb_count += 1;
            trace_log_counter(ebb_state.trace, ebb_state.stats.ebb_count);
        }

        /* Resets the PMC */
        unsafe {
            count_pmc(1, sample_period);
        }
    }

    if unsafe { ebb_state.stats.ebb_count } == NUMBER_OF_EBBS {
        /* Reset but leave counters frozen */
        unsafe {
            reset_ebb_with_clear_mask(MMCR0_PMAO);
        }
    } else {
        /* Unfreezes */
        unsafe {
            reset_ebb();
        }
    }

    /* Do some stuff to chew some cycles and pop the counter */
    siar = unsafe { mfspr(SPRN_SIAR) };
    unsafe {
        trace_log_reg(ebb_state.trace, SPRN_SIAR, siar);
    }

    val = unsafe { mfspr(SPRN_PMC1) };
    unsafe {
        trace_log_reg(ebb_state.trace, SPRN_PMC1, val);
    }

    val = unsafe { mfspr(SPRN_MMCR0) };
    unsafe {
        trace_log_reg(ebb_state.trace, SPRN_MMCR0, val);
    }
}

unsafe extern "C" fn back_to_back_ebbs() -> c_int {
    let mut event = event {
        attr: perf_event_attr {
            exclude_kernel: 0,
            exclude_hv: 0,
            exclude_idle: 0,
        },
    };

    SKIP_IF!(!unsafe { ebb_is_supported() });

    unsafe {
        event_init_named(&mut event, 0x1001e, c"cycles".as_ptr());
        event_leader_ebb_init(&mut event);
    }

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    FAIL_IF!(unsafe { event_open(&mut event) != 0 });

    unsafe {
        setup_ebb_handler(ebb_callee);
    }

    FAIL_IF!(unsafe { ebb_event_enable(&mut event) != 0 });

    unsafe {
        sample_period = 5;

        ebb_freeze_pmcs();
        mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
        ebb_global_enable();
        ebb_unfreeze_pmcs();
    }

    while unsafe { ebb_state.stats.ebb_count } < NUMBER_OF_EBBS {
        FAIL_IF!(unsafe { core_busy_loop() != 0 });
    }

    unsafe {
        ebb_global_disable();
        ebb_freeze_pmcs();

        dump_ebb_state();

        event_close(&mut event);
    }

    FAIL_IF!(unsafe { ebb_state.stats.ebb_count != NUMBER_OF_EBBS });

    0
}

fn main() -> c_int {
    unsafe { test_harness(back_to_back_ebbs, c"back_to_back_ebbs".as_ptr()) }
}
