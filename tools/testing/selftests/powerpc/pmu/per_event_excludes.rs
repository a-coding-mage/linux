// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong};

// C dependencies:
// #define _GNU_SOURCE
// #include <elf.h>
// #include <limits.h>
// #include <stdio.h>
// #include <stdbool.h>
// #include <string.h>
// #include <sys/prctl.h>
// #include "event.h"
// #include "lib.h"
// #include "utils.h"

/*
 * Test that per-event excludes work.
 */

const INT_MAX: c_int = 2147483647;

extern "C" {
    static PPC_FEATURE2_ARCH_2_07: c_ulong;
    static PERF_COUNT_HW_INSTRUCTIONS: u64;
    static PERF_TYPE_HARDWARE: u32;
    static PR_TASK_PERF_EVENTS_ENABLE: c_int;
    static PR_TASK_PERF_EVENTS_DISABLE: c_int;

    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn event_init_opts(e: *mut event, config: u64, type_: u32, name: *const c_char);
    fn event_open(e: *mut event) -> c_int;
    fn event_open_with_group(e: *mut event, group_fd: c_int) -> c_int;
    fn event_read(e: *mut event) -> c_int;
    fn event_report(e: *mut event);
    fn event_close(e: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub disabled: u64,
    pub exclude_user: u64,
    pub exclude_kernel: u64,
    pub exclude_hv: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_result {
    pub value: u64,
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub result: event_result,
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
            return 0;
        }
    };
}

unsafe extern "C" fn per_event_excludes() -> c_int {
    let mut events: [event; 4] = core::mem::zeroed();
    let mut e: *mut event;
    let mut i: c_int;

    SKIP_IF!(have_hwcap2(PPC_FEATURE2_ARCH_2_07) == 0);

    /*
     * We need to create the events disabled, otherwise the running/enabled
     * counts don't match up.
     */
    e = &mut events[0];
    event_init_opts(
        e,
        PERF_COUNT_HW_INSTRUCTIONS,
        PERF_TYPE_HARDWARE,
        b"instructions\0".as_ptr() as *const c_char,
    );
    (*e).attr.disabled = 1;

    e = &mut events[1];
    event_init_opts(
        e,
        PERF_COUNT_HW_INSTRUCTIONS,
        PERF_TYPE_HARDWARE,
        b"instructions(k)\0".as_ptr() as *const c_char,
    );
    (*e).attr.disabled = 1;
    (*e).attr.exclude_user = 1;
    (*e).attr.exclude_hv = 1;

    e = &mut events[2];
    event_init_opts(
        e,
        PERF_COUNT_HW_INSTRUCTIONS,
        PERF_TYPE_HARDWARE,
        b"instructions(h)\0".as_ptr() as *const c_char,
    );
    (*e).attr.disabled = 1;
    (*e).attr.exclude_user = 1;
    (*e).attr.exclude_kernel = 1;

    e = &mut events[3];
    event_init_opts(
        e,
        PERF_COUNT_HW_INSTRUCTIONS,
        PERF_TYPE_HARDWARE,
        b"instructions(u)\0".as_ptr() as *const c_char,
    );
    (*e).attr.disabled = 1;
    (*e).attr.exclude_hv = 1;
    (*e).attr.exclude_kernel = 1;

    FAIL_IF!(event_open(&mut events[0]) != 0);

    /*
     * The open here will fail if we don't have per event exclude support,
     * because the second event has an incompatible set of exclude settings
     * and we're asking for the events to be in a group.
     */
    i = 1;
    while i < 4 {
        FAIL_IF!(event_open_with_group(&mut events[i as usize], events[0].fd) != 0);
        i += 1;
    }

    /*
     * Even though the above will fail without per-event excludes we keep
     * testing in order to be thorough.
     */
    prctl(PR_TASK_PERF_EVENTS_ENABLE);

    /* Spin for a while */
    i = 0;
    while i < INT_MAX {
        asm!("", options(nostack, preserves_flags));
        i = i.wrapping_add(1);
    }

    prctl(PR_TASK_PERF_EVENTS_DISABLE);

    i = 0;
    while i < 4 {
        FAIL_IF!(event_read(&mut events[i as usize]) != 0);
        event_report(&mut events[i as usize]);
        i += 1;
    }

    /*
     * We should see that all events have enabled == running. That
     * shows that they were all on the PMU at once.
     */
    i = 0;
    while i < 4 {
        FAIL_IF!(events[i as usize].result.running != events[i as usize].result.enabled);
        i += 1;
    }

    /*
     * We can also check that the result for instructions is >= all the
     * other counts. That's because it is counting all instructions while
     * the others are counting a subset.
     */
    i = 1;
    while i < 4 {
        FAIL_IF!(events[0].result.value < events[i as usize].result.value);
        i += 1;
    }

    i = 0;
    while i < 4 {
        event_close(&mut events[i as usize]);
        i += 1;
    }

    0
}

pub unsafe extern "C" fn main() -> c_int {
    test_harness(
        per_event_excludes,
        b"per_event_excludes\0".as_ptr() as *const c_char,
    )
}
