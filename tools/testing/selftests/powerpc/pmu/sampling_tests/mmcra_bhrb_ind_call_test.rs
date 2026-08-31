// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

// C dependencies: <stdio.h>, <stdlib.h>, "../event.h", "misc.h", "utils.h"

use core::ffi::c_int;
use core::mem::MaybeUninit;

unsafe extern "C" {
    fn indirect_branch_loop();

    fn check_pvr_for_sampling_tests() -> c_int;
    fn have_hwcap2(feature: u64) -> c_int;
    fn event_init_sampling(event: *mut event, event_code: u64);
    fn event_open(event: *mut event) -> c_int;
    fn event_sample_buf_mmap(fd: c_int, pages: c_int) -> *mut core::ffi::c_void;
    fn event_enable(event: *mut event) -> c_int;
    fn event_disable(event: *mut event) -> c_int;
    fn get_intr_regs(event: *mut event, mmap_buffer: *mut core::ffi::c_void) -> *mut u64;
    fn get_mmcra_ifm(mmcra: u64, shift: c_int) -> u64;
    fn get_reg_value(intr_regs: *mut u64, reg_name: *const u8) -> u64;
    fn event_close(event: *mut event);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const u8) -> c_int;

    static platform_extended_mask: u64;
}

/* Instructions */
const EventCode: u64 = 0x500fa;

/* ifm field for indirect branch mode */
const IFM_IND_BRANCH: u64 = 0x2;

/*
 * A perf sampling test for mmcra
 * field: ifm for bhrb ind_call.
 */
unsafe extern "C" fn mmcra_bhrb_ind_call_test() -> c_int {
    let mut event = MaybeUninit::<event>::uninit();
    let intr_regs: *mut u64;

    /*
     * Check for platform support for the test.
     * This test is only aplicable on ISA v3.1
     */
    SKIP_IF!(check_pvr_for_sampling_tests());
    SKIP_IF!(have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0);

    /* Init the event for the sampling test */
    event_init_sampling(event.as_mut_ptr(), EventCode);
    let mut event = event.assume_init();
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
    event.attr.branch_sample_type = PERF_SAMPLE_BRANCH_IND_CALL;
    event.attr.exclude_kernel = 1;

    FAIL_IF!(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event));

    /* workload to make the event overflow */
    indirect_branch_loop();

    FAIL_IF!(event_disable(&mut event));

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null());

    /* Verify that ifm bit is set properly in MMCRA */
    FAIL_IF!(get_mmcra_ifm(get_reg_value(intr_regs, c"MMCRA".as_ptr() as *const u8), 5) != IFM_IND_BRANCH);

    event_close(&mut event);
    0
}

pub unsafe extern "C" fn main() -> c_int {
    test_harness(
        mmcra_bhrb_ind_call_test,
        c"mmcra_bhrb_ind_call_test".as_ptr() as *const u8,
    )
}
