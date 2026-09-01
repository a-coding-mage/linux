// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

type u64 = c_ulonglong;

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_regs_intr: u64,
}

extern "C" {
    static platform_extended_mask: u64;

    fn thirty_two_instruction_loop(loops: c_int);
    fn check_pvr_for_sampling_tests() -> c_int;
    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_mmcr0_pmcjce(value: u64, width: c_int) -> c_int;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
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
            return 0;
        }
    };
}

/*
 * A perf sampling test for mmcr0
 * field: pmcjce
 */
unsafe extern "C" fn mmcr0_pmcjce() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut intr_regs: *mut u64;

    /* Check for platform support for the test */
    SKIP_IF!(check_pvr_for_sampling_tests() != 0);

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, 0x500fa);
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF!(event_open(&mut event) != 0);
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event) != 0);

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    FAIL_IF!(event_disable(&mut event) != 0);

    /* Check for sample count */
    FAIL_IF!(collect_samples(event.mmap_buffer) == 0);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null());

    /* Verify that pmcjce field is set in MMCR0 */
    FAIL_IF!(
        get_mmcr0_pmcjce(
            get_reg_value(intr_regs, b"MMCR0\0".as_ptr() as *const c_char),
            5,
        ) == 0
    );

    event_close(&mut event);
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    return test_harness(mmcr0_pmcjce, b"mmcr0_pmcjce\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
