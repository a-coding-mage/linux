// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C dependencies: <stdio.h>, <stdlib.h>, <stdbool.h>, "ebb.h"

/*
 * Test of counting cycles while manipulating the user accessible bits in MMCR2.
 */

/* We use two values because the first freezes PMC1 and so we would get no EBBs */
const MMCR2_EXPECTED_1: u64 = 0x4020100804020000; /* (FC1P|FC2P|FC3P|FC4P|FC5P|FC6P) */
const MMCR2_EXPECTED_2: u64 = 0x0020100804020000; /* (     FC2P|FC3P|FC4P|FC5P|FC6P) */

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
    pub ebb_count: u64,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

unsafe extern "C" {
    static mut ebb_state: ebb_state_t;
    static sample_period: u64;

    static standard_ebb_callee: unsafe extern "C" fn();

    fn printf(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;

    fn ebb_is_supported() -> bool;
    fn event_init_named(event: *mut event, config: u64, name: *const core::ffi::c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> core::ffi::c_int;
    fn ebb_enable_pmc_counting(pmc: core::ffi::c_int);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> core::ffi::c_int;
    fn pmc_sample_period(period: u64) -> u64;
    fn mtspr(spr: core::ffi::c_int, value: u64);
    fn core_busy_loop() -> core::ffi::c_int;
    fn mfspr(spr: core::ffi::c_int) -> u64;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_ebb_state();
    fn event_close(event: *mut event);
    fn test_harness(
        test: unsafe extern "C" fn() -> core::ffi::c_int,
        name: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

unsafe extern "C" {
    static SPRN_PMC1: core::ffi::c_int;
    static SPRN_MMCR2: core::ffi::c_int;
}

macro_rules! FAIL_IF {
    ($condition:expr) => {
        if $condition {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition {
            return 4;
        }
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cycles_with_mmcr2() -> core::ffi::c_int {
    let mut event = core::mem::MaybeUninit::<event>::uninit();
    let mut val: u64;
    let mut expected: [u64; 2] = [0; 2];
    let mut actual: u64;
    let mut i: core::ffi::c_int;
    let mut bad_mmcr2: bool;

    SKIP_IF!(!unsafe { ebb_is_supported() });

    unsafe {
        event_init_named(event.as_mut_ptr(), 0x1001e, c"cycles".as_ptr());
        event_leader_ebb_init(event.as_mut_ptr());
    }

    let event = event.as_mut_ptr();

    unsafe {
        (*event).attr.exclude_kernel = 1;
        (*event).attr.exclude_hv = 1;
        (*event).attr.exclude_idle = 1;
    }

    FAIL_IF!(unsafe { event_open(event) != 0 });

    unsafe {
        ebb_enable_pmc_counting(1);
        setup_ebb_handler(standard_ebb_callee);
        ebb_global_enable();
    }

    FAIL_IF!(unsafe { ebb_event_enable(event) != 0 });

    unsafe {
        mtspr(SPRN_PMC1, pmc_sample_period(sample_period));
    }

    /* XXX Set of MMCR2 must be after enable */
    expected[0] = MMCR2_EXPECTED_1;
    expected[1] = MMCR2_EXPECTED_2;
    i = 0;
    bad_mmcr2 = false;
    actual = 0;

    /* Make sure we loop until we take at least one EBB */
    while (unsafe { ebb_state.stats.ebb_count } < 20 && !bad_mmcr2)
        || unsafe { ebb_state.stats.ebb_count } < 1
    {
        unsafe {
            mtspr(SPRN_MMCR2, expected[(i % 2) as usize]);
        }

        FAIL_IF!(unsafe { core_busy_loop() != 0 });

        val = unsafe { mfspr(SPRN_MMCR2) };
        if val != expected[(i % 2) as usize] {
            bad_mmcr2 = true;
            actual = val;
        }

        i += 1;
    }

    unsafe {
        ebb_global_disable();
        ebb_freeze_pmcs();

        dump_ebb_state();

        event_close(event);
    }

    FAIL_IF!(unsafe { ebb_state.stats.ebb_count == 0 });

    if bad_mmcr2 {
        unsafe {
            printf(c"Bad MMCR2 value seen is 0x%lx\n".as_ptr(), actual);
        }
    }

    FAIL_IF!(bad_mmcr2);

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> core::ffi::c_int {
    unsafe { test_harness(cycles_with_mmcr2, c"cycles_with_mmcr2".as_ptr()) }
}
