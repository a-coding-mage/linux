// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

use core::ffi::{c_char, c_int, c_ulonglong, c_void};

type u64 = c_ulonglong;

const EventCode: u64 = 0x500fa;
const PPC_FEATURE2_ARCH_3_1: u64 = 0x80000000;

#[repr(C)]
pub struct perf_event_attr {
    pub sample_regs_intr: u64,
    pub exclude_kernel: u64,
}

#[repr(C)]
pub struct event {
    pub attr: perf_event_attr,
    pub fd: c_int,
    pub mmap_buffer: *mut c_void,
}

unsafe extern "C" {
    static platform_extended_mask: u64;

    fn thirty_two_instruction_loop(loops: c_int);

    fn check_pvr_for_sampling_tests() -> c_int;
    fn have_hwcap2(feature: u64) -> c_int;

    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut c_void) -> *mut u64;
    fn get_reg_value(intr_regs: *mut u64, name: *const c_char) -> u64;
    fn get_mmcra_bhrb_disable(mmcra: u64, bit: c_int) -> c_int;
    fn event_close(event: *mut event);

    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return 0;
        }
    };
}

macro_rules! FAIL_IF {
    ($condition:expr) => {
        if $condition != 0 {
            return 1;
        }
    };
}

/*
 * A perf sampling test for mmcra
 * field: bhrb_disable.
 */
unsafe extern "C" fn mmcra_bhrb_disable_no_branch_test() -> c_int {
    let mut event: event = core::mem::zeroed();
    let intr_regs: *mut u64;

    /*
     * Check for platform support for the test.
     * This test is only aplicable on ISA v3.1
     */
    SKIP_IF!(check_pvr_for_sampling_tests());
    SKIP_IF!((have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0) as c_int);

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.exclude_kernel = 1;

    FAIL_IF!(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event));

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    FAIL_IF!(event_disable(&mut event));

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null() as c_int);

    /* Verify that bhrb_disable bit is set in MMCRA for non-branch samples */
    FAIL_IF!(
        (get_mmcra_bhrb_disable(get_reg_value(intr_regs, c"MMCRA".as_ptr()), 5) == 0) as c_int
    );

    event_close(&mut event);
    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(
            mmcra_bhrb_disable_no_branch_test,
            c"mmcra_bhrb_disable_no_branch_test".as_ptr(),
        ));
    }
}
