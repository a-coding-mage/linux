// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

/*
 * C includes translated as external dependencies:
 * sched.h, signal.h, stdbool.h, stdio.h, stdlib.h, and "ebb.h".
 */

use core::ffi::{c_char, c_int, c_ulong};
use core::mem::MaybeUninit;

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
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

unsafe extern "C" {
    static mut ebb_state: ebb_state_t;
    static sample_period: u64;

    static SPRN_BESCR: c_int;
    static SPRN_MMCR0: c_int;
    static SPRN_PMC1: c_int;
    static BESCR_PMEO: u64;

    fn mfspr(spr: c_int) -> u64;
    fn mtspr(spr: c_int, val: u64);
    fn count_pmc(pmc: c_int, sample_period: u64);
    fn reset_ebb();
    fn ebb_is_supported() -> bool;
    fn event_init_named(event: *mut event, code: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn pmc_sample_period(sample_period: u64) -> u64;
    fn core_busy_loop() -> c_int;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_ebb_state();
    fn event_close(event: *mut event);
    fn eat_cpu(function: unsafe extern "C" fn() -> c_int) -> c_int;
    fn test_harness(function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn sched_yield() -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

static mut MMCR0_MISMATCH: bool = false;
static mut BEFORE: u64 = 0;
static mut AFTER: u64 = 0;

unsafe extern "C" fn syscall_ebb_callee() {
    let val: u64;

    val = mfspr(SPRN_BESCR);
    if (val & BESCR_PMEO) == 0 {
        ebb_state.stats.spurious = ebb_state.stats.spurious.wrapping_add(1);
        reset_ebb();
        return;
    }

    ebb_state.stats.ebb_count = ebb_state.stats.ebb_count.wrapping_add(1);
    count_pmc(1, sample_period);

    BEFORE = mfspr(SPRN_MMCR0);

    /* Try and get ourselves scheduled, to force a PMU context switch */
    sched_yield();

    AFTER = mfspr(SPRN_MMCR0);
    if BEFORE != AFTER {
        MMCR0_MISMATCH = true;
    }

    reset_ebb();
}

unsafe fn skip_if(condition: bool) -> c_int {
    if condition {
        return 4;
    }

    0
}

unsafe fn fail_if(condition: bool) -> c_int {
    if condition {
        return 1;
    }

    0
}

unsafe extern "C" fn test_body() -> c_int {
    let mut event_uninit = MaybeUninit::<event>::uninit();
    let event = event_uninit.as_mut_ptr();

    let mut rc = skip_if(!ebb_is_supported());
    if rc != 0 {
        return rc;
    }

    event_init_named(event, 0x1001e, c"cycles".as_ptr());
    event_leader_ebb_init(event);

    (*event).attr.exclude_kernel = 1;
    (*event).attr.exclude_hv = 1;
    (*event).attr.exclude_idle = 1;

    rc = fail_if(event_open(event) != 0);
    if rc != 0 {
        return rc;
    }

    setup_ebb_handler(syscall_ebb_callee);
    ebb_global_enable();

    rc = fail_if(ebb_event_enable(event) != 0);
    if rc != 0 {
        return rc;
    }

    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));

    while ebb_state.stats.ebb_count < 20 && !MMCR0_MISMATCH {
        rc = fail_if(core_busy_loop() != 0);
        if rc != 0 {
            return rc;
        }
    }

    ebb_global_disable();
    ebb_freeze_pmcs();

    dump_ebb_state();

    if MMCR0_MISMATCH {
        printf(
            c"Saw MMCR0 before 0x%lx after 0x%lx\n".as_ptr(),
            BEFORE as c_ulong,
            AFTER as c_ulong,
        );
    }

    event_close(event);

    rc = fail_if(ebb_state.stats.ebb_count == 0);
    if rc != 0 {
        return rc;
    }

    rc = fail_if(MMCR0_MISMATCH);
    if rc != 0 {
        return rc;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pmae_handling() -> c_int {
    eat_cpu(test_body)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    test_harness(pmae_handling, c"pmae_handling".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
