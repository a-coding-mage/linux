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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct event_attr {
    pub sample_regs_intr: u64,
}

#[repr(C)]
pub struct event {
    pub attr: event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

unsafe extern "C" {
    fn thirty_two_instruction_loop(loops: c_int);

    static platform_extended_mask: u64;

    fn check_pvr_for_sampling_tests() -> c_int;
    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, mmap_pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_mmcr0_fc56(value: u64, expected: c_int) -> c_int;
    fn get_mmcr0_pmc1ce(value: u64, expected: c_int) -> c_int;
    fn get_reg_value(intr_regs: *mut u64, reg_name: *const c_char) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;

    fn SKIP_IF(cond: c_int);
    fn FAIL_IF(cond: c_int);
}

/*
 * A perf sampling test for mmcr0
 * fields: fc56, pmc1ce.
 */
unsafe extern "C" fn mmcr0_fc56_pmc1ce() -> c_int {
    let mut event: event = core::mem::zeroed();
    let intr_regs: *mut u64;

    /* Check for platform support for the test */
    SKIP_IF(check_pvr_for_sampling_tests());

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, 0x1001e);
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF(event_enable(&mut event));

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    FAIL_IF(event_disable(&mut event));

    /* Check for sample count */
    FAIL_IF((collect_samples(event.mmap_buffer) == 0) as c_int);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF(intr_regs.is_null() as c_int);

    /* Verify that fc56, pmc1ce fields are set in MMCR0 */
    FAIL_IF((get_mmcr0_fc56(get_reg_value(intr_regs, c"MMCR0".as_ptr()), 1) == 0) as c_int);
    FAIL_IF((get_mmcr0_pmc1ce(get_reg_value(intr_regs, c"MMCR0".as_ptr()), 1) == 0) as c_int);

    event_close(&mut event);
    0
}

pub unsafe fn main() -> c_int {
    test_harness(mmcr0_fc56_pmc1ce, c"mmcr0_fc56_pmc1ce".as_ptr())
}
