// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2022, Kajol Jain, IBM Corp.
 */

// Dependencies in the original C source:
// #include <stdio.h>
// #include <stdlib.h>
// #include "../event.h"
// #include "misc.h"
// #include "utils.h"

unsafe extern "C" {
    fn thirty_two_instruction_loop_with_ll_sc(loops: u64, ll_sc_target: *mut u64);
}

/* The data cache was reloaded from local core's L3 due to a demand load */
const EventCode: u64 = 0x1340000001c040;

/*
 * A perf sampling test for mmcr3
 * fields.
 */
unsafe extern "C" fn mmcr3_src() -> i32 {
    let mut event: event = core::mem::zeroed();
    let mut intr_regs: *mut u64;
    let mut dummy: u64 = core::mem::zeroed();

    /* Check for platform support for the test */
    SKIP_IF(check_pvr_for_sampling_tests());
    SKIP_IF(have_hwcap2(PPC_FEATURE2_ARCH_3_1) == 0);

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF(event_open(&mut event) != 0);
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF(event_enable(&mut event) != 0);

    /* workload to make event overflow */
    thirty_two_instruction_loop_with_ll_sc(1000000, &mut dummy);

    FAIL_IF(event_disable(&mut event) != 0);

    /* Check for sample count */
    FAIL_IF(!collect_samples(event.mmap_buffer));

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF(intr_regs.is_null());

    /*
     * Verify that src field of MMCR3 match with
     * corresponding event code field
     */
    FAIL_IF(
        EV_CODE_EXTRACT(event.attr.config, mmcr3_src)
            != get_mmcr3_src(get_reg_value(intr_regs, c"MMCR3".as_ptr()), 1),
    );

    event_close(&mut event);
    return 0;
}

pub unsafe fn main() -> i32 {
    return test_harness(Some(mmcr3_src), c"mmcr3_src".as_ptr());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
