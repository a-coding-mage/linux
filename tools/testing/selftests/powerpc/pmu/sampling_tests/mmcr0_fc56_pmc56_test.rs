// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Athira Rajeev, IBM Corp.
 */

// C dependencies: <stdio.h>, <stdlib.h>, "../event.h", "misc.h", "utils.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type u64 = u64;

#[repr(C)]
pub struct event_attr {
    pub sample_regs_intr: u64,
}

#[repr(C)]
pub struct event {
    pub attr: event_attr,
    pub fd: i32,
    pub mmap_buffer: *mut core::ffi::c_void,
}

extern "C" {
    static platform_extended_mask: u64;

    fn thirty_two_instruction_loop(loops: i32);
    fn check_pvr_for_sampling_tests() -> i32;
    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> i32;
    fn event_sample_buf_mmap(fd: i32, pages: i32) -> *mut core::ffi::c_void;
    fn event_enable(event: *mut event) -> i32;
    fn event_disable(event: *mut event) -> i32;
    fn collect_samples(mmap_buffer: *mut core::ffi::c_void) -> i32;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut core::ffi::c_void) -> *mut u64;
    fn get_reg_value(intr_regs: *mut u64, name: *const core::ffi::c_char) -> u64;
    fn get_mmcr0_fc56(value: u64, pmc: i32) -> i32;
    fn event_close(event: *mut event);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> i32,
        name: *const core::ffi::c_char,
    ) -> i32;
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
            return 4;
        }
    };
}

/*
 * A perf sampling test for mmcr0
 * fields: fc56_pmc56
 */
unsafe extern "C" fn mmcr0_fc56_pmc56() -> i32 {
    let mut event: event = core::mem::zeroed();
    let intr_regs: *mut u64;

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

    /* Verify that fc56 is not set in MMCR0 when using PMC5 */
    FAIL_IF!(get_mmcr0_fc56(get_reg_value(intr_regs, b"MMCR0\0".as_ptr() as *const _), 5) != 0);

    event_close(&mut event);
    return 0;
}

pub unsafe fn main() -> i32 {
    return test_harness(mmcr0_fc56_pmc56, b"mmcr0_fc56_pmc56\0".as_ptr() as *const _);
}
