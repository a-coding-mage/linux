// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter support for e500 family processors.
 *
 * Copyright 2008-2009 Paul Mackerras, IBM Corporation.
 * Copyright 2010 Freescale Semiconductor, Inc.
 */

// Linux and architecture symbols referenced by this translation are supplied
// by the surrounding kernel bindings.

static mut E500_GENERIC_EVENTS: [i32; PERF_COUNT_HW_MAX as usize] = {
    let mut events = [0; PERF_COUNT_HW_MAX as usize];
    events[PERF_COUNT_HW_CPU_CYCLES as usize] = 1;
    events[PERF_COUNT_HW_INSTRUCTIONS as usize] = 2;
    events[PERF_COUNT_HW_CACHE_MISSES as usize] = 41; // Data L1 cache reloads
    events[PERF_COUNT_HW_BRANCH_INSTRUCTIONS as usize] = 12;
    events[PERF_COUNT_HW_BRANCH_MISSES as usize] = 15;
    events[PERF_COUNT_HW_STALLED_CYCLES_FRONTEND as usize] = 18;
    events[PERF_COUNT_HW_STALLED_CYCLES_BACKEND as usize] = 19;
    events
};

/*
 * Table of generalized cache-related events.
 * 0 means not supported, -1 means nonsensical, other values
 * are event codes.
 */
static mut E500_CACHE_EVENTS: [[[i32; C_RESULT_MAX as usize]; C_OP_MAX as usize]; C_MAX as usize] = {
    let mut events = [[[0; C_RESULT_MAX as usize]; C_OP_MAX as usize]; C_MAX as usize];

    /* D-cache misses are not split into read/write/prefetch; use raw event 41. */
    events[C_L1D as usize][C_OP_READ as usize] = [27, 0];
    events[C_L1D as usize][C_OP_WRITE as usize] = [28, 0];
    events[C_L1D as usize][C_OP_PREFETCH as usize] = [29, 0];

    events[C_L1I as usize][C_OP_READ as usize] = [2, 60];
    events[C_L1I as usize][C_OP_WRITE as usize] = [-1, -1];
    events[C_L1I as usize][C_OP_PREFETCH as usize] = [0, 0];

    events[C_DTLB as usize][C_OP_READ as usize] = [26, 66];
    events[C_DTLB as usize][C_OP_WRITE as usize] = [-1, -1];
    events[C_DTLB as usize][C_OP_PREFETCH as usize] = [-1, -1];

    events[C_BPU as usize][C_OP_READ as usize] = [12, 15];
    events[C_BPU as usize][C_OP_WRITE as usize] = [-1, -1];
    events[C_BPU as usize][C_OP_PREFETCH as usize] = [-1, -1];

    events[C_NODE as usize][C_OP_READ as usize] = [-1, -1];
    events[C_NODE as usize][C_OP_WRITE as usize] = [-1, -1];
    events[C_NODE as usize][C_OP_PREFETCH as usize] = [-1, -1];
    events
};

static mut NUM_EVENTS: u32 = 128;

/* Upper half of event id is PMLCb, for threshold events */
unsafe fn e500_xlate_event(event_id: u64) -> u64 {
    let event_low = event_id as u32;
    let mut ret: u64;

    if event_low >= NUM_EVENTS {
        return 0;
    }

    ret = FSL_EMB_EVENT_VALID as u64;

    if event_low >= 76 && event_low <= 81 {
        ret |= FSL_EMB_EVENT_RESTRICTED as u64;
        ret |= event_id & (FSL_EMB_EVENT_THRESHMUL as u64 | FSL_EMB_EVENT_THRESH as u64);
    } else if event_id & (FSL_EMB_EVENT_THRESHMUL as u64 | FSL_EMB_EVENT_THRESH as u64) != 0 {
        /* Threshold requested on non-threshold event */
        return 0;
    }

    ret
}

static mut E500_PMU: FslEmbPmu = FslEmbPmu {
    name: b"e500 family\0".as_ptr(),
    n_counter: 4,
    n_restricted: 2,
    xlate_event: Some(e500_xlate_event),
    n_generic: E500_GENERIC_EVENTS.len(),
    generic_events: E500_GENERIC_EVENTS.as_ptr(),
    cache_events: E500_CACHE_EVENTS.as_ptr(),
};

unsafe fn init_e500_pmu() -> i32 {
    let pvr: u32 = mfspr(SPRN_PVR);

    /* ec500mc */
    if PVR_VER(pvr) == PVR_VER_E500MC || PVR_VER(pvr) == PVR_VER_E5500 {
        NUM_EVENTS = 256;
    }
    /* e500 */
    else if PVR_VER(pvr) != PVR_VER_E500V1 && PVR_VER(pvr) != PVR_VER_E500V2 {
        return -ENODEV;
    }

    register_fsl_emb_pmu(&raw mut E500_PMU)
}

// early_initcall(init_e500_pmu);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
