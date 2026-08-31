// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013, Michael Ellerman, IBM Corp.
 */

// C dependencies: stdio.h, stdbool.h, string.h, sys/prctl.h, event.h, utils.h, lib.h

use core::ffi::{c_char, c_double, c_int};

use crate::*;

type s64 = i64;

unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn printf(format: *const c_char, ...) -> c_int;

    fn thirty_two_instruction_loop(loops: u64);
}

unsafe fn setup_event(e: *mut event, config: u64, name: *mut c_char) {
    unsafe {
        event_init_opts(e, config, PERF_TYPE_HARDWARE, name);

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

    unsafe {
        prctl(PR_TASK_PERF_EVENTS_ENABLE as c_int);

        /* Run for 1M instructions */
        thirty_two_instruction_loop(instructions >> 5);

        prctl(PR_TASK_PERF_EVENTS_DISABLE as c_int);

        event_read(events.add(0));
        event_read(events.add(1));

        expected = instructions.wrapping_add(overhead) as s64;
        difference = (*events.add(0)).result.value as s64 - expected;
        percentage = difference as c_double / (*events.add(0)).result.value as c_double * 100.0;

        if report {
            event_report(events.add(0));
            event_report(events.add(1));

            printf(
                b"Looped for %llu instructions, overhead %llu\n\0".as_ptr() as *const c_char,
                instructions,
                overhead,
            );
            printf(
                b"Expected %llu\n\0".as_ptr() as *const c_char,
                expected,
            );
            printf(
                b"Actual   %llu\n\0".as_ptr() as *const c_char,
                (*events.add(0)).result.value,
            );
            printf(
                b"Delta    %lld, %f%%\n\0".as_ptr() as *const c_char,
                difference,
                percentage,
            );
        }

        event_reset(events.add(0));
        event_reset(events.add(1));

        if difference < 0 {
            difference = -difference;
        }

        /* Tolerate a difference below 0.0001 % */
        difference *= 10000 * 100;
        if difference / (*events.add(0)).result.value as s64 != 0 {
            return -1;
        }
    }

    0
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
                    b"Replacing overhead %llu with %llu\n\0".as_ptr() as *const c_char,
                    overhead,
                    current,
                );
                overhead = current;
            }
            i += 1;
        }
    }

    overhead
}

unsafe extern "C" fn test_body() -> c_int {
    let mut events: [event; 2] = unsafe { core::mem::zeroed() };
    let overhead: u64;

    unsafe {
        setup_event(
            &mut events[0],
            PERF_COUNT_HW_INSTRUCTIONS,
            b"instructions\0".as_ptr() as *mut c_char,
        );
        setup_event(
            &mut events[1],
            PERF_COUNT_HW_CPU_CYCLES,
            b"cycles\0".as_ptr() as *mut c_char,
        );

        if event_open(&mut events[0]) != 0 {
            perror(b"perf_event_open\0".as_ptr() as *const c_char);
            return -1;
        }

        if event_open_with_group(&mut events[1], events[0].fd) != 0 {
            perror(b"perf_event_open\0".as_ptr() as *const c_char);
            return -1;
        }

        overhead = determine_overhead(events.as_mut_ptr());
        printf(
            b"Overhead of null loop: %llu instructions\n\0".as_ptr() as *const c_char,
            overhead,
        );

        /* Run for 1Mi instructions */
        if do_count_loop(events.as_mut_ptr(), 1000000, overhead, true) != 0 {
            return 1;
        }

        /* Run for 10Mi instructions */
        if do_count_loop(events.as_mut_ptr(), 10000000, overhead, true) != 0 {
            return 1;
        }

        /* Run for 100Mi instructions */
        if do_count_loop(events.as_mut_ptr(), 100000000, overhead, true) != 0 {
            return 1;
        }

        /* Run for 1Bi instructions */
        if do_count_loop(events.as_mut_ptr(), 1000000000, overhead, true) != 0 {
            return 1;
        }

        /* Run for 16Bi instructions */
        if do_count_loop(events.as_mut_ptr(), 16000000000, overhead, true) != 0 {
            return 1;
        }

        /* Run for 64Bi instructions */
        if do_count_loop(events.as_mut_ptr(), 64000000000, overhead, true) != 0 {
            return 1;
        }

        event_close(&mut events[0]);
        event_close(&mut events[1]);
    }

    0
}

unsafe extern "C" fn count_instructions() -> c_int {
    unsafe { eat_cpu(Some(test_body)) }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { test_harness(Some(count_instructions), b"count_instructions\0".as_ptr() as *const c_char) }
}
