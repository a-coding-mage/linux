// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

// C dependencies: <stdio.h>, <stdlib.h>, "../event.h", "misc.h", "utils.h"

unsafe extern "C" {
    fn thirty_two_instruction_loop(loops: i32);
}

/* Instructions */
const EventCode: u64 = 0x500fa;

/*
 * A perf sampling test for mmcra
 * field: bhrb_disable.
 */
unsafe fn mmcra_bhrb_disable_test() -> i32 {
    let mut event: event = unsafe { ::core::mem::zeroed() };
    let mut intr_regs: *mut u64;

    /*
     * Check for platform support for the test.
     * This test is only aplicable on ISA v3.1
     */
    SKIP_IF!(unsafe { check_pvr_for_sampling_tests() });
    SKIP_IF!(!unsafe { have_hwcap2(PPC_FEATURE2_ARCH_3_1) });

    /* Init the event for the sampling test */
    unsafe { event_init_sampling(&mut event, EventCode) };
    event.attr.sample_regs_intr = platform_extended_mask;
    event.attr.sample_type |= PERF_SAMPLE_BRANCH_STACK;
    event.attr.branch_sample_type = PERF_SAMPLE_BRANCH_ANY;
    event.attr.exclude_kernel = 1;

    FAIL_IF!(unsafe { event_open(&mut event) });
    event.mmap_buffer = unsafe { event_sample_buf_mmap(event.fd, 1) };

    FAIL_IF!(unsafe { event_enable(&mut event) });

    /* workload to make the event overflow */
    unsafe { thirty_two_instruction_loop(10000) };

    FAIL_IF!(unsafe { event_disable(&mut event) });

    intr_regs = unsafe { get_intr_regs(&mut event, event.mmap_buffer) };

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null());

    /* Verify that bhrb_disable bit is set in MMCRA */
    FAIL_IF!(unsafe { get_mmcra_bhrb_disable(get_reg_value(intr_regs, c"MMCRA".as_ptr()), 5) });

    unsafe { event_close(&mut event) };
    0
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    unsafe { test_harness(Some(mmcra_bhrb_disable_test), c"mmcra_bhrb_disable_test".as_ptr()) }
}
