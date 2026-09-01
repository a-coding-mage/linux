// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// Rust translation of dependencies from:
// #include <stdio.h>
// #include <stdlib.h>
// #include "ebb.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct perf_event_attr {
    pub exclude_kernel: c_ulong,
    pub exclude_hv: c_ulong,
    pub exclude_idle: c_ulong,
}

#[repr(C)]
pub struct ebb_stats {
    pub ebb_count: c_int,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

unsafe extern "C" {
    static mut ebb_state: ebb_state_t;
    static mut sample_period: u64;
    static SPRN_PMC1: c_int;

    fn ebb_is_supported() -> c_int;
    fn event_init_named(event: *mut event, event_code: c_ulong, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn ebb_enable_pmc_counting(pmc: c_int);
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn standard_ebb_callee();
    fn ebb_global_enable();
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn mtspr(spr: c_int, value: u64);
    fn pmc_sample_period(period: u64) -> u64;
    fn core_busy_loop() -> c_int;
    fn ebb_check_mmcr0() -> c_int;
    fn ebb_global_disable();
    fn ebb_freeze_pmcs();
    fn dump_ebb_state();
    fn event_close(event: *mut event);
    fn ebb_check_count(pmc: c_int, period: u64, tolerance: c_int) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/*
 * Basic test that counts user cycles and takes EBBs.
 */
unsafe extern "C" fn cycles() -> c_int {
    let mut event: event = core::mem::zeroed();

    if ebb_is_supported() == 0 {
        return 0;
    }

    event_init_named(&mut event, 0x1001e, c"cycles".as_ptr());
    event_leader_ebb_init(&mut event);

    event.attr.exclude_kernel = 1;
    event.attr.exclude_hv = 1;
    event.attr.exclude_idle = 1;

    if event_open(&mut event) != 0 {
        return 1;
    }

    ebb_enable_pmc_counting(1);
    setup_ebb_handler(standard_ebb_callee);
    ebb_global_enable();
    if ebb_event_enable(&mut event) != 0 {
        return 1;
    }

    mtspr(SPRN_PMC1, pmc_sample_period(sample_period));

    while ebb_state.stats.ebb_count < 10 {
        if core_busy_loop() != 0 {
            return 1;
        }
        if ebb_check_mmcr0() != 0 {
            return 1;
        }
    }

    ebb_global_disable();
    ebb_freeze_pmcs();

    dump_ebb_state();

    event_close(&mut event);

    if ebb_state.stats.ebb_count == 0 {
        return 1;
    }
    if ebb_check_count(1, sample_period, 100) == 0 {
        return 1;
    }

    0
}

fn main() -> c_int {
    unsafe { test_harness(cycles, c"cycles".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
