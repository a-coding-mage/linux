// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

// C includes translated as external dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};

type u64 = u64;

#[repr(C)]
pub struct perf_event_attr {
    pub sample_type: u64,
    pub sample_regs_intr: u64,
    pub branch_sample_type: u64,
    pub exclude_kernel: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

extern "C" {
    fn thirty_two_instruction_loop(loops: c_int);

    static platform_extended_mask: u64;
    static PERF_SAMPLE_BRANCH_STACK: u64;
    static PERF_SAMPLE_BRANCH_ANY: u64;

    fn check_pvr_for_sampling_tests() -> c_int;
    fn event_init_sampling(event: *mut event, event_code: c_int);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, mmap_pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_mmcra_ifm(mmcra: u64, shift: c_int) -> u64;
    fn get_reg_value(intr_regs: *mut u64, reg: *const c_char) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/* Instructions */
const EventCode: c_int = 0x500fa;

/* ifm field for any branch mode */
const IFM_ANY_BRANCH: u64 = 0x0;

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
            return 4;
        }
    };
}

/*
 * A perf sampling test for mmcra
 * field: ifm for bhrb any call.
 */
unsafe extern "C" fn mmcra_bhrb_any_test() -> c_int {
    let mut event: event = core::mem::zeroed();
    let intr_regs: *mut u64;

    /* Check for platform support for the test */
    SKIP_IF!(check_pvr_for_sampling_tests() != 0);

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
    event.attr.branch_sample_type = PERF_SAMPLE_BRANCH_ANY;
    event.attr.exclude_kernel = 1;

    FAIL_IF!(event_open(&mut event) != 0);
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event) != 0);

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    FAIL_IF!(event_disable(&mut event) != 0);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null());

    /* Verify that ifm bit is set properly in MMCRA */
    FAIL_IF!(get_mmcra_ifm(get_reg_value(intr_regs, b"MMCRA\0".as_ptr() as *const c_char), 5) != IFM_ANY_BRANCH);

    event_close(&mut event);
    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            mmcra_bhrb_any_test,
            b"mmcra_bhrb_any_test\0".as_ptr() as *const c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
