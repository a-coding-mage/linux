// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// Dependencies from "ebb.h" are declared here as external symbols.

#[repr(C)]
pub struct event {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ebb_stats {
    pub ebb_count: i32,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

unsafe extern "C" {
    static mut ebb_state: ebb_state_t;

    static standard_ebb_callee: unsafe extern "C" fn();
    static write_pmc1: unsafe extern "C" fn();

    static SPRN_PMC1: i32;
    static SPRN_EBBHR: i32;
    static SPRN_EBBRR: i32;
    static SPRN_BESCR: i32;
    static sample_period: u64;

    fn ebb_is_supported() -> i32;
    fn event_init_named(event: *mut event, event_code: u32, name: *const i8);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> i32;
    fn ebb_enable_pmc_counting(pmc: i32);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> i32;
    fn mtspr(spr: i32, value: u64);
    fn pmc_sample_period(period: u64) -> u64;
    fn core_busy_loop() -> i32;
    fn ebb_global_disable();
    fn event_close(event: *mut event);
    fn catch_sigill(fn_: unsafe extern "C" fn()) -> i32;
    fn mfspr(spr: i32) -> u64;
    fn test_harness(test: unsafe extern "C" fn() -> i32, name: *const i8) -> i32;

    fn SKIP_IF(condition: bool);
    fn FAIL_IF(condition: bool);
}

/*
 * Test that closing the EBB event clears MMCR0_PMCC, preventing further access
 * by userspace to the PMU hardware.
 */

pub unsafe extern "C" fn close_clears_pmcc() -> i32 {
    let mut event: event = event { _unused: [] };

    unsafe {
        SKIP_IF(ebb_is_supported() == 0);

        event_init_named(&mut event, 0x1001e, c"cycles".as_ptr());
        event_leader_ebb_init(&mut event);

        FAIL_IF(event_open(&mut event) != 0);

        ebb_enable_pmc_counting(1);
        setup_ebb_handler(standard_ebb_callee);
        ebb_global_enable();
        FAIL_IF(ebb_event_enable(&mut event) != 0);

        mtspr(SPRN_PMC1, pmc_sample_period(sample_period));

        while ebb_state.stats.ebb_count < 1 {
            FAIL_IF(core_busy_loop() != 0);
        }

        ebb_global_disable();
        event_close(&mut event);

        FAIL_IF(ebb_state.stats.ebb_count == 0);

        /* The real test is here, do we take a SIGILL when writing PMU regs now
         * that we have closed the event. We expect that we will. */

        FAIL_IF(catch_sigill(write_pmc1) != 0);

        /* We should still be able to read EBB regs though */
        mfspr(SPRN_EBBHR);
        mfspr(SPRN_EBBRR);
        mfspr(SPRN_BESCR);
    }

    0
}

pub unsafe extern "C" fn main() -> i32 {
    unsafe { test_harness(close_clears_pmcc, c"close_clears_pmcc".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
