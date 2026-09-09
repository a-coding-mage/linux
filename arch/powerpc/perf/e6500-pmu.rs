// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter support for e6500 family processors.
 *
 * Author: Priyanka Jain, Priyanka.Jain@freescale.com
 * Based on e500-pmu.c
 * Copyright 2013 Freescale Semiconductor, Inc.
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 */

/* Dependencies supplied by the surrounding kernel translation unit. */

/*
 * Map of generic hardware event types to hardware events
 * Zero if unsupported
 */
static mut e6500_generic_events: [i32; 5] = [
    1,  // PERF_COUNT_HW_CPU_CYCLES
    2,  // PERF_COUNT_HW_INSTRUCTIONS
    221, // PERF_COUNT_HW_CACHE_MISSES
    12, // PERF_COUNT_HW_BRANCH_INSTRUCTIONS
    15, // PERF_COUNT_HW_BRANCH_MISSES
];

/* C(x) = PERF_COUNT_HW_CACHE_##x */

/*
 * Table of generalized cache-related events.
 * 0 means not supported, -1 means nonsensical, other values
 * are event codes.
 */
static mut e6500_cache_events: [[[i32; 2]; 3]; 7] = [
    [
        [27, 222],
        [28, 223],
        [29, 0],
    ],
    [
        [2, 254],
        [-1, -1],
        [37, 0],
    ],
    /* Assuming LL means L2, it's not a good match for this model.
     * It does not have separate read/write events (but it does have
     * separate instruction/data events).
     */
    [[0, 0]; 3],
    /*
     * There are data/instruction MMU misses, but that's a miss on
     * the chip's internal level-one TLB which is probably not
     * what the user wants.  Instead, unified level-two TLB misses
     * are reported here.
     */
    [
        [26, 66],
        [-1, -1],
        [-1, -1],
    ],
    [
        [12, 15],
        [-1, -1],
        [-1, -1],
    ],
    [[-1, -1]; 3],
];

static mut num_events: i32 = 512;

/* Upper half of event id is PMLCb, for threshold events */
unsafe fn e6500_xlate_event(event_id: u64) -> u64 {
    let event_low = event_id as u32;
    if event_low >= num_events as u32
        || (event_id & (FSL_EMB_EVENT_THRESHMUL | FSL_EMB_EVENT_THRESH)) != 0
    {
        return 0;
    }

    FSL_EMB_EVENT_VALID
}

static mut e6500_pmu: fsl_emb_pmu = fsl_emb_pmu {
    name: "e6500 family",
    n_counter: 6,
    n_restricted: 0,
    xlate_event: Some(e6500_xlate_event),
    n_generic: e6500_generic_events.len(),
    generic_events: e6500_generic_events.as_ptr(),
    cache_events: &e6500_cache_events as *const _,
};

unsafe fn init_e6500_pmu() -> i32 {
    let pvr: u32 = mfspr(SPRN_PVR);

    if PVR_VER(pvr) != PVR_VER_E6500 {
        return -ENODEV;
    }

    register_fsl_emb_pmu(&raw mut e6500_pmu)
}

early_initcall!(init_e6500_pmu);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
