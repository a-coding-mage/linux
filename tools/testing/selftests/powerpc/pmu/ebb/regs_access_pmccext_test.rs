// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2021, Athira Rajeev, IBM Corp.
 */

// C dependencies translated from:
// #include <stdio.h>
// #include <stdlib.h>
// #include <setjmp.h>
// #include <signal.h>
// #include "ebb.h"

#[repr(C)]
pub struct event {
    // Defined by the external ebb/selftest support translated from "ebb.h".
    _private: [u8; 0],
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
    static mut sample_period: u64;
    static PPC_FEATURE2_ARCH_3_1: libc::c_ulong;
    static SPRN_PMC1: libc::c_int;

    fn ebb_is_supported() -> libc::c_int;
    fn event_init_named(event: *mut event, event_code: libc::c_uint, name: *const libc::c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> libc::c_int;
    fn ebb_enable_pmc_counting(value: libc::c_int);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn standard_ebb_callee();
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> libc::c_int;
    fn mtspr(spr: libc::c_int, value: u64);
    fn pmc_sample_period(sample_period: u64) -> u64;
    fn core_busy_loop() -> libc::c_int;
    fn ebb_global_disable();
    fn event_close(event: *mut event);
    fn have_hwcap2(feature: libc::c_ulong) -> libc::c_int;
    fn catch_sigill(function: unsafe extern "C" fn()) -> libc::c_int;
    fn dump_ebb_state();
    fn test_harness(
        test_function: unsafe extern "C" fn() -> libc::c_int,
        name: *const libc::c_char,
    ) -> libc::c_int;
}

macro_rules! FAIL_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition {
            return 0;
        }
    };
}

/*
 * Test that closing the EBB event clears MMCR0_PMCC and
 * sets MMCR0_PMCCEXT preventing further read access to the
 * group B PMU registers.
 */
unsafe extern "C" fn regs_access_pmccext() -> libc::c_int {
    let mut event = ::core::mem::MaybeUninit::<event>::uninit();
    let event = event.as_mut_ptr();

    SKIP_IF!(ebb_is_supported() == 0);

    event_init_named(event, 0x1001e, c"cycles".as_ptr());
    event_leader_ebb_init(event);

    FAIL_IF!(event_open(event));

    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    FAIL_IF!(ebb_event_enable(event));

    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));

    while ebb_state.stats.ebb_count < 1 {
        FAIL_IF!(core_busy_loop());
    }

    ebb_global_disable();
    event_close(event);

    FAIL_IF!((ebb_state.stats.ebb_count == 0) as libc::c_int);

    /*
     * For ISA v3.1, verify the test takes a SIGILL when reading
     * PMU regs after the event is closed. With the control bit
     * in MMCR0 (PMCCEXT) restricting access to group B PMU regs,
     * sigill is expected.
     */
    if have_hwcap2(PPC_FEATURE2_ARCH_3_1) != 0 {
        FAIL_IF!(catch_sigill(dump_ebb_state));
    } else {
        dump_ebb_state();
    }

    0
}

pub unsafe extern "C" fn main() -> libc::c_int {
    test_harness(regs_access_pmccext, c"regs_access_pmccext".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
