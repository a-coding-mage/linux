// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

/*
 * C dependencies removed from executable Rust:
 * #include <stdio.h>
 * #include <stdlib.h>
 * #include "../event.h"
 * #include "misc.h"
 * #include "utils.h"
 */

use core::ffi::{c_char, c_int, c_void};

type u64 = u64;

/*
 * Primary PMU event used here is PM_MRK_INST_CMPL (0x401e0)
 * Threshold event selection used is issue to complete for cycles
 * Sampling criteria is Load only sampling
 */
const p9_EventCode: u64 = 0x13E35340401e0;
const p10_EventCode: u64 = 0x35340401e0;

// Types and fields are supplied by the translated equivalents of the included
// C headers.
#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

#[repr(C)]
pub struct perf_event_attr {
    pub config1: u64,
    pub sample_regs_intr: u64,
}

const PPC_FEATURE2_ARCH_3_1: u64 = 0;

extern "C" {
    static platform_extended_mask: u64;

    fn check_pvr_for_sampling_tests() -> c_int;
    fn check_for_compat_mode() -> c_int;
    fn have_hwcap2(feature: u64) -> c_int;
    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn collect_samples(mmap_buffer: *mut c_void) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_thresh_cmp_val(event: event) -> u64;
    fn get_mmcra_thd_cmp(mmcra: u64, shift: c_int) -> u64;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn thirty_two_instruction_loop_with_ll_sc(loops: u64, ll_sc_target: *mut u64);
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond != 0 {
            return 0;
        }
    };
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond != 0 {
            return 1;
        }
    };
}

/* A perf sampling test to test mmcra fields */
unsafe extern "C" fn mmcra_thresh_cmp() -> c_int {
    let mut event: event = core::mem::zeroed();
    let mut intr_regs: *mut u64;
    let mut dummy: u64 = 0;

    /* Check for platform support for the test */
    SKIP_IF!(check_pvr_for_sampling_tests());

    /* Skip for comapt mode */
    SKIP_IF!(check_for_compat_mode());

    /* Init the event for the sampling test */
    if have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0 {
        event_init_sampling(&mut event, p9_EventCode);
    } else {
        event_init_sampling(&mut event, p10_EventCode);
        event.attr.config1 = 1000;
    }

    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF!(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event));

    /* workload to make the event overflow */
    thirty_two_instruction_loop_with_ll_sc(1000000, &mut dummy);

    FAIL_IF!(event_disable(&mut event));

    /* Check for sample count */
    FAIL_IF!((collect_samples(event.mmap_buffer) == 0) as c_int);

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null() as c_int);

    /* Verify that thresh cmp match with the corresponding event code fields */
    FAIL_IF!((get_thresh_cmp_val(event) !=
            get_mmcra_thd_cmp(get_reg_value(intr_regs, b"MMCRA\0".as_ptr() as *const c_char), 4)) as c_int);

    event_close(&mut event);
    return 0;
}

pub unsafe fn main() -> c_int {
    FAIL_IF!(test_harness(mmcra_thresh_cmp, b"mmcra_thresh_cmp\0".as_ptr() as *const c_char));
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
