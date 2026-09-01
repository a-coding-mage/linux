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

extern "C" {
    fn thirty_two_instruction_loop(loops: ::std::os::raw::c_int);

    fn check_pvr_for_sampling_tests() -> ::std::os::raw::c_int;
    fn have_hwcap2(feature: u64) -> ::std::os::raw::c_int;
    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> ::std::os::raw::c_int;
    fn event_sample_buf_mmap(fd: ::std::os::raw::c_int, mmap_pages: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
    fn event_enable(event: *mut event) -> ::std::os::raw::c_int;
    fn event_disable(event: *mut event) -> ::std::os::raw::c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut ::std::os::raw::c_void) -> *mut u64;
    fn get_mmcra_ifm(value: u64, shift: ::std::os::raw::c_int) -> u64;
    fn get_reg_value(intr_regs: *mut u64, reg: *const ::std::os::raw::c_char) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(
        test: Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    static platform_extended_mask: u64;
}

// Types, constants, and helper macros are supplied by the translated headers.
extern "C" {
    type event;
}

extern "C" {
    static PPC_FEATURE2_ARCH_3_1: u64;
    static PERF_SAMPLE_BRANCH_STACK: u64;
    static PERF_SAMPLE_BRANCH_COND: u64;
}

extern "C" {
    fn SKIP_IF(condition: ::std::os::raw::c_int);
    fn FAIL_IF(condition: ::std::os::raw::c_int);
}

/* Instructions */
const EventCode: u64 = 0x500fa;

/* ifm field for conditional branch mode */
const IFM_COND_BRANCH: u64 = 0x3;

/*
 * A perf sampling test for mmcra
 * field: ifm for bhrb cond call.
 */
unsafe extern "C" fn mmcra_bhrb_cond_test() -> ::std::os::raw::c_int {
    let mut event: event = ::std::mem::zeroed();
    let mut intr_regs: *mut u64;

    /*
     * Check for platform support for the test.
     * This test is only aplicable on ISA v3.1
     */
    SKIP_IF(check_pvr_for_sampling_tests());
    SKIP_IF((have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0) as ::std::os::raw::c_int);

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
    event.attr.branch_sample_type = PERF_SAMPLE_BRANCH_COND;
    event.attr.exclude_kernel = 1;

    FAIL_IF(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF(event_enable(&mut event));

    /* workload to make the event overflow */
    thirty_two_instruction_loop(10000);

    FAIL_IF(event_disable(&mut event));

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF(intr_regs.is_null() as ::std::os::raw::c_int);

    /* Verify that ifm bit is set properly in MMCRA */
    FAIL_IF(
        (get_mmcra_ifm(get_reg_value(intr_regs, b"MMCRA\0".as_ptr() as *const ::std::os::raw::c_char), 5)
            != IFM_COND_BRANCH) as ::std::os::raw::c_int,
    );

    event_close(&mut event);
    0
}

fn main() {
    unsafe {
        ::std::process::exit(test_harness(
            Some(mmcra_bhrb_cond_test),
            b"mmcra_bhrb_cond_test\0".as_ptr() as *const ::std::os::raw::c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
