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

use core::ffi::{c_char, c_int, c_ulonglong, c_void};
use core::mem::MaybeUninit;

#[repr(C)]
pub struct perf_event_attr {
    pub sample_regs_intr: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

unsafe extern "C" {
    fn thirty_two_instruction_loop(loops: c_int);

    static platform_extended_mask: u64;

    fn check_pvr_for_sampling_tests() -> c_int;
    fn event_init_sampling(event: *mut event, event_code: c_ulonglong);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, mmap_pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_reg_value(intr_regs: *mut u64, reg: *const c_char) -> u64;
    fn get_mmcr0_pmae(value: u64, shift: c_int) -> c_int;
    fn get_mmcr0_pmao(value: u64, shift: c_int) -> c_int;
    fn event_close(event: *mut event);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

const MAGIC_SKIP_RETURN_VALUE: c_int = 4;

/*
 * A perf sampling test for mmcr0
 * fields : pmae, pmao.
 */
unsafe extern "C" fn mmcr0_exceptionbits() -> c_int {
    let mut event_storage = MaybeUninit::<event>::uninit();
    let event = event_storage.as_mut_ptr();
    let intr_regs: *mut u64;

    /* Check for platform support for the test */
    if check_pvr_for_sampling_tests() != 0 {
        return MAGIC_SKIP_RETURN_VALUE;
    }

    /* Init the event for the sampling test */
    event_init_sampling(event, 0x500fa);
    (*event).attr.sample_regs_intr = platform_extended_mask;
    if event_open(event) != 0 {
        return 1;
    }
    (*event).mmap_buffer = event_sample_buf_mmap((*event).fd, 1);

    if event_enable(event) != 0 {
        return 1;
    }

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    if event_disable(event) != 0 {
        return 1;
    }

    /* Check for sample count */
    if collect_samples((*event).mmap_buffer) == 0 {
        return 1;
    }

    intr_regs = get_intr_regs(event, (*event).mmap_buffer);

    /* Check for intr_regs */
    if intr_regs.is_null() {
        return 1;
    }

    /* Verify that pmae is cleared and pmao is set in MMCR0 */
    if get_mmcr0_pmae(get_reg_value(intr_regs, c"MMCR0".as_ptr()), 5) != 0 {
        return 1;
    }
    if get_mmcr0_pmao(get_reg_value(intr_regs, c"MMCR0".as_ptr()), 5) == 0 {
        return 1;
    }

    event_close(event);
    0
}

fn main() -> c_int {
    unsafe { test_harness(mmcr0_exceptionbits, c"mmcr0_exceptionbits".as_ptr()) }
}
