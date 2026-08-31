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

/*
 * Primary PMU event used here is PM_MRK_INST_CMPL (0x401e0)
 * Threshold event selection used is issue to complete for cycles
 * Sampling criteria is Load only sampling
 */
const EventCode: u64 = 0x35340401e0;

extern "C" {
    fn thirty_two_instruction_loop_with_ll_sc(loops: u64, ll_sc_target: *mut u64);
}

/* A perf sampling test to test mmcra fields */
fn mmcra_thresh_marked_sample() -> i32 {
    let mut event: event = unsafe { std::mem::zeroed() };
    let mut intr_regs: *mut u64;
    let mut dummy: u64 = 0;

    /* Check for platform support for the test */
    SKIP_IF!(check_pvr_for_sampling_tests());

    /* Init the event for the sampling test */
    event_init_sampling(&mut event, EventCode);
    event.attr.sample_regs_intr = platform_extended_mask;
    FAIL_IF!(event_open(&mut event));
    event.mmap_buffer = event_sample_buf_mmap(event.fd, 1);

    FAIL_IF!(event_enable(&mut event));

    /* workload to make the event overflow */
    unsafe {
        thirty_two_instruction_loop_with_ll_sc(1000000, &mut dummy);
    }

    FAIL_IF!(event_disable(&mut event));

    /* Check for sample count */
    FAIL_IF!(!collect_samples(event.mmap_buffer));

    intr_regs = get_intr_regs(&mut event, event.mmap_buffer);

    /* Check for intr_regs */
    FAIL_IF!(intr_regs.is_null());

    /*
     * Verify that thresh sel/start/stop, marked, random sample
     * eligibility, sdar mode and sample mode fields match with
     * the corresponding event code fields
     */
    FAIL_IF!(
        EV_CODE_EXTRACT!(event.attr.config, thd_sel)
            != get_mmcra_thd_sel(get_reg_value(intr_regs, "MMCRA"), 4)
    );
    FAIL_IF!(
        EV_CODE_EXTRACT!(event.attr.config, thd_start)
            != get_mmcra_thd_start(get_reg_value(intr_regs, "MMCRA"), 4)
    );
    FAIL_IF!(
        EV_CODE_EXTRACT!(event.attr.config, thd_stop)
            != get_mmcra_thd_stop(get_reg_value(intr_regs, "MMCRA"), 4)
    );
    FAIL_IF!(
        EV_CODE_EXTRACT!(event.attr.config, marked)
            != get_mmcra_marked(get_reg_value(intr_regs, "MMCRA"), 4)
    );
    FAIL_IF!(
        (EV_CODE_EXTRACT!(event.attr.config, sample) >> 2)
            != get_mmcra_rand_samp_elig(get_reg_value(intr_regs, "MMCRA"), 4)
    );
    FAIL_IF!(
        (EV_CODE_EXTRACT!(event.attr.config, sample) & 0x3)
            != get_mmcra_sample_mode(get_reg_value(intr_regs, "MMCRA"), 4)
    );
    FAIL_IF!(
        EV_CODE_EXTRACT!(event.attr.config, sm)
            != get_mmcra_sm(get_reg_value(intr_regs, "MMCRA"), 4)
    );

    event_close(&mut event);
    0
}

fn main() -> i32 {
    test_harness(
        mmcra_thresh_marked_sample,
        "mmcra_thresh_marked_sample",
    )
}
