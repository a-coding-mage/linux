/*
 * Copyright 2013, Michael Ellerman, IBM Corp.
 * Licensed under GPLv2.
 */

// C dependencies removed from executable Rust:
// stdio.h, stdbool.h, string.h, sys/prctl.h, event.h, utils.h, lib.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_double, c_int, c_ulong};

type u64 = u64;
type s64 = i64;

const PR_TASK_PERF_EVENTS_ENABLE: c_int = 32;
const PR_TASK_PERF_EVENTS_DISABLE: c_int = 33;

const PERF_TYPE_HARDWARE: c_int = 0;
const PERF_TYPE_RAW: c_int = 4;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;

const PPC_FEATURE2_ARCH_2_07: c_ulong = 0x80000000;

#[repr(C)]
pub struct perf_event_attr {
    pub disabled: u64,
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
    pub attr: perf_event_attr,
    pub result: event_result,
    pub fd: c_int,
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn prctl(option: c_int, ...) -> c_int;

    fn thirty_two_instruction_loop_with_ll_sc(loops: u64, ll_sc_target: *mut u64);

    fn event_init_opts(e: *mut event, config: u64, type_: c_int, name: *mut c_char);
    fn event_read(e: *mut event);
    fn event_report(e: *mut event);
    fn event_reset(e: *mut event);
    fn event_open(e: *mut event) -> c_int;
    fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    fn event_close(e: *mut event);

    fn have_hwcap2(feature: c_ulong) -> bool;
    fn eat_cpu(test: unsafe extern "C" fn() -> c_int) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return -1;
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

unsafe fn setup_event(e: *mut event, config: u64, type_: c_int, name: *mut c_char) {
    unsafe {
        event_init_opts(e, config, type_, name);

        (*e).attr.disabled = 1;
        (*e).attr.exclude_kernel = 1;
        (*e).attr.exclude_hv = 1;
        (*e).attr.exclude_idle = 1;
    }
}

unsafe fn do_count_loop(
    events: *mut event,
    instructions: u64,
    overhead: u64,
    report: bool,
) -> c_int {
    let mut difference: s64;
    let expected: s64;
    let percentage: c_double;
    let mut dummy: u64 = 0;

    unsafe {
        prctl(PR_TASK_PERF_EVENTS_ENABLE);

        /* Run for 1M instructions */
        thirty_two_instruction_loop_with_ll_sc(instructions >> 5, &mut dummy);

        prctl(PR_TASK_PERF_EVENTS_DISABLE);

        event_read(events.add(0));
        event_read(events.add(1));
        event_read(events.add(2));

        expected = (instructions + overhead + ((*events.add(2)).result.value * 10)) as s64;
        difference = (*events.add(0)).result.value as s64 - expected;
        percentage = difference as c_double / (*events.add(0)).result.value as c_double * 100.0;

        if report {
            printf(c"-----\n".as_ptr());
            event_report(events.add(0));
            event_report(events.add(1));
            event_report(events.add(2));

            printf(
                c"Looped for %llu instructions, overhead %llu\n".as_ptr(),
                instructions,
                overhead,
            );
            printf(c"Expected %llu\n".as_ptr(), expected as u64);
            printf(c"Actual   %llu\n".as_ptr(), (*events.add(0)).result.value);
            printf(c"Delta    %lld, %f%%\n".as_ptr(), difference, percentage);
        }

        event_reset(events.add(0));
        event_reset(events.add(1));
        event_reset(events.add(2));

        if difference < 0 {
            difference = -difference;
        }

        /* Tolerate a difference below 0.0001 % */
        difference *= 10000 * 100;
        if difference as u64 / (*events.add(0)).result.value != 0 {
            return -1;
        }

        return 0;
    }
}

/* Count how many instructions it takes to do a null loop */
unsafe fn determine_overhead(events: *mut event) -> u64 {
    let mut current: u64;
    let mut overhead: u64;
    let mut i: c_int;

    unsafe {
        do_count_loop(events, 0, 0, false);
        overhead = (*events.add(0)).result.value;

        i = 0;
        while i < 100 {
            do_count_loop(events, 0, 0, false);
            current = (*events.add(0)).result.value;
            if current < overhead {
                printf(
                    c"Replacing overhead %llu with %llu\n".as_ptr(),
                    overhead,
                    current,
                );
                overhead = current;
            }
            i += 1;
        }

        return overhead;
    }
}

const PM_MRK_STCX_FAIL: u64 = 0x03e158;
const PM_STCX_FAIL: u64 = 0x01e058;

unsafe extern "C" fn test_body() -> c_int {
    let mut events: [event; 3] = unsafe { core::mem::zeroed() };
    let overhead: u64;

    unsafe {
        // The STCX_FAIL event we use works on Power8 or later
        SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));

        setup_event(
            &mut events[0],
            PERF_COUNT_HW_INSTRUCTIONS,
            PERF_TYPE_HARDWARE,
            c"instructions".as_ptr() as *mut c_char,
        );
        setup_event(
            &mut events[1],
            PERF_COUNT_HW_CPU_CYCLES,
            PERF_TYPE_HARDWARE,
            c"cycles".as_ptr() as *mut c_char,
        );
        setup_event(
            &mut events[2],
            PM_STCX_FAIL,
            PERF_TYPE_RAW,
            c"stcx_fail".as_ptr() as *mut c_char,
        );

        if event_open(&mut events[0]) != 0 {
            perror(c"perf_event_open".as_ptr());
            return -1;
        }

        if event_open_with_group(&mut events[1], events[0].fd) != 0 {
            perror(c"perf_event_open".as_ptr());
            return -1;
        }

        if event_open_with_group(&mut events[2], events[0].fd) != 0 {
            perror(c"perf_event_open".as_ptr());
            return -1;
        }

        overhead = determine_overhead(events.as_mut_ptr());
        printf(c"Overhead of null loop: %llu instructions\n".as_ptr(), overhead);

        /* Run for 1Mi instructions */
        FAIL_IF!(do_count_loop(events.as_mut_ptr(), 1000000, overhead, true) != 0);

        /* Run for 10Mi instructions */
        FAIL_IF!(do_count_loop(events.as_mut_ptr(), 10000000, overhead, true) != 0);

        /* Run for 100Mi instructions */
        FAIL_IF!(do_count_loop(events.as_mut_ptr(), 100000000, overhead, true) != 0);

        /* Run for 1Bi instructions */
        FAIL_IF!(do_count_loop(events.as_mut_ptr(), 1000000000, overhead, true) != 0);

        /* Run for 16Bi instructions */
        FAIL_IF!(do_count_loop(events.as_mut_ptr(), 16000000000, overhead, true) != 0);

        event_close(&mut events[0]);
        event_close(&mut events[1]);

        return 0;
    }
}

unsafe extern "C" fn count_ll_sc() -> c_int {
    unsafe { return eat_cpu(test_body); }
}

pub unsafe fn main() -> c_int {
    unsafe { return test_harness(count_ll_sc, c"count_ll_sc".as_ptr()); }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
