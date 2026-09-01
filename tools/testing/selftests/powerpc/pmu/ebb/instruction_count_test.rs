// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// Original C dependencies:
// #define _GNU_SOURCE
// #include <stdio.h>
// #include <stdbool.h>
// #include <string.h>
// #include <sys/prctl.h>
// #include "ebb.h"

use core::ffi::{c_char, c_double, c_int, c_ulong, c_ulonglong};

/*
 * Run a calibrated instruction loop and count instructions executed using
 * EBBs. Make sure the counts look right.
 */

#[repr(C)]
pub struct event_attr {
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
    pub exclude_idle: u64,
}

#[repr(C)]
pub struct event_result {
    pub value: u64,
}

#[repr(C)]
pub struct event {
    pub attr: event_attr,
    pub result: event_result,
}

#[repr(C)]
pub struct ebb_stats {
    pub pmc_count: [u64; 6],
    pub ebb_count: c_int,
    pub spurious: u64,
}

#[repr(C)]
pub struct ebb_state_t {
    pub stats: ebb_stats,
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;

    fn thirty_two_instruction_loop(loops: u64);

    static mut sample_period: u64;
    static mut ebb_state: ebb_state_t;

    static SPRN_MMCR0: c_int;
    static SPRN_BESCR: c_int;
    static MMCR0_FC: u64;
    static MMCR0_PMAO: u64;
    static BESCR_PMEO: u64;
    static COUNTER_OVERFLOW: u64;

    fn clear_ebb_stats();
    fn mb();
    fn mtspr(spr: c_int, val: u64);
    fn mfspr(spr: c_int) -> u64;
    fn count_pmc(pmc: c_int, period: u64);
    fn reset_ebb_with_clear_mask(mask: u64);
    fn reset_ebb();
    fn ebb_is_supported() -> bool;
    fn event_init_named(event: *mut event, config: u64, name: *const c_char);
    fn event_leader_ebb_init(event: *mut event);
    fn event_open(event: *mut event) -> c_int;
    fn ebb_event_enable(event: *mut event) -> c_int;
    fn setup_ebb_handler(handler: unsafe extern "C" fn());
    fn ebb_global_enable();
    fn ebb_global_disable();
    fn event_close(event: *mut event);
    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
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
        if $cond != 0 {
            return -1;
        }
    };
}

static mut counters_frozen: bool = true;

unsafe fn do_count_loop(event: *mut event, instructions: u64, overhead: u64, report: bool) -> c_int {
    let mut difference: i64;
    let expected: i64;
    let percentage: c_double;

    unsafe {
        clear_ebb_stats();

        counters_frozen = false;
        mb();
        mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & !MMCR0_FC);

        thirty_two_instruction_loop(instructions >> 5);

        counters_frozen = true;
        mb();
        mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) | MMCR0_FC);

        count_pmc(4, sample_period);

        (*event).result.value = ebb_state.stats.pmc_count[4 - 1];
        expected = (instructions + overhead) as i64;
        difference = (*event).result.value as i64 - expected;
        percentage = difference as c_double / (*event).result.value as c_double * 100.0;

        if report {
            printf(
                c_str!("Looped for %lu instructions, overhead %lu\n"),
                instructions as c_ulong,
                overhead as c_ulong,
            );
            printf(c_str!("Expected %lu\n"), expected as c_ulong);
            printf(c_str!("Actual   %llu\n"), (*event).result.value as c_ulonglong);
            printf(
                c_str!("Delta    %ld, %f%%\n"),
                difference as c_ulong,
                percentage,
            );
            printf(c_str!("Took %d EBBs\n"), ebb_state.stats.ebb_count);
        }

        if difference < 0 {
            difference = -difference;
        }

        /* Tolerate a difference of up to 0.0001 % */
        difference *= 10000 * 100;
        if difference as u64 / (*event).result.value != 0 {
            return -1;
        }
    }

    0
}

/* Count how many instructions it takes to do a null loop */
unsafe fn determine_overhead(event: *mut event) -> u64 {
    let mut current: u64;
    let mut overhead: u64;
    let mut i: c_int;

    unsafe {
        do_count_loop(event, 0, 0, false);
        overhead = (*event).result.value;

        i = 0;
        while i < 100 {
            do_count_loop(event, 0, 0, false);
            current = (*event).result.value;
            if current < overhead {
                printf(
                    c_str!("Replacing overhead %lu with %lu\n"),
                    overhead as c_ulong,
                    current as c_ulong,
                );
                overhead = current;
            }
            i += 1;
        }
    }

    overhead
}

unsafe extern "C" fn pmc4_ebb_callee() {
    let val: u64;

    unsafe {
        val = mfspr(SPRN_BESCR);
        if !(val & BESCR_PMEO != 0) {
            ebb_state.stats.spurious += 1;
        } else {
            ebb_state.stats.ebb_count += 1;
            count_pmc(4, sample_period);
        }

        if counters_frozen {
            reset_ebb_with_clear_mask(MMCR0_PMAO);
        } else {
            reset_ebb();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn instruction_count() -> c_int {
    let mut event: event = unsafe { core::mem::zeroed() };
    let overhead: u64;

    unsafe {
        SKIP_IF!(!ebb_is_supported());

        event_init_named(&mut event, 0x400FA, c_str!("PM_RUN_INST_CMPL"));
        event_leader_ebb_init(&mut event);
        event.attr.exclude_kernel = 1;
        event.attr.exclude_hv = 1;
        event.attr.exclude_idle = 1;

        FAIL_IF!(event_open(&mut event));
        FAIL_IF!(ebb_event_enable(&mut event));

        sample_period = COUNTER_OVERFLOW;

        setup_ebb_handler(pmc4_ebb_callee);
        mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & !MMCR0_FC);
        ebb_global_enable();

        overhead = determine_overhead(&mut event);
        printf(
            c_str!("Overhead of null loop: %lu instructions\n"),
            overhead as c_ulong,
        );

        /* Run for 1M instructions */
        FAIL_IF!(do_count_loop(&mut event, 0x100000, overhead, true));

        /* Run for 10M instructions */
        FAIL_IF!(do_count_loop(&mut event, 0xa00000, overhead, true));

        /* Run for 100M instructions */
        FAIL_IF!(do_count_loop(&mut event, 0x6400000, overhead, true));

        /* Run for 1G instructions */
        FAIL_IF!(do_count_loop(&mut event, 0x40000000, overhead, true));

        /* Run for 16G instructions */
        FAIL_IF!(do_count_loop(&mut event, 0x400000000, overhead, true));

        /* Run for 64G instructions */
        FAIL_IF!(do_count_loop(&mut event, 0x1000000000, overhead, true));

        /* Run for 128G instructions */
        FAIL_IF!(do_count_loop(&mut event, 0x2000000000, overhead, true));

        ebb_global_disable();
        event_close(&mut event);

        printf(c_str!("Finished OK\n"));
    }

    0
}

fn main() {
    unsafe {
        test_harness_set_timeout(300);
        std::process::exit(test_harness(instruction_count, c_str!("instruction_count")));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
